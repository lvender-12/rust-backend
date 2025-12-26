use axum::{
    Router, middleware::from_fn, routing::{get, post}
};
use axum_test::TestServer;
use http::StatusCode;
use serde_json::json;

use crate::{
    configs::db, controllers::user_controller::{get_all_user, get_user, insert_user}, middlewares::api_middleware::api_key_middleware, routes::fallback::{fallback, not_allowed}
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


/// =======================
/// TEST POST
/// =======================

#[tokio::test]
async fn insert_user_success() {
    cleanup_users().await; // bersihkan dulu

    let server = server();

    let unique_email = format!("damar_{}@test.com", chrono::Utc::now().timestamp());

    let res = server
        .post("/user")
        .add_header("X-API-KEY", api_key())
        .json(&json!({
            "name": "damar",
            "email": unique_email,
            "password": "rahasia123"
        }))
        .await;

    assert_eq!(res.status_code(), StatusCode::CREATED);

    cleanup_users().await; // bersihkan lagi
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


/// =======================
/// TEST POST /user/search
/// =======================

#[tokio::test]
async fn search_user_by_name_success() {
    let server = server();

    // insert dummy user
    server
        .post("/user")
        .add_header("X-API-KEY", api_key())
        .json(&json!({
            "name": "DamarTest",
            "email": "damar@test.com",
            "password": "123456"
        }))
        .await;

    let res = server
        .post("/user/search")
        .add_header("X-API-KEY", api_key())
        .json(&json!({
            "by": "name",
            "value": "DamarTest"
        }))
        .await;

    assert_eq!(res.status_code(), StatusCode::OK);

    // <-- perbaikan: tambahkan tipe generic di sini
    let body: serde_json::Value = res.json::<serde_json::Value>();

    assert!(body.as_array().unwrap().len() > 0);

    cleanup_users().await;
}


#[tokio::test]
async fn search_user_by_email_success() {
    let server = server();

    // insert dummy user
    server
        .post("/user")
        .add_header("X-API-KEY", api_key())
        .json(&json!({
            "name": "DamarTest",
            "email": "damar_search@test.com",
            "password": "123456"
        }))
        .await;

    let res = server
        .post("/user/search")
        .add_header("X-API-KEY", api_key())
        .json(&json!({
            "by": "email",
            "value": "damar_search@test.com"
        }))
        .await;

    assert_eq!(res.status_code(), StatusCode::OK);

    // <-- tambahkan generic type di sini
    let body: serde_json::Value = res.json::<serde_json::Value>();

    assert!(body.as_array().unwrap().len() > 0);

    cleanup_users().await;
}

#[tokio::test]
async fn search_user_without_api_key() {
    let server = server();

    let res = server
        .post("/user/search")
        .json(&json!({
            "by": "name",
            "value": "DamarTest"
        }))
        .await;

    assert_eq!(res.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn search_user_wrong_api_key() {
    let server = server();

    let res = server
        .post("/user/search")
        .add_header("X-API-KEY", "SALAH")
        .json(&json!({
            "by": "name",
            "value": "DamarTest"
        }))
        .await;

    assert_eq!(res.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn search_user_invalid_json() {
    let server = server();

    let res = server
        .post("/user/search")
        .add_header("X-API-KEY", api_key())
        .json("{ invalid json }")
        .await;

    assert_eq!(res.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
}