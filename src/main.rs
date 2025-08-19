use {
    handlers::{admin::*, books::*, games::*, projects::*, reviews::*, wplace::*},
    rocket::{http::Method, launch},
    rocket_cors::{AllowedOrigins, CorsOptions},
};

pub mod auth;
pub mod db;
pub mod handlers;
pub mod models;
pub mod schema;

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    rocket::build()
        .attach(cors.to_cors().unwrap())
        .mount("/wplace", wplace_routes())
        .mount("/reviews", reviews_routes())
        .mount("/projects", projects_routes())
        .mount("/read-watch", read_watch_routes())
        .mount("/games", games_routes())
        .mount("/admin", admin_routes())
}
