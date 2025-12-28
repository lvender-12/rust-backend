use axum::Router;

use crate::routes::{fallback::{fallback, not_allowed}, guest_route::routes_guest, login_route::routes_login};

pub mod fallback;
pub mod login_route;
pub mod guest_route;

pub fn user_route() -> Router{
    Router::new()
        .merge(routes_login())
        .merge(routes_guest())
        .fallback(fallback)
        .method_not_allowed_fallback(not_allowed)
}