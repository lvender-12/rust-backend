use axum::{Json, extract::Path};
use http::{ StatusCode};
use validator::Validate;
use crate::{configs::db, errors::app_error::AppError, models::user_model::{SeacrhBy, SearchQuery, User, UserInsert}, utils::utils::{check_email, hashing_password}};

pub async fn get_all_user()-> Result<(StatusCode, Json<Vec<User>>), AppError> {
    let pool = db::get_pool().await?;
    let result= sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool).await?;

    Ok((StatusCode::OK, Json(result)))
}

pub async fn insert_user(payload: Json<UserInsert>) -> Result<(StatusCode,String), AppError> {
    payload.validate().map_err(AppError::ValidationError)?;

    let pool = db::get_pool().await?;

    let name = payload.name.trim();
    let email = payload.email.trim();
    let password_hash = hashing_password(&payload.password.trim()).await?;

    if check_email(email).await? {
        return Err(AppError::Conflict);
    }

    sqlx::query("INSERT INTO users (name, email, password) VALUE (?, ?, ?)")
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .execute(&pool).await?;
    
    Ok((StatusCode::CREATED, "User berhasil dibuat".to_string()))
}

pub async fn get_user(payload: Json<SearchQuery>) -> Result<(StatusCode, Json<Vec<User>>), AppError> {
    payload.validate().map_err(AppError::ValidationError)?;

    let pool = db::get_pool().await?;
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
    .fetch_all(&pool).await?;
    Ok((StatusCode::OK, Json(result_vec)))
}

pub async fn delete_user(Path(user_email): Path<String>)-> Result<(StatusCode, Json<String>), AppError> {
    let pool = db::get_pool().await?;
    let result = sqlx::query("DELETE FROM users WHERE email = ?")
        .bind(user_email)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0{
        return Err(AppError::NotFound);
    }

    Ok((StatusCode::NO_CONTENT, Json("User deleted successfully".to_string())))
}