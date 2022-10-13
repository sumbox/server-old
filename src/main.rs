mod routes;
mod boxes;
mod prisma;

use std::{sync::Arc, net::SocketAddr};
use routes::*;
use axum::{
    routing::{post}, Router, extract::Extension,
};
use prisma::PrismaClient;


#[tokio::main]
async fn main(){
    
    dotenvy::dotenv().ok();

    let client = prisma::new_client().await.expect("Failed to connect to Database. Is your DATABASE_URL set?");
    let state = Arc::new(State {client});

    let app = Router::new()
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
        .route("/api/box/new", post(new_box))
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