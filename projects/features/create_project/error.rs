use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::JsonDeserializerRejection;
use validator::ValidationErrors;

use crate::features::common::api_error::{ApiError, Response};

#[derive(thiserror::Error, Debug)]
pub enum CreateProjectError {
    #[error(transparent)]
    Db(#[from] sqlx::Error),
    #[error("user not found")]
    NotFound {
        user_response: String,
        details: String,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum CreateProjectErrorResponse {
    #[error("Failed to create project: {0}")]
    CreateProjectError(#[from] CreateProjectError),
    #[error("Failed to deserialize JSON: {0}")]
    JsonDeserializationError(#[from] JsonDeserializerRejection),
    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrors),
}
impl IntoResponse for CreateProjectErrorResponse {
    fn into_response(self) -> axum::response::Response {
        ApiError::into_response(self)
    }
}
impl ApiError for CreateProjectErrorResponse {
    fn error_response(&self) -> Response {
        let (status, error, message) = match self {
            Self::CreateProjectError(err) => match err {
                CreateProjectError::Db(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR".to_string(),
                    "An internal database error occurred. Please try again later.".to_string(),
                ),
                CreateProjectError::NotFound { user_response, .. } => (
                    StatusCode::NOT_FOUND,
                    "NOT_FOUND_ERROR".to_string(),
                    user_response.to_string(),
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
