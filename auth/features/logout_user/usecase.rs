use crate::features::logout_user::error::LogoutError;
use sha2::{Digest, Sha256};
use sqlx::MySqlPool;

pub async fn logout_user_usecase(id: u64, pool: &MySqlPool) -> Result<(), LogoutError> {
    sqlx::query(
        r#"
            UPDATE sessions
            SET revoked = TRUE
            WHERE user_id = ?;
        "#,
    )
    .bind(id)
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(Into::into)
}
