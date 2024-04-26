use axum::Router;
use axum::routing::{get, post};
use sqlx::PgPool;

use crate::book::handlers::{create_book, list_books};

mod types;
mod handlers;

pub fn get_books_routes() -> Router<PgPool> {
    let books_routes = Router::new()
        .route("/", post(create_book))
        .route("/", get(list_books));
    books_routes
}