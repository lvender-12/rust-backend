use axum::{extract::Request, middleware::Next, response::Response};

use crate::{errors::app_error::AppError, utils::utils::load_config};

pub async fn api_key_middleware(req: Request, next: Next)->Result<Response, AppError>{
    let config = load_config()?;
    let valid_key = config.server.api_key;
    let header_key = req.headers().get("X-API-KEY").and_then(|v|v.to_str().ok());
    if header_key != Some(&valid_key.as_str()){
        return Err(AppError::Unauthorized);
    }
    Ok(next.run(req).await)
}