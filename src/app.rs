use axum::{middleware, Router};
use axum::routing::get;
use crate::auth::middleware::authn_middleware;
use crate::{auth, book};
use crate::configs::get_db_pool;

pub async fn app() -> Router {
    let authenticated_routes = Router::new()
        .nest("/books", book::get_books_routes())
        .layer(middleware::from_fn(authn_middleware));

    let pool = get_db_pool().await;
    let routes = Router::new()
        .route("/", get(root))
        .nest("/auth", auth::get_auth_routes())
        .nest("/api", authenticated_routes)
        .with_state(pool);
    routes
}

async fn root() -> &'static str {
    "Running on port 3000 "
}