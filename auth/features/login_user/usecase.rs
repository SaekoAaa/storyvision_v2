use crate::constants::{ACCESS_EXPIRY_SECONDS, REFRESH_EXPIRY_DAYS};
use crate::features::login_user::error::LoginError;
use crate::model::{ProjectId, UserId};
use crate::features::crypto::hash::hash_password;
use crate::features::crypto::jwt::create_jwt_token;
use argon2::password_hash::SaltString;
use sha2::{Digest, Sha256};
use sqlx::MySqlPool;
use sqlx::prelude::FromRow;
use std::net::SocketAddr;
use time::{Duration, OffsetDateTime};

pub struct LoginData {
    pub id: u64,
    pub refresh_token: String,
    pub access_token: String,
}

pub async fn login_user_usecase(
    email: &str,
    password: &str,
    salt_string: &SaltString,
    token_secret: &str,
    connect_info: SocketAddr,
    pool: &MySqlPool,
) -> Result<LoginData, LoginError> {
    let password_hash = hash_password(&password, salt_string)?;
    let user_addr = connect_info.to_string();
    match sqlx::query_as(
        r#"
                SELECT id FROM users
                WHERE email = ? AND password_hash = ?
            "#,
    )
    .bind(&email)
    .bind(password_hash)
    .fetch_optional(pool)
    .await?
    {
        None => {
            return Err(LoginError::NotFound {
                user_response: "User not found".to_string(),
                details: format!("Failed to find email: {}", &email),
            });
        }
        Some(UserId { id }) => {
            let refresh_token = uuid::Uuid::new_v4().to_string();
            let refresh_token_hash = &format!("{:X}", Sha256::digest(refresh_token.as_bytes()));
            let expires_at =
                OffsetDateTime::now_utc().saturating_add(Duration::days(REFRESH_EXPIRY_DAYS));

            tracing::debug!("token: {}", refresh_token);
            tracing::debug!("Create: {}", refresh_token_hash);
            sqlx::query(
                r#"
            INSERT INTO sessions (user_id, refresh_token_hash, expires_at, ip_address, device_info)
            VALUES (?, ?, ?, ?, ?);
            "#,
            )
            .bind(id)
            .bind(refresh_token_hash)
            .bind(expires_at)
            .bind(Some(user_addr))
            .bind(None::<String>)
            .execute(pool)
            .await?;
            let project_list: Vec<u64> =
                sqlx::query_as("select project_id as id from project_members where user_id = ?")
                    .bind(id)
                    .fetch_all(pool)
                    .await?
                    .into_iter()
                    .map(|e: ProjectId| e.id)
                    .collect();
            let access_token = create_jwt_token(
                project_list,
                id,
                Duration::seconds(ACCESS_EXPIRY_SECONDS),
                &token_secret,
            )?;
            Ok(LoginData {
                id,
                refresh_token: refresh_token.to_string(),
                access_token,
            })
        }
    }
}
