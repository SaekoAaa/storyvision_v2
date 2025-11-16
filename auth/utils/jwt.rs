use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(Debug, Deserialize, Serialize)]
pub struct JWTClaims {
    pub sub: u64,
    pub iat: i64,
    pub exp: i64,
    pub role: String,
    pub projects: Vec<u64>,
}

pub fn create_jwt_token(
    project_list: Vec<u64>,
    user_id: u64,
    expiry_offset: Duration,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let current_time = OffsetDateTime::now_utc();
    let expires_at = current_time.saturating_add(expiry_offset);
    let user_claims = JWTClaims {
        sub: user_id,
        iat: current_time.unix_timestamp(),
        exp: expires_at.unix_timestamp(),
        role: String::from("user"),
        projects: project_list,
    };
    encode(
        &Header::default(),
        &user_claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}
pub fn validate_jwt_token(
    token: &str,
    secret: &str,
) -> jsonwebtoken::errors::Result<TokenData<JWTClaims>> {
    decode::<JWTClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
}
#[cfg(test)]
pub mod test {
    use time::Duration;

    use crate::utils::jwt::{create_jwt_token, validate_jwt_token};

    #[test]
    fn test_validate_jwt() -> anyhow::Result<()> {
        let secret = "123";
        let jwt = create_jwt_token(vec![], 5, Duration::minutes(3), secret)?;
        assert!(validate_jwt_token(&jwt, secret).is_ok());
        let jwt2 = create_jwt_token(vec![], 5, Duration::minutes(-3), secret)?;
        assert!(validate_jwt_token(&jwt2, secret).is_err());
        Ok(())
    }
}
