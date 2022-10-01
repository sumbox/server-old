use std::{fs, env};

use diesel::{PgConnection, Connection};
use dotenvy::dotenv;
use openssl::{rsa::Rsa, symm::Cipher};

pub fn init() -> (PgConnection, String) {
    dotenv().ok();

    if !fs::metadata("keys").is_ok() {
        fs::create_dir("keys").unwrap();
        generate_keys();
    }

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    return (conn, String::from(""))
}

fn generate_keys() {
    let rsa = Rsa::generate(1024).unwrap();
    let mut key = String::new();
    for _ in 0..32 {
        key.push((rand::random::<u8>() % 26 + 97) as char);
    }
    
    let private_key: Vec<u8> = rsa.private_key_to_pem_passphrase(Cipher::aes_128_cbc(), key.as_bytes()).unwrap();
    let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();
    
    fs::write("keys/private.pem", private_key).unwrap();
    fs::write("keys/public.pem", public_key).unwrap();
}