use axum::Router;
use axum::routing::post;
use sqlx::PgPool;

pub mod handlers;
pub mod types;
pub mod middleware;

pub fn get_auth_routes() -> Router<PgPool> {
    let auth_routes = Router::new()
        .route("/register", post(handlers::register))
        .route("/sign-in", post(handlers::sign_in));
    auth_routes
}