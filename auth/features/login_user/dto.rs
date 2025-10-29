use std::borrow::Cow;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::features::common::api_response::ApiResponse;

#[derive(Deserialize, validator::Validate, ToSchema)]
pub struct LoginUserRequest<'a> {
    #[serde(borrow)]
    #[validate(email)]
    #[schema(
        example = "user@example.com",
        max_length = 30,
        min_length = 5,
        default = "user@example.com",
        pattern = r"^[^@\s]+@[^@\s]+\.[^@\s]+$"
    )]
    pub email: Cow<'a, str>,
    #[serde(borrow)]
    #[validate(length(min = 8, max = 30))]
    #[schema(
        example = "password12345",
        max_length = 8,
        min_length = 30,
        default = "password12345"
    )]
    pub password: Cow<'a, str>,
}

#[derive(Serialize, ToSchema)]
pub struct LoginUserResponse {
    pub id: u64,
    pub email: String,
    pub access_token: String,
}
// impl IntoResponse for LoginUserResponse {
//     fn into_response(self) -> Response {
//         //Metrics Logic
//         (
//             StatusCode::OK,
//             Json(self)
//         ).into_response()
//     }
// }