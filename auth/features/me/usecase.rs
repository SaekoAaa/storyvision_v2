use {
    crate::{
        constants,
        features::me::error::GetUserError,
        model::*,
        features::crypto::{hash::hash_password, jwt::create_jwt_token},
    },
    argon2::password_hash::SaltString,
    sha2::{Digest, Sha256},
    sqlx::prelude::FromRow,
    std::net::SocketAddr,
    time::{Duration, OffsetDateTime},
};
#[derive(FromRow)]
pub struct GetUser {
    pub email: String,
    pub role: String,
}

pub async fn get_user_usecase(
    pool: &sqlx::MySqlPool,
    user_id: u64,
) -> Result<GetUser, GetUserError> {
    let user = sqlx::query_as::<_, GetUser>("select email, role from users where id = ?")
        .bind(user_id)
        .fetch_optional(pool)
        .await?;
    match user {
        Some(user) => Ok(user),
        None => Err(GetUserError::NotFound),
    }
}
