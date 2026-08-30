use axum::{http::StatusCode, response::IntoResponse};

pub trait ApiResponse: IntoResponse {
    fn status_code(&self) -> StatusCode;
}

pub type HandlerResult<T, E> = Result<T, E>;
