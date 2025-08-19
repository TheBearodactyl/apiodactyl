use handlers::{books::*, games::*, projects::*, reviews::*, wplace::*};
use rocket::{http::Method, launch, routes};
use rocket_cors::{AllowedOrigins, CorsOptions};

mod db;
mod handlers;
mod models;
mod schema;

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
        .mount(
            "/wplace",
            routes![
                get_screenshot,
                create_screenshot,
                update_screenshot,
                delete_screenshot,
                patch_screenshot
            ],
        )
        .mount(
            "/reviews",
            routes![
                get_review,
                create_review,
                update_review,
                delete_review,
                patch_review
            ],
        )
        .mount(
            "/projects",
            routes![
                get_project,
                create_project,
                update_project,
                delete_project,
                patch_project
            ],
        )
        .mount(
            "/read-watch",
            routes![
                get_books,
                get_book_by_id,
                post_books,
                update_book,
                patch_book,
                delete_book,
                bulk_delete_books,
                bulk_update_books
            ],
        )
        .mount(
            "/games",
            routes![
                get_games,
                get_game_by_id,
                post_games,
                update_game,
                patch_game,
                delete_game,
                bulk_delete_games,
                bulk_update_games
            ],
        )
}
