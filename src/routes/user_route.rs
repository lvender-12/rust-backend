use axum::{Router, routing::{ get, post}};

use crate::{controllers::user_controller::{get_all_user, insert_user}, routes::fallback::{ fallback, not_allowed}};


pub fn routes() -> Router{
    Router::new()
        .route("/user", post(insert_user))
        .route("/user", get(get_all_user))
        .fallback(fallback)
        .method_not_allowed_fallback(not_allowed)
}