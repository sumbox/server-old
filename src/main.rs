use diesel::{pg::PgConnection, prelude::*};
use dotenvy::dotenv;
use std::{env, time::SystemTime};
use log::{info};

mod database;
// use database::*;
use server::schema;
mod routes;
use routes::*;
use server::models;

use axum::{
    routing::{get}, Router
};

use std::net::SocketAddr;
#[tokio::main]
async fn main(){
    let mut _connection = establish_connection();
    info!("Database Connection Established at {:?}", SystemTime::now());

    let app : Router<()> = Router::new()
        .route("/:id", get(index));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
