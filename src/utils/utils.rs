use axum::response::IntoResponse;
use http::StatusCode;
use sqlx::{Error, mysql::MySqlQueryResult};

pub async fn response_query(result: Result<MySqlQueryResult, Error>,success_msg:&str) -> impl IntoResponse{
    match result {
        Ok(query)=>(
            StatusCode::OK,
            format!("{} affected rows:{}", success_msg, query.rows_affected())
        ),
        Err(e)=>(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Terjadi Error : {}", e)
        ),
    }
}