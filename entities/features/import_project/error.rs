use crate::features::common::api_error::{ApiError, Response};
use axum::http::StatusCode;
use axum::response::IntoResponse;

#[derive(thiserror::Error, Debug)]
pub enum ImportGraphError {
    #[error("database error: {0}")]
    DatabaseError(#[from] neo4rs::Error),

    #[error("access denied to project")]
    AccessDenied,

    #[error("invalid multipart data")]
    InvalidMultipart,

    #[error("missing file field 'file' in form-data")]
    FileFieldMissing,

    #[error("failed to read uploaded file")]
    FileReadError,

    #[error("failed to parse JSON: {0}")]
    JsonParseError(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ImportGraphErrorResponse {
    #[error("Failed to import graph: {0}")]
    ImportGraphError(#[from] ImportGraphError),
}

impl IntoResponse for ImportGraphErrorResponse {
    fn into_response(self) -> axum::response::Response {
        ApiError::into_response(self)
    }
}

impl ApiError for ImportGraphErrorResponse {
    fn error_response(&self) -> Response {
        let (status, error, message) = match self {
            Self::ImportGraphError(err) => match err {
                ImportGraphError::DatabaseError(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR".to_string(),
                    "An internal database error occurred. Please try again later.".to_string(),
                ),
                ImportGraphError::AccessDenied => (
                    StatusCode::FORBIDDEN,
                    "ACCESS_DENIED".to_string(),
                    "You don't have access to this project.".to_string(),
                ),
                ImportGraphError::InvalidMultipart => (
                    StatusCode::BAD_REQUEST,
                    "INVALID_MULTIPART".to_string(),
                    "Invalid multipart/form-data payload.".to_string(),
                ),
                ImportGraphError::FileFieldMissing => (
                    StatusCode::BAD_REQUEST,
                    "FILE_FIELD_MISSING".to_string(),
                    "Missing 'file' field in multipart/form-data.".to_string(),
                ),
                ImportGraphError::FileReadError => (
                    StatusCode::BAD_REQUEST,
                    "FILE_READ_ERROR".to_string(),
                    "Failed to read uploaded file.".to_string(),
                ),
                ImportGraphError::JsonParseError(e) => (
                    StatusCode::BAD_REQUEST,
                    "JSON_PARSE_ERROR".to_string(),
                    format!("Failed to parse JSON: {}", e),
                ),
            },
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
