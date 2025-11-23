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
    dto::{CreateConnectionRequest, CreateConnectionResponse},
    error::{CreateConnectionError, CreateConnectionErrorResponse},
    usecase::create_connection_usecase,
};

pub async fn create_connection_handler(
    State(state): State<Arc<AppState>>,
    Extension(user_data): Extension<UserData>,
    Path(project_id): Path<u64>,
    Json(request): Json<CreateConnectionRequest>,
) -> HandlerResult<impl IntoResponse, CreateConnectionErrorResponse> {
    request.validate()?;

    if !user_data.projects_list.contains(&project_id) {
        return Err(CreateConnectionError::AccessDenied.into());
    }

    let conn = create_connection_usecase(user_data.id, request, &state.graph, project_id).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateConnectionResponse {
            id: conn.id,
            project_id: conn.project_id,
            from_entity_id: conn.from_id,
            to_entity_id: conn.to_id,
            relation_id: conn.relation_id,
            relation_type: conn.relation_type,
        }),
    ))
}
