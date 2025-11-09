use crate::features::delete_project::error::DeleteProjectError;
pub async fn delete_project_usecase(
    project_id: u64,
    owner_id: u64,
    pool: &sqlx::MySqlPool,
) -> Result<(), DeleteProjectError> {
    let project_owner_id: Option<u64> =
        sqlx::query_scalar("SELECT owner_id FROM projects WHERE id = ?")
            .bind(project_id)
            .fetch_optional(pool)
            .await?;
    if let None = project_owner_id {
        return Err(DeleteProjectError::NotFound {
            user_response: "Permission denied".into(),
            details: "Project owner not found".into(),
        });
    }
    if project_owner_id != Some(owner_id) {
        return Err(DeleteProjectError::NotAProjectOwner);
    }
    sqlx::query("DELETE FROM projects WHERE id = ?")
        .bind(project_id)
        .execute(pool)
        .await?;
    Ok(())
}
