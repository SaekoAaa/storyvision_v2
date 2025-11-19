use axum::http::StatusCode;
use axum::response::IntoResponse;
use neo4rs::DeError;
use validator::ValidationErrors;

use crate::features::common::api_error::{ApiError, Response};

#[derive(thiserror::Error, Debug)]
pub enum GetEventsError {
    #[error("database error: {0}")]
    DatabaseError(#[from] neo4rs::Error),

    #[error("access denied to project")]
    AccessDenied,
    #[error("decode error: {0}")]
    DecodeError(#[from] DeError),
}

#[derive(Debug, thiserror::Error)]
pub enum GetEventsErrorResponse {
    #[error("Failed to get events: {0}")]
    GetEventsError(#[from] GetEventsError),

    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrors),
}

impl IntoResponse for GetEventsErrorResponse {
    fn into_response(self) -> axum::response::Response {
        ApiError::into_response(self)
    }
}

impl ApiError for GetEventsErrorResponse {
    fn error_response(&self) -> Response {
        let (status, error, message) = match self {
            Self::GetEventsError(err) => match err {
                GetEventsError::DatabaseError(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR".to_string(),
                    "An internal database error occurred. Please try again later.".to_string(),
                ),
                GetEventsError::AccessDenied => (
                    StatusCode::FORBIDDEN,
                    "ACCESS_DENIED".to_string(),
                    "You don't have access to this project.".to_string(),
                ),
                GetEventsError::DecodeError(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DECODE_ERROR".to_string(),
                    format!("An error occurred while decoding data: {}", err),
                ),
            },
            Self::ValidationError(e) => (
                StatusCode::BAD_REQUEST,
                "VALIDATION_ERROR".to_string(),
                e.to_string(),
            ),
        };

        Response {
            status,
            error,
            message,
        }
    }

    fn error_message(&self) -> String {
        self.to_string()
    }
}
