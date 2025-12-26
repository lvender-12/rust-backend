use axum::{
    Router,
    routing::{get},
    middleware::from_fn,
};
use axum_test::TestServer;
use http::StatusCode;
use serde_json::json;

use crate::{
    configs::db, controllers::user_controller::{get_all_user, insert_user}, middlewares::api_middleware::api_key_middleware, routes::fallback::{fallback, not_allowed}
};

/// =======================
/// Helper
/// =======================

fn api_key() -> &'static str {
    "hgdshdfrhdrhdftjdftjfdtjdf"
}

fn app() -> Router {
    Router::new()
        .route("/user", get(get_all_user).post(insert_user))
        .layer(from_fn(api_key_middleware))
        .fallback(fallback)
        .method_not_allowed_fallback(not_allowed)
}

fn server() -> TestServer {
    TestServer::new(app()).unwrap()
}

/// =======================
/// TEST GET
/// =======================

#[tokio::test]
async fn get_user_success() {
    let server = server();

    let res = server
        .get("/user")
        .add_header("X-API-KEY", api_key())
        .await;

    assert_eq!(res.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn get_user_without_api_key() {
    let server = server();

    let res = server
        .get("/user")
        .await;

    assert_eq!(res.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn get_user_wrong_api_key() {
    let server = server();

    let res = server
        .get("/user")
        .add_header("X-API-KEY", "SALAH")
        .await;

    assert_eq!(res.status_code(), StatusCode::UNAUTHORIZED);
}

async fn cleanup_users() {
    let pool = db::get_pool().await.unwrap();
    sqlx::query("DELETE FROM users WHERE email LIKE '%@test.com'")
        .execute(&pool)
        .await
        .unwrap();
}

/// =======================
/// TEST POST
/// =======================

#[tokio::test]
async fn insert_user_success() {
    let server = server();

    let res = server
        .post("/user")
        .add_header("X-API-KEY", api_key())
        .json(&json!({
            "name": "damar",
            "email": "damar@test.com",
            "password": "rahasia123"
        }))
        .await;
    
    assert_eq!(res.status_code(), StatusCode::CREATED);
    cleanup_users().await;
}

#[tokio::test]
async fn insert_user_invalid_json() {
    let server = server();

    let res = server
        .post("/user")
        .add_header("X-API-KEY", api_key())
        .json("{ invalid json }")
        .await;

    assert_eq!(res.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn insert_user_without_api_key() {
    let server = server();

    let res = server
        .post("/user")
        .json(&json!({
            "name": "damar",
            "email": "damar@gmail.com",
            "password": "rahasia"
        }))
        .await;

    assert_eq!(res.status_code(), StatusCode::UNAUTHORIZED);
}

/// =======================
/// METHOD NOT ALLOWED
/// =======================

#[tokio::test]
async fn put_user_not_allowed() {
    let server = server();

    let res = server
        .put("/user")
        .add_header("X-API-KEY", api_key())
        .await;

    assert_eq!(res.status_code(), StatusCode::METHOD_NOT_ALLOWED);
}

/// =======================
/// FALLBACK
/// =======================

#[tokio::test]
async fn route_not_found() {
    let server = server();

    let res = server
        .get("/tidak-ada")
        .add_header("X-API-KEY", api_key())
        .await;

    assert_eq!(res.status_code(), StatusCode::NOT_FOUND);
}
