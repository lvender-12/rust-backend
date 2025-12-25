use axum::extract::Request;
use http::StatusCode;


pub async fn fallback(request:Request)->(StatusCode, String){
    (
        StatusCode::NOT_FOUND,
        format!("Page {} is not found", request.uri().path())
    )
}

pub async fn not_allowed(request:Request)->(StatusCode, String){
    (
        StatusCode::METHOD_NOT_ALLOWED,
        format!("method not allowed for path {}", request.uri().path())
    )
}