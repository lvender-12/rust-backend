use axum::{Json, response::{IntoResponse, Response}};
use sqlx::{Error, mysql::MySqlQueryResult};
use crate::{configs::db, models::user_model::UserInsert, utils::utils::response_query};

pub async fn insert_user(payload: Json<UserInsert>) -> Response  {
    let pool = db::get_pool().await.unwrap();
    let name = payload.name.clone();
    let result: Result<MySqlQueryResult, Error> = sqlx::query("INSERT INTO users (name) VALUE (?)")
        .bind(name)
        .execute(&pool).await;
    response_query(result, "Berhasil memasukan data").await.into_response()
}


