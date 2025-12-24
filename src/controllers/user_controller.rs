use axum::response::{IntoResponse, Response};
use sqlx::{Error, mysql::MySqlQueryResult};
use crate::{configs::db, utils::utils::response_query};

pub async fn insert_user() -> Response {
    let pool = db::get_pool().await.unwrap();
    let name = "damar";
    let result: Result<MySqlQueryResult, Error> = sqlx::query("INSERT INTO users (name) VALUE (?)")
        .bind(name)
        .execute(&pool).await;
    response_query(result, "Berhasil memasukan data").await.into_response()

}


