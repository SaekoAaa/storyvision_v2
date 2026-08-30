use std::sync::Arc;

use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::features::{
    common::{ProjectState, UserData, api_response::HandlerResult},
    list_projects::{
        dto::ProjectResponse, error::ListProjectErrorResponse, usecase::list_projects_usecase,
    },
};

pub async fn list_projects_handler(
    State(state): State<Arc<ProjectState>>,
    Extension(user): Extension<UserData>,
) -> HandlerResult<impl IntoResponse, ListProjectErrorResponse> {
    let project_list = list_projects_usecase(user.id, &state.pool).await?;
    let project_list_response = project_list
        .into_iter()
        .map(ProjectResponse::from)
        .collect::<Vec<_>>();
    Ok((StatusCode::OK, Json::from(project_list_response)))
}
