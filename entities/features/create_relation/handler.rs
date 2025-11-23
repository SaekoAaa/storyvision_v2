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
    dto::{CreateRelationRequest, CreateRelationResponse},
    error::{CreateRelationError, CreateRelationErrorResponse},
    usecase::create_relation_usecase,
};

pub async fn create_relation_handler(
    State(state): State<Arc<AppState>>,
    Extension(user_data): Extension<UserData>,
    Path(project_id): Path<u64>,
    Json(request): Json<CreateRelationRequest>,
) -> HandlerResult<impl IntoResponse, CreateRelationErrorResponse> {
    request.validate()?;

    if !user_data.projects_list.contains(&project_id) {
        return Err(CreateRelationError::AccessDenied.into());
    }

    let relation = create_relation_usecase(user_data.id, request, &state.graph, project_id).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateRelationResponse {
            id: relation.id,
            project_id: relation.project_id,
            name: relation.name,
            relation_type: relation.relation_type,
            description: relation.description,
        }),
    ))
}
