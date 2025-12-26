use std::borrow::Cow;

use argon2::{Argon2, password_hash::{SaltString, rand_core::OsRng, PasswordHasher}};
use axum::response::IntoResponse;
use config::{Config, File, FileFormat};
use http::StatusCode;
use sqlx::{Error, mysql::MySqlQueryResult};
use validator::ValidationError;

use crate::models::config_model::AppConfig;

//untuk mengubah response dari query menjadi response axum
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


//untuk hashing password menggunakan argon2
pub async fn hashing_password(password:&str)->Result<String,argon2::Error>{
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
    Ok(password_hash)
}

//untuk mengecek apakah email sudah terdaftar
pub async fn check_email(email:&str)->Result<bool, Error>{
    let pool = crate::configs::db::get_pool().await.unwrap();
    let result: (i64,) = sqlx::query_as("SELECT COUNT(*) as count FROM users WHERE email = ?")
        .bind(email)
        .fetch_one(&pool).await?;
    Ok(result.0 > 0)
}


//untuk validasi tld email
pub fn validate_email_tld(email:&str)->Result<(), ValidationError>{
    //simple check format email
    if !email.contains('@'){
        return Err(ValidationError::new("email tidak valid")
            .with_message(Cow::from("Format email tidak valid"))    
        );
    }

    //check tld email
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 || !parts[1].contains('.') {
        return Err(ValidationError::new("email tidak valid")
            .with_message(Cow::from("Format email tidak valid"))    
        );
    }

    //check panjang tld
    let domain_parts: Vec<&str> = parts[1].split('.').collect();
    if domain_parts.is_empty() || domain_parts.last().unwrap().len() < 2 {
        return Err(ValidationError::new("email tidak valid")
            .with_message(Cow::from("Format email tidak valid"))    
        );
    }


    Ok(())
}

pub fn load_config() -> AppConfig {
    Config::builder()
        .add_source(File::new("config.yaml", FileFormat::Yaml))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap()
}