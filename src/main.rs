mod routes;
use routes::*;

// use database::*;
// use server::schema;
// use server::models;
use server::init::*;
// use server::ideas::*;

use axum::{
    routing::{post}, Router
};

use std::net::SocketAddr;


#[tokio::main]
async fn main(){
    
    let _ = init();
    
    let app = Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/new", post(new_idea));
        // .layer(Extension(share));
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .expect("Failed to Start Server");
}