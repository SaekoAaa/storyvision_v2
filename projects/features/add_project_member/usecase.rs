use sqlx::MySqlPool;

use crate::features::add_project_member::error::AddProjectMemberError;

pub async fn add_member_to_project_usecase(
    owner_id: u64,
    project_id: u64,
    member_id: u64,
    pool: &MySqlPool,
) -> Result<(), AddProjectMemberError> {
    let project_owner_id: Option<u64> =
        sqlx::query_scalar("SELECT owner_id FROM projects WHERE id = ?")
            .bind(project_id)
            .fetch_optional(pool)
            .await?;
    if let None = project_owner_id {
        return Err(AddProjectMemberError::NotFound {
            user_response: "Permission denied".into(),
            details: "Project owner not found".into(),
        });
    }
    if project_owner_id != Some(owner_id) {
        return Err(AddProjectMemberError::NotAProjectOwner);
    }
    let is_in_project: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM project_members WHERE project_id = ? AND user_id = ?",
    )
    .bind(project_id)
    .bind(member_id)
    .fetch_one(pool)
    .await?;

    if is_in_project > 0 {
        return Err(AddProjectMemberError::PermissionDenied(
            "User is in project already".to_string(),
        ));
    }
    sqlx::query(
        "INSERT INTO project_members (project_id, user_id)
                    VALUES (?, ?)",
    )
    .bind(project_id)
    .bind(member_id)
    .execute(pool)
    .await?;
    Ok(())
}
