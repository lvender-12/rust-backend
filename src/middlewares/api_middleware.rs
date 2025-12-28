use axum::{extract::Request, middleware::Next, response::Response};
use axum_extra::extract::CookieJar;

use crate::{errors::app_error::AppError, utils::utils::{jwt_verify, load_config}};

pub async fn api_key_middleware(req: Request, next: Next)->Result<Response, AppError>{
    let config = load_config()?;
    let valid_key = config.server.api_key;
    let header_key = req.headers().get("X-API-KEY").and_then(|v|v.to_str().ok());
    if header_key != Some(&valid_key.as_str()){
        return Err(AppError::Unauthorized);
    }
    Ok(next.run(req).await)
}

pub async fn check_login(req: Request, next: Next) -> Result<Response, AppError> {
    // Ambil CookieJar dari request extensions
    let cookies = CookieJar::from_headers(req.headers());

    // Ambil cookie "jwt"
    let jwt = cookies
        .get("jwt")
        .ok_or(AppError::Unauthorized)? // konversi Option -> Result
        .value()
        .trim();

    // Verifikasi token
    let claims = jwt_verify(jwt).await?;

    // Bisa simpan claims di request extensions untuk handler
    let mut req = req;
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

pub async fn check_guest(req: Request, next: Next) -> Result<Response, AppError> {
    let cookies = CookieJar::from_headers(req.headers());

    if let Some(cookie) = cookies.get("jwt") {
        let jwt = cookie.value().trim();

        if jwt_verify(jwt).await.is_ok() {
            return Err(AppError::Forbidden);
        }
    }

    Ok(next.run(req).await)
}