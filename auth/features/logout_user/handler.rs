use axum::Extension;

use crate::constants::REFRESH_TOKEN_ACCESS_PATH;
use crate::features::common::AuthState;
use crate::features::common::openapi::{BaseErrorResponseSchema, InternalErrorResponse};
use crate::features::logout_user::error::LogoutErrorResponse;
use crate::features::logout_user::usecase::logout_user_usecase;
use crate::model::UserData;
use {
    axum::{extract::State, http::StatusCode, response::IntoResponse},
    axum_extra::extract::{CookieJar, cookie::Cookie},
    std::sync::Arc,
    time::Duration,
    utoipa::OpenApi,
    uuid::Uuid,
};

#[derive(Debug, OpenApi)]
#[openapi(paths(handler_logout_user))]
pub struct LogoutUserOpenApi;

/// Logout user
///
/// - Get token from cookie
/// - Revoke token in DB
/// - Add empty token in cookie
#[utoipa::path(
    post, path = "/logout", tag = "auth",
    responses(
        (status = OK, description = "Logged out"),
        (status = UNAUTHORIZED, description = "Invalid or expired refresh token", body = BaseErrorResponseSchema),
        (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
    ),
    security(
        ("refresh_token" = [])
    )
)]
pub async fn handler_logout_user(
    State(app_state): State<Arc<AuthState>>,
    jar: CookieJar,
    Extension(userdata): Extension<UserData>,
) -> impl IntoResponse {
    if let Err(e) = logout_user_usecase(userdata.id, &app_state.pool).await {
        tracing::debug!(?e)
    };
    let new_jar = jar.add(
        Cookie::build(("refresh", ""))
            .http_only(true)
            .secure(app_state.secure_cookies)
            .same_site(axum_extra::extract::cookie::SameSite::Lax)
            .path(REFRESH_TOKEN_ACCESS_PATH)
            .max_age(Duration::ZERO)
            .build(),
    );
    (new_jar, StatusCode::OK)
}
