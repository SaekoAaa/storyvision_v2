use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::features::{
    common::{ProjectState, UserData, api_response::HandlerResult},
    list_project_members::{
        dto::ProjectMemberResponse, error::ListProjectMembersErrorResponse,
        usecase::list_project_members_usecase,
    },
};

pub async fn list_project_members_handler(
    State(state): State<Arc<ProjectState>>,
    Path(project_id): Path<u64>,
    Extension(user): Extension<UserData>,
) -> HandlerResult<impl IntoResponse, ListProjectMembersErrorResponse> {
    let pm_list = list_project_members_usecase(project_id, user.id, &state.pool).await?;
    let project_members_response = pm_list
        .into_iter()
        .map(ProjectMemberResponse::from)
        .collect::<Vec<_>>();
    Ok((StatusCode::OK, Json::from(project_members_response)))
}
