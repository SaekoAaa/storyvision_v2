use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCharacterRequest {
    #[validate(range(min = 1))]
    pub project_id: u64,

    #[validate(length(min = 1, max = 200))]
    pub name: String,

    #[validate(length(max = 1000))]
    pub description: Option<String>,

    pub attributes: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct CreateCharacterResponse {
    pub id: String,
    pub project_id: u64,
    pub name: String,
    pub description: Option<String>,
}
