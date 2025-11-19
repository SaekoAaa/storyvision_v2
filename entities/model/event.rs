use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub project_id: u64,
    pub name: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub timestamp: Option<String>, // RFC3339 строка
    pub attributes: Option<serde_json::Value>,
}
