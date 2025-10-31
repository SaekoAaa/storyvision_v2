use crate::features::common::api_error::{ApiError, Response};
use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::JsonDeserializerRejection;
use serde_json::json;

#[derive(thiserror::Error, Debug)]
pub enum RefreshTokenError {
    #[error("db error: {0}")]
    Db(#[from] sqlx::Error),

    #[error("JWT error: {0}")]
    JsonWebTokenError(#[from] jsonwebtoken::errors::Error),
    #[error("JWT token not found")]
    InvalidRefreshToken,
}

#[derive(Debug, thiserror::Error)]
pub enum RefreshTokenErrorResponse {
    #[error("User login failed: {0}")]
    RefreshTokenError(#[from] RefreshTokenError),
    #[error("Failed to deserialize JSON: {0}")]
    JsonDeserializationError(#[from] JsonDeserializerRejection),
    #[error("Refresh token not found")]
    MissingRefreshToken,
    #[error("Refresh token expired or invalid")]
    RefreshTokenInvalid,
}
impl IntoResponse for RefreshTokenErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let err = self.error_response();
        (
            err.status,
            Json(json!({
                "error": err.error,
                "message": err.message
            })),
        )
            .into_response()
    }
}

impl ApiError for RefreshTokenErrorResponse {
    fn error_response(&self) -> Response {
        let (status, error, message) = match self {
            Self::RefreshTokenError(err) => match err {
                RefreshTokenError::Db(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR".to_string(),
                    "An internal database error occurred. Please try again later.".to_string(),
                ),
                RefreshTokenError::JsonWebTokenError(_) => (
                    StatusCode::UNAUTHORIZED,
                    "JWT_ERROR".to_string(),
                    "Failed to validate or decode JWT token.".to_string(),
                ),
                RefreshTokenError::InvalidRefreshToken => (
                    StatusCode::UNAUTHORIZED,
                    "INVALID_REFRESH_TOKEN".to_string(),
                    "The provided refresh token is invalid or expired.".to_string(),
                ),
            },

            Self::JsonDeserializationError(_) => (
                StatusCode::BAD_REQUEST,
                "INVALID_JSON".to_string(),
                "The provided JSON body is invalid or malformed.".to_string(),
            ),

            RefreshTokenErrorResponse::MissingRefreshToken => (
                StatusCode::UNAUTHORIZED,
                "MISSING_REFRESH_TOKEN".to_string(),
                self.to_string(),
            ),
            RefreshTokenErrorResponse::RefreshTokenInvalid => (
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

    fn error_message(&self) -> String {
        self.to_string()
    }
}
