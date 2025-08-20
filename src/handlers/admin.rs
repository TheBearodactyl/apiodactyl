use crate::auth::{AdminUser, AuthService, User};
use crate::errors::AuthError;
use crate::models::ApiKey;
use rocket::serde::json::Json;
use rocket::{State, delete, get, post, routes};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Deserialize)]
pub struct CreateKeyRequest {
    pub is_admin: Option<bool>,
}

#[derive(Deserialize)]
pub struct RevokeKeyRequest {
    pub api_key: String,
}

#[derive(Serialize)]
pub struct ApiKeyResponse {
    pub id: i32,
    pub is_admin: bool,
    pub created_at: chrono::NaiveDateTime,
    pub last_used_at: Option<chrono::NaiveDateTime>,
}

impl From<ApiKey> for ApiKeyResponse {
    fn from(api_key: ApiKey) -> Self {
        Self {
            id: api_key.id,
            is_admin: api_key.is_admin,
            created_at: api_key.created_at,
            last_used_at: api_key.last_used_at,
        }
    }
}

#[post("/create-key", data = "<request>")]
pub fn create_api_key(
    _admin: AdminUser,
    request: Option<Json<CreateKeyRequest>>,
    auth_service: &State<AuthService>,
) -> Result<Json<Value>, AuthError> {
    let is_admin = request.as_ref().and_then(|r| r.is_admin).unwrap_or(false);

    let new_key = AuthService::generate_api_key();

    match auth_service.create_api_key(&new_key, is_admin) {
        Ok(api_key) => Ok(Json(json!({
            "message": "API key created successfully",
            "api_key": new_key,
            "details": ApiKeyResponse::from(api_key)
        }))),
        Err(e) => Err(e),
    }
}

#[delete("/revoke-key", data = "<request>")]
pub fn revoke_api_key(
    _admin: AdminUser,
    request: Json<RevokeKeyRequest>,
    auth_service: &State<AuthService>,
) -> Result<Json<Value>, AuthError> {
    auth_service.revoke_api_key(&request.api_key)?;

    Ok(Json(json!({
        "message": "API key revoked successfully"
    })))
}

#[get("/list-keys")]
pub fn list_api_keys(
    _admin: AdminUser,
    auth_service: &State<AuthService>,
) -> Result<Json<Vec<ApiKeyResponse>>, AuthError> {
    let keys = auth_service.list_api_keys()?;
    let response: Vec<ApiKeyResponse> = keys.into_iter().map(ApiKeyResponse::from).collect();
    Ok(Json(response))
}

#[get("/profile")]
pub fn get_profile(user: User) -> Json<Value> {
    Json(json!({
        "id": user.id(),
        "is_admin": user.is_admin(),
        "created_at": user.created_at(),
        "last_used_at": user.last_used_at()
    }))
}

#[get("/is-admin")]
pub fn is_admin(user: User) -> Json<bool> {
    Json(user.is_admin())
}

#[post("/cleanup-cache")]
pub fn cleanup_cache(_admin: AdminUser, auth_service: &State<AuthService>) -> Json<Value> {
    auth_service.cleanup_cache();
    Json(json!({
        "message": "Cache cleanup completed"
    }))
}

pub fn admin_routes() -> Vec<rocket::Route> {
    routes![
        create_api_key,
        revoke_api_key,
        list_api_keys,
        get_profile,
        is_admin,
        cleanup_cache
    ]
}
