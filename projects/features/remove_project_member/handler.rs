use std::sync::Arc;

use axum::{
    Extension,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::JsonDeserializer;

use crate::features::{
    common::{ProjectState, UserData, api_response::HandlerResult},
    remove_project_member::{
        dto::RemoveProjectMemberRequest, error::RemoveProjectMemberErrorResponse,
        usecase::remove_member_from_project_usecase,
    },
};

pub async fn remove_member_from_project_handler(
    State(state): State<Arc<ProjectState>>,
    Path(project_id): Path<u64>,
    Extension(user): Extension<UserData>,
    serial_data: JsonDeserializer<RemoveProjectMemberRequest>,
) -> HandlerResult<impl IntoResponse, RemoveProjectMemberErrorResponse> {
    let RemoveProjectMemberRequest { member_id } = serial_data.deserialize()?;
    remove_member_from_project_usecase(user.id, project_id, member_id, &state.pool).await?;
    Ok(StatusCode::OK)
}
