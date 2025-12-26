use axum::{Json, response::{IntoResponse, Response}};
use http::{ StatusCode};
use sqlx::{Error, mysql::MySqlQueryResult};
use validator::Validate;
use crate::{configs::db, models::user_model::{SearchQuery, User, UserInsert, SeacrhBy}, utils::utils::{check_email, hashing_password, response_query}};

pub async fn get_all_user()-> impl IntoResponse {
    let pool = db::get_pool().await.unwrap();
    let result= sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool).await.unwrap();

    (StatusCode::OK, Json(result))
}

pub async fn insert_user(payload: Json<UserInsert>) -> Response  {
    if let Err(errors) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(errors),
        ).into_response();
    }

    let pool = db::get_pool().await.unwrap();

    let name = payload.name.trim();
    let email = payload.email.trim();
    let password_hash = hashing_password(&payload.password.trim()).await.unwrap();

    if check_email(email).await.unwrap() {
        return (
            StatusCode::CONFLICT,
            format!("Email {} sudah terdaftar", email)
        ).into_response();
    }

    let result: Result<MySqlQueryResult, Error> = sqlx::query("INSERT INTO users (name, email, password) VALUE (?, ?, ?)")
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .execute(&pool).await;
    response_query(result, "Berhasil memasukan data", StatusCode::CREATED).await.into_response()
}

pub async fn get_user(payload: Json<SearchQuery>) -> impl IntoResponse {
     if let Err(errors) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(errors),
        ).into_response();
    }

    let pool = db::get_pool().await.unwrap();
    let by = &payload.by;
    let value = format!("%{}%", payload.value.trim());
    let result_vec = sqlx::query_as::<_, User>
    (
        match by {
            SeacrhBy::Name => "SELECT * FROM users WHERE name LIKE ?",
            SeacrhBy::Email => "SELECT * FROM users WHERE email LIKE ?",
        }
    )
    .bind(value)
    .fetch_all(&pool).await.unwrap();
    (StatusCode::OK, Json(result_vec)).into_response()
}