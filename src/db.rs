use std::env;

use diesel::{Connection, PgConnection};
use dotenvy::*;

pub fn connect_db() -> PgConnection {
    dotenv().ok();
    let dburl = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&dburl).unwrap_or_else(|_| panic!("Error connecting to {dburl}"))
}
