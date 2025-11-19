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
    dto::{GetConnectionsPagination, GetConnectionsRequest, GetConnectionsResponse},
    error::{GetConnectionsError, GetConnectionsErrorResponse},
    usecase::get_connections_usecase,
};

pub async fn get_connections_handler(
    State(state): State<Arc<AppState>>,
    Extension(user_data): Extension<UserData>,
    Query(pagination): Query<GetConnectionsPagination>,
    Json(request): Json<GetConnectionsRequest>,
) -> HandlerResult<impl IntoResponse, GetConnectionsErrorResponse> {
    pagination.validate()?;
    request.validate()?;

    if !user_data.projects_list.contains(&request.project_id) {
        return Err(GetConnectionsError::AccessDenied.into());
    }

    let response = get_connections_usecase(request, pagination, &state.graph).await?;

    Ok((StatusCode::OK, Json(response)))
}
