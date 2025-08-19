use crate::db::connect_db;
use crate::models::{NewProject, Project, UpdateProject};
use crate::schema::projects;
use crate::auth::AuthenticatedUser;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post, put, routes};

#[post("/", format = "json", data = "<new_project>")]
pub fn create_project(_user: AuthenticatedUser, new_project: Json<NewProject<'_>>) -> Json<Project> {
    let mut conn = connect_db();
    let new_project = new_project.into_inner();
    diesel::insert_into(projects::table)
        .values(&new_project)
        .execute(&mut conn)
        .expect("Error inserting new review");

    Json(Project {
        id: 0,
        name: new_project.name.to_string(),
        description: new_project.description.to_string(),
        cover_image: Some(new_project.cover_image.unwrap().to_string()),
        tags: Some(
            new_project
                .tags
                .unwrap()
                .into_iter()
                .map(|w| Some(w.unwrap().to_string()))
                .collect(),
        ),
        source: new_project.source.to_string(),
        install_command: Some(new_project.install_command.unwrap().to_string()),
    })
}

#[get("/<id>")]
pub fn get_project(id: i32) -> Option<Json<Project>> {
    let mut conn = connect_db();
    projects::table
        .find(id)
        .first::<Project>(&mut conn)
        .ok()
        .map(Json)
}

#[get("/")]
pub fn get_projects() -> Option<Json<Vec<Project>>> {
    let mut conn = connect_db();
    projects::table
        .load::<Project>(&mut conn)
        .ok()
        .map(Json)
}

#[put("/<id>", format = "json", data = "<update_data>")]
pub fn update_project(_user: AuthenticatedUser, id: i32, update_data: Json<UpdateProject>) -> Option<Json<Project>> {
    let mut conn = connect_db();
    diesel::update(projects::table.find(id))
        .set(&update_data.into_inner())
        .execute(&mut conn)
        .ok()?;

    projects::table
        .find(id)
        .first::<Project>(&mut conn)
        .ok()
        .map(Json)
}

#[delete("/<id>")]
pub fn delete_project(_user: AuthenticatedUser, id: i32) -> Option<Json<Project>> {
    let mut conn = connect_db();
    let project = projects::table.find(id).first::<Project>(&mut conn).ok()?;
    diesel::delete(projects::table.find(id))
        .execute(&mut conn)
        .ok()?;

    Some(Json(project))
}

#[patch("/<id>", format = "json", data = "<update_data>")]
pub fn patch_project(_user: AuthenticatedUser, id: i32, update_data: Json<UpdateProject>) -> Option<Json<Project>> {
    let mut conn = connect_db();

    diesel::update(projects::table.find(id))
        .set(&update_data.into_inner())
        .execute(&mut conn)
        .ok()?;

    projects::table
        .find(id)
        .first::<Project>(&mut conn)
        .ok()
        .map(Json)
}

pub fn projects_routes() -> Vec<rocket::Route> {
    routes![
        create_project,
        get_project,
        get_projects,
        update_project,
        delete_project,
        patch_project,
    ]
}
