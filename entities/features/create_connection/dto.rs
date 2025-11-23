use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateConnectionRequest {
    #[validate(length(min = 1))]
    pub from_entity_id: String,

    #[validate(length(min = 1))]
    pub to_entity_id: String,

    #[validate(length(min = 1))]
    pub relation_id: String,

    /// Доп. свойства конкретного экземпляра связи
    pub attributes: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct CreateConnectionResponse {
    pub id: String,
    pub project_id: u64,
    pub from_entity_id: String,
    pub to_entity_id: String,
    pub relation_id: String,
    pub relation_type: String,
}
