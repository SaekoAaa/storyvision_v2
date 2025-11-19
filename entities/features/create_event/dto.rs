use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateEventRequest {
    #[validate(range(min = 1))]
    pub project_id: u64,

    #[validate(length(min = 1, max = 200))]
    pub name: String,

    #[validate(length(max = 500))]
    pub location: Option<String>,

    #[validate(length(max = 2000))]
    pub description: Option<String>,

    /// Строка с датой/временем, например "2024-01-15T10:00:00Z"
    pub timestamp: Option<String>,

    pub attributes: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct CreateEventResponse {
    pub id: String,
    pub project_id: u64,
    pub name: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub timestamp: Option<String>,
}
