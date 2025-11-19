use axum::http::StatusCode;
use axum::response::IntoResponse;
use neo4rs::DeError;
use validator::ValidationErrors;

use crate::features::common::api_error::{ApiError, Response};

#[derive(thiserror::Error, Debug)]
pub enum CreateConnectionError {
    #[error("database error: {0}")]
    DatabaseError(#[from] neo4rs::Error),

    #[error("access denied to project")]
    AccessDenied,

    #[error("entity not found: {0}")]
    EntityNotFound(String),

    #[error("relation not found")]
    RelationNotFound,
    #[error("decode error: {0}")]
    DecodeError(#[from] DeError),
    #[error("connection already exists between entities")]
    ConnectionAlreadyExists,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateConnectionErrorResponse {
    #[error("Failed to create connection: {0}")]
    CreateConnectionError(#[from] CreateConnectionError),

    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrors),
}

impl IntoResponse for CreateConnectionErrorResponse {
    fn into_response(self) -> axum::response::Response {
        ApiError::into_response(self)
    }
}

impl ApiError for CreateConnectionErrorResponse {
    fn error_response(&self) -> Response {
        let (status, error, message) = match self {
            Self::CreateConnectionError(err) => match err {
                CreateConnectionError::DatabaseError(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR".to_string(),
                    "An internal database error occurred. Please try again later.".to_string(),
                ),
                CreateConnectionError::AccessDenied => (
                    StatusCode::FORBIDDEN,
                    "ACCESS_DENIED".to_string(),
                    "You don't have access to this project.".to_string(),
                ),
                CreateConnectionError::EntityNotFound(id) => (
                    StatusCode::NOT_FOUND,
                    "ENTITY_NOT_FOUND".to_string(),
                    format!("Entity with id '{}' not found in this project.", id),
                ),
                CreateConnectionError::RelationNotFound => (
                    StatusCode::NOT_FOUND,
                    "RELATION_NOT_FOUND".to_string(),
                    "Relation template not found in this project.".to_string(),
                ),
                CreateConnectionError::ConnectionAlreadyExists => (
                    StatusCode::CONFLICT,
                    "CONNECTION_ALREADY_EXISTS".to_string(),
                    "Connection between these entities with this relation already exists."
                        .to_string(),
                ),
                CreateConnectionError::DecodeError(err) => (
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
