use {
    crate::features::common::api_error::ApiError,
    axum::{http::StatusCode, response::IntoResponse},
};

pub trait ApiResponse: IntoResponse {
    fn status_code(&self) -> StatusCode;
}

pub type HandlerResult<T, E: ApiError> = Result<T, E>;
