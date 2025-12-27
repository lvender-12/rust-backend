use axum::{Router, middleware::from_fn, routing::{ delete, get, post, put}};

use crate::{controllers::user_controller::{delete_user, edit_user, get_all_user, get_user, get_user_edit, insert_user}, middlewares::api_middleware::api_key_middleware, routes::fallback::{ fallback, not_allowed}};


pub fn routes() -> Router{
    Router::new()
        .route("/user", get(get_all_user))
        .route("/user", post(insert_user))
        .route("/user/search", post(get_user))
        .route("/user/", delete(delete_user))
        .route("/user/", get(get_user_edit))
        .route("/user/{id}", put(edit_user))
        .layer(from_fn(api_key_middleware))
        .fallback(fallback)
        .method_not_allowed_fallback(not_allowed)
}