use crate::features::common::api_error::{ApiError, Response};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use neo4rs::DeError;
use validator::ValidationErrors;

#[derive(thiserror::Error, Debug)]
pub enum GetProjectGraphError {
    #[error("database error: {0}")]
    DatabaseError(#[from] neo4rs::Error),

    #[error("decode error: {0}")]
    DecodeError(#[from] DeError),

    #[error("access denied to project")]
    AccessDenied,
}

#[derive(Debug, thiserror::Error)]
pub enum GetProjectGraphErrorResponse {
    #[error("Failed to get project graph: {0}")]
    GetProjectGraphError(#[from] GetProjectGraphError),

    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrors),
}

impl IntoResponse for GetProjectGraphErrorResponse {
    fn into_response(self) -> axum::response::Response {
        ApiError::into_response(self)
    }
}

impl ApiError for GetProjectGraphErrorResponse {
    fn error_response(&self) -> Response {
        let (status, error, message) = match self {
            Self::GetProjectGraphError(err) => match err {
                GetProjectGraphError::DatabaseError(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR".to_string(),
                    "An internal database error occurred. Please try again later.".to_string(),
                ),
                GetProjectGraphError::DecodeError(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DECODE_ERROR".to_string(),
                    "Failed to decode database row.".to_string(),
                ),
                GetProjectGraphError::AccessDenied => (
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
