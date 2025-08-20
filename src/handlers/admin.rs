use crate::auth::{AdminUser, AuthenticatedUser};
use rocket::serde::json::Json;
use rocket::{get, post, routes};

#[post("/create-key")]
pub fn create_api_key(_admin: AdminUser) -> Json<serde_json::Value> {
    use crate::auth::{create_api_key, generate_api_key};

    let new_key = generate_api_key();

    match create_api_key(&new_key, false) {
        Ok(_) => Json(serde_json::json!({
            "message": "API key created successfully",
            "api_key": new_key
        })),
        Err(_) => Json(serde_json::json!({
            "error": "Failed to create API key"
        })),
    }
}

#[get("/is-admin")]
pub fn is_admin(user: AuthenticatedUser) -> Json<bool> {
    Json(user.is_admin())
}

pub fn admin_routes() -> Vec<rocket::Route> {
    routes![create_api_key, is_admin]
}
