use argon2::Argon2;
use argon2::PasswordHash;
use argon2::PasswordHasher;
use argon2::PasswordVerifier;
use argon2::password_hash::SaltString;
pub fn hash_password(
    password: &str,
    salt: &SaltString,
) -> Result<String, argon2::password_hash::Error> {
    let hashed_password = Argon2::default().hash_password(password.as_bytes(), salt)?;
    let password_hash_string = hashed_password.serialize().to_string();
    Ok(password_hash_string)
}
pub fn verify_password(password: &str, password_hash: &str) -> argon2::password_hash::Result<()> {
    let parsed_hash = PasswordHash::new(password_hash)?;
    argon2::Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
}

#[cfg(test)]
mod test {
    use crate::features::crypto::hash::{hash_password, verify_password};
    use argon2::password_hash::{SaltString, rand_core::OsRng};

    #[test]
    fn test_password_hash() -> anyhow::Result<()> {
        let salt = SaltString::generate(OsRng);
        let salt2 = SaltString::generate(OsRng);
        let password = "wpd";
        let password_hash = hash_password(password, &salt)?;
        let password_hash_2 = hash_password(password, &salt)?;
        assert_eq!(password_hash, password_hash_2, "Hash deterministic");
        let password_hash_3 = hash_password(password, &salt2)?;
        assert_ne!(password_hash, password_hash_3, "Salt changes hash");
        assert!(
            verify_password(password, &password_hash).is_ok(),
            "Password verfication works"
        );
        Ok(())
    }
}
