use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct GetEventsPagination {
    #[validate(range(min = 1))]
    pub page: u32,

    #[validate(range(min = 1, max = 100))]
    pub per_page: u32,

    pub search: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EventItem {
    pub id: String,
    pub name: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub timestamp: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetEventsResponse {
    pub items: Vec<EventItem>,
    pub page: u32,
    pub per_page: u32,
    pub total: u32,
    pub has_more: bool,
}
