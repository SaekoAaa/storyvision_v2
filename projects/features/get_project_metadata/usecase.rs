use sqlx::MySqlPool;

use crate::{features::get_project_metadata::error::GetProjectError, model::Project};

pub async fn get_project_usecase(
    owner_id: u64,
    project_id: u64,
    pool: &MySqlPool,
) -> Result<Option<Project>, GetProjectError> {
    let project = sqlx::query_as::<_, Project>(
        "SELECT a.id as id, a.owner_id as owner_id, b.email as owner_name, a.name as name, a.valid_name as valid_name, a.description as description
        FROM projects a left join users b on a.owner_id = b.id WHERE a.owner_id = ? AND a.id = ? LIMIT 1",
    )
    .bind(owner_id)
    .bind(project_id)
    .fetch_optional(pool)
    .await?;
    Ok(project)
}
