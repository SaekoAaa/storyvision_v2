use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use axum_extra::extract::JsonDeserializerRejection;
use serde_json::json;
use validator::{ValidationError, ValidationErrors};
use crate::features::common::api_error::{ApiError, Response};

#[derive(thiserror::Error, Debug)]
pub enum LoginError {
    #[error("db error")]
    Db(#[from] sqlx::Error),
    #[error("user not found")]
    NotFound {
        user_response: String,
        details: String,
    },
    #[error("password hashing error: {0}")]
    PasswordHashingError(#[from] argon2::password_hash::Error),
    #[error("JWT error: {0}")]
    JsonWebTokenError(#[from] jsonwebtoken::errors::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum LoginErrorResponse {
    #[error("User login failed: {0}")]
    LoginError(#[from] LoginError),
    #[error("Failed to deserialize JSON: {0}")]
    JsonDeserializationError(#[from] JsonDeserializerRejection),
    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrors),
}
impl IntoResponse for LoginErrorResponse {
    fn into_response(self) -> axum::response::Response {
        ApiError::into_response(self)
    }
}
impl ApiError for LoginErrorResponse {
    fn error_response(&self) -> Response {
        let (status, error, message) = match self {
            Self::LoginError(err) => match err {
                LoginError::Db(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR".to_string(),
                    "An internal database error occurred. Please try again later.".to_string(),
                ),

                LoginError::PasswordHashingError(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "PASSWORD_HASH_ERROR".to_string(),
                    "Could not securely hash the password.".to_string(),
                ),

                LoginError::JsonWebTokenError(_) => (
                    StatusCode::UNAUTHORIZED,
                    "JWT_ERROR".to_string(),
                    "Failed to validate or decode JWT token.".to_string(),
                ),
                LoginError::NotFound { user_response, .. } => (
                        StatusCode::NOT_FOUND,
                        "NOT_FOUND_ERROR".to_string(),
                        user_response.to_string(),
                    )
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
                )
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
