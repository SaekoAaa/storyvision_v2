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
    list_characters::dto::ListCharacterPagination,
};

use super::{
    dto::ListCharactersResponse,
    error::{ListCharactersError, ListCharactersErrorResponse},
    usecase::list_characters_usecase,
};

pub async fn list_characters_handler(
    State(state): State<Arc<AppState>>,
    Extension(user_data): Extension<UserData>,
    Query(query): Query<ListCharacterPagination>,

    Path(project_id): Path<u64>,
) -> HandlerResult<impl IntoResponse, ListCharactersErrorResponse> {
    query.validate()?;

    // Проверка доступа к проекту
    if !user_data.projects_list.contains(&project_id) {
        return Err(ListCharactersError::AccessDenied.into());
    }

    let response = list_characters_usecase(
        query.page,
        query.per_page,
        query.search,
        project_id,
        &state.graph,
    )
    .await?;

    Ok((StatusCode::OK, Json(response)))
}
