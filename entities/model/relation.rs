use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub id: String,
    pub project_id: u64,
    pub name: String,
    pub relation_type: String,
    pub description: Option<String>,
    pub attributes: Option<serde_json::Value>,
}
