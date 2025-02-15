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

    match hasher.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow!("{err}")),
    }
}

#[cfg(test)]
mod unit_tests_hash_service {
    use super::*;
    use anyhow::Result;

    #[test]
    fn hashes_password() -> Result<()> {
        struct TestCase {
            title: String,
            password: String,
        }

        let test_cases = vec![TestCase {
            title: "Correctly hashes password".into(),
            password: "TestPassword".into(),
        }];

        for t in test_cases {
            let hash_result = hash_password(&t.password);

            assert!(hash_result.is_ok(), "{}", t.title);
        }

        Ok(())
    }

    #[test]
    fn verifies_password() -> Result<()> {
        struct TestCase {
            title: String,
            password: String,
            verified_password: String,
        }

        let test_cases = vec![
            TestCase {
                title: "Successfully verifies hashed password with matching password".into(),
                password: "TestPassword".into(),
                verified_password: "TestPassword".into(),
            },
            TestCase {
                title: "Fails to verify hashed password with not matching password".into(),
                password: "TestPassword".into(),
                verified_password: "Testpassword".into(),
            },
        ];

        for t in test_cases {
            let password_hash = hash_password(&t.password)?;

            let verify_result = verify_password_hash(&t.verified_password, &password_hash);

            if t.password == t.verified_password {
                assert!(verify_result.is_ok(), "{}", t.title)
            } else {
                assert!(verify_result.is_err(), "{}", t.title)
            }
        }

        Ok(())
    }
}
