use std::sync::Arc;

use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::JsonDeserializer;
use validator::Validate;

use crate::features::{
    common::{ProjectState, UserData, api_response::HandlerResult},
    create_project::{
        dto::{CreateProjectRequest, CreateProjectResponse},
        error::CreateProjectErrorResponse,
        usecase::{ProjectData, create_project_usecase},
    },
};

pub async fn create_project_handler(
    State(state): State<Arc<ProjectState>>,
    Extension(user): Extension<UserData>,
    serial_data: JsonDeserializer<CreateProjectRequest<'_>>,
) -> HandlerResult<impl IntoResponse, CreateProjectErrorResponse> {
    let project_data = serial_data.deserialize()?;
    project_data.validate()?;
    let ProjectData {
        id,
        name,
        valid_name,
        description,
        owner_id,
    } = create_project_usecase(
        &project_data.project_name,
        user.id,
        project_data.description.as_deref().unwrap_or(""),
        &state.pool,
    )
    .await?;
    Ok((
        StatusCode::OK,
        Json::from(CreateProjectResponse {
            id,
            name: name.to_owned(),
            valid_name,
            owner_id,
            description,
        }),
    ))
}
