use std::{time::SystemTime};
use log::{info};

mod utils;
mod routes;
mod database;

use utils::*;
use routes::*;
// use database::*;
use server::schema;
use server::models;

use axum::{
    routing::{post}, Router
};

use std::net::SocketAddr;
#[tokio::main]
async fn main(){
    let (mut _connection, _) = init();
    info!("Database Connection Established at {:?}", SystemTime::now());

    let app = Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout));
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .expect("Failed to Start Server");
}