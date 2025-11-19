use axum::http::StatusCode;
use axum::response::IntoResponse;
use validator::ValidationErrors;

use crate::features::common::api_error::{ApiError, Response};

#[derive(thiserror::Error, Debug)]
pub enum CreateEventError {
    #[error("database error: {0}")]
    DatabaseError(#[from] neo4rs::Error),

    #[error("access denied to project")]
    AccessDenied,

    #[error("event with name '{0}' already exists in project")]
    EventAlreadyExists(String),
}

#[derive(Debug, thiserror::Error)]
pub enum CreateEventErrorResponse {
    #[error("Failed to create event: {0}")]
    CreateEventError(#[from] CreateEventError),

    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrors),
}

impl IntoResponse for CreateEventErrorResponse {
    fn into_response(self) -> axum::response::Response {
        ApiError::into_response(self)
    }
}

impl ApiError for CreateEventErrorResponse {
    fn error_response(&self) -> Response {
        let (status, error, message) = match self {
            Self::CreateEventError(err) => match err {
                CreateEventError::DatabaseError(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR".to_string(),
                    "An internal database error occurred. Please try again later.".to_string(),
                ),
                CreateEventError::AccessDenied => (
                    StatusCode::FORBIDDEN,
                    "ACCESS_DENIED".to_string(),
                    "You don't have access to this project.".to_string(),
                ),
                CreateEventError::EventAlreadyExists(name) => (
                    StatusCode::CONFLICT,
                    "EVENT_ALREADY_EXISTS".to_string(),
                    format!("Event with name '{}' already exists in this project.", name),
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
