use axum::{ response::IntoResponse};
use http::StatusCode;
use sqlx::{Error};
use crate::configs::db;

async fn insert_user(name:&str) -> Result<(), Error> {
    let pool = db::get_pool().await?;
    sqlx::query("INSERT INTO users (name) VALUE (?)")
        .bind(name)
        .execute(&pool).await?;
    Ok(())
}


pub async fn insert()->impl IntoResponse{
    let name = "damar";
    match insert_user(name).await {
        Ok(msg)=>msg.into_response(),
        Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}