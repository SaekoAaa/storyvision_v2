use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub id: String,
    pub project_id: u64,
    pub from_id: String,
    pub to_id: String,
    pub relation_id: String,
    pub relation_type: String,
    pub attributes: Option<serde_json::Value>,
}
