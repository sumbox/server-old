use axum::{extract::Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::{env::var, fs};
use jsonwebtoken::{encode, Header, EncodingKey};
use std::time::SystemTime;
use axum_extra::extract::cookie::{CookieJar, Cookie};

pub async fn login(Json(body): Json<User>, jar:CookieJar) -> Result<(CookieJar, String), (StatusCode, String)> {
    if body.is_valid() {
        if jar.get("sumboxlogin").is_none() {
            let token = Claims::encode(&body);
            Ok(( jar.add(Cookie::new("sumboxlogin", token)), String::from("OK")))
        } else {
            return Err({
                (StatusCode::UNAUTHORIZED, "Already Logged In".to_string())
            })
        }
    }   else {
        return Err({
            (StatusCode::UNAUTHORIZED, "Invalid Credentials".to_string())
        })
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
