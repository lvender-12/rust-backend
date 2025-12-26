use axum::{Router, middleware::from_fn, routing::{ get, post}};

use crate::{controllers::user_controller::{get_all_user, insert_user}, middlewares::api_middleware::api_key_middleware, routes::fallback::{ fallback, not_allowed}};


pub fn routes() -> Router{
    Router::new()
        .route("/user", post(insert_user))
        .route("/user", get(get_all_user))
        .layer(from_fn(api_key_middleware))
        .fallback(fallback)
        .method_not_allowed_fallback(not_allowed)
}