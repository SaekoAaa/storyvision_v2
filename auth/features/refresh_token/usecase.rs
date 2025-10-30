use sha2::{Digest, Sha256};
use sqlx::MySqlPool;
use time::Duration;
use crate::features::refresh_token::error::RefreshTokenError;
use crate::model::User;
use crate::utils::jwt::create_jwt_token;

pub async fn refresh_token_usecase(refresh_token: &str, token_secret: &str, pool: &MySqlPool) -> Result<String, RefreshTokenError> {
    let refresh_token_hash = format!("{:x}", Sha256::digest(refresh_token.as_bytes()));
    match sqlx::query_as(r#"SELECT user_id FROM sessions WHERE refresh_token_hash = ? LIMIT 1"#)
        .bind(refresh_token_hash)
        .fetch_optional(pool)
        .await? {
        Some(User { id, ..}) => {
            let new_access_token = create_jwt_token(id, Duration::minutes(15), &token_secret)?;
            Ok(new_access_token)
        },
        None => Err(RefreshTokenError::InvalidRefreshToken),
    }

}