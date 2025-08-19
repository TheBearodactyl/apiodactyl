use std::env;

use diesel::{Connection, PgConnection};
use dotenvy::*;

pub fn connect_db() -> PgConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {db_url}"))
}