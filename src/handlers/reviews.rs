use crate::auth::AuthenticatedUser;
use crate::db::connect_db;
use crate::models::{NewReview, Review, UpdateReview};
use crate::schema::reviews;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::{delete, get, options, patch, post, put, routes, FromForm};
use rocket::http::Status;

#[derive(FromForm, Debug)]
pub struct ReviewQuery {
    chapter: Option<i32>,
    rating: Option<i32>,
    description: Option<String>,
    thoughts: Option<String>,
    is_first_chapter: Option<bool>,
}

#[options("/")]
pub fn reviews_opts() -> Json<Vec<(&'static str, &'static str)>> {
    Json(vec![
	    ("Allow", "POST, GET, PUT, DELETE, PATCH"),
        ("Content-Type", "application/json"),
        ("Access-Control-Allow-Origin", "*"),
        ("Access-Control-Allow-Methods", "POST, GET, PUT, DELETE, PATCH")
    ])
}

#[post("/", format = "json", data = "<new_review>")]
pub fn create_review(_user: AuthenticatedUser, new_review: Json<NewReview<'_>>) -> Json<Review> {
    let mut conn = connect_db();
    let new_review = new_review.into_inner();
    diesel::insert_into(reviews::table)
        .values(&new_review)
        .execute(&mut conn)
        .expect("Error inserting new review");

    Json(Review {
        id: 0,
        chapter: new_review.chapter,
        description: new_review.description.to_string(),
        rating: new_review.rating,
        thoughts: new_review.thoughts.to_string(),
    })
}

#[get("/search?<query..>")]
pub fn search_reviews(query: ReviewQuery) -> Result<Json<Vec<Review>>, Status> {
    use crate::schema::reviews::dsl::*;

    let mut conn = connect_db();
    let mut review_query = reviews.into_boxed();

    if let Some(chapter_filter) = &query.chapter {
        review_query = review_query.filter(chapter.eq(chapter_filter));
    }

    if let Some(rating_filter) = &query.rating {
        review_query = review_query.filter(rating.eq(rating_filter));
    }

    if let Some(description_filter) = &query.description {
        review_query = review_query.filter(description.ilike(format!("%{}%", description_filter)));
    }

    if let Some(thought_filter) = &query.thoughts {
        review_query = review_query.filter(thoughts.ilike(format!("%{}%", thought_filter)));
    }

	if let Some(_) = &query.is_first_chapter {
        review_query = review_query.filter(chapter.eq(1));
    }

    let results = review_query
        .load::<Review>(&mut conn)
        .map_err(|_| Status::InternalServerError)?;

    let filtered_reviews = results;

    Ok(Json(filtered_reviews))
}

#[get("/<review_id>")]
pub fn get_review_by_id(review_id: i32) -> Result<Json<Review>, Status> {
    use crate::schema::reviews::dsl::*;
    let mut conn = connect_db();
    let review = reviews
        .filter(id.eq(&review_id))
        .first::<Review>(&mut conn)
        .map_err(|_| Status::NotFound)?;

    Ok(Json(review))
}

#[get("/")]
pub fn get_reviews() -> Option<Json<Vec<Review>>> {
    let mut conn = connect_db();
    reviews::table.load::<Review>(&mut conn).ok().map(Json)
}

#[put("/<id>", format = "json", data = "<update_data>")]
pub fn update_review(
    _user: AuthenticatedUser,
    id: i32,
    update_data: Json<UpdateReview>,
) -> Option<Json<Review>> {
    let mut conn = connect_db();
    diesel::update(reviews::table.find(id))
        .set(&update_data.into_inner())
        .execute(&mut conn)
        .ok()?;

    reviews::table
        .find(id)
        .first::<Review>(&mut conn)
        .ok()
        .map(Json)
}

#[delete("/<id>")]
pub fn delete_review(_user: AuthenticatedUser, id: i32) -> Option<Json<Review>> {
    let mut conn = connect_db();
    let review = reviews::table.find(id).first::<Review>(&mut conn).ok()?;
    diesel::delete(reviews::table.find(id))
        .execute(&mut conn)
        .ok()?;

    Some(Json(review))
}

#[patch("/<id>", format = "json", data = "<update_data>")]
pub fn patch_review(
    _user: AuthenticatedUser,
    id: i32,
    update_data: Json<UpdateReview>,
) -> Option<Json<Review>> {
    let mut conn = connect_db();

    diesel::update(reviews::table.find(id))
        .set(&update_data.into_inner())
        .execute(&mut conn)
        .ok()?;

    reviews::table
        .find(id)
        .first::<Review>(&mut conn)
        .ok()
        .map(Json)
}

pub fn reviews_routes() -> Vec<rocket::Route> {
    routes![
        reviews_opts,
        create_review,
        search_reviews,
        get_reviews,
        get_review_by_id,
        update_review,
        delete_review,
        patch_review,
    ]
}
