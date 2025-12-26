use std::time::Duration;
use sqlx::{Error, MySql, Pool, mysql::MySqlPoolOptions};
use crate::{ utils::utils::load_config};


async fn create_pool(url: String) -> Result<Pool<MySql>, Error> {
    MySqlPoolOptions::new()
        .max_connections(20)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(60))
        .connect(&url)
        .await
}

pub async  fn get_pool()-> Result<Pool<MySql>, Error>{
    let cfg = load_config();
    let db = cfg.database;

    let host = db.host;
    let port = db.port;
    let user = db.user;
    let password = db.password;
    let name = db.name;

    let database_url = format!("mysql://{}:{}@{}:{}/{}", user, password, host, port, name);
    create_pool(database_url).await
}