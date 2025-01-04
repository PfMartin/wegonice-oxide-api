use anyhow::{anyhow, Result};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let hasher = Argon2::default();
    match hasher.hash_password(password.as_bytes(), &salt) {
        Ok(password_hash) => Ok(password_hash.to_string()),
        Err(err) => Err(anyhow!("{err}")),
    }
}

pub fn verify_password_hash(password: &str, password_hash: &str) -> Result<()> {
    let hasher = Argon2::default();

    let parsed_hash = match PasswordHash::new(password_hash) {
        Ok(hash) => hash,
        Err(err) => return Err(anyhow!("{err}")),
    };

    match hasher.verify_password(&password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow!("{err}")),
    }
}
