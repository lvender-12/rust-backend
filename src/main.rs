use axum::{Router, serve};
use tokio::net::TcpListener;

mod routes;
mod controllers;
mod configs;
mod models;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(routes::user_route());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("server running in 0.0.0.0:3000");
    let _=serve(listener, app).await.unwrap();
}
