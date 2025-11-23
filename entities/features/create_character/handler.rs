use axum::{
    Json,
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;
use validator::Validate;

use crate::features::common::{AppState, UserData, api_response::HandlerResult};

use super::{
    dto::{CreateCharacterRequest, CreateCharacterResponse},
    error::{CreateCharacterError, CreateCharacterErrorResponse},
    usecase::create_character_usecase,
};

pub async fn create_character_handler(
    State(state): State<Arc<AppState>>,
    Extension(user_data): Extension<UserData>,
    Path(project_id): Path<u64>,
    Json(payload): Json<CreateCharacterRequest>,
) -> HandlerResult<impl IntoResponse, CreateCharacterErrorResponse> {
    // Валидация входных данных
    payload.validate()?;

    // Проверка доступа к проекту
    if !user_data.projects_list.contains(&project_id) {
        return Err(CreateCharacterError::AccessDenied.into());
    }

    // Создание персонажа
    let character =
        create_character_usecase(user_data.id, payload, &state.graph, project_id).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateCharacterResponse {
            id: character.id,
            project_id: project_id,
            name: character.name,
            description: character.description,
        }),
    ))
}
