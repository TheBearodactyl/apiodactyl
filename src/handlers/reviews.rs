use crate::db::connect_db as establish_connection;
use crate::models::{NewReview, Review, UpdateReview};
use crate::schema::reviews;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post, put};

#[post("/", format = "json", data = "<new_review>")]
pub fn create_review(new_review: Json<NewReview<'_>>) -> Json<Review> {
    let mut conn = establish_connection();
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

#[get("/<id>")]
pub fn get_review(id: i32) -> Option<Json<Review>> {
    let mut conn = establish_connection();
    reviews::table
        .find(id)
        .first::<Review>(&mut conn)
        .ok()
        .map(Json)
}

#[put("/<id>", format = "json", data = "<update_data>")]
pub fn update_review(id: i32, update_data: Json<UpdateReview>) -> Option<Json<Review>> {
    let mut conn = establish_connection();
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
pub fn delete_review(id: i32) -> Option<Json<Review>> {
    let mut conn = establish_connection();
    let review = reviews::table.find(id).first::<Review>(&mut conn).ok()?;
    diesel::delete(reviews::table.find(id))
        .execute(&mut conn)
        .ok()?;

    Some(Json(review))
}

#[patch("/<id>", format = "json", data = "<update_data>")]
pub fn patch_review(id: i32, update_data: Json<UpdateReview>) -> Option<Json<Review>> {
    let mut conn = establish_connection();

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
