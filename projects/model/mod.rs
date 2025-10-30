#[derive(sqlx::FromRow, Debug)]
pub struct Project {
    pub id: u64,
    pub name: String,
    pub valid_name: String,
    pub owner_id: u64,
    pub description: String,
}
pub type ProjectId = u64;
#[derive(sqlx::FromRow, Debug)]
pub struct ProjectMember {
    pub id: u64,
    pub email: String,
}
