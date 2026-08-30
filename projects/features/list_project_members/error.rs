use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::JsonDeserializerRejection;
use validator::ValidationErrors;

use crate::features::common::api_error::{ApiError, Response};

#[derive(thiserror::Error, Debug)]
pub enum ListProjectMembersError {
    #[error(transparent)]
    Db(#[from] sqlx::Error),
    #[error("user not found")]
    NotFound {
        user_response: String,
        details: String,
    },
    #[error("User not in project")]
    NotInProject,
    #[error("permission denied: {0}")]
    PermissionDenied(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ListProjectMembersErrorResponse {
    #[error("Failed to create project: {0}")]
    ListProjectMembers(#[from] ListProjectMembersError),
    #[error("Failed to deserialize JSON: {0}")]
    JsonDeserialization(#[from] JsonDeserializerRejection),
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),
}
impl IntoResponse for ListProjectMembersErrorResponse {
    fn into_response(self) -> axum::response::Response {
        ApiError::into_response(self)
    }
}
impl ApiError for ListProjectMembersErrorResponse {
    fn error_response(&self) -> Response {
        let (status, error, message) = match self {
            Self::ListProjectMembers(err) => match err {
                ListProjectMembersError::Db(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR".to_string(),
                    "An internal database error occurred. Please try again later.".to_string(),
                ),
                ListProjectMembersError::NotFound { user_response, .. } => (
                    StatusCode::NOT_FOUND,
                    "NOT_FOUND_ERROR".to_string(),
                    user_response.to_string(),
                ),
                ListProjectMembersError::PermissionDenied(response) => (
                    StatusCode::FORBIDDEN,
                    "PERMISSION_DENIED".to_string(),
                    response.to_string(),
                ),
                ListProjectMembersError::NotInProject => (
                    StatusCode::FORBIDDEN,
                    "NOT_IN_PROJECT".to_string(),
                    "You are not a member of this project.".to_string(),
                ),
            },

            Self::JsonDeserialization(_) => (
                StatusCode::BAD_REQUEST,
                "INVALID_JSON".to_string(),
                "The provided JSON body is invalid or malformed.".to_string(),
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
