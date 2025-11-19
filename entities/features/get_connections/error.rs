use axum::http::StatusCode;
use axum::response::IntoResponse;
use neo4rs::DeError;
use validator::ValidationErrors;

use crate::features::common::api_error::{ApiError, Response};

#[derive(thiserror::Error, Debug)]
pub enum GetConnectionsError {
    #[error("database error: {0}")]
    DatabaseError(#[from] neo4rs::Error),

    #[error("decode error: {0}")]
    DecodeError(#[from] DeError),

    #[error("access denied to project")]
    AccessDenied,
}

#[derive(Debug, thiserror::Error)]
pub enum GetConnectionsErrorResponse {
    #[error("Failed to get connections: {0}")]
    GetConnectionsError(#[from] GetConnectionsError),

    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrors),
}

impl IntoResponse for GetConnectionsErrorResponse {
    fn into_response(self) -> axum::response::Response {
        ApiError::into_response(self)
    }
}

impl ApiError for GetConnectionsErrorResponse {
    fn error_response(&self) -> Response {
        let (status, error, message) = match self {
            Self::GetConnectionsError(err) => match err {
                GetConnectionsError::DatabaseError(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR".to_string(),
                    "An internal database error occurred. Please try again later.".to_string(),
                ),
                GetConnectionsError::DecodeError(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DECODE_ERROR".to_string(),
                    "Failed to decode database row.".to_string(),
                ),
                GetConnectionsError::AccessDenied => (
                    StatusCode::FORBIDDEN,
                    "ACCESS_DENIED".to_string(),
                    "You don't have access to this project.".to_string(),
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
