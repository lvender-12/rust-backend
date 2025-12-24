use axum::{Router, routing::{ post}};

use crate::controllers::user_controller;

pub fn routes() -> Router{
    Router::new()
        .route("/user", post(user_controller::insert))
}