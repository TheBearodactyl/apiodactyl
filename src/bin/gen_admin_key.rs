use sha2::{Digest, Sha256};
use uuid::Uuid;

fn main() {
    let admin_key = format!("ak_{}", Uuid::new_v4().simple());
    let mut hasher = Sha256::new();
    hasher.update(admin_key.as_bytes());
    let key_hash = hex::encode(hasher.finalize());

    println!("Admin Key: {}", admin_key);
    println!("Admin Key Hash (use this in your migration): {}", key_hash);
    println!("\nAppend the below to your `create_api_keys` migration:");
    println!(
        "INSERT INTO api_keys (key_hash, is_admin) VALUES ('{}', TRUE);",
        key_hash
    );
}
