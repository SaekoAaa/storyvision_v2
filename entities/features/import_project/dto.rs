use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ImportCharacterDto {
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub attributes: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ImportEventDto {
    pub id: Option<String>,
    pub name: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub timestamp: Option<String>,
    pub attributes: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ImportRelationDto {
    pub id: Option<String>,
    pub name: String,
    pub relation_type: String,
    pub description: Option<String>,
    pub attributes: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ImportConnectionDto {
    pub id: Option<String>,
    pub from_id: String,
    pub to_id: String,
    pub relation_id: String,
    pub attributes: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ImportProjectGraphRequest {
    pub project_id: u64,
    pub characters: Vec<ImportCharacterDto>,
    pub events: Vec<ImportEventDto>,
    pub relations: Vec<ImportRelationDto>,
    pub connections: Vec<ImportConnectionDto>,
}
