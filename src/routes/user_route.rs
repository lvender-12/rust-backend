use axum::{Router, routing::{ post}};

use crate::{controllers::user_controller::insert_user, routes::fallback::{ fallback, not_allowed}};


pub fn routes() -> Router{
    Router::new()
        .route("/user", post(insert_user))
        .fallback(fallback)
        .method_not_allowed_fallback(not_allowed)
}