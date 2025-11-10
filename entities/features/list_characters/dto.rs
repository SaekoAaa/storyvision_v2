use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ListCharactersRequest {
    pub project_id: u64,
}
#[derive(Debug, Deserialize)]
pub struct ListCharactersPagination {
    pub page: usize,
    pub per_page: usize,
}

#[derive(Serialize)]
pub struct Character {
    pub id: u64,
    pub name: String,
    pub age: u32,
    pub gender: String,
    pub source_media: String,
}
