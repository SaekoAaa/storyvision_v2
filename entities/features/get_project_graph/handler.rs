use axum::{
    Json,
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;
use validator::Validate;

use crate::features::{
    common::{AppState, UserData, api_response::HandlerResult},
    get_project_graph::dto::GetProjectGraphQuery,
};

use super::{
    error::{GetProjectGraphError, GetProjectGraphErrorResponse},
    usecase::get_project_graph_usecase,
};

pub async fn get_project_graph_handler(
    State(state): State<Arc<AppState>>,
    Extension(user_data): Extension<UserData>,
    Query(request): Query<GetProjectGraphQuery>,

    Path(project_id): Path<u64>,
) -> HandlerResult<impl IntoResponse, GetProjectGraphErrorResponse> {
    request.validate()?;

    if !user_data.projects_list.contains(&project_id) {
        return Err(GetProjectGraphError::AccessDenied.into());
    }

    let graph = get_project_graph_usecase(request, project_id, &state.graph).await?;

    Ok((StatusCode::OK, Json(graph)))
}
