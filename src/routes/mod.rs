use axum::Router;

pub mod user_route;
pub mod fallback;

pub fn user_route() -> Router{
    Router::new()
        .merge(user_route::routes())
}