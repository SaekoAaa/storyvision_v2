use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: String,
    pub project_id: u64,
    pub name: String,
    pub description: Option<String>,
    pub attributes: Option<serde_json::Value>,
}
