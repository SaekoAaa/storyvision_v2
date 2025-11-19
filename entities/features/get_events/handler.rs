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
    dto::{GetEventsPagination, GetEventsRequest, GetEventsResponse},
    error::{GetEventsError, GetEventsErrorResponse},
    usecase::get_events_usecase,
};

pub async fn get_events_handler(
    State(state): State<Arc<AppState>>,
    Extension(user_data): Extension<UserData>,
    Query(pagination): Query<GetEventsPagination>,
    Json(request): Json<GetEventsRequest>,
) -> HandlerResult<impl IntoResponse, GetEventsErrorResponse> {
    pagination.validate()?;
    request.validate()?;

    if !user_data.projects_list.contains(&request.project_id) {
        return Err(GetEventsError::AccessDenied.into());
    }

    let response = get_events_usecase(request, pagination, &state.graph).await?;

    Ok((StatusCode::OK, Json(response)))
}
