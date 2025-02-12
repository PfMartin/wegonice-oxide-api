use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::model::user::{Role, UserAuthInfo};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: Role,
    exp: i64,
}

pub fn generate_jwt(
    auth_info: &UserAuthInfo,
    token_validity_duration_h: i64,
    token_key: &str,
) -> Result<String> {
    let claims = Claims {
        sub: auth_info.email.to_string(),
        role: auth_info.role,
        exp: (Utc::now() + Duration::hours(token_validity_duration_h)).timestamp(),
    };

    let secret = token_key.as_bytes();

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )?;

    Ok(token)
}

#[cfg(test)]
mod unit_tests_token_service {
    use jsonwebtoken::{decode, DecodingKey, Validation};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn generates_jwt() -> Result<()> {
        let secret = "testSecret";
        let user_auth_info = UserAuthInfo {
            email: "test@email.com".into(),
            password_hash: "test".into(),
            role: Role::User,
            is_activated: true,
        };

        let token = generate_jwt(&user_auth_info, 1, secret)?;
        let decoded_token = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )?;

        let decoded_claims = decoded_token.claims;

        assert_eq!(decoded_claims.sub, user_auth_info.email.to_string());
        assert_eq!(decoded_claims.role, user_auth_info.role);

        Ok(())
    }
}
