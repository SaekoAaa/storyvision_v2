use axum::Extension;
use serde_json::{Value, json};
use tracing_subscriber::field::debug;
use validator::{Validate, ValidationErrors};

use crate::features::me::{
    dto::GetUserResponse, error::GetUserErrorResponse, usecase::get_user_usecase,
};

use {
    crate::{
        constants::REFRESH_TOKEN_ACCESS_PATH,
        features::common::{
            AuthState,
            api_response::HandlerResult,
            openapi::{BaseErrorResponseSchema, InternalErrorResponse},
        },
        model::UserData,
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
pub async fn handler_get_user(
    State(app_state): State<Arc<AuthState>>,
    Extension(userdata): Extension<UserData>,
) -> HandlerResult<impl IntoResponse, GetUserErrorResponse> {
    tracing::debug!("---");
    let register_data = get_user_usecase(&app_state.pool, userdata.id).await?;
    Ok((
        StatusCode::OK,
        Json::from(GetUserResponse {
            email: register_data.email,
            role: register_data.role,
        }),
    ))
}
