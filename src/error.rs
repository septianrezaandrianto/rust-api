use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;


#[derive(Debug, Error)]
pub enum AppError {
    #[error("not found")]
    NotFound,
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error)
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND.into_response(),
            AppError::Db(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}