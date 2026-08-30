use {argon2::password_hash::SaltString, std::sync::Arc};

pub mod api_error;
pub mod api_response;
pub mod openapi;
pub struct AuthState {
    pub pool: Arc<sqlx::MySqlPool>,
    pub token_secret: String,
    pub saltstring: SaltString,
    pub secure_cookies: bool,
}
