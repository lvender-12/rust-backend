use axum::{
    Router, middleware::from_fn, routing::{delete, get, post, put}
};
use axum_test::TestServer;
use http::StatusCode;
use serde_json::json;

use crate::{
    configs::db,
    controllers::user_controller::{delete_user, edit_user, get_all_user, get_user, get_user_edit, insert_user, login_user},
    middlewares::api_middleware::{api_key_middleware},
    routes::fallback::{fallback, not_allowed},
    utils::utils::create_jwt
};

/// =======================
/// Helper Functions
/// =======================

fn api_key() -> &'static str {
    "hgdshdfrhdrhdftjdftjfdtjdf"
}

fn app() -> Router {
    Router::new()
        .route("/user", get(get_all_user))
        .route("/user", post(insert_user))
        .route("/user/search", post(get_user))
        .route("/user", delete(delete_user))
        .route("/user/edit", get(get_user_edit))
        .route("/user/{id}", put(edit_user))
        .layer(from_fn(api_key_middleware))
        .layer(from_fn(crate::middlewares::api_middleware::check_login))
        .fallback(fallback)
        .method_not_allowed_fallback(not_allowed)
}

fn guest_app() -> Router {
    Router::new()
        .route("/login", post(login_user))
        .layer(from_fn(api_key_middleware))
        .layer(from_fn(crate::middlewares::api_middleware::check_guest))
}

fn server() -> TestServer {
    TestServer::new(app()).unwrap()
}

fn guest_server() -> TestServer {
    TestServer::new(guest_app()).unwrap()
}

async fn cleanup_users() {
    let pool = db::get_pool().await.unwrap();
    sqlx::query("DELETE FROM users WHERE email LIKE '%@test.com'")
        .execute(&pool)
        .await
        .unwrap();
}

async fn get_jwt(user_id: u64) -> String {
    create_jwt(user_id).unwrap()
}

/// =======================
/// Guest Route Tests (/login)
/// =======================

#[tokio::test]
async fn guest_can_access_login() {
    let server = guest_server();
    let res = server.post("/login")
        .add_header("X-API-KEY", api_key())
        .json(&json!({"email": "guest@test.com","password": "123456"}))
        .await;
    assert_ne!(res.status_code(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn logged_in_cannot_access_login() {
    let server = guest_server();
    let token = get_jwt(1).await;
    let cookie = format!("jwt={}", token);

    let res = server.post("/login")
        .add_header("X-API-KEY", api_key())
        .add_header("Cookie", &cookie)
        .json(&json!({"email": "guest@test.com","password": "123456"}))
        .await;

    assert_eq!(res.status_code(), StatusCode::FORBIDDEN);
}

/// =======================
/// GET /user Tests (all combinations)
/// =======================

#[tokio::test]
async fn get_user_no_api_no_token() {
    let server = server();
    let res = server.get("/user").await;
    assert_eq!(res.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn get_user_api_key_no_token() {
    let server = server();
    let res = server.get("/user")
        .add_header("X-API-KEY", api_key())
        .await;
    assert_eq!(res.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn get_user_no_api_token() {
    let server = server();
    let token = get_jwt(1).await;
    let cookie = format!("jwt={}", token);
    let res = server.get("/user")
        .add_header("Cookie", &cookie)
        .await;
    assert_eq!(res.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn get_user_api_key_valid_token() {
    cleanup_users().await;
    let server = server();
    let token = get_jwt(1).await;
    let cookie = format!("jwt={}", token);

    let res = server.get("/user")
        .add_header("X-API-KEY", api_key())
        .add_header("Cookie", &cookie)
        .await;

    assert_eq!(res.status_code(), StatusCode::OK);
}

/// =======================
/// POST /user Tests (all combinations)
/// =======================

#[tokio::test]
async fn insert_user_no_api_no_token() {
    cleanup_users().await;
    let server = server();
    let res = server.post("/user")
        .json(&json!({"name": "TestUser","email": "noapi@test.com","password": "123456"}))
        .await;
    assert_eq!(res.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn insert_user_api_key_no_token() {
    cleanup_users().await;
    let server = server();
    let res = server.post("/user")
        .add_header("X-API-KEY", api_key())
        .json(&json!({"name": "TestUser","email": "apikey@test.com","password": "123456"}))
        .await;
    assert_eq!(res.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn insert_user_no_api_token() {
    cleanup_users().await;
    let server = server();
    let token = get_jwt(1).await;
    let cookie = format!("jwt={}", token);
    let res = server.post("/user")
        .add_header("Cookie", &cookie)
        .json(&json!({"name": "TestUser","email": "notoken@test.com","password": "123456"}))
        .await;
    assert_eq!(res.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn insert_user_api_key_valid_token() {
    cleanup_users().await;
    let server = server();
    let token = get_jwt(1).await;
    let cookie = format!("jwt={}", token);

    let res = server.post("/user")
        .add_header("X-API-KEY", api_key())
        .add_header("Cookie", &cookie)
        .json(&json!({"name": "ValidUser","email": "valid@test.com","password": "123456"}))
        .await;

    assert_eq!(res.status_code(), StatusCode::CREATED);
    cleanup_users().await;
}

/// =======================
/// DELETE /user Tests (all combinations)
/// =======================

#[tokio::test]
async fn delete_user_api_key_valid_token() {
    cleanup_users().await;
    let server = server();
    let token = get_jwt(1).await;
    let cookie = format!("jwt={}", token);

    // Insert dulu
    let pool = db::get_pool().await.unwrap();
    let rec = sqlx::query("INSERT INTO users (name,email,password) VALUES (?, ?, ?)")
        .bind("DelUser")
        .bind("delete@test.com")
        .bind("123456")
        .execute(&pool)
        .await
        .unwrap();
    let user_id = rec.last_insert_id();

    let res = server.delete("/user")
        .add_header("X-API-KEY", api_key())
        .add_header("Cookie", &cookie)
        .add_query_param("id", &user_id.to_string())
        .await;

    assert_eq!(res.status_code(), StatusCode::NO_CONTENT);
    cleanup_users().await;
}

/// =======================
/// PUT /user/{id} Tests (edit user, all combinations)
/// =======================

#[tokio::test]
async fn edit_user_api_key_valid_token() {
    cleanup_users().await;
    let server = server();
    let token = get_jwt(1).await;
    let cookie = format!("jwt={}", token);

    let pool = db::get_pool().await.unwrap();
    let rec = sqlx::query("INSERT INTO users (name,email,password) VALUES (?, ?, ?)")
        .bind("EditUser")
        .bind("edit@test.com")
        .bind("123456")
        .execute(&pool)
        .await
        .unwrap();
    let user_id = rec.last_insert_id();

    let res = server.put(&format!("/user/{}", user_id))
        .add_header("X-API-KEY", api_key())
        .add_header("Cookie", &cookie)
        .json(&json!({"name": "EditedUser","email": "edit@test.com"}))
        .await;

    assert_eq!(res.status_code(), StatusCode::OK);
    cleanup_users().await;
}
