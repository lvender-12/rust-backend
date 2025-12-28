use argon2::password_hash::Error as PasswordHashError;
use axum::response::IntoResponse;
use http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error")]
    Db(#[from] sqlx::Error),

    #[error("Validation error")]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("Password hashing error")]
    HashError(#[from] PasswordHashError),

    #[error("Config error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("Chrono error: {0}")]
    ChronoError(#[from] chrono::ParseError),

    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error("Cookie error")]
    CookieError,

    #[error("Data not found")]
    NotFound,

    #[error("Unauthorized access")]
    Unauthorized,

    #[error("Internal server error")]
    InternalServerError,

    #[error("Bad request")]
    BadRequest,

    #[error("Conflict")]
    Conflict,

    #[error("Forbidden access")]
    Forbidden,

}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Db(e) => {
                eprintln!("DB ERROR: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
            AppError::NotFound => {
                (StatusCode::NOT_FOUND, "Data not found").into_response()
            }
            AppError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "Unauthorized access").into_response()
            }
            AppError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
            AppError::BadRequest => {
                (StatusCode::BAD_REQUEST, "Bad request").into_response()
            }
            AppError::Conflict => {
                (StatusCode::CONFLICT, "Conflict").into_response()
            }
            AppError::ValidationError(e) => {
                eprintln!("Validation ERROR: {:?}", e);
                (StatusCode::BAD_REQUEST, "Validation error").into_response()
            }
            AppError::HashError(e) => {
                eprintln!("Hashing ERROR: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Password hashing error").into_response()
            }
            AppError::ConfigError(e) => {
                eprintln!("Config ERROR: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error").into_response()
            }
            AppError::ChronoError(e) => {
                eprintln!("Chrono ERROR: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Chrono error").into_response()
            }
            AppError::JwtError(e) => {
                eprintln!("JWT ERROR: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "JWT error").into_response()
            }
            AppError::CookieError => {
                eprintln!("Cookie ERROR");
                (StatusCode::INTERNAL_SERVER_ERROR, "Cookie error").into_response()
            }
            AppError::Forbidden => {
                (StatusCode::FORBIDDEN, "Forbidden access").into_response()
            }
        }
    }
}
