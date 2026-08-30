use crate::features::crypto::jwt::create_jwt_token;
use crate::model::{ProjectId, UserId};
use crate::{constants::ACCESS_EXPIRY_SECONDS, features::refresh_token::error::RefreshTokenError};
use sha2::{Digest, Sha256};
use sqlx::MySqlPool;
use time::Duration;

pub async fn refresh_token_usecase(
    refresh_token: &str,
    token_secret: &str,
    pool: &MySqlPool,
) -> Result<String, RefreshTokenError> {
    let refresh_token_hash = format!("{:X}", Sha256::digest(refresh_token.as_bytes()));
    match sqlx::query_as(
        r#"SELECT user_id as id FROM sessions WHERE refresh_token_hash = ? LIMIT 1"#,
    )
    .bind(refresh_token_hash)
    .fetch_optional(pool)
    .await?
    {
        Some(UserId { id }) => {
            let project_list: Vec<u64> =
                sqlx::query_as("select project_id as id from project_members where user_id = ?")
                    .bind(id)
                    .fetch_all(pool)
                    .await?
                    .into_iter()
                    .map(|e: ProjectId| e.id)
                    .collect();
            let new_access_token = create_jwt_token(
                project_list,
                id,
                Duration::seconds(ACCESS_EXPIRY_SECONDS),
                token_secret,
            )?;
            Ok(new_access_token)
        }
        None => Err(RefreshTokenError::InvalidRefreshToken),
    }
}
