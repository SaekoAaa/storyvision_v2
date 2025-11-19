use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct GetRelationsRequest {
    #[validate(range(min = 1))]
    pub project_id: u64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct GetRelationsPagination {
    #[validate(range(min = 1))]
    pub page: u32,

    #[validate(range(min = 1, max = 100))]
    pub per_page: u32,

    pub search: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RelationItem {
    pub id: String,
    pub name: String,
    pub relation_type: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetRelationsResponse {
    pub items: Vec<RelationItem>,
    pub page: u32,
    pub per_page: u32,
    pub total: u32,
    pub has_more: bool,
}
