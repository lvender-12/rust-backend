use axum::{
    Router, middleware::from_fn, routing::{delete, get, post, put}
};
use axum_test::TestServer;
use http::StatusCode;
use serde_json::json;

use crate::{
    configs::db, controllers::user_controller::{delete_user, edit_user, get_all_user, get_user, get_user_edit, insert_user}, middlewares::api_middleware::api_key_middleware, routes::fallback::{fallback, not_allowed}
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
        .route("/user", delete(delete_user))
        .route("/user/edit", get(get_user_edit)) // query param
        .route("/user/{id}", put(edit_user))     // path param
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
/// GET /user
/// =======================

#[tokio::test]
async fn get_user_success() {
    let server = server();
    let res = server.get("/user").add_header("X-API-KEY", api_key()).await;
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
    let res = server.get("/user").add_header("X-API-KEY", "SALAH").await;
    assert_eq!(res.status_code(), StatusCode::UNAUTHORIZED);
}

/// =======================
/// POST /user
/// =======================

#[tokio::test]
async fn insert_user_success() {
    cleanup_users().await;

    let server = server();
    let email = "damar_insert@test.com";

    let res = server.post("/user")
        .add_header("X-API-KEY", api_key())
        .json(&json!({
            "name": "Damar",
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
    let res = server.post("/user")
        .add_header("X-API-KEY", api_key())
        .json("{ invalid json }")
        .await;

    assert_eq!(res.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn insert_user_without_api_key() {
    let server = server();
    let res = server.post("/user")
        .json(&json!({
            "name": "Damar",
            "email": "damar@test.com",
            "password": "123456"
        }))
        .await;

    assert_eq!(res.status_code(), StatusCode::UNAUTHORIZED);
}

/// =======================
/// PUT /user/{id} Method Not Allowed
/// =======================

#[tokio::test]
async fn put_user_not_allowed() {
    let server = server();
    let res = server.put("/user").add_header("X-API-KEY", api_key()).await;
    assert_eq!(res.status_code(), StatusCode::METHOD_NOT_ALLOWED);
}

/// =======================
/// FALLBACK
/// =======================

#[tokio::test]
async fn route_not_found() {
    let server = server();
    let res = server.get("/tidak-ada").add_header("X-API-KEY", api_key()).await;
    assert_eq!(res.status_code(), StatusCode::NOT_FOUND);
}

/// =======================
/// SEARCH /user/search
/// =======================

#[tokio::test]
async fn search_user_by_name_success() {
    cleanup_users().await;

    let server = server();
    let email = "damar_search@test.com";

    server.post("/user")
        .add_header("X-API-KEY", api_key())
        .json(&json!({
            "name": "DamarSearch",
            "email": email,
            "password": "123456"
        }))
        .await;

    let res = server.post("/user/search")
        .add_header("X-API-KEY", api_key())
        .json(&json!({"by": "name", "value": "DamarSearch"}))
        .await;

    assert_eq!(res.status_code(), StatusCode::OK);
    let body: serde_json::Value = res.json::<serde_json::Value>();
    assert!(body.as_array().unwrap().len() > 0);

    cleanup_users().await;
}

#[tokio::test]
async fn search_user_by_email_success() {
    cleanup_users().await;

    let server = server();
    let email = "damar_email@test.com";

    server.post("/user")
        .add_header("X-API-KEY", api_key())
        .json(&json!({
            "name": "DamarEmail",
            "email": email,
            "password": "123456"
        }))
        .await;

    let res = server.post("/user/search")
        .add_header("X-API-KEY", api_key())
        .json(&json!({"by": "email", "value": email}))
        .await;

    assert_eq!(res.status_code(), StatusCode::OK);
    let body: serde_json::Value = res.json::<serde_json::Value>();
    assert!(body.as_array().unwrap().len() > 0);

    cleanup_users().await;
}

#[tokio::test]
async fn search_user_without_api_key() {
    let server = server();
    let res = server.post("/user/search")
        .json(&json!({"by": "name", "value": "Damar"}))
        .await;

    assert_eq!(res.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn search_user_wrong_api_key() {
    let server = server();
    let res = server.post("/user/search")
        .add_header("X-API-KEY", "SALAH")
        .json(&json!({"by": "name", "value": "Damar"}))
        .await;

    assert_eq!(res.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn search_user_invalid_json() {
    let server = server();
    let res = server.post("/user/search")
        .add_header("X-API-KEY", api_key())
        .json("{ invalid json }")
        .await;

    assert_eq!(res.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
}

/// =======================
/// GET /user/edit (query param)
/// =======================

#[tokio::test]
async fn get_user_edit_success() {
    cleanup_users().await;

    let pool = db::get_pool().await.unwrap();
    let rec = sqlx::query("INSERT INTO users (name,email,password) VALUES (?, ?, ?)")
        .bind("DamarEdit")
        .bind("damar_edit@test.com")
        .bind("123456")
        .execute(&pool)
        .await
        .unwrap();
    let user_id = rec.last_insert_id();

    let server = server();
    let response = server.get("/user/edit")
        .add_header("X-API-KEY", api_key())
        .add_query_param("id", &user_id.to_string())
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    cleanup_users().await;
}

/// =======================
/// DELETE /user (query param)
/// =======================

#[tokio::test]
async fn delete_user_success() {
    cleanup_users().await;

    let pool = db::get_pool().await.unwrap();

    let rec = sqlx::query("INSERT INTO users (name,email,password) VALUES (?, ?, ?)")
        .bind("DamarEdit")
        .bind("damar_edit@test.com")
        .bind("123456")
        .execute(&pool)
        .await
        .unwrap();

    // Ambil ID terakhir
    let user_id = rec.last_insert_id();
    println!("Inserted user id: {}", user_id);

    let server = server();
    let response = server.delete("/user")
        .add_header("X-API-KEY", api_key())
        .add_query_param("id", &user_id.to_string())
        .await;

    assert_eq!(response.status_code(), StatusCode::NO_CONTENT);

    cleanup_users().await;
}

#[tokio::test]
async fn delete_user_not_found() {
    let server = server();
    let response = server.delete("/user")
        .add_header("X-API-KEY", api_key())
        .add_query_param("id", "999999") // ID yang tidak ada
        .await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_user_without_api_key() {
    let server = server();
    let response = server.delete("/user")
        .add_query_param("id", "1")
        .await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn delete_user_wrong_api_key() {
    let server = server();
    let response = server.delete("/user")
        .add_header("X-API-KEY", "SALAH")
        .add_query_param("id", "1")
        .await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

/// =======================
/// PUT /user/{id} (path param)
/// =======================

#[tokio::test]
async fn edit_user_success() {
    cleanup_users().await;

    let pool = db::get_pool().await.unwrap();

    // Insert user dummy
    let rec = sqlx::query("INSERT INTO users (name,email,password) VALUES (?, ?, ?)")
        .bind("DamarEdit")
        .bind("damar_edit@test.com")
        .bind("123456")
        .execute(&pool)
        .await
        .unwrap();

    let user_id = rec.last_insert_id();
    println!("Inserted user id: {}", user_id);

    let server = server();

    // Kirim semua field yang required
    let response = server.put(&format!("/user/{}", user_id))
        .add_header("X-API-KEY", api_key())
        .json(&json!({
            "name": "DamarEdited",
            "email": "damar_edit@test.com"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    // Optionally, cek hasil edit
    let body: serde_json::Value = response.json::<serde_json::Value>();
    assert_eq!(body["name"], "DamarEdited");
    assert_eq!(body["email"], "damar_edit@test.com");

    cleanup_users().await;
}
