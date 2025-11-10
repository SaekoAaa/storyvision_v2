use axum::http::StatusCode;
use sqlx::MySqlPool;

use crate::features::{common::UserData, create_character::dto::CreateCharacterRequest};

pub async fn create_character_usecase(
    pool: &MySqlPool,
    character: CreateCharacterRequest<'_>,
    member_id: u64,
) -> Result<(), super::error::Error> {
    Ok(())
}
