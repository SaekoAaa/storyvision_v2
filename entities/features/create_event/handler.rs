use std::sync::Arc;

use axum::{
    Json,
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use validator::Validate;

use crate::features::common::{AppState, UserData, api_response::HandlerResult};

use super::{
    dto::{CreateEventRequest, CreateEventResponse},
    error::{CreateEventError, CreateEventErrorResponse},
    usecase::create_event_usecase,
};

pub async fn create_event_handler(
    State(state): State<Arc<AppState>>,
    Extension(user_data): Extension<UserData>,
    Path(project_id): Path<u64>,
    Json(request): Json<CreateEventRequest>,
) -> HandlerResult<impl IntoResponse, CreateEventErrorResponse> {
    request.validate()?;

    if !user_data.projects_list.contains(&project_id) {
        return Err(CreateEventError::AccessDenied.into());
    }

    let event = create_event_usecase(user_data.id, request, &state.graph, project_id).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateEventResponse {
            id: event.id,
            project_id: event.project_id,
            name: event.name,
            location: event.location,
            description: event.description,
            timestamp: event.timestamp,
        }),
    ))
}
