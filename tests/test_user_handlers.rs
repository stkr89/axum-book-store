use axum::body::Body;
use http::{Request, StatusCode};
use tower::util::ServiceExt;

use axumbookstore::app::app;
use axumbookstore::auth::types::RegisterRequest;

mod utils;

#[tokio::test]
async fn test_root() {
    utils::common();
    let app = app().await;
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_should_return_bad_request() {
    // given
    utils::common();
    let app = app().await;
    let request = RegisterRequest {
        first_name: "".to_string(),
        last_name: "".to_string(),
        email: "".to_string(),
        password: "".to_string(),
        role: "".to_string(),
    };
    let request_body = Body::from(serde_json::to_string(&request).unwrap());

    // when
    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/auth/register")
                .header("Content-Type", "application/json")
                .body(request_body)
                .unwrap()
        )
        .await
        .unwrap();

    // then
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}