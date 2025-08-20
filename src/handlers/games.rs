use crate::auth::User;
use crate::db::connect_db;
use crate::models::{Game, NewGame, UpdateGame};
use diesel::prelude::*;
use rocket::form::FromForm;
use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::{delete, get, http::Status, patch, post, put, routes};
use std::collections::HashMap;

#[derive(FromForm, Debug)]
pub struct GameQuery {
    title: Option<String>,
    developer: Option<String>,
    genre: Option<String>,
    tag: Option<String>,
    status: Option<String>,
    explicit: Option<String>,
    bad: Option<String>,
    #[field(name = "minProgress")]
    min_progress: Option<i32>,
    #[field(name = "maxProgress")]
    max_progress: Option<i32>,
    #[field(name = "exactProgress")]
    exact_progress: Option<i32>,
    #[field(name = "minRating")]
    min_rating: Option<i32>,
    #[field(name = "maxRating")]
    max_rating: Option<i32>,
    #[field(name = "exactRating")]
    exact_rating: Option<i32>,
    sort: Option<String>,
}

#[derive(Deserialize)]
pub struct BulkDeleteFilter {
    developer: Option<String>,
    status: Option<String>,
}

#[derive(Deserialize)]
pub struct BulkUpdatePayload {
    filter: HashMap<String, String>,
    update: HashMap<String, serde_json::Value>,
}

#[derive(Serialize)]
pub struct ApiResponse {
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    deleted: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<usize>,
}

#[get("/search?<query..>")]
pub fn get_games(query: GameQuery) -> Result<Json<Vec<Game>>, Status> {
    use crate::schema::games::dsl::*;

    let mut conn = connect_db();
    let mut game_query = games.into_boxed();

    if let Some(title_filter) = &query.title {
        game_query = game_query.filter(title.ilike(format!("%{}%", title_filter)));
    }

    if let Some(author_filter) = &query.developer {
        game_query = game_query.filter(developer.ilike(format!("%{}%", author_filter)));
    }

    if let Some(status_filter) = &query.status {
        game_query = game_query.filter(status.eq(status_filter));
    }

    if let Some(bad_filter) = &query.bad {
        match bad_filter.as_str() {
            "true" => game_query = game_query.filter(bad.eq(true)),
            "false" => game_query = game_query.filter(bad.eq(false)),
            _ => {}
        }
    }

    if let Some(explicit_filter) = &query.explicit {
        match explicit_filter.as_str() {
            "true" => game_query = game_query.filter(explicit.eq(true)),
            "false" => game_query = game_query.filter(explicit.eq(false)),
            _ => {}
        }
    }

    if let Some(min_rating_filter) = query.min_rating {
        game_query = game_query.filter(rating.ge(min_rating_filter));
    }

    if let Some(max_rating_filter) = query.max_rating {
        game_query = game_query.filter(rating.le(max_rating_filter));
    }

    if let Some(exact_rating_filter) = query.exact_rating {
        game_query = game_query.filter(rating.eq(exact_rating_filter));
    }

    if let Some(min_progress_filter) = query.min_progress {
        game_query = game_query.filter(percent.ge(min_progress_filter));
    }

    if let Some(max_progress_filter) = query.max_progress {
        game_query = game_query.filter(percent.le(max_progress_filter));
    }

    if let Some(exact_progress_filter) = query.exact_progress {
        game_query = game_query.filter(percent.eq(exact_progress_filter));
    }

    if let Some(sort_by) = &query.sort {
        match sort_by.as_str() {
            "title" => game_query = game_query.order(title.asc()),
            "author" => game_query = game_query.order(developer.asc()),
            "rating" => game_query = game_query.order(rating.desc()),
            _ => {}
        }
    }

    let results = game_query
        .load::<Game>(&mut conn)
        .map_err(|_| Status::InternalServerError)?;

    let mut filtered_results = results;

    if let Some(genre_filter) = &query.genre {
        filtered_results.retain(|game| {
            game.genres.iter().any(|g| {
                if let Some(genre) = g {
                    genre.to_lowercase().contains(&genre_filter.to_lowercase())
                } else {
                    false
                }
            })
        });
    }

    if let Some(tag_filter) = &query.tag {
        filtered_results.retain(|game| {
            game.tags.iter().any(|t| {
                if let Some(tag) = t {
                    tag.to_lowercase().contains(&tag_filter.to_lowercase())
                } else {
                    false
                }
            })
        });
    }

    Ok(Json(filtered_results))
}

#[get("/<game_id>")]
pub fn get_game_by_id(game_id: i32) -> Result<Json<Game>, Status> {
    use crate::schema::games::dsl::*;

    let mut conn = connect_db();

    let game = games
        .filter(id.eq(&game_id))
        .first::<Game>(&mut conn)
        .map_err(|_| Status::NotFound)?;

    Ok(Json(game))
}

#[post("/", format = "json", data = "<new_game>")]
pub fn post_games(_user: User, new_game: Json<NewGame>) -> Result<Json<Game>, Status> {
    _user.require_admin().expect("User is not admin");
    use crate::schema::games;

    let mut conn = connect_db();

    let created_game = diesel::insert_into(games::table)
        .values(&new_game.into_inner())
        .get_result::<Game>(&mut conn)
        .map_err(|_| Status::InternalServerError)?;

    Ok(Json(created_game))
}

#[put("/<game_id>", format = "json", data = "<updated_game>")]
pub fn update_game(
    _user: User,
    game_id: i32,
    updated_game: Json<UpdateGame>,
) -> Result<Json<Game>, Status> {
    _user.require_admin().expect("User is not admin");
    use crate::schema::games::dsl::*;

    let mut conn = connect_db();

    let game = diesel::update(games.filter(id.eq(&game_id)))
        .set(&updated_game.into_inner())
        .get_result::<Game>(&mut conn)
        .map_err(|_| Status::NotFound)?;

    Ok(Json(game))
}

#[patch("/<game_id>", format = "json", data = "<patch_data>")]
pub fn patch_game(
    _user: User,
    game_id: i32,
    patch_data: Json<UpdateGame>,
) -> Result<Json<Game>, Status> {
    _user.require_admin().expect("User is not admin");
    use crate::schema::games::dsl::*;

    let mut conn = connect_db();

    let game = diesel::update(games.filter(id.eq(&game_id)))
        .set(&patch_data.into_inner())
        .get_result::<Game>(&mut conn)
        .map_err(|_| Status::NotFound)?;

    Ok(Json(game))
}

#[delete("/<game_id>")]
pub fn delete_game(_user: User, game_id: i32) -> Result<Json<ApiResponse>, Status> {
    _user.require_admin().expect("User is not admin");
    use crate::schema::games::dsl::*;

    let mut conn = connect_db();

    let rows_deleted = diesel::delete(games.filter(id.eq(&game_id)))
        .execute(&mut conn)
        .map_err(|_| Status::InternalServerError)?;

    if rows_deleted > 0 {
        Ok(Json(ApiResponse {
            message: "game deleted".to_string(),
            deleted: None,
            updated: None,
            count: None,
        }))
    } else {
        Err(Status::NotFound)
    }
}

#[delete("/bulk", format = "json", data = "<filter>")]
pub fn bulk_delete_games(
    _user: User,
    filter: Json<BulkDeleteFilter>,
) -> Result<Json<ApiResponse>, Status> {
    _user.require_admin().expect("User is not admin");
    use crate::schema::games::dsl::*;

    let mut conn = connect_db();
    let filter = filter.into_inner();

    let mut delete_query = diesel::delete(games).into_boxed();

    if let Some(developer_filter) = &filter.developer {
        delete_query = delete_query.filter(developer.eq(developer_filter));
    }

    if let Some(status_filter) = &filter.status {
        delete_query = delete_query.filter(status.eq(status_filter));
    }

    let deleted_count = delete_query
        .execute(&mut conn)
        .map_err(|_| Status::InternalServerError)? as i32;

    Ok(Json(ApiResponse {
        message: "bulk delete complete".to_string(),
        deleted: Some(deleted_count),
        updated: None,
        count: None,
    }))
}

#[patch("/bulk", format = "json", data = "<payload>")]
pub fn bulk_update_games(
    _user: User,
    payload: Json<BulkUpdatePayload>,
) -> Result<Json<ApiResponse>, Status> {
    _user.require_admin().expect("User is not admin");
    use crate::schema::games::dsl::*;

    let mut conn = connect_db();
    let payload = payload.into_inner();

    let mut update_query = diesel::update(games).into_boxed();

    if let Some(developer_filter) = payload.filter.get("developer") {
        update_query = update_query.filter(developer.eq(developer_filter));
    }

    if let Some(status_filter) = payload.filter.get("status") {
        update_query = update_query.filter(status.eq(status_filter));
    }

    let mut update_game = UpdateGame {
        title: None,
        developer: None,
        genres: None,
        tags: None,
        rating: None,
        status: None,
        description: None,
        my_thoughts: None,
        links: None,
        cover_image: None,
        explicit: None,
        percent: None,
        bad: None,
    };

    if let Some(new_status) = payload.update.get("status")
        && let Some(status_str) = new_status.as_str()
    {
        update_game.status = Some(status_str.to_string());
    }

    if let Some(new_rating) = payload.update.get("rating")
        && let Some(rating_num) = new_rating.as_f64()
    {
        update_game.rating = Some(rating_num as i32);
    }

    let updated_count = update_query
        .set(&update_game)
        .execute(&mut conn)
        .map_err(|_| Status::InternalServerError)? as i32;

    Ok(Json(ApiResponse {
        message: "bulk update complete".to_string(),
        deleted: None,
        updated: Some(updated_count),
        count: None,
    }))
}

pub fn games_routes() -> Vec<rocket::Route> {
    routes![
        get_games,
        get_game_by_id,
        post_games,
        update_game,
        patch_game,
        delete_game,
        bulk_delete_games,
        bulk_update_games
    ]
}
