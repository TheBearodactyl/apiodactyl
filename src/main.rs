use {
    auth::AuthService,
    handlers::index,
    handlers::{admin::*, books::*, games::*, projects::*, reviews::*, wplace::*, *},
    rocket::{catchers, http::Method, launch, routes},
    rocket_cors::{AllowedOrigins, CorsOptions},
};

pub mod auth;
pub mod db;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod schema;

pub mod cli;

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![
                Method::Get,
                Method::Post,
                Method::Patch,
                Method::Put,
                Method::Delete,
            ]
            .into_iter()
            .map(From::from)
            .collect(),
        )
        .allow_credentials(true);

    let catchers = catchers![catch401, catch404, catch500];

    let auth_service = AuthService::new();
    if std::env::args().len() > 1
        && let Err(e) = cli::handle_cli(auth_service.clone())
    {
        eprintln!("CLI Error: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = auth_service.clone().ensure_admin_exists() {
        eprintln!("Failed to initialize admin: {}", e);
        std::process::exit(1);
    }

    rocket::build()
        .manage(auth_service)
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
