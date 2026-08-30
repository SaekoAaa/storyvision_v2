use crate::features::update_project_metadata::error::UpdateProjectMetadataError;

pub async fn update_project_metadata_usecase(
    project_id: u64,
    new_name: &str,
    new_description: &str,
    owner_id: u64,
    pool: &sqlx::MySqlPool,
) -> Result<(), UpdateProjectMetadataError> {
    let project_owner_id: Option<u64> =
        sqlx::query_scalar("SELECT owner_id FROM projects WHERE id = ?")
            .bind(project_id)
            .fetch_optional(pool)
            .await?;
    if project_owner_id.is_none() {
        return Err(UpdateProjectMetadataError::NotFound {
            user_response: "Permission denied".into(),
            details: "Project owner not found".into(),
        });
    }
    if project_owner_id != Some(owner_id) {
        return Err(UpdateProjectMetadataError::NotAProjectOwner);
    }
    sqlx::query("UPDATE projects SET name = ?, description = ? WHERE id = ? AND owner_id = ?")
        .bind(new_name)
        .bind(new_description)
        .bind(project_id)
        .bind(owner_id)
        .execute(pool)
        .await?;
    Ok(())
}
