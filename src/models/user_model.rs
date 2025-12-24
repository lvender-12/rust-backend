use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct User{
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct UserInsert{
    pub name: String,
}