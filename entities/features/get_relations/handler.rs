use std::sync::Arc;

use axum::{
    Json,
    extract::{Extension, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use validator::Validate;

use crate::features::common::{AppState, UserData, api_response::HandlerResult};

use super::{
    dto::{GetRelationsPagination, GetRelationsRequest, GetRelationsResponse},
    error::{GetRelationsError, GetRelationsErrorResponse},
    usecase::get_relations_usecase,
};

pub async fn get_relations_handler(
    State(state): State<Arc<AppState>>,
    Extension(user_data): Extension<UserData>,
    Query(pagination): Query<GetRelationsPagination>,
    Json(request): Json<GetRelationsRequest>,
) -> HandlerResult<impl IntoResponse, GetRelationsErrorResponse> {
    pagination.validate()?;
    request.validate()?;

    if !user_data.projects_list.contains(&request.project_id) {
        return Err(GetRelationsError::AccessDenied.into());
    }

    let response = get_relations_usecase(request, pagination, &state.graph).await?;

    Ok((StatusCode::OK, Json(response)))
}
