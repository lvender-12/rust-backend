use axum::{Router, middleware::from_fn, routing::{ post}};

use crate::{controllers::user_controller::login_user, middlewares::api_middleware::{api_key_middleware, check_guest}};


pub fn routes_guest() -> Router{
    Router::new()
        .route("/login", post(login_user))
        .layer(from_fn(api_key_middleware))
        .layer(from_fn(check_guest))
}