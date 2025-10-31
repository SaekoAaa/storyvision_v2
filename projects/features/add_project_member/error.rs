use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::JsonDeserializerRejection;
use validator::ValidationErrors;

use crate::features::common::api_error::{ApiError, Response};

#[derive(thiserror::Error, Debug)]
pub enum AddProjectMemberError {
    #[error("db error")]
    Db(#[from] sqlx::Error),
    #[error("user not found")]
    NotFound {
        user_response: String,
        details: String,
    },
    #[error("not a project owner")]
    NotAProjectOwner,
    #[error("permission denied: {0}")]
    PermissionDenied(String),
}

#[derive(Debug, thiserror::Error)]
pub enum AddProjectMemberErrorResponse {
    #[error("Failed to create project: {0}")]
    AddProjectMemberError(#[from] AddProjectMemberError),
    #[error("Failed to deserialize JSON: {0}")]
    JsonDeserializationError(#[from] JsonDeserializerRejection),
    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrors),
}
impl IntoResponse for AddProjectMemberErrorResponse {
    fn into_response(self) -> axum::response::Response {
        ApiError::into_response(self)
    }
}
impl ApiError for AddProjectMemberErrorResponse {
    fn error_response(&self) -> Response {
        let (status, error, message) = match self {
            Self::AddProjectMemberError(err) => match err {
                AddProjectMemberError::Db(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR".to_string(),
                    "An internal database error occurred. Please try again later.".to_string(),
                ),
                AddProjectMemberError::NotFound { user_response, .. } => (
                    StatusCode::NOT_FOUND,
                    "NOT_FOUND_ERROR".to_string(),
                    user_response.to_string(),
                ),
                AddProjectMemberError::PermissionDenied(response) => (
                    StatusCode::FORBIDDEN,
                    "PERMISSION_DENIED".to_string(),
                    response.to_string(),
                ),
                AddProjectMemberError::NotAProjectOwner => (
                    StatusCode::FORBIDDEN,
                    "NOT_A_PROJECT_OWNER".to_string(),
                    "You are not the owner of this project.".to_string(),
                ),
            },

            Self::JsonDeserializationError(_) => (
                StatusCode::BAD_REQUEST,
                "INVALID_JSON".to_string(),
                "The provided JSON body is invalid or malformed.".to_string(),
            ),
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
