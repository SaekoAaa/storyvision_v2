use sqlx::MySqlPool;

use crate::{features::list_projects::error::ListProjectError, model::Project};

pub async fn list_projects_usecase(
    owner_id: u64,
    pool: &MySqlPool,
) -> Result<Vec<Project>, ListProjectError> {
    let project_list = sqlx::query_as::<_, Project>(
        "SELECT id, owner_id, name, valid_name, description FROM projects WHERE owner_id = ?",
    )
    .bind(owner_id)
    .fetch_all(pool)
    .await?;
    Ok(project_list)
}
