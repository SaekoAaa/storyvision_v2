use crate::features::common::api_error::{ApiError, Response};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::JsonDeserializerRejection;

#[derive(thiserror::Error, Debug)]
pub enum LogoutError {
    #[error("db error")]
    Db(#[from] sqlx::Error),
}

#[derive(Debug, thiserror::Error)]
#[expect(dead_code, reason = "Reserved for the logout handler error response")]
pub enum LogoutErrorResponse {
    #[error("User login failed: {0}")]
    LogoutError(#[from] LogoutError),
    #[error("Failed to deserialize JSON: {0}")]
    JsonDeserializationError(#[from] JsonDeserializerRejection),
    #[error("Refresh token not found")]
    MissingRefreshToken,
    #[error("Refresh token expired or invalid")]
    RefreshTokenInvalid,
}
impl IntoResponse for LogoutErrorResponse {
    fn into_response(self) -> axum::response::Response {
        ApiError::into_response(self)
    }
}
impl ApiError for LogoutErrorResponse {
    fn error_message(&self) -> String {
        self.to_string()
    }

    fn error_response(&self) -> Response {
        let (status, error, message) = match self {
            Self::LogoutError(err) => match err {
                LogoutError::Db(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR".to_string(),
                    "An internal database error occurred. Please try again later.".to_string(),
                ),
            },

            Self::JsonDeserializationError(_) => (
                StatusCode::BAD_REQUEST,
                "INVALID_JSON".to_string(),
                "The provided JSON body is invalid or malformed.".to_string(),
            ),

            LogoutErrorResponse::MissingRefreshToken => (
                StatusCode::UNAUTHORIZED,
                "MISSING_REFRESH_TOKEN".to_string(),
                self.to_string(),
            ),
            LogoutErrorResponse::RefreshTokenInvalid => (
                StatusCode::UNAUTHORIZED,
                "REFRESH_TOKEN_INVALID".to_string(),
                self.to_string(),
            ),
        };
        Response {
            status,
            error,
            message,
        }
    }
}
