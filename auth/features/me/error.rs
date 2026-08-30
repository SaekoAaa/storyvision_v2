use {
    crate::features::common::api_error::{ApiError, Response},
    axum::{http::StatusCode, response::IntoResponse},
};

#[derive(Debug, thiserror::Error)]
pub enum GetUserError {
    #[error("db error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("unauthorized: {0}")]
    Unauthorized(#[from] argon2::password_hash::Error),
    #[error("User not found")]
    NotFound,
}
#[derive(Debug, thiserror::Error)]
pub enum GetUserErrorResponse {
    #[error("Error getting user: {0}")]
    GetUserError(#[from] GetUserError),
    #[error("Access token not found")]
    AccessTokenNotFound,
    #[error("Access token verification failed")]
    AccessTokenVerificationFailed,
}

impl IntoResponse for GetUserErrorResponse {
    fn into_response(self) -> axum::response::Response {
        ApiError::into_response(self)
    }
}

impl ApiError for GetUserErrorResponse {
    fn error_message(&self) -> String {
        self.to_string()
    }

    fn error_response(&self) -> Response {
        let (status, error, message) = match self {
            Self::GetUserError(err) => match err {
                GetUserError::Db(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR".to_string(),
                    "An internal database error occurred. Please try again later.".to_string(),
                ),
                GetUserError::NotFound => (
                    StatusCode::UNAUTHORIZED,
                    "USER_NOT_FOUND".to_string(),
                    "User not found.".to_string(),
                ),
                GetUserError::Unauthorized(_) => (
                    StatusCode::UNAUTHORIZED,
                    "UNAUTHORIZED".to_string(),
                    "You are not authorized to perform this action.".to_string(),
                ),
            },
            Self::AccessTokenNotFound => (
                StatusCode::NOT_FOUND,
                "ACCESS_TOKEN_NOT_FOUND".to_string(),
                "Access token not found.".to_string(),
            ),
            Self::AccessTokenVerificationFailed => (
                StatusCode::UNAUTHORIZED,
                "ACCESS_TOKEN_VERIFICATION_FAILED".to_string(),
                "Access token verification failed.".to_string(),
            ),
        };
        Response {
            status,
            error,
            message,
        }
    }
}
