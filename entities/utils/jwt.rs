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
