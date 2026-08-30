use crate::features::common::api_error::{ApiError, Response};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::JsonDeserializerRejection;
use validator::ValidationErrors;

#[derive(thiserror::Error, Debug)]
pub enum LoginError {
    #[error("db error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("user not found")]
    NotFound {
        user_response: String,
        details: String,
    },
    #[error("password hashing error: {0}")]
    PasswordHashing(#[from] argon2::password_hash::Error),
    #[error("JWT error: {0}")]
    JsonWebToken(#[from] jsonwebtoken::errors::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum LoginErrorResponse {
    #[error("User login failed: {0}")]
    Login(#[from] LoginError),
    #[error("Failed to deserialize JSON: {0}")]
    JsonDeserialization(#[from] JsonDeserializerRejection),
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),
}
impl IntoResponse for LoginErrorResponse {
    fn into_response(self) -> axum::response::Response {
        ApiError::into_response(self)
    }
}
impl ApiError for LoginErrorResponse {
    fn error_message(&self) -> String {
        self.to_string()
    }

    fn error_response(&self) -> Response {
        let (status, error, message) = match self {
            Self::Login(err) => match err {
                LoginError::Db(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR".to_string(),
                    "An internal database error occurred. Please try again later.".to_string(),
                ),

                LoginError::PasswordHashing(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "PASSWORD_HASH_ERROR".to_string(),
                    "Could not securely hash the password.".to_string(),
                ),

                LoginError::JsonWebToken(_) => (
                    StatusCode::UNAUTHORIZED,
                    "JWT_ERROR".to_string(),
                    "Failed to validate or decode JWT token.".to_string(),
                ),
                LoginError::NotFound { user_response, .. } => (
                    StatusCode::NOT_FOUND,
                    "NOT_FOUND_ERROR".to_string(),
                    user_response.to_string(),
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
}
