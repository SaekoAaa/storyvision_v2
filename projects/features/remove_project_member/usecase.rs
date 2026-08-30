use sqlx::MySqlPool;

use crate::features::remove_project_member::error::RemoveProjectMemberError;

pub async fn remove_member_from_project_usecase(
    owner_id: u64,
    member_id: u64,
    project_id: u64,
    pool: &MySqlPool,
) -> Result<(), RemoveProjectMemberError> {
    let project_owner_id: Option<u64> =
        sqlx::query_scalar("SELECT owner_id FROM projects WHERE id = ?")
            .bind(project_id)
            .fetch_optional(pool)
            .await?;
    if project_owner_id.is_none() {
        return Err(RemoveProjectMemberError::NotFound {
            user_response: "Permission denied".into(),
            details: "Project owner not found".into(),
        });
    }
    if project_owner_id != Some(owner_id) {
        return Err(RemoveProjectMemberError::NotAProjectOwner);
    }
    sqlx::query("DELETE FROM project_members WHERE user_id = ? AND project_id = ?")
        .bind(member_id)
        .bind(project_id)
        .execute(pool)
        .await?;
    Ok(())
}
