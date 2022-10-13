use crate::routes::auth::validate_login;
use crate::{
    data::{create_box, update_box, delete_box},
    State,
};
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    Extension,
};
use axum_extra::extract::cookie::CookieJar;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub enum WebError {
    Unauthorized,
}

impl WebError {
    pub fn get(self) -> (StatusCode, String) {
        match self {
            WebError::Unauthorized => (StatusCode::UNAUTHORIZED, "Already Logged In".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Box {
    title: String,
    body: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateBox {
    title: Option<String>,
    body: Option<String>,
}

pub async fn new_box(
    Json(body): Json<Box>,
    Extension(state): Extension<Arc<State>>,
    jar: CookieJar,
) -> Result<String, (StatusCode, String)> {
    if !validate_login(&jar) {
        return Err(WebError::Unauthorized.get());
    }

    create_box(&state.client, &body.title, &body.body)
        .await
        .expect("Failed to create Box");
    Ok(String::from("OK"))
}

// Updating the box
pub async fn upd_box(
    Json(body): Json<UpdateBox>,
    Path(id): Path<i32>,
    Extension(state): Extension<Arc<State>>,
    jar: CookieJar,
) -> Result<String, (StatusCode, String)> {
    if !validate_login(&jar) {
        return Err(WebError::Unauthorized.get());
    }

    match update_box(&state.client, id, body.title, body.body).await {
        Ok(()) => Ok(String::from("OK")),
        Err(_) => Err((StatusCode::NOT_FOUND, "Not Found".to_string())),
    }
}

pub async fn del_box(Path(id): Path<i32>, Extension(state): Extension<Arc<State>>,jar: CookieJar) -> Result<String, (StatusCode, String)> {
    if !validate_login(&jar) {
        return Err(WebError::Unauthorized.get());
    }

    match delete_box(&state.client, id).await {
        Ok(()) => Ok(String::from("OK")),
        Err(_) => Err((StatusCode::NOT_FOUND, "Not Found".to_string())),
    }}