use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use validator::ValidationErrors;

use crate::features::common::api_error::{ApiError, Response};

#[derive(thiserror::Error, Debug)]
pub enum CreateCharacterError {
    #[error("database error: {0}")]
    DatabaseError(#[from] neo4rs::Error),

    #[error("character with name '{0}' already exists in project")]
    CharacterAlreadyExists(String),

    #[error("access denied to project")]
    AccessDenied,

    #[error("project not found")]
    ProjectNotFound,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateCharacterErrorResponse {
    #[error("Failed to create character: {0}")]
    CreateCharacterError(#[from] CreateCharacterError),

    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrors),
}

impl IntoResponse for CreateCharacterErrorResponse {
    fn into_response(self) -> axum::response::Response {
        ApiError::into_response(self)
    }
}

impl ApiError for CreateCharacterErrorResponse {
    fn error_response(&self) -> Response {
        let (status, error, message) = match self {
            Self::CreateCharacterError(err) => match err {
                CreateCharacterError::DatabaseError(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR".to_string(),
                    "An internal database error occurred. Please try again later.".to_string(),
                ),
                CreateCharacterError::CharacterAlreadyExists(name) => (
                    StatusCode::CONFLICT,
                    "CHARACTER_ALREADY_EXISTS".to_string(),
                    format!(
                        "Character with name '{}' already exists in this project.",
                        name
                    ),
                ),
                CreateCharacterError::AccessDenied => (
                    StatusCode::FORBIDDEN,
                    "ACCESS_DENIED".to_string(),
                    "You don't have access to this project.".to_string(),
                ),
                CreateCharacterError::ProjectNotFound => (
                    StatusCode::NOT_FOUND,
                    "PROJECT_NOT_FOUND".to_string(),
                    "The specified project does not exist.".to_string(),
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
