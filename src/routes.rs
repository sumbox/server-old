use axum::{extract::Json, http::{StatusCode}};
use serde::{Deserialize, Serialize};
use std::{env::var, fs};
use jsonwebtoken::{encode, Header, EncodingKey};
use std::time::SystemTime;

pub async fn login(Json(body): Json<User>) -> (StatusCode, &'static str) {
    if body.is_valid() {
        let _token = Claims::encode(&body);
        
        return (StatusCode::OK, "OK");
    }   else {
        return (StatusCode::UNAUTHORIZED, "Invalid Credentials")
    }
}

#[derive(Deserialize, Debug)]
pub struct User {
    email: String,
    password: String,
}

impl User {
    pub fn is_valid(&self) -> bool {
        let email = var("AUTH_EMAIL").expect("AUTH_EMAIL must be set");
        let password = var("AUTH_PASSWORD").expect("AUTH_PASSWORD must be set");

        if self.email==email && self.password==password {
            return true
        } else {
            return false
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    email: String,
    exp: usize,
}

impl Claims {
    pub fn encode(user: &User) -> String {
        let claims = Claims {
            email: user.email.clone(),
            exp: {
                (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards").as_secs() as usize / 1000) + 10*60
            },
        };

        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(fs::read_to_string("keys/private.pem").expect("Failed to open Keys file").as_ref())).expect("Failed to encode cookie");

        return token
    }
}