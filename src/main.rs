extern crate log;

use dotenv::dotenv;
use axumbookstore::app::*;

pub mod common;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let routes = app().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes).await.unwrap();
}