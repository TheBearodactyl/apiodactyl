use {
    crate::schema::*,
    chrono::NaiveDateTime,
    diesel::{pg::Pg, prelude::*},
    serde::Deserialize,
    serde::Serialize,
};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = reviews)]
#[diesel(check_for_backend(Pg))]
pub struct Review {
    pub id: i32,
    pub chapter: i32,
    pub description: String,
    pub rating: i32,
    pub thoughts: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = reviews)]
#[diesel(check_for_backend(Pg))]
pub struct NewReview<'a> {
    pub chapter: i32,
    pub description: &'a str,
    pub rating: i32,
    pub thoughts: &'a str,
}

#[derive(Deserialize, AsChangeset, Serialize)]
#[diesel(table_name = reviews)]
#[diesel(check_for_backend(Pg))]
pub struct UpdateReview {
    description: Option<String>,
    rating: Option<i32>,
    thoughts: Option<String>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = wplace)]
#[diesel(check_for_backend(Pg))]
pub struct WplaceScreenshot {
    pub id: i32,
    pub alt: String,
    pub coverimage: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = wplace)]
#[diesel(check_for_backend(Pg))]
pub struct NewWplaceScreenshot<'a> {
    pub alt: &'a str,
    pub coverimage: &'a str,
}

#[derive(Deserialize, AsChangeset, Serialize)]
#[diesel(table_name = wplace)]
#[diesel(check_for_backend(Pg))]
pub struct UpdateWplaceScreenshot {
    alt: Option<String>,
    coverimage: Option<String>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(Pg))]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub tags: Option<Vec<Option<String>>>,
    pub source: String,
    pub cover_image: Option<String>,
    pub install_command: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(Pg))]
pub struct NewProject<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub tags: Option<Vec<Option<&'a str>>>,
    pub source: &'a str,
    pub cover_image: Option<&'a str>,
    pub install_command: Option<&'a str>,
}

#[derive(Deserialize, AsChangeset, Serialize, Debug)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(Pg))]
pub struct UpdateProject {
    pub name: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<Option<String>>>,
    pub source: Option<String>,
    pub cover_image: Option<String>,
    pub install_command: Option<String>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = books)]
#[diesel(check_for_backend(Pg))]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub genres: Vec<Option<String>>,
    pub tags: Vec<Option<String>>,
    pub rating: i32,
    pub status: String,
    pub description: String,
    pub my_thoughts: String,
    pub links: Option<serde_json::Value>,
    pub cover_image: String,
    pub explicit: bool,
    pub color: Option<String>,
}

#[derive(Insertable, Deserialize, Serialize, Debug)]
#[diesel(table_name = books)]
#[diesel(check_for_backend(Pg))]
pub struct NewBook<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub genres: Vec<&'a str>,
    pub tags: Vec<&'a str>,
    pub rating: i32,
    pub status: &'a str,
    pub description: &'a str,
    pub my_thoughts: &'a str,
    pub links: Option<serde_json::Value>,
    pub cover_image: &'a str,
    pub explicit: bool,
    pub color: Option<&'a str>,
}

#[derive(Deserialize, AsChangeset, Serialize, Debug)]
#[diesel(table_name = books)]
#[diesel(check_for_backend(Pg))]
pub struct UpdateBook {
    pub title: Option<String>,
    pub author: Option<String>,
    pub genres: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub rating: Option<i32>,
    pub status: Option<String>,
    pub description: Option<String>,
    pub my_thoughts: Option<String>,
    pub links: Option<serde_json::Value>,
    pub cover_image: Option<String>,
    pub explicit: Option<bool>,
    pub color: Option<String>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = games)]
#[diesel(check_for_backend(Pg))]
pub struct Game {
    pub id: String,
    pub title: String,
    pub developer: String,
    pub genres: Vec<Option<String>>,
    pub tags: Vec<Option<String>>,
    pub rating: i32,
    pub status: String,
    pub description: String,
    pub my_thoughts: String,
    pub links: Option<serde_json::Value>,
    pub cover_image: String,
    pub explicit: bool,
    pub percent: i32,
    pub bad: bool,
}

#[derive(Insertable, Deserialize, Serialize, Debug)]
#[diesel(table_name = games)]
#[diesel(check_for_backend(Pg))]
pub struct NewGame<'a> {
    pub title: &'a str,
    pub developer: &'a str,
    pub genres: Vec<&'a str>,
    pub tags: Vec<&'a str>,
    pub rating: i32,
    pub status: &'a str,
    pub description: &'a str,
    pub my_thoughts: &'a str,
    pub links: Option<serde_json::Value>,
    pub cover_image: &'a str,
    pub explicit: bool,
    pub percent: i32,
    pub bad: bool,
}

#[derive(Deserialize, AsChangeset, Serialize, Debug)]
#[diesel(table_name = games)]
#[diesel(check_for_backend(Pg))]
pub struct UpdateGame {
    pub title: Option<String>,
    pub developer: Option<String>,
    pub genres: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub rating: Option<i32>,
    pub status: Option<String>,
    pub description: Option<String>,
    pub my_thoughts: Option<String>,
    pub links: Option<serde_json::Value>,
    pub cover_image: Option<String>,
    pub explicit: Option<bool>,
    pub percent: Option<i32>,
    pub bad: Option<bool>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = api_keys)]
pub struct ApiKey {
    pub id: i32,
    pub key_hash: String,
    pub is_admin: bool,
    pub created_at: NaiveDateTime,
    pub last_used_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, Serialize, Debug)]
#[diesel(table_name = api_keys)]
pub struct NewApiKey {
    pub key_hash: String,
    pub is_admin: bool,
}
