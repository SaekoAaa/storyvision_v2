use std::sync::Arc;

pub mod api_error;
pub mod api_response;
pub struct EntityState {
    pub pool: sqlx::MySqlPool,
}
#[derive(Clone)]
pub struct UserData {
    pub id: u64,
    pub role: String,
}
