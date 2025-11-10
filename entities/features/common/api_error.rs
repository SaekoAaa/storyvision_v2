use axum::Json;
use serde_json::json;
use axum::{http::StatusCode, response::IntoResponse};

pub struct Response {
    pub status: StatusCode,
    pub error: String,
    pub message: String,
}

pub trait ApiError: Sized  {
    fn error_message(&self) -> String;
    fn error_response(&self) -> Response;
    fn into_response(self) -> axum::response::Response {
        tracing::error!("Error caught: {}", self.error_message());
        let err = self.error_response();
        (
            err.status,
            Json(json!({
                "error": err.error,
                "message": err.message
            })),
        )
            .into_response()
    }
}