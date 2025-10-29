use axum::debug_handler;
use {
    crate::{
        constants::REFRESH_TOKEN_ACCESS_PATH,
        features::{
            common::{
                api_response::HandlerResult
                ,
                openapi::{BaseErrorResponseSchema, InternalErrorResponse},
                AuthState,
            },
            register_user::{
                dto::{RegisterUserRequest, RegisterUserResponse},
                error::RegisterErrorResponse,
                usecase::register_user,
            },
        },
    },
    axum::{
        extract::{ConnectInfo, State},
        http::StatusCode,
        response::IntoResponse,
        Json,
    },
    axum_extra::extract::{cookie::Cookie, CookieJar, JsonDeserializer},
    std::{net::SocketAddr, sync::Arc},
    time::Duration,
    utoipa::OpenApi,
};
#[derive(Debug, OpenApi)]
#[openapi(paths(handler_register_user))]
pub struct RegisterUserOpenApi;

#[utoipa::path(
    post,
    path = "/register",
    tag = "auth",
    request_body = RegisterUserRequest,
    responses(
        (status = OK, description = "User registered successfully", body = RegisterUserResponse),
        (status = BAD_REQUEST, description = "Data validation failed", body = BaseErrorResponseSchema),
        (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
    )
)]
pub async fn handler_register_user(
    State(app_state): State<Arc<AuthState>>,
    ConnectInfo(connect_info): ConnectInfo<SocketAddr>,
    jar: CookieJar,
    serial_data: JsonDeserializer<RegisterUserRequest<'_>>,
) -> HandlerResult<impl IntoResponse, RegisterErrorResponse> {
    let RegisterUserRequest { email, password } = serial_data.deserialize()?;
    let register_data = register_user(
        &app_state.pool,
        &email,
        &password,
        &app_state.saltstring,
        &app_state.token_secret,
        connect_info,
    )
    .await?;
    let token_jar = jar.add(
        Cookie::build(("refresh", register_data.refresh_token.to_string()))
            .http_only(true)
            .secure(true)
            .same_site(axum_extra::extract::cookie::SameSite::Lax)
            .path(REFRESH_TOKEN_ACCESS_PATH)
            .max_age(Duration::days(15))
            .build(),
    );
    Ok((
        StatusCode::OK,
        token_jar,
        Json::from(RegisterUserResponse {
            id: register_data.id,
            email: email.to_string(),
            access_token: register_data.access_token,
        }),
    ))
}
