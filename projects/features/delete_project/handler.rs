use std::sync::Arc;

use axum::{
    Extension,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::features::{
    common::{ProjectState, UserData, api_response::HandlerResult},
    delete_project::{error::DeleteProjectErrorResponse, usecase::delete_project_usecase},
};

pub async fn delete_project_handler(
    State(state): State<Arc<ProjectState>>,
    Path(project_id): Path<u64>,
    Extension(user): Extension<UserData>,
) -> HandlerResult<impl IntoResponse, DeleteProjectErrorResponse> {
    delete_project_usecase(project_id, user.id, &state.pool).await?;
    Ok(StatusCode::OK)
}
