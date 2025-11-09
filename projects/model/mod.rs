use sqlx::prelude::FromRow;

#[derive(sqlx::FromRow, Debug)]
pub struct Project {
    pub id: u64,
    pub name: String,
    pub valid_name: String,
    pub owner_id: u64,
    pub description: String,
}
pub type ProjectId = u64;

#[derive(Debug, sqlx::FromRow)]
pub struct ProjectMember {
    pub member_id: u64,
    pub user_id: u64,
    pub user_email: String,
    pub project_id: u64,
    pub project_name: String,
    pub is_owner: bool,
}
