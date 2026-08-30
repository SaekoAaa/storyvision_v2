use axum::Extension;

use crate::features::me::{
    dto::GetUserResponse, error::GetUserErrorResponse, usecase::get_user_usecase,
};

use {
    crate::{
        features::common::{AuthState, api_response::HandlerResult},
        model::UserData,
    },
    axum::{Json, extract::State, http::StatusCode, response::IntoResponse},
    std::sync::Arc,
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
