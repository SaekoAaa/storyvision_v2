use sha2::{Digest, Sha256};
use sqlx::MySqlPool;
use crate::features::logout_user::error::LogoutError;

pub async fn logout_user_usecase(refresh_token: &str, pool: &MySqlPool) -> Result<(), LogoutError> {
    let refresh_token_hash = &format!("{:X}", Sha256::digest(refresh_token));
    sqlx::query(
        r#"
            UPDATE sessions
            SET revoked = TRUE
            WHERE refresh_token_hash = ?;
        "#,
    )
        .bind(refresh_token_hash)
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(Into::into)
}