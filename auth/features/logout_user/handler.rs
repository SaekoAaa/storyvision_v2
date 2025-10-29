use {
    axum::{
        Json,
        extract::{ConnectInfo, State},
        http::StatusCode,
        response::IntoResponse,
    },
    axum_extra::extract::{CookieJar, JsonDeserializer, cookie::Cookie},
    std::{net::SocketAddr, sync::Arc},
    time::Duration,
    utoipa::OpenApi,
    uuid::Uuid,
};
use crate::constants::REFRESH_TOKEN_ACCESS_PATH;
use crate::features::common::AuthState;
use crate::features::common::openapi::{BaseErrorResponseSchema, InternalErrorResponse};
use crate::features::logout_user::error::{LogoutErrorResponse};
use crate::features::logout_user::usecase::logout_user_usecase;

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
) -> Result<impl IntoResponse, Result<(CookieJar, LogoutErrorResponse), LogoutErrorResponse>> {
    match jar.get("refresh") {
        Some(cookie) => {
            let token = cookie.value();

            if token.is_empty() {
                let new_jar = jar.add(
                    Cookie::build(("refresh", ""))
                        .http_only(true)
                        .secure(true)
                        .same_site(axum_extra::extract::cookie::SameSite::Lax)
                        .path(REFRESH_TOKEN_ACCESS_PATH)
                        .max_age(Duration::ZERO)
                        .build(),
                );
                return Err(Ok((new_jar, LogoutErrorResponse::MissingRefreshToken)));
            };
            if Uuid::parse_str(token).is_err() {
                let new_jar = jar.add(
                    Cookie::build(("refresh", ""))
                        .http_only(true)
                        .secure(true)
                        .same_site(axum_extra::extract::cookie::SameSite::Lax)
                        .path(REFRESH_TOKEN_ACCESS_PATH)
                        .max_age(Duration::ZERO)
                        .build(),
                );
                return Err(Ok((new_jar, LogoutErrorResponse::RefreshTokenInvalid)));
            }
            logout_user_usecase(token, &app_state.pool)
                .await.map_err(|e| Err(Into::into(e)))?;
            let new_jar = jar.add(
                Cookie::build(("refresh", ""))
                    .http_only(true)
                    .secure(true)
                    .same_site(axum_extra::extract::cookie::SameSite::Lax)
                    .path(REFRESH_TOKEN_ACCESS_PATH)
                    .max_age(Duration::days(15))
                    .build(),
            );
            Ok((StatusCode::OK, new_jar))
        }
        None => Err(Err(LogoutErrorResponse::MissingRefreshToken)),
    }
}
