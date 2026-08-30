use crate::constants::REFRESH_TOKEN_ACCESS_PATH;
use crate::features::common::AuthState;
use crate::features::common::api_response::HandlerResult;
use crate::features::common::openapi::{BaseErrorResponseSchema, InternalErrorResponse};
use crate::features::login_user::dto::{LoginUserRequest, LoginUserResponse};
use crate::features::login_user::error::LoginErrorResponse;
use crate::features::login_user::usecase::{LoginData, login_user_usecase};
use axum::Json;
use axum::extract::{ConnectInfo, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::{CookieJar, JsonDeserializer};
use std::net::SocketAddr;
use std::sync::Arc;
use time::Duration;
use utoipa::OpenApi;
use validator::Validate;

#[derive(Debug, OpenApi)]
#[openapi(paths(handler_login_user))]
pub struct LoginUserOpenApi;

/// Log in user
///
/// - Verify credentials
/// - Check for user in DB
/// - Add refresh token
/// - Return refresh token
#[utoipa::path(post, path = "/login", tag = "auth",
    request_body = LoginUserRequest,
    responses(
        (status = OK, description = "User registered successfully", body = LoginUserResponse),
        (status = NOT_FOUND, description = "Invalid credentials or user not found", body = BaseErrorResponseSchema),
        (status = BAD_REQUEST, description = "Data validation failed", body = BaseErrorResponseSchema),
        (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
    )
)]
pub async fn handler_login_user(
    State(app_state): State<Arc<AuthState>>,
    ConnectInfo(connect_info): ConnectInfo<SocketAddr>,
    jar: CookieJar,
    serial_data: JsonDeserializer<LoginUserRequest<'_>>,
) -> HandlerResult<impl IntoResponse, LoginErrorResponse> {
    let login_user_request = serial_data.deserialize()?;
    login_user_request.validate()?;
    let LoginUserRequest { email, password } = login_user_request;
    let LoginData {
        id,
        refresh_token,
        access_token,
    } = login_user_usecase(
        &email,
        &password,
        &app_state.saltstring,
        &app_state.token_secret,
        connect_info,
        &app_state.pool,
    )
    .await?;
    let token_jar = jar.add(
        Cookie::build(("refresh", refresh_token.to_string()))
            .http_only(true)
            .secure(app_state.secure_cookies)
            .same_site(axum_extra::extract::cookie::SameSite::Lax)
            .path(REFRESH_TOKEN_ACCESS_PATH)
            .max_age(Duration::days(15))
            .build(),
    );
    Ok((
        StatusCode::OK,
        token_jar,
        Json::from(LoginUserResponse {
            id,
            email: email.into_owned(),
            access_token,
        }),
    ))
}
