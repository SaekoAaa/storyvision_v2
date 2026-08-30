use std::borrow::Cow;

use axum::{http::StatusCode, response::IntoResponse};

use crate::features::common::api_error::{ApiError, Response};

#[derive(Debug, thiserror::Error)]
pub enum DeleteProjectError {
    #[error(transparent)]
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
pub enum DeleteProjectErrorResponse {
    #[error("Error during project deletion: {0}")]
    DeleteProjectError(#[from] DeleteProjectError),
}

impl IntoResponse for DeleteProjectErrorResponse {
    fn into_response(self) -> axum::response::Response {
        ApiError::into_response(self)
    }
}
impl ApiError for DeleteProjectErrorResponse {
    fn error_message(&self) -> String {
        self.to_string()
    }

    fn error_response(&self) -> Response {
        let (status, error, message) = match self {
            Self::DeleteProjectError(err) => match err {
                DeleteProjectError::NotAProjectOwner => (
                    StatusCode::FORBIDDEN,
                    "NOT_A_PROJECT_OWNER".to_string(),
                    "You are not the owner of this project.".to_string(),
                ),
                DeleteProjectError::NotFound { user_response, .. } => (
                    StatusCode::NOT_FOUND,
                    "NOT_FOUND_ERROR".to_string(),
                    user_response.to_string(),
                ),
                DeleteProjectError::DatabaseError(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_SERVER_ERROR".to_string(),
                    "An internal database error occurred. Please try again later.".to_string(),
                ),
            },
        };
        Response {
            status,
            error,
            message,
        }
    }
}
