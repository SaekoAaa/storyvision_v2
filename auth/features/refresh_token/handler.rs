use crate::features::common::AuthState;
use crate::features::common::api_response::HandlerResult;
use crate::features::common::openapi::{BaseErrorResponseSchema, InternalErrorResponse};
use crate::features::refresh_token::dto::RefreshTokenResponse;
use crate::features::refresh_token::error::RefreshTokenErrorResponse;
use crate::features::refresh_token::usecase::refresh_token_usecase;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use std::sync::Arc;
use utoipa::OpenApi;

#[derive(Debug, OpenApi)]
#[openapi(paths(handler_refresh_token))]
pub struct RefreshTokenOpenApi;

/// Get new access token
///
/// - Get token from cookie
/// - Search for user
/// - If successful return new access_token
#[utoipa::path(post, path = "/refresh", tag = "auth",
    responses(
        (status = OK, description = "User registered successfully", body = RefreshTokenResponse),
        (status = NOT_FOUND, description = "Invalid credentials or user not found", body = BaseErrorResponseSchema),
        (status = UNAUTHORIZED, description = "Invalid or expired refresh token", body = BaseErrorResponseSchema),
        (status = BAD_REQUEST, description = "Data validation failed", body = BaseErrorResponseSchema),
        (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
    ),
    security(
        ("refresh_token" = [])
    )
)]
pub async fn handler_refresh_token(
    State(app_state): State<Arc<AuthState>>,
    jar: CookieJar,
) -> HandlerResult<impl IntoResponse, RefreshTokenErrorResponse> {
    match jar.get("refresh") {
        Some(token) => {
            let access_token =
                refresh_token_usecase(token.value(), &app_state.token_secret, &app_state.pool)
                    .await?;
            Ok((
                StatusCode::OK,
                Json::from(RefreshTokenResponse { access_token }),
            ))
        }
        None => Err(RefreshTokenErrorResponse::MissingRefreshToken),
    }
}
