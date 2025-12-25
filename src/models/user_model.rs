use chrono::{DateTime, Utc};
use validator::Validate;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive( FromRow, Debug, Deserialize, Serialize)]
pub struct User{
    pub id: u64,
    pub name: String,
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    password: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Validate)]
pub struct UserInsert{
    #[validate(length(min = 3, message = "Nama minimal 3 karakter"))]
    pub name: String,
    #[validate(length(min = 5, message = "Password minimal 5 karakter"))]
    pub password: String,
}