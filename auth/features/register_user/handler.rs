use serde_json::{Value, json};
use validator::{Validate, ValidationErrors};
use {
    crate::{
        constants::REFRESH_TOKEN_ACCESS_PATH,
        features::{
            common::{
                AuthState,
                api_response::HandlerResult,
                openapi::{BaseErrorResponseSchema, InternalErrorResponse},
            },
            register_user::{
                dto::{RegisterUserRequest, RegisterUserResponse},
                error::RegisterErrorResponse,
                usecase::register_user,
            },
        },
    },
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
};
#[derive(Debug, OpenApi)]
#[openapi(paths(handler_register_user))]
pub struct RegisterUserOpenApi;

fn validation_errors_to_json(errors: ValidationErrors) -> Value {
    let mut map = serde_json::Map::new();

    for (field, kind) in errors.field_errors().iter() {
        let messages: Vec<String> = kind
            .iter()
            .map(|e| {
                if let Some(msg) = &e.message {
                    msg.to_string()
                } else {
                    format!("validation failed on constraint: {:?}", e.code)
                }
            })
            .collect();

        map.insert(field.to_string(), json!(messages));
    }

    Value::Object(map)
}

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
    let register_user_request = serial_data.deserialize()?;
    register_user_request
        .validate()
        .map_err(|e| RegisterErrorResponse::ValidationError(validation_errors_to_json(e)))?;
    let RegisterUserRequest { email, password } = register_user_request;
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
            .secure(app_state.secure_cookies)
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
