use axum::{extract::Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use server::ideas::create_idea;
use std::{env::var, time::{Duration}};
use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation, Algorithm, errors::ErrorKind};
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

pub async fn logout(jar:CookieJar) -> Result<(CookieJar, String), (StatusCode, String)> {
    if jar.get("sumboxlogin").is_some() {
        Ok((jar.remove(Cookie::named("sumboxlogin")), String::from("OK")))
    } else {
        return Err({
            (StatusCode::UNAUTHORIZED, "Not Logged In".to_string())
        })
    }
}

pub async fn new_idea(Json(body) : Json<Idea>, jar: CookieJar) -> Result<String, (StatusCode, String)> {
    if validate_login(&jar) {
        create_idea(&body.title, &body.body);
        Ok(String::from("OK"))
    } else {
        return Err({
            (StatusCode::UNAUTHORIZED, "Not Logged In".to_string())
        })
    }

}


fn validate_login(jar: &CookieJar) -> bool {
    let cookie : Option<&Cookie> = Some(jar.get("sumboxlogin")).unwrap_or(None);
    if cookie.is_none() {
        return false
    }

    let mut validation = Validation::new(Algorithm::HS256);
    validation.sub = Some(var("AUTH_EMAIL").expect("AUTH_SECRET Should be set"));
 
   match decode::<Claims>(&cookie.unwrap().value(), &DecodingKey::from_secret(var("AUTH_SECRET").expect("AUTH_SECRET Should be set").as_ref()), &validation) {
        Ok(_) => true,
        Err(_) => false,
    }
}


#[derive(Deserialize)]
pub struct User {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Idea {
    title: String,
    body: String
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
    sub: String,
    exp: usize,
}

impl Claims {
    pub fn encode(user: &User) -> String {
        let claims = Claims {
            sub: user.email.clone(),
            exp: 10000000000
        };

        let token = encode(
            &Header::default(), &claims,
             &EncodingKey::from_secret(var("AUTH_SECRET").expect("AUTH_SECRET should be set").as_ref())).expect("Failed to encode cookie");

        return token
    }
}
