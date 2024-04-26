use std::env;
use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use http::StatusCode;
use jsonwebtoken::{Algorithm, encode, EncodingKey, Header};
use log::{error, info};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{server_error_response, validate_request};
use crate::common::error::APIError;
use crate::auth::types::{Claims, RegisterRequest, RegisterResponse, SignInRequest, User};

pub async fn sign_in(State(db_pool): State<PgPool>, Json(req): Json<SignInRequest>) -> impl IntoResponse {
    validate_request!(req);

    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, first_name, last_name, email, role, password FROM "user"
        WHERE email = $1
        "#,
        req.email
    )
        .fetch_one(&db_pool)
        .await;

    let user = match user {
        Ok(user) => user,
        Err(e) => {
            error!("Failed to fetch user: {}", e.to_string());
            return Err(APIError::Unauthorized("User does not exist".to_string()));
        }
    };

    if user.password != req.password {
        error!("Invalid password");
        return Err(APIError::Unauthorized("Invalid password".to_string()));
    }

    match generate_token(&user) {
        Ok(token) => Ok((StatusCode::OK, Json(token)).into_response()),
        Err(e) => {
            error!("Failed to generate token: {}", e.to_string());
            server_error_response!(e, "Failed to generate token".to_string())
        }
    }
}

fn generate_token(user: &User) -> Result<String, Box<dyn std::error::Error>> {
    let expiration = (chrono::Utc::now() + chrono::Duration::hours(12)).timestamp() as usize;
    let claims = Claims {
        exp: expiration,
        id: user.id.to_string(),
        first_name: user.first_name.to_string(),
        last_name: user.last_name.to_string(),
        email: user.email.to_string(),
        role: user.role.to_string(),
    };
    let header = Header::new(Algorithm::HS256);
    let secret_key = env::var("AUTH_SECRET_KEY")
        .expect("AUTH_SECRET_KEY environment variable not found");
    let key = EncodingKey::from_secret(secret_key.as_ref());

    encode(&header, &claims, &key).map_err(|e| e.into())
}

pub async fn register(State(db_pool): State<PgPool>, Json(req): Json<RegisterRequest>) -> impl IntoResponse {
    validate_request!(req);

    let id = Uuid::new_v4();
    let result = sqlx::query_as!(
        RegisterResponse,
        r#"
        INSERT INTO "user" (id, first_name, last_name, email, password, role)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, first_name, last_name, email, role
        "#,
        id.to_string(), req.first_name, req.last_name, req.email, req.password, req.role
    )
        .fetch_one(&db_pool)
        .await;

    match result {
        Ok(user) => {
            info!("User registered successfully");
            Ok((StatusCode::CREATED, Json(user).into_response()))
        }
        Err(e) => server_error_response!(e, "Failed to register user".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::{decode, DecodingKey, Validation};
    use super::*;

    #[test]
    fn test_generate_token() {
        // given
        env::set_var("AUTH_SECRET_KEY", "secret");
        let user = User {
            id: Uuid::new_v4().to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john.doe@gmail.com".to_string(),
            role: "user".to_string(),
            password: "password".to_string(),
        };
        let validation = Validation::new(Algorithm::HS256);
        let secret_key = env::var("AUTH_SECRET_KEY")
            .expect("AUTH_SECRET_KEY environment variable not found");

        // when
        let token = super::generate_token(&user).unwrap();
        let claims = decode::<Claims>(&token,
                                      &DecodingKey::from_secret(secret_key.as_bytes()),
                                      &validation).expect("Failed to decode token");

        // then
        assert!(!token.is_empty());
        assert_eq!(claims.claims.id, user.id);
        assert_eq!(claims.claims.first_name, user.first_name);
        assert_eq!(claims.claims.last_name, user.last_name);
        assert_eq!(claims.claims.email, user.email);
        assert_eq!(claims.claims.role, user.role);
    }
}