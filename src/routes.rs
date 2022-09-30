use axum::extract::Path;

pub async fn index(Path(id): Path<String>) -> String {
    format!("Hello, {}!", &id)
}