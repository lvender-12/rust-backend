use axum::{Router, routing::{ post}};

use crate::controllers::user_controller::insert_user;


pub fn routes() -> Router{
    Router::new()
        .route("/user", post(insert_user))
}