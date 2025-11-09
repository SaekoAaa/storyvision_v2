use sqlx::MySqlPool;

use crate::{features::list_project_members::error::ListProjectMembersError, model::ProjectMember};

pub async fn list_project_members_usecase(
    project_id: u64,
    member_id: u64,
    pool: &MySqlPool,
) -> Result<Vec<ProjectMember>, ListProjectMembersError> {
    tracing::debug!(project_id, member_id);
    let in_project: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM project_members WHERE user_id = ? and project_id = ?",
    )
    .bind(member_id)
    .bind(project_id)
    .fetch_one(pool)
    .await?;
    if in_project == 0 {
        return Err(ListProjectMembersError::NotInProject);
    }
    let project_members = sqlx::query_as::<_, ProjectMember>(
        r#"
            SELECT
                pm.id AS member_id,
                u.id AS user_id,
                u.email AS user_email,
                p.id AS project_id,
                p.name AS project_name,
                (p.owner_id = u.id) AS is_owner
            FROM project_members pm
            INNER JOIN users u ON pm.user_id = u.id
            INNER JOIN projects p ON pm.project_id = p.id WHERE pm.project_id = ?;"#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;
    Ok(project_members)
}
