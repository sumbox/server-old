mod routes;
mod data;
mod prisma;

use crate::routes::auth::{logout, login};
use std::{sync::Arc, net::SocketAddr};
use axum::{
    routing::{post}, Router, extract::Extension,
};
use prisma::PrismaClient;
use crate::routes::boxes::{new_box, upd_box};

#[tokio::main]
async fn main(){
    dotenvy::dotenv().ok();

    let client = prisma::new_client().await.expect("Failed to connect to Database. Is your DATABASE_URL set?");
    let state = Arc::new(State {client});

    let app = Router::new()
        .route("/api/login", post(login))
        .route("/api/logout", post(logout))
        .route("/api/box/new", post(new_box))
        .route("/api/box/update/:id", post(upd_box))
        .layer(Extension(state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .expect("Failed to Start Server");
}

pub struct State {
    pub client: PrismaClient
}