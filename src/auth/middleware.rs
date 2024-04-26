use std::env;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use jsonwebtoken::{Algorithm, decode, DecodingKey, Validation};
use log::error;

use crate::common::error::APIError;
use crate::auth::types::Claims;

pub async fn authn_middleware(req: Request, next: Next) -> Result<impl IntoResponse, Response> {
    let unauthorized_error = Err(APIError::Unauthorized("Invalid Authorization header".to_string()).into_response());

    let auth_header = match req.headers().get("Authorization") {
        None => {
            error!("Authorization header not found");
            return unauthorized_error
        }
        Some(val) => val
    };

    let auth_header =match auth_header.to_str() {
        Ok(val) => val,
        Err(e) => {
            error!("Failed to read Authorization header: {:?}", e);
            return unauthorized_error;
        }
    };

    if !auth_header.starts_with("Bearer ") {
        error!("Invalid Authorization header: {}", auth_header);
        return unauthorized_error;
    }

    let token = &auth_header[7..];
    let validation = Validation::new(Algorithm::HS256);

    let secret_key = match env::var("AUTH_SECRET_KEY") {
        Ok(k) => k,
        Err(e) => {
            error!("Failed to read AUTH_SECRET_KEY environment variable: {:?}", e);
            return Err(APIError::ServerError("Something went wrong".to_string()).into_response());
        }
    };

    match decode::<Claims>(token, &DecodingKey::from_secret(secret_key.as_bytes()), &validation) {
        Ok(_) => { Ok(next.run(req).await) }
        Err(e) => {
            error!("Failed to decode token: {:?}", e);
            return unauthorized_error;
        }
    }
}