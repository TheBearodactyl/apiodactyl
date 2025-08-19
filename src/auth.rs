use diesel::prelude::*;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use sha2::{Digest, Sha256};

use crate::db::connect_db;
use crate::models::ApiKey;
use crate::schema::api_keys;

pub struct AuthenticatedUser {
    pub api_key: ApiKey,
}

pub struct AdminUser {
    pub api_key: ApiKey,
}

impl AuthenticatedUser {
    pub fn is_admin(&self) -> bool {
        self.api_key.is_admin
    }
}

fn hash_api_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hex::encode(hasher.finalize())
}

fn validate_api_key(key: &str) -> Option<ApiKey> {
    let mut conn = connect_db();
    let key_hash = hash_api_key(key);

    api_keys::table
        .filter(api_keys::key_hash.eq(key_hash))
        .first::<ApiKey>(&mut conn)
        .ok()
}

fn update_last_used(key_id: i32) {
    let mut conn = connect_db();
    let _ = diesel::update(api_keys::table.find(key_id))
        .set(api_keys::last_used_at.eq(diesel::dsl::now))
        .execute(&mut conn);
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = &'static str;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = match request.headers().get_one("Authorization") {
            Some(header) => header,
            None => return Outcome::Error((Status::Unauthorized, "Missing Authorization header")),
        };

        let api_key = if let Some(key) = auth_header.strip_prefix("Bearer ") {
            key.to_string()
        } else {
            return Outcome::Error((Status::Unauthorized, "Invalid Authorization header"));
        };

        match validate_api_key(&api_key) {
            Some(key_record) => {
                let key_id = key_record.id;
                tokio::task::spawn_blocking(move || {
                    update_last_used(key_id);
                });

                Outcome::Success(AuthenticatedUser {
                    api_key: key_record,
                })
            }
            None => Outcome::Error((Status::Unauthorized, "Invalid API key")),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = &'static str;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match AuthenticatedUser::from_request(request).await {
            Outcome::Success(user) => {
                if user.api_key.is_admin {
                    Outcome::Success(AdminUser {
                        api_key: user.api_key,
                    })
                } else {
                    Outcome::Error((Status::Forbidden, "Admin access required"))
                }
            }
            Outcome::Error(e) => Outcome::Error(e),
            Outcome::Forward(s) => Outcome::Forward(s),
        }
    }
}

pub fn generate_api_key() -> String {
    use uuid::Uuid;
    format!("ak_{}", Uuid::new_v4().simple())
}

pub fn create_api_key(key: &str, is_admin: bool) -> Result<ApiKey, diesel::result::Error> {
    use crate::models::NewApiKey;
    let mut conn = connect_db();

    let new_key = NewApiKey {
        key_hash: hash_api_key(key),
        is_admin,
    };

    diesel::insert_into(api_keys::table)
        .values(&new_key)
        .get_result(&mut conn)
}
