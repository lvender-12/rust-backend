// test/user_testing.rs
use axum::{Router, routing::{get, post}};
use axum_test::TestServer;
use rand::{distributions::Alphanumeric, Rng};
use crate::{
    controllers::user_controller::{get_all_user, insert_user},
    configs::db,
    models::user_model::UserInsert
};
use http::StatusCode;

/// Fungsi buat setup router
fn app() -> Router {
    Router::new()
        .route("/user", post(insert_user))
        .route("/user", get(get_all_user))
}

/// Generate email random untuk test
fn random_email() -> String {
    let rand_str: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    format!("{}@test.com", rand_str)
}

/// Hapus user test yang dibuat (cleanup)
async fn cleanup_test_users() {
    let pool = db::get_pool().await.unwrap();
    sqlx::query("DELETE FROM users WHERE email LIKE '%@test.com'")
        .execute(&pool).await.unwrap();
}

#[tokio::test]
async fn test_insert_user_success() {
    cleanup_test_users().await;

    let server = TestServer::new(app()).unwrap();

    let request = UserInsert {
        name: "Damar Test".to_string(),
        email: random_email(),
        password:"123456".to_string(),
    };

    let response = server.post("/user").json(&request).await;

    response.assert_status(StatusCode::CREATED);
    response.assert_text_contains("Berhasil memasukan data");

    cleanup_test_users().await;
}

#[tokio::test]
async fn test_insert_user_bad_request() {
    cleanup_test_users().await;

    let server = TestServer::new(app()).unwrap();

    // Nama terlalu pendek → harus gagal 400
    let request = UserInsert {
        name: "Da".to_string(),
        email: random_email(),
        password: "12345".to_string(),
    };

    let response = server.post("/user").json(&request).await;
    response.assert_status(StatusCode::BAD_REQUEST);

    cleanup_test_users().await;
}

#[tokio::test]
async fn test_insert_user_conflict() {
    cleanup_test_users().await;

    let pool = db::get_pool().await.unwrap();
    let server = TestServer::new(app()).unwrap();
    let email = "conflict_test@test.com";

    // insert manual via pool (bukan via TestServer)
    sqlx::query("INSERT INTO users (name, email, password) VALUES (?, ?, ?)")
        .bind("damar")
        .bind(email)
        .bind("hash123")
        .execute(&pool).await.unwrap();

    // insert kedua via TestServer → harus conflict
    let request2 = UserInsert {
        name: "Damar2".into(),
        email: email.into(),
        password: "67890".into(),
    };
    let response2 = server.post("/user").json(&request2).await;
    response2.assert_status(StatusCode::CONFLICT);


    cleanup_test_users().await;
}


#[tokio::test]
async fn test_get_all_user() {
    cleanup_test_users().await;

    let server = TestServer::new(app()).unwrap();

    // Insert 2 user dulu supaya GET ada data
    let email1 = random_email();
    let email2 = random_email();

    let request1 = UserInsert {
        name: "Damar".to_string(),
        email: email1,
        password: "12345".to_string(),
    };

    let request2 = UserInsert {
        name: "Damar2".to_string(),
        email: email2,
        password: "67890".to_string(),
    };

    let _ = server.post("/user").json(&request1).await;
    let _ = server.post("/user").json(&request2).await;

    let response = server.get("/user").await;
    response.assert_status_ok();

    // body harus array JSON
    response.assert_text_contains("[");
    
    cleanup_test_users().await;
}
