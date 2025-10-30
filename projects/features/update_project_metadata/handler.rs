use std::sync::Arc;

use axum::{
    Extension,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::JsonDeserializer;
use validator::Validate;

use crate::features::{
    common::{ProjectState, UserData, api_response::HandlerResult},
    update_project_metadata::{
        dto::UpdateProjectMetadataRequest, error::UpdateProjectMetadataErrorResponse,
        usecase::update_project_metadata_usecase,
    },
};

pub async fn update_project_metadata_handler(
    State(state): State<Arc<ProjectState>>,
    Path(project_id): Path<u64>,
    Extension(user): Extension<UserData>,
    serial_data: JsonDeserializer<UpdateProjectMetadataRequest<'_>>,
) -> HandlerResult<impl IntoResponse, UpdateProjectMetadataErrorResponse> {
    let project_data = serial_data.deserialize()?;
    project_data.validate()?;
    update_project_metadata_usecase(
        project_id,
        &project_data.new_project_name,
        project_data.new_description.as_deref().unwrap_or(""),
        user.id,
        &state.pool,
    )
    .await?;
    Ok(StatusCode::OK)
}
