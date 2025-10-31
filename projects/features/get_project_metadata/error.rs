use std::borrow::Cow;

use axum::{http::StatusCode, response::IntoResponse};

use crate::features::common::api_error::{ApiError, Response};

#[derive(Debug, thiserror::Error)]
pub enum GetProjectError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Entity not found: {details}")]
    NotFound {
        user_response: Cow<'static, str>,
        details: Cow<'static, str>,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum GetProjectErrorResponse {
    #[error("Error during project deletion: {0}")]
    GetProjectError(#[from] GetProjectError),
    #[error("Not found: {message}")]
    NotFound {
        message: Cow<'static, str>,
        response: String,
    },
}

impl IntoResponse for GetProjectErrorResponse {
    fn into_response(self) -> axum::response::Response {
        ApiError::into_response(self)
    }
}
impl ApiError for GetProjectErrorResponse {
    fn error_response(&self) -> Response {
        let (status, error, message) = match self {
            Self::GetProjectError(err) => match err {
                GetProjectError::NotFound { user_response, .. } => (
                    StatusCode::NOT_FOUND,
                    "NOT_FOUND_ERROR".to_string(),
                    user_response.to_string(),
                ),
                GetProjectError::DatabaseError(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_SERVER_ERROR".to_string(),
                    "An internal database error occurred. Please try again later.".to_string(),
                ),
            },
            GetProjectErrorResponse::NotFound { message, response } => (
                StatusCode::NOT_FOUND,
                "PROJECT_NOT_FOUND".to_string(),
                response.to_owned(),
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
