use sqlx::MySqlPool;

use crate::{features::get_project_metadata::error::GetProjectError, model::Project};

pub async fn get_project_usecase(
    owner_id: u64,
    project_id: u64,
    pool: &MySqlPool,
) -> Result<Option<Project>, GetProjectError> {
    let project = sqlx::query_as::<_, Project>(
        "SELECT id, owner_id, name, valid_name, description FROM projects WHERE owner_id = ? AND project_id = ? LIMIT 1",
    )
    .bind(owner_id)
    .bind(project_id)
    .fetch_optional(pool)
    .await?;
    Ok(project)
}
