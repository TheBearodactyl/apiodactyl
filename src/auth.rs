use diesel::prelude::*;
use rocket::State;
use rocket::request::{FromRequest, Outcome, Request};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::db::connect_db;
use crate::errors::AuthError;
use crate::models::{ApiKey, NewApiKey};
use crate::schema::api_keys;

#[derive(Clone, Debug)]
struct CacheEntry {
    api_key: ApiKey,
    cached_at: Instant,
}

#[derive(Default)]
pub struct ApiKeyCache {
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
}

impl ApiKeyCache {
    pub fn new() -> Self {
        Self::default()
    }

    fn get(&self, key_hash: &str) -> Option<ApiKey> {
        let cache = self.cache.read().ok()?;
        let entry = cache.get(key_hash)?;

        if entry.cached_at.elapsed() < Duration::from_secs(300) {
            Some(entry.api_key.clone())
        } else {
            None
        }
    }

    fn insert(&self, key_hash: String, api_key: ApiKey) {
        if let Ok(mut cache) = self.cache.write() {
            cache.insert(
                key_hash,
                CacheEntry {
                    api_key,
                    cached_at: Instant::now(),
                },
            );
        }
    }

    fn remove(&self, key_hash: &str) {
        if let Ok(mut cache) = self.cache.write() {
            cache.remove(key_hash);
        }
    }

    pub fn cleanup_expired(&self) {
        if let Ok(mut cache) = self.cache.write() {
            cache.retain(|_, entry| entry.cached_at.elapsed() < Duration::from_secs(300));
        }
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub api_key: ApiKey,
}

impl User {
    pub fn id(&self) -> i32 {
        self.api_key.id
    }

    pub fn is_admin(&self) -> bool {
        self.api_key.is_admin
    }

    pub fn created_at(&self) -> chrono::NaiveDateTime {
        self.api_key.created_at
    }

    pub fn last_used_at(&self) -> Option<chrono::NaiveDateTime> {
        self.api_key.last_used_at
    }

    pub fn require_admin(&self) -> Result<(), AuthError> {
        if self.is_admin() {
            Ok(())
        } else {
            Err(AuthError::InsufficientPermissions)
        }
    }

    pub fn as_api_key(&self) -> &ApiKey {
        &self.api_key
    }
}

pub struct AdminUser(pub User);

impl std::ops::Deref for AdminUser {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct AuthService {
    cache: ApiKeyCache,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            cache: ApiKeyCache::new(),
        }
    }

    fn hash_api_key(key: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn validate_api_key(&self, key: &str) -> Result<ApiKey, AuthError> {
        let key_hash = Self::hash_api_key(key);

        if let Some(cached_key) = self.cache.get(&key_hash) {
            return Ok(cached_key);
        }

        let mut conn = connect_db();
        let api_key = api_keys::table
            .filter(api_keys::key_hash.eq(&key_hash))
            .first::<ApiKey>(&mut conn)
            .map_err(|_| AuthError::InvalidKey)?;

        self.cache.insert(key_hash, api_key.clone());

        Ok(api_key)
    }

    pub async fn update_last_used(&self, key_id: i32) {
        tokio::task::spawn_blocking(move || {
            if let Ok(mut conn) = std::panic::catch_unwind(|| connect_db()) {
                let _ = diesel::update(api_keys::table.find(key_id))
                    .set(api_keys::last_used_at.eq(diesel::dsl::now))
                    .execute(&mut conn);
            }
        })
        .await
        .ok();
    }

    pub fn generate_api_key() -> String {
        use uuid::Uuid;
        format!("ak_{}", Uuid::new_v4().simple())
    }

    pub fn create_api_key(&self, key: &str, is_admin: bool) -> Result<ApiKey, AuthError> {
        let mut conn = connect_db();
        let key_hash = Self::hash_api_key(key);

        let new_key = NewApiKey {
            key_hash: key_hash.clone(),
            is_admin,
        };

        let api_key: ApiKey = diesel::insert_into(api_keys::table)
            .values(&new_key)
            .get_result(&mut conn)?;

        self.cache.insert(key_hash, api_key.clone());

        Ok(api_key)
    }

    pub fn revoke_api_key(&self, key: &str) -> Result<(), AuthError> {
        let mut conn = connect_db();
        let key_hash = Self::hash_api_key(key);

        diesel::delete(api_keys::table.filter(api_keys::key_hash.eq(&key_hash)))
            .execute(&mut conn)?;

        self.cache.remove(&key_hash);

        Ok(())
    }

    pub fn list_api_keys(&self) -> Result<Vec<ApiKey>, AuthError> {
        let mut conn = connect_db();
        Ok(api_keys::table
            .select(ApiKey::as_select())
            .load(&mut conn)?)
    }

    pub fn cleanup_cache(&self) {
        self.cache.cleanup_expired();
    }

    fn extract_bearer_token(auth_header: &str) -> Result<&str, AuthError> {
        auth_header
            .strip_prefix("Bearer ")
            .ok_or(AuthError::InvalidFormat)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = AuthError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_service = match request.guard::<&State<AuthService>>().await {
            Outcome::Success(service) => service,
            _ => {
                return Outcome::Error((
                    rocket::http::Status::InternalServerError,
                    AuthError::Database(diesel::result::Error::NotFound),
                ));
            }
        };

        let auth_header = match request.headers().get_one("Authorization") {
            Some(header) => header,
            None => {
                return Outcome::Error((
                    rocket::http::Status::Unauthorized,
                    AuthError::MissingHeader,
                ));
            }
        };

        let api_key = match AuthService::extract_bearer_token(auth_header) {
            Ok(key) => key,
            Err(e) => return Outcome::Error((rocket::http::Status::Unauthorized, e)),
        };

        match auth_service.validate_api_key(api_key) {
            Ok(key_record) => {
                let key_id = key_record.id;
                let auth_service_clone = auth_service.inner().clone();
                tokio::spawn(async move {
                    auth_service_clone.update_last_used(key_id).await;
                });

                Outcome::Success(User {
                    api_key: key_record,
                })
            }
            Err(e) => Outcome::Error((rocket::http::Status::Unauthorized, e)),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = AuthError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match User::from_request(request).await {
            Outcome::Success(user) => {
                if user.is_admin() {
                    Outcome::Success(AdminUser(user))
                } else {
                    Outcome::Error((
                        rocket::http::Status::Forbidden,
                        AuthError::InsufficientPermissions,
                    ))
                }
            }
            Outcome::Error((status, e)) => Outcome::Error((status, e)),
            Outcome::Forward(s) => Outcome::Forward(s),
        }
    }
}

impl Clone for AuthService {
    fn clone(&self) -> Self {
        Self {
            cache: ApiKeyCache {
                cache: Arc::clone(&self.cache.cache),
            },
        }
    }
}
