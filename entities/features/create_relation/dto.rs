use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRelationRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: String,

    /// Логический тип/категория отношения, например "TRANSFER", "MAIN_ROLE"
    #[validate(length(min = 1, max = 100))]
    pub relation_type: String,

    #[validate(length(max = 2000))]
    pub description: Option<String>,

    pub attributes: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct CreateRelationResponse {
    pub id: String,
    pub project_id: u64,
    pub name: String,
    pub relation_type: String,
    pub description: Option<String>,
}
