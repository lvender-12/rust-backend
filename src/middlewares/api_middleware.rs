use axum::{extract::Request, middleware::Next, response::Response};
use http::{StatusCode};

use crate::utils::utils::load_config;

pub async fn api_key_middleware(req: Request, next: Next)->Response{
    let config = load_config();
    let valid_key = config.server.api_key;
    let header_key = req.headers().get("X-API-KEY");
    if header_key.is_none() || header_key.unwrap().to_str().unwrap() != valid_key {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Unauthorized".into())
            .unwrap();
    }
    next.run(req).await
}