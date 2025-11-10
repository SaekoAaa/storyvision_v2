use axum::{Extension, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::JsonDeserializer;

use crate::features::{
    common::{EntityState, UserData, api_response::HandlerResult},
    create_character::dto::CreateCharacterRequest,
};

pub async fn create_character_handler(
    State(state): State<EntityState>,
    Extension(user): Extension<UserData>,
    serial_data: JsonDeserializer<super::dto::CreateCharacterRequest<'_>>,
) -> HandlerResult<impl IntoResponse, super::error::ErrorResponse> {
    let char: CreateCharacterRequest = serial_data.deserialize()?;
    super::usecase::create_character_usecase(&state.pool, char, user.id).await?;
    Ok(StatusCode::OK)
}
