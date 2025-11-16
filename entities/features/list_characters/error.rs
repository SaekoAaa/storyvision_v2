use axum::{http::StatusCode, response::IntoResponse};
use validator::ValidationErrors;

use crate::features::common::api_error::{ApiError, Response};

#[derive(thiserror::Error, Debug)]
pub enum ListCharactersError {
    #[error("database error: {0}")]
    DatabaseError(#[from] neo4rs::Error),

    #[error("access denied to project")]
    AccessDenied,
}

#[derive(Debug, thiserror::Error)]
pub enum ListCharactersErrorResponse {
    #[error("Failed to list characters: {0}")]
    ListCharactersError(#[from] ListCharactersError),

    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrors),
}

impl IntoResponse for ListCharactersErrorResponse {
    fn into_response(self) -> axum::response::Response {
        ApiError::into_response(self)
    }
}

impl ApiError for ListCharactersErrorResponse {
    fn error_response(&self) -> Response {
        let (status, error, message) = match self {
            Self::ListCharactersError(err) => match err {
                ListCharactersError::DatabaseError(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR".to_string(),
                    "An internal database error occurred. Please try again later.".to_string(),
                ),
                ListCharactersError::AccessDenied => (
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
