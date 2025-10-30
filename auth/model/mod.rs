
use sqlx::prelude::FromRow;
use time::OffsetDateTime;

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: u64,
    pub email: String,
    pub password_hash: Vec<u8>,
    pub created_at: OffsetDateTime,
    pub role: String,
}

#[derive(FromRow, Debug)]
pub struct UserId {
    pub id: u64,
}
#[derive(sqlx::FromRow, Debug)]
pub struct Session {
    pub id: u64,
    pub user_id: u64,
    pub expires_at: OffsetDateTime,
    pub revoked: bool,
    pub device_info: Option<String>,
    pub ip_address: Option<String>,
}
