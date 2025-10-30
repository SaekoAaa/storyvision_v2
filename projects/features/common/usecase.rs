pub async fn get_project_owner_id(project_id: u64, pool: &sqlx::MySqlPool) -> Option<u64> {
    let owner_id: Option<u64> = sqlx::query_scalar("SELECT owner_id FROM projects WHERE id = ?")
        .bind(project_id)
        .fetch_optional(pool)
        .await?;
    owner_id
}
