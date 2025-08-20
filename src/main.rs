use {
    handlers::{admin::*, books::*, games::*, projects::*, reviews::*, wplace::*, *},
    libapiodactyl::{auth::AuthService, handlers::index},
    rocket::{catchers, http::Method, launch, routes},
    rocket_cors::{AllowedOrigins, CorsOptions},
};

pub mod errors;
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
            vec![Method::Get, Method::Post, Method::Patch, Method::Put, Method::Delete]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    let catchers = catchers![catch401, catch404, catch500];

    rocket::build()
        .manage(AuthService::new())
        .attach(cors.to_cors().unwrap())
        .register("/", catchers)
        .mount("/", routes![index])
        .mount("/wplace", wplace_routes())
        .mount("/reviews", reviews_routes())
        .mount("/projects", projects_routes())
        .mount("/read-watch", read_watch_routes())
        .mount("/games", games_routes())
        .mount("/admin", admin_routes())
}
