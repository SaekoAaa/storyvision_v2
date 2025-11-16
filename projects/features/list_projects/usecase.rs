use sqlx::MySqlPool;

use crate::{features::list_projects::error::ListProjectError, model::Project};

pub async fn list_projects_usecase(
    owner_id: u64,
    pool: &MySqlPool,
) -> Result<Vec<Project>, ListProjectError> {
    let project_list = sqlx::query_as::<_, Project>(
        "SELECT a.id as id, a.owner_id as owner_id, b.email as owner_name, a.name as name, a.valid_name as valid_name,
        a.description as description FROM projects a left join users b on a.owner_id = b.id WHERE owner_id = ?",
    )
    .bind(owner_id)
    .fetch_all(pool)
    .await?;
    Ok(project_list)
}
