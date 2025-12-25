use argon2::{Argon2, password_hash::{SaltString, rand_core::OsRng, PasswordHasher}};
use axum::response::IntoResponse;
use http::StatusCode;
use sqlx::{Error, mysql::MySqlQueryResult};

pub async fn response_query(result: Result<MySqlQueryResult, Error>,success_msg:&str, status_code: StatusCode) -> impl IntoResponse{
    match result {
        Ok(query)=>(
            status_code,
            format!("{} affected rows:{}", success_msg, query.rows_affected())
        ),
        Err(e)=>(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Terjadi Error : {}", e)
        ),
    }
}

pub async fn hashing_password(password:&str)->Result<String,argon2::Error>{
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
    Ok(password_hash)
}