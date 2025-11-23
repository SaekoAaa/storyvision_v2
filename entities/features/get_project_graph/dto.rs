use serde::Deserialize;
use serde::Serialize;
use validator::Validate;
#[derive(Debug, Serialize)]
pub struct GraphNode {
    pub id: String,        // id ноды (Character/Event)
    pub label: String,     // имя для отображения
    pub node_type: String, // "Character" | "Event" | "Relation" (если захочешь)
}

#[derive(Debug, Serialize)]
pub struct GraphEdge {
    pub id: String,        // id связи (CONNECTION)
    pub from: String,      // id from-нод
    pub to: String,        // id to-нод
    pub edge_type: String, // relation_type, например "MAIN_ROLE"
}

#[derive(Debug, Serialize)]
pub struct ProjectGraphResponse {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}
#[derive(Debug, Deserialize, Validate)]
pub struct GetProjectGraphQuery {
    /// Максимум нод (safety, чтобы не положить клиент / browser)
    pub max_nodes: Option<u32>,

    /// Фильтр по типу связей
    pub relation_types: Option<Vec<String>>,
}
