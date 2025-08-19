use diesel::QueryDsl;
use diesel::{ExpressionMethods, OptionalExtension, RunQueryDsl};
use dotenvy::dotenv;
use libapiodactyl::db::connect_db;
use libapiodactyl::models::NewApiKey;
use libapiodactyl::schema::api_keys;
use sha2::{Digest, Sha256};
use std::env;

fn hash_api_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hex::encode(hasher.finalize())
}

fn main() {
    dotenv().ok();

    let admin_key = match env::var("BEARO_API_TOKEN") {
        Ok(key) => {
            if key.is_empty() {
                eprintln!("Error: BEARO_API_TOKEN not found");
                std::process::exit(1);
            }
            key
        }
        Err(_) => {
            eprintln!("Error: BEARO_API_TOKEN must be set");
            eprintln!("Usage: BEARO_API_TOKEN=your_secret_key cargo run --bin setup_admin");
            std::process::exit(1);
        }
    };

    if admin_key.len() < 32 {
        eprintln!("Warning: Admin key is shorter than recommended (32+ characters)");
    }

    if !admin_key.starts_with("ak_") {
        eprintln!("Warning: Admin key doesn't follow expected format (ak_...)");
    }

    println!("Setting up admin key");
    let key_hash = hash_api_key(&admin_key);

    let mut conn = connect_db();
    let existing_admin = api_keys::table
        .filter(api_keys::is_admin.eq(true))
        .first::<libapiodactyl::models::ApiKey>(&mut conn)
        .optional();

    match existing_admin {
        Ok(Some(_)) => {
            println!("Admin key already exists.");
            std::process::exit(0);
        }
        Ok(None) => {
            let new_admin_key = NewApiKey {
                key_hash,
                is_admin: true,
            };

            match diesel::insert_into(api_keys::table)
                .values(&new_admin_key)
                .execute(&mut conn)
            {
                Ok(_) => {
                    println!("admin token created successfully!");
                }
                Err(e) => {
                    eprintln!("failed to create admin token: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("failed to check for existing admin token: {}", e);
            std::process::exit(1);
        }
    }
}
