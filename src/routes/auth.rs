use axum::Json;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::env::var;

use axum::http::StatusCode;
use axum_extra::extract::{cookie::Cookie, CookieJar};
use serde::{Deserialize, Serialize};

use crate::routes::boxes::WebError;

#[derive(Deserialize)]
pub struct User {
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

pub async fn login(
    Json(body): Json<User>,
    jar: CookieJar,
) -> Result<(CookieJar, String), (StatusCode, String)> {
    if !body.is_valid() {
        return Err((StatusCode::UNAUTHORIZED, "Invalid Credentials".to_string()));
    }

    if !jar.get("sumboxlogin").is_none() {
        return Err((StatusCode::UNAUTHORIZED, "Already Logged In".to_string()));
    }

    let token = Claims::encode(&body);
    Ok((
        jar.add(Cookie::new("sumboxlogin", token)),
        String::from("OK"),
    ))
}

pub async fn logout(jar: CookieJar) -> Result<(CookieJar, String), (StatusCode, String)> {
    if !jar.get("sumboxlogin").is_some() {
        return Err(WebError::Unauthorized.get());
    }

    Ok((jar.remove(Cookie::named("sumboxlogin")), String::from("OK")))
}

pub fn validate_login(jar: &CookieJar) -> bool {
    let cookie: Option<&Cookie> = Some(jar.get("sumboxlogin")).unwrap_or(None);
    if cookie.is_none() {
        return false;
    }

    let mut validation = Validation::new(Algorithm::HS256);
    validation.sub = Some(var("AUTH_EMAIL").expect("AUTH_SECRET Should be set"));

    match decode::<Claims>(
        cookie.unwrap().value(),
        &DecodingKey::from_secret(
            var("AUTH_SECRET")
                .expect("AUTH_SECRET Should be set")
                .as_ref(),
        ),
        &validation,
    ) {
        Ok(_) => true,
        Err(_) => false,
    }
}

impl User {
    pub fn is_valid(&self) -> bool {
        let email = var("AUTH_EMAIL").expect("AUTH_EMAIL must be set");
        let password = var("AUTH_PASSWORD").expect("AUTH_PASSWORD must be set");

        self.email == email && self.password == password
    }
}

impl Claims {
    pub fn encode(user: &User) -> String {
        let claims = Claims {
            sub: user.email.clone(),
            exp: 10000000000,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(
                var("AUTH_SECRET")
                    .expect("AUTH_SECRET should be set")
                    .as_ref(),
            ),
        )
        .expect("Failed to encode cookie");

        token
    }
}
