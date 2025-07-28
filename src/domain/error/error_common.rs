use axum::http::StatusCode;
use thiserror::Error;

#[derive(Clone, Error, Debug)]
pub enum ErrorCommon {
    #[allow(dead_code)]
    #[error("Internal Server Error")]
    InternalServerError,
    #[allow(dead_code)]
    #[error("{message}")]
    CustomError {
        status_code: StatusCode,
        message: String,
    },
}
