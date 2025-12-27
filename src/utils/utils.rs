use std::borrow::Cow;
use argon2::password_hash::Error as PasswordHashError;
use argon2::{Argon2, password_hash::{SaltString, rand_core::OsRng, PasswordHasher}};
use config::{Config, File, FileFormat};
use validator::ValidationError;

use crate::errors::app_error::AppError;
use crate::models::config_model::AppConfig;

//untuk hashing password menggunakan argon2
pub async fn hashing_password(password:&str)->Result<String,PasswordHashError>{
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(password_hash)
}

//untuk mengecek apakah email sudah terdaftar
pub async fn check_email(email:&str)->Result<bool, AppError>{
    let pool = crate::configs::db::get_pool().await.unwrap();
    let result: (i64,) = sqlx::query_as("SELECT COUNT(*) as count FROM users WHERE email = ?")
        .bind(email)
        .fetch_one(&pool).await?;
    Ok(result.0 > 0)
}


//untuk validasi tld email
pub fn validate_email_tld(email:&str)->Result<(), ValidationError>{
    //simple check format email
    let (_, domain) = email.split_once('@')
        .ok_or_else(||ValidationError::new("email tidak valid")
            .with_message(Cow::from("format email tidak falid")))?;

    //check tld email
    let tld = domain.rsplit('.').next()
        .ok_or_else(||ValidationError::new("email tidak valid")
            .with_message(Cow::from("Format email tidak valid")))?;

    //check panjang tld
    if tld.len() < 2 {
        return Err(ValidationError::new("email tidak valid")
            .with_message(Cow::from("Format email tidak valid"))    
        );
    }

    Ok(())
}

pub fn load_config() -> Result<AppConfig, AppError> {
    let config = Config::builder()
        .add_source(File::new("config.yaml", FileFormat::Yaml))
        .build()?
        .try_deserialize()?;
    Ok(config)
}