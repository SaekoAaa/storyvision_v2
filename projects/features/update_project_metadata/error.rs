use std::borrow::Cow;

use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::JsonDeserializerRejection;
use validator::ValidationErrors;

use crate::features::common::api_error::{ApiError, Response};

#[derive(Debug, thiserror::Error)]
pub enum UpdateProjectMetadataError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Entity not found: {details}")]
    NotFound {
        user_response: Cow<'static, str>,
        details: Cow<'static, str>,
    },
    #[error("Not a project owner")]
    NotAProjectOwner,
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateProjectMetadataErrorResponse {
    #[error("Error during project deletion: {0}")]
    UpdateProjectMetadata(#[from] UpdateProjectMetadataError),
    #[error("Failed to deserialize JSON: {0}")]
    JsonDeserialization(#[from] JsonDeserializerRejection),
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),
}

impl IntoResponse for UpdateProjectMetadataErrorResponse {
    fn into_response(self) -> axum::response::Response {
        ApiError::into_response(self)
    }
}
impl ApiError for UpdateProjectMetadataErrorResponse {
    fn error_response(&self) -> Response {
        let (status, error, message) = match self {
            Self::UpdateProjectMetadata(err) => match err {
                UpdateProjectMetadataError::NotAProjectOwner => (
                    StatusCode::FORBIDDEN,
                    "NOT_A_PROJECT_OWNER".to_string(),
                    "You are not the owner of this project.".to_string(),
                ),
                UpdateProjectMetadataError::NotFound { user_response, .. } => (
                    StatusCode::NOT_FOUND,
                    "NOT_FOUND_ERROR".to_string(),
                    user_response.to_string(),
                ),
                UpdateProjectMetadataError::DatabaseError(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_SERVER_ERROR".to_string(),
                    "An internal database error occurred. Please try again later.".to_string(),
                ),
            },
            Self::JsonDeserialization(e) => (
                StatusCode::BAD_REQUEST,
                "INVALID_JSON".to_string(),
                e.to_string(),
            ),
            Self::Validation(e) => (
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
