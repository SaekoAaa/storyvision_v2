use {
    crate::{
        constants,
        model::*
        ,
        utils::{hash::hash_password, jwt::create_jwt_token},
    },
    argon2::password_hash::SaltString,
    sha2::{Digest, Sha256},
    std::net::SocketAddr,
    time::{Duration, OffsetDateTime},
};
use crate::features::register_user::error::RegisterError;

pub struct RegisterData {
    pub id: u64,
    pub refresh_token: String,
    pub access_token: String,
}

pub async fn register_user(
    pool: &sqlx::MySqlPool,
    email: &str,
    password: &str,
    saltstring: &SaltString,
    token_secret: &str,
    connect_info: SocketAddr,
) -> Result<RegisterData, RegisterError> {
    let hashed_password = hash_password(password, saltstring)?;
    sqlx::query(
        r#"
        INSERT INTO users (email, password_hash)
        VALUES (?, ?);
    "#,
    )
    .bind(email)
    .bind(hashed_password)
    .execute(pool)
    .await?;
    let UserId { id } = sqlx::query_as::<_, UserId>(
        r#"
                SELECT id FROM users WHERE email = ? LIMIT 1;
            "#,
    )
    .bind(email)
    .fetch_one(pool)
    .await?;
    let refresh_token = uuid::Uuid::new_v4();
    sqlx::query(
        r#"
            INSERT INTO sessions (user_id, refresh_token_hash, expires_at, ip_address, device_info)
            VALUES (?, ?, ?, ?, ?);
            "#,
    )
    .bind(id)
    .bind(&format!("{:X}", Sha256::digest(refresh_token)))
    .bind(OffsetDateTime::now_utc().saturating_add(Duration::days(constants::REFRESH_EXPIRY_DAYS)))
    .bind(Some(&connect_info.to_string()))
    .bind(None::<String>)
    .execute(pool)
    .await
    .map(|_| ())?;
    let access_token = create_jwt_token(id, Duration::minutes(15), token_secret)?;
    Ok(RegisterData {
        id,
        refresh_token: refresh_token.to_string(),
        access_token,
    })
}
