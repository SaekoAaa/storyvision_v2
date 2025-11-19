use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct GetConnectionsRequest {
    #[validate(range(min = 1))]
    pub project_id: u64,

    /// Если указан, вернем связи, где эта сущность либо from, либо to
    pub entity_id: Option<String>,

    /// Фильтр по relation_id (шаблон)
    pub relation_id: Option<String>,

    /// Фильтр по строковому типу связи
    pub relation_type: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct GetConnectionsPagination {
    #[validate(range(min = 1))]
    pub page: u32,

    #[validate(range(min = 1, max = 100))]
    pub per_page: u32,
}

#[derive(Debug, Serialize)]
pub struct ConnectionItem {
    pub id: String,
    pub from_entity_id: String,
    pub to_entity_id: String,
    pub relation_id: String,
    pub relation_type: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetConnectionsResponse {
    pub items: Vec<ConnectionItem>,
    pub page: u32,
    pub per_page: u32,
    pub total: u32,
    pub has_more: bool,
}
