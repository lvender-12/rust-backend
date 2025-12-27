use axum::{
    Router,
    middleware::from_fn,
    routing::{get, post, delete},
};
use axum_test::TestServer;
use http::StatusCode;
use serde_json::json;

use crate::{
    configs::db,
    controllers::user_controller::{
        get_all_user,
        get_user,
        insert_user,
        delete_user,
    },
    middlewares::api_middleware::api_key_middleware,
    routes::fallback::{fallback, not_allowed},
};

/// =======================
/// Helper
/// =======================

fn api_key() -> &'static str {
    "hgdshdfrhdrhdftjdftjfdtjdf"
}

fn app() -> Router {
    Router::new()
        .route("/user", get(get_all_user))
        .route("/user", post(insert_user))
        .route("/user/search", post(get_user))
        .route("/user/{email}", delete(delete_user))
        .layer(from_fn(api_key_middleware))
        .fallback(fallback)
        .method_not_allowed_fallback(not_allowed)
}

fn server() -> TestServer {
    TestServer::new(app()).unwrap()
}

async fn cleanup_users() {
    let pool = db::get_pool().await.unwrap();
    sqlx::query("DELETE FROM users WHERE email LIKE '%@test.com'")
        .execute(&pool)
        .await
        .unwrap();
}

/// =======================
/// TEST GET /user
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

    let res = server.get("/user").await;

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

/// =======================
/// TEST POST /user
/// =======================

#[tokio::test]
async fn insert_user_success() {
    let server = server();
    let email = "insert_user@test.com";

    let res = server
        .post("/user")
        .add_header("X-API-KEY", api_key())
        .json(&json!({
            "name": "Insert Test",
            "email": email,
            "password": "123456"
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
            "name": "Test",
            "email": "test@test.com",
            "password": "123456"
        }))
        .await;

    assert_eq!(res.status_code(), StatusCode::UNAUTHORIZED);
}

/// =======================
/// TEST POST /user/search
/// =======================

#[tokio::test]
async fn search_user_by_name_success() {
    let server = server();

    server
        .post("/user")
        .add_header("X-API-KEY", api_key())
        .json(&json!({
            "name": "SearchName",
            "email": "search_name@test.com",
            "password": "123456"
        }))
        .await;

    let res = server
        .post("/user/search")
        .add_header("X-API-KEY", api_key())
        .json(&json!({
            "by": "name",
            "value": "SearchName"
        }))
        .await;

    assert_eq!(res.status_code(), StatusCode::OK);

    let body: serde_json::Value = res.json();
    assert!(body.as_array().unwrap().len() > 0);

    cleanup_users().await;
}

#[tokio::test]
async fn search_user_by_email_success() {
    let server = server();
    let email = "search_email@test.com";

    server
        .post("/user")
        .add_header("X-API-KEY", api_key())
        .json(&json!({
            "name": "SearchEmail",
            "email": email,
            "password": "123456"
        }))
        .await;

    let res = server
        .post("/user/search")
        .add_header("X-API-KEY", api_key())
        .json(&json!({
            "by": "email",
            "value": email
        }))
        .await;

    assert_eq!(res.status_code(), StatusCode::OK);

    let body: serde_json::Value = res.json();
    assert!(body.as_array().unwrap().len() > 0);

    cleanup_users().await;
}

/// =======================
/// TEST DELETE /user/{email}
/// =======================

#[tokio::test]
async fn delete_user_success() {
    let server = server();
    let email = "delete_user@test.com";

    server
        .post("/user")
        .add_header("X-API-KEY", api_key())
        .json(&json!({
            "name": "Delete User",
            "email": email,
            "password": "123456"
        }))
        .await;

    let res = server
        .delete(&format!("/user/{}", email))
        .add_header("X-API-KEY", api_key())
        .await;

    assert_eq!(res.status_code(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn delete_user_not_found() {
    let server = server();

    let res = server
        .delete("/user/not_found@test.com")
        .add_header("X-API-KEY", api_key())
        .await;

    assert_eq!(res.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_user_without_api_key() {
    let server = server();

    let res = server
        .delete("/user/delete_user@test.com")
        .await;

    assert_eq!(res.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn delete_user_wrong_api_key() {
    let server = server();

    let res = server
        .delete("/user/delete_user@test.com")
        .add_header("X-API-KEY", "SALAH")
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
