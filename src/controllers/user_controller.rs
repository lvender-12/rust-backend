use axum::{Json, response::{IntoResponse, Response}};
use http::{ StatusCode};
use sqlx::{Error, mysql::MySqlQueryResult};
use validator::Validate;
use crate::{configs::db, models::user_model::{User, UserInsert}, utils::utils::{hashing_password, response_query}};

pub async fn insert_user(payload: Json<UserInsert>) -> Response  {
    if let Err(errors) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(errors),
        ).into_response();
    }
    let pool = db::get_pool().await.unwrap();
    let name = payload.name.trim();
    let password_hash = hashing_password(&payload.password.trim()).await.unwrap();
    let result: Result<MySqlQueryResult, Error> = sqlx::query("INSERT INTO users (name, password) VALUE (?, ?)")
        .bind(name)
        .bind(password_hash)
        .execute(&pool).await;
    response_query(result, "Berhasil memasukan data", StatusCode::CREATED).await.into_response()
}

pub async fn get_all_user()-> impl IntoResponse {
    let pool = db::get_pool().await.unwrap();
    let result= sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool).await.unwrap();

    (StatusCode::OK, Json(result))
}
