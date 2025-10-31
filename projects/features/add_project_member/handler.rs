use std::sync::Arc;

use axum::{
    Extension,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::JsonDeserializer;

use crate::features::{
    add_project_member::{
        dto::AddMemberRequest, error::AddProjectMemberErrorResponse,
        usecase::add_member_to_project_usecase,
    },
    common::{ProjectState, UserData, api_response::HandlerResult},
};

pub async fn add_project_member_handler(
    State(state): State<Arc<ProjectState>>,
    Path(project_id): Path<u64>,
    Extension(owner): Extension<UserData>,
    serial_data: JsonDeserializer<AddMemberRequest>,
) -> HandlerResult<impl IntoResponse, AddProjectMemberErrorResponse> {
    let AddMemberRequest { member_id } = serial_data.deserialize()?;
    add_member_to_project_usecase(owner.id, project_id, member_id, &state.pool).await?;
    Ok(StatusCode::OK)
}
