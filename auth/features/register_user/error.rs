use {
    crate::features::common::api_error::{ApiError, Response},
    axum::{http::StatusCode, response::IntoResponse},
    axum_extra::extract::JsonDeserializerRejection,
};

#[derive(Debug, thiserror::Error)]
pub enum RegisterError {
    #[error("email already registered")]
    EmailAlreadyExists,
    #[error("db error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("password hashing error: {0}")]
    PasswordHashing(#[from] argon2::password_hash::Error),
    #[error("JWT error: {0}")]
    JsonWebToken(#[from] jsonwebtoken::errors::Error),
}
#[derive(Debug, thiserror::Error)]
pub enum RegisterErrorResponse {
    #[error("User registration failed: {0}")]
    Register(#[from] RegisterError),
    #[error("Failed to deserialize JSON: {0}")]
    JsonDeserialization(#[from] JsonDeserializerRejection),
    #[error("Validation error: {0}")]
    Validation(serde_json::Value),
}

impl IntoResponse for RegisterErrorResponse {
    fn into_response(self) -> axum::response::Response {
        ApiError::into_response(self)
    }
}

impl ApiError for RegisterErrorResponse {
    fn error_message(&self) -> String {
        self.to_string()
    }

    fn error_response(&self) -> Response {
        let (status, error, message) = match self {
            Self::Register(err) => match err {
                RegisterError::EmailAlreadyExists => (
                    StatusCode::CONFLICT,
                    "EMAIL_ALREADY_EXISTS".to_string(),
                    "A user with this email address already exists.".to_string(),
                ),

                RegisterError::Db(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR".to_string(),
                    "An internal database error occurred. Please try again later.".to_string(),
                ),

                RegisterError::PasswordHashing(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "PASSWORD_HASH_ERROR".to_string(),
                    "Could not securely hash the password.".to_string(),
                ),

                RegisterError::JsonWebToken(_) => (
                    StatusCode::UNAUTHORIZED,
                    "JWT_ERROR".to_string(),
                    "Failed to validate or decode JWT token.".to_string(),
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
