use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ListCharactersRequest {
    #[validate(range(min = 1))]
    pub project_id: u64,
}
#[derive(Debug, Deserialize, Validate)]
pub struct ListCharacterPagination {
    pub page: u32,
    pub per_page: u32,
    pub search: Option<String>,
}
#[derive(Debug, Serialize)]
pub struct CharacterItem {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListCharactersResponse {
    pub characters: Vec<CharacterItem>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
    pub has_more: bool,
}
