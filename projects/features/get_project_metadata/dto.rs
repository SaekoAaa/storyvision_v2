use serde::Serialize;

use crate::model::Project;

#[derive(Serialize)]
pub struct ProjectResponse {
    pub id: u64,
    pub name: String,
    pub valid_name: String,
    pub owner_id: u64,
    pub owner_name: String,
    pub description: String,
}
impl From<Project> for ProjectResponse {
    fn from(value: Project) -> Self {
        Self {
            id: value.id,
            name: value.name,
            valid_name: value.valid_name,
            owner_id: value.owner_id,
            owner_name: value.owner_name,
            description: value.description,
        }
    }
}
