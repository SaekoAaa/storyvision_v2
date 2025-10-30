use sqlx::MySqlPool;

use crate::features::create_project::error::CreateProjectError;

pub struct ProjectData<'a> {
    pub id: u64,
    pub name: &'a str,
    pub valid_name: String,
    pub description: String,
    pub owner_id: u64,
}

pub async fn create_project_usecase<'a>(
    project_name: &'a str,
    owner_id: u64,
    description: &str,
    pool: &MySqlPool,
) -> Result<ProjectData<'a>, CreateProjectError> {
    let valid_name = project_name.replace(' ', "_").to_lowercase();

    let res = sqlx::query(
        "INSERT INTO projects (owner_id, name, valid_name, description)
                    VALUES (?, ?, ?, ?)",
    )
    .bind(owner_id)
    .bind(project_name)
    .bind(&valid_name)
    .bind(description)
    .execute(pool)
    .await?;

    let id = res.last_insert_id();
    Ok(ProjectData {
        id,
        name: project_name,
        description: description.to_string(),
        valid_name,
        owner_id,
    })
}
