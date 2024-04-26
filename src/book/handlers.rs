use log::{error, info};
use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use http::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{server_error_response, validate_request};
use crate::book::types::{BookResponse, CreateBookRequest};
use crate::common::error::APIError;

pub async fn list_books(State(db_pool): State<PgPool>) -> impl IntoResponse {
    let result = sqlx::query_as!(
        BookResponse,
        r#"
        SELECT id, title, pages, author FROM book
        "#,
    )
        .fetch_all(&db_pool)
        .await;

    match result {
        Ok(books) => {
            Ok((StatusCode::OK, Json(books).into_response()))
        }
        Err(e) => server_error_response!(e, "Failed to list books".to_string()),
    }
}

pub async fn create_book(State(db_pool): State<PgPool>, Json(req): Json<CreateBookRequest>) -> impl IntoResponse {
    validate_request!(req);

    let id = Uuid::new_v4();
    let result = sqlx::query_as!(
        BookResponse,
        r#"
        INSERT INTO public.book(id, title, pages, author)
        VALUES ($1, $2, $3, $4)
        RETURNING id, title, pages, author
        "#,
        id.to_string(), req.title, req.pages, req.author
    )
        .fetch_one(&db_pool)
        .await;

    match result {
        Ok(book) => {
            info!("Book created successfully with id: {}", book.id);
            Ok((StatusCode::CREATED, Json(book).into_response()))
        }
        Err(e) => server_error_response!(e, "Failed to create book".to_string()),
    }
}