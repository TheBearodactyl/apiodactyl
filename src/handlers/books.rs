use {
    crate::{
        auth::User,
        db::connect_db,
        models::{Book, NewBook, UpdateBook},
        schema::books,
    },
    diesel::prelude::*,
    rocket::{
        Route, delete,
        form::FromForm,
        get,
        http::Status,
        patch, post, put, routes,
        serde::{Deserialize, Serialize, json::Json},
    },
    std::collections::HashMap,
};

#[derive(FromForm, Debug)]
pub struct BookQuery {
    title: Option<String>,
    author: Option<String>,
    genre: Option<String>,
    tag: Option<String>,
    status: Option<String>,
    explicit: Option<String>,
    #[field(name = "minRating")]
    min_rating: Option<i32>,
    #[field(name = "maxRating")]
    max_rating: Option<i32>,
    sort: Option<String>,
}

#[derive(Deserialize)]
pub struct BulkDeleteFilter {
    author: Option<String>,
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

#[get("/?<query..>")]
pub fn get_books(query: BookQuery) -> Result<Json<Vec<Book>>, Status> {
    use crate::schema::books::dsl::*;

    let mut conn = connect_db();
    let mut book_query = books.into_boxed();

    if let Some(title_filter) = &query.title {
        book_query = book_query.filter(title.ilike(format!("%{}%", title_filter)));
    }

    if let Some(author_filter) = &query.author {
        book_query = book_query.filter(author.ilike(format!("%{}%", author_filter)));
    }

    if let Some(status_filter) = &query.status {
        book_query = book_query.filter(status.eq(status_filter));
    }

    if let Some(explicit_filter) = &query.explicit {
        match explicit_filter.as_str() {
            "true" => book_query = book_query.filter(explicit.eq(true)),
            "false" => book_query = book_query.filter(explicit.eq(false)),
            _ => {}
        }
    }

    if let Some(min_rating_filter) = query.min_rating {
        book_query = book_query.filter(rating.ge(min_rating_filter));
    }

    if let Some(max_rating_filter) = query.max_rating {
        book_query = book_query.filter(rating.le(max_rating_filter));
    }

    if let Some(sort_by) = &query.sort {
        match sort_by.as_str() {
            "title" => book_query = book_query.order(title.asc()),
            "author" => book_query = book_query.order(author.asc()),
            "rating" => book_query = book_query.order(rating.desc()),
            _ => {}
        }
    }

    let results = book_query
        .load::<Book>(&mut conn)
        .map_err(|_| Status::InternalServerError)?;

    let mut filtered_results = results;

    if let Some(genre_filter) = &query.genre {
        filtered_results.retain(|book| {
            book.genres.iter().any(|g| {
                if let Some(genre) = g {
                    genre.to_lowercase().contains(&genre_filter.to_lowercase())
                } else {
                    false
                }
            })
        });
    }

    if let Some(tag_filter) = &query.tag {
        filtered_results.retain(|book| {
            book.tags.iter().any(|t| {
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

#[get("/<book_id>")]
pub fn get_book_by_id(book_id: i32) -> Result<Json<Book>, Status> {
    use crate::schema::books::dsl::*;

    let mut conn = connect_db();

    let book = books
        .filter(id.eq(&book_id))
        .first::<Book>(&mut conn)
        .map_err(|_| Status::NotFound)?;

    Ok(Json(book))
}

#[post("/", format = "json", data = "<new_book>")]
pub fn post_books(_user: User, new_book: Json<NewBook<'_>>) -> Result<Json<Book>, Status> {
    _user.require_admin().expect("User is not admin");
    let mut conn = connect_db();
    let new_book = new_book.into_inner();
    diesel::insert_into(books::table)
        .values(&new_book)
        .execute(&mut conn)
        .expect("Error inserting new book");

    Ok(Json(Book {
        id: 0,
        title: new_book.title.to_string(),
        author: new_book.author.to_string(),
        genres: new_book
            .genres
            .iter()
            .map(|a| Some(a.to_string()))
            .collect::<Vec<Option<String>>>(),
        tags: new_book
            .tags
            .iter()
            .map(|a| Some(a.to_string()))
            .collect::<Vec<Option<String>>>(),
        rating: new_book.rating,
        status: new_book.status.to_string(),
        description: new_book.description.to_string(),
        my_thoughts: new_book.my_thoughts.to_string(),
        links: new_book.links,
        cover_image: new_book.cover_image.to_string(),
        explicit: new_book.explicit,
        color: Some(new_book.color.unwrap().to_string()),
    }))
}

#[put("/<book_id>", format = "json", data = "<updated_book>")]
pub fn update_book(
    _user: User,
    book_id: i32,
    updated_book: Json<UpdateBook>,
) -> Result<Json<Book>, Status> {
    _user.require_admin().expect("User is not admin");
    use crate::schema::books::dsl::*;

    let mut conn = connect_db();

    let book = diesel::update(books.filter(id.eq(&book_id)))
        .set(&updated_book.into_inner())
        .get_result::<Book>(&mut conn)
        .map_err(|_| Status::NotFound)?;

    Ok(Json(book))
}

#[patch("/<book_id>", format = "json", data = "<patch_data>")]
pub fn patch_book(
    _user: User,
    book_id: i32,
    patch_data: Json<UpdateBook>,
) -> Result<Json<Book>, Status> {
    _user.require_admin().expect("User is not admin");
    use crate::schema::books::dsl::*;

    let mut conn = connect_db();

    let book = diesel::update(books.filter(id.eq(&book_id)))
        .set(&patch_data.into_inner())
        .get_result::<Book>(&mut conn)
        .map_err(|_| Status::NotFound)?;

    Ok(Json(book))
}

#[delete("/<book_id>")]
pub fn delete_book(_user: User, book_id: i32) -> Result<Json<ApiResponse>, Status> {
    _user.require_admin().expect("User is not admin");
    use crate::schema::books::dsl::*;

    let mut conn = connect_db();

    let rows_deleted = diesel::delete(books.filter(id.eq(&book_id)))
        .execute(&mut conn)
        .map_err(|_| Status::InternalServerError)?;

    if rows_deleted > 0 {
        Ok(Json(ApiResponse {
            message: "book deleted".to_string(),
            deleted: None,
            updated: None,
            count: None,
        }))
    } else {
        Err(Status::NotFound)
    }
}

#[delete("/bulk", format = "json", data = "<filter>")]
pub fn bulk_delete_books(
    _user: User,
    filter: Json<BulkDeleteFilter>,
) -> Result<Json<ApiResponse>, Status> {
    _user.require_admin().expect("User is not admin");
    use crate::schema::books::dsl::*;

    let mut conn = connect_db();
    let filter = filter.into_inner();

    let mut delete_query = diesel::delete(books).into_boxed();

    if let Some(author_filter) = &filter.author {
        delete_query = delete_query.filter(author.eq(author_filter));
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
pub fn bulk_update_books(
    _user: User,
    payload: Json<BulkUpdatePayload>,
) -> Result<Json<ApiResponse>, Status> {
    _user.require_admin().expect("User is not admin");
    use crate::schema::books::dsl::*;

    let mut conn = connect_db();
    let payload = payload.into_inner();

    let mut update_query = diesel::update(books).into_boxed();

    if let Some(author_filter) = payload.filter.get("author") {
        update_query = update_query.filter(author.eq(author_filter));
    }

    if let Some(status_filter) = payload.filter.get("status") {
        update_query = update_query.filter(status.eq(status_filter));
    }

    let mut update_book = UpdateBook {
        title: None,
        author: None,
        genres: None,
        tags: None,
        rating: None,
        status: None,
        description: None,
        my_thoughts: None,
        links: None,
        cover_image: None,
        explicit: None,
        color: None,
    };

    if let Some(new_status) = payload.update.get("status")
        && let Some(status_str) = new_status.as_str()
    {
        update_book.status = Some(status_str.to_string());
    }

    if let Some(new_rating) = payload.update.get("rating")
        && let Some(rating_num) = new_rating.as_f64()
    {
        update_book.rating = Some(rating_num as i32);
    }

    let updated_count = update_query
        .set(&update_book)
        .execute(&mut conn)
        .map_err(|_| Status::InternalServerError)? as i32;

    Ok(Json(ApiResponse {
        message: "bulk update complete".to_string(),
        deleted: None,
        updated: Some(updated_count),
        count: None,
    }))
}

pub fn read_watch_routes() -> Vec<Route> {
    routes![
        get_books,
        get_book_by_id,
        post_books,
        update_book,
        patch_book,
        delete_book,
        bulk_delete_books,
        bulk_update_books
    ]
}
