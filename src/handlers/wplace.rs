use diesel::{QueryDsl, RunQueryDsl};
use rocket::{delete, get, patch, post, put, routes, serde::json::Json};

use crate::auth::AuthenticatedUser;
use crate::{
    db::connect_db,
    models::{NewWplaceScreenshot, UpdateWplaceScreenshot, WplaceScreenshot},
    schema::wplace,
};

#[post("/", format = "json", data = "<new_screenshot>")]
pub fn create_screenshot(
    _user: AuthenticatedUser,
    new_screenshot: Json<NewWplaceScreenshot<'_>>,
) -> Json<WplaceScreenshot> {
    let mut conn = connect_db();
    let new_screenshot = new_screenshot.into_inner();
    diesel::insert_into(wplace::table)
        .values(&new_screenshot)
        .execute(&mut conn)
        .expect("Failed inserting new screenshot");

    Json(WplaceScreenshot {
        id: 0,
        alt: new_screenshot.alt.to_string(),
        coverimage: new_screenshot.coverimage.to_string(),
    })
}

#[get("/<id>")]
pub fn get_screenshot(id: i32) -> Option<Json<WplaceScreenshot>> {
    let mut conn = connect_db();
    wplace::table
        .find(id)
        .first::<WplaceScreenshot>(&mut conn)
        .ok()
        .map(Json)
}

#[get("/")]
pub fn get_screenshots() -> Option<Json<Vec<WplaceScreenshot>>> {
    let mut conn = connect_db();
    wplace::table
        .load::<WplaceScreenshot>(&mut conn)
        .ok()
        .map(Json)
}

#[put("/<id>", format = "json", data = "<update_data>")]
pub fn update_screenshot(
    _user: AuthenticatedUser,
    id: i32,
    update_data: Json<UpdateWplaceScreenshot>,
) -> Option<Json<WplaceScreenshot>> {
    let mut conn = connect_db();
    diesel::update(wplace::table.find(id))
        .set(&update_data.into_inner())
        .execute(&mut conn)
        .expect("Failed to set updated data");

    wplace::table
        .find(id)
        .first::<WplaceScreenshot>(&mut conn)
        .ok()
        .map(Json)
}

#[delete("/<id>")]
pub fn delete_screenshot(_user: AuthenticatedUser, id: i32) -> Option<Json<WplaceScreenshot>> {
    let mut conn = connect_db();
    let screenshot = wplace::table
        .find(id)
        .first::<WplaceScreenshot>(&mut conn)
        .expect("Failed to find screenshot");

    diesel::delete(wplace::table.find(id))
        .execute(&mut conn)
        .expect("Failed to delete screenshot");

    Some(Json(screenshot))
}

#[patch("/<id>", format = "json", data = "<update_data>")]
pub fn patch_screenshot(
    _user: AuthenticatedUser,
    id: i32,
    update_data: Json<UpdateWplaceScreenshot>,
) -> Option<Json<WplaceScreenshot>> {
    let mut conn = connect_db();

    diesel::update(wplace::table.find(id))
        .set(&update_data.into_inner())
        .execute(&mut conn)
        .ok()?;

    wplace::table
        .find(id)
        .first::<WplaceScreenshot>(&mut conn)
        .ok()
        .map(Json)
}

pub fn wplace_routes() -> Vec<rocket::Route> {
    routes![
        create_screenshot,
        get_screenshot,
        get_screenshots,
        update_screenshot,
        delete_screenshot,
        patch_screenshot,
    ]
}
