use std::time::Duration;

use sqlx::{Error, MySql, Pool, mysql::MySqlPoolOptions};


pub async  fn get_pool()-> Result<Pool<MySql>, Error>{
    let url = "mysql://root:@localhost:3306/backend";
    MySqlPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(60))
        .connect(url).await
}