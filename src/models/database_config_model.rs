use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig{
    pub host: String,
    pub port: i32,
    pub user: String,
    pub password: String,
    pub name : String
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
}