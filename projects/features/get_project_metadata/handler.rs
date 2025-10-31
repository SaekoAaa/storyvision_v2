use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::features::{
    common::{ProjectState, UserData, api_response::HandlerResult},
    get_project_metadata::{
        dto::ProjectResponse, error::GetProjectErrorResponse, usecase::get_project_usecase,
    },
};

pub async fn list_projects_handler(
    State(state): State<Arc<ProjectState>>,
    Path(project_id): Path<u64>,
    Extension(user): Extension<UserData>,
) -> HandlerResult<impl IntoResponse, GetProjectErrorResponse> {
    let project = get_project_usecase(user.id, project_id, &state.pool).await?;
    match project {
        Some(project) => Ok((StatusCode::OK, Json::from(ProjectResponse::from(project)))),
        None => Err(GetProjectErrorResponse::NotFound {
            message: format!("Project {} not found for user {}", project_id, user.id).into(),
            response: "Failed to find project".into(),
        }),
    }
}
