use crate::features::list_characters::{dto::Character, error::Error};

pub async fn list_characters_usecase(
    pool: &sqlx::MySqlPool,
    project_id: u64,
    page: usize,
    per_page: usize,
    user_id: u64,
) -> Result<Vec<Character>, Error> {
    todo!("Implement list_characters_usecase");
    Ok(characters)
}
