use std::borrow::Cow;
use argon2::{PasswordHash, PasswordVerifier};
use argon2::password_hash::Error as PasswordHashError;
use argon2::{Argon2, password_hash::{SaltString, rand_core::OsRng, PasswordHasher}};
use chrono::{Duration, Utc};
use config::{Config, File, FileFormat};
use jsonwebtoken::{EncodingKey, Header, encode};
use validator::ValidationError;


use crate::errors::app_error::AppError;
use crate::models::config_model::AppConfig;
use crate::models::user_model::{Claims};

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

pub fn create_jwt(user_id: u64) -> Result<String, AppError> {
    let conf = load_config()?;
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Valid TimeStamp")
        .timestamp();

    let claims = Claims{
        sub: user_id.to_owned(),
        exp: expiration,
    };

    let encoded = encode(&Header::default(), &claims, &EncodingKey::from_secret(conf.jwt_secret.as_ref()))?;
    Ok(encoded)
}

pub async fn verify_password(hash: &str, password: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|_|AppError::Unauthorized)?;

    let is_valid = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(is_valid)
}
