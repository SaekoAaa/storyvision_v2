use std::sync::Arc;

use axum::{
    Json,
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use validator::Validate;

use crate::features::{
    common::{AppState, UserData, api_response::HandlerResult},
    get_connections::dto::GetConnectionsQuery,
};

use super::{
    error::{GetConnectionsError, GetConnectionsErrorResponse},
    usecase::get_connections_usecase,
};

pub async fn get_connections_handler(
    State(state): State<Arc<AppState>>,
    Extension(user_data): Extension<UserData>,
    Query(query): Query<GetConnectionsQuery>,
    Path(project_id): Path<u64>,
) -> HandlerResult<impl IntoResponse, GetConnectionsErrorResponse> {
    query.validate()?;

    if !user_data.projects_list.contains(&project_id) {
        return Err(GetConnectionsError::AccessDenied.into());
    }

    let response = get_connections_usecase(query, &state.graph, project_id).await?;

    Ok((StatusCode::OK, Json(response)))
}
