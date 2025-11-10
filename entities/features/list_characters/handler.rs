use axum::{
    Extension, Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::JsonDeserializer;

use crate::features::{
    common::{EntityState, UserData, api_response::HandlerResult},
    list_characters::dto::{Character, ListCharactersPagination, ListCharactersRequest},
};

pub async fn list_characters_handler(
    State(state): State<EntityState>,
    Extension(user): Extension<UserData>,
    pagination: Query<ListCharactersPagination>,
    serial_data: JsonDeserializer<super::dto::ListCharactersRequest>,
) -> HandlerResult<impl IntoResponse, super::error::ErrorResponse> {
    let ListCharactersRequest { project_id } = serial_data.deserialize()?;

    let characters = super::usecase::list_characters_usecase(
        &state.pool,
        project_id,
        pagination.page,
        pagination.per_page,
        user.id,
    )
    .await?;
    let characters_response = characters
        .into_iter()
        .map(Character::from)
        .collect::<Vec<_>>();
    Ok((StatusCode::OK, Json::from(characters_response)))
}
