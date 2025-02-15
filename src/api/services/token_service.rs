use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::model::user::{Role, UserAuthInfo};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: Role,
    pub exp: i64,
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

pub fn decode_jwt(token: &str, token_key: &str) -> Result<Claims> {
    let decoded_token = decode::<Claims>(
        token,
        &DecodingKey::from_secret(token_key.as_ref()),
        &Validation::default(),
    )?;

    let decoded_claims = decoded_token.claims;

    Ok(decoded_claims)
}

#[cfg(test)]
mod unit_tests_token_service {
    use pretty_assertions::assert_eq;

    use crate::test_utils::assert_date_in_range;

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

    #[test]
    fn decodes_jwt() -> Result<()> {
        let email = "test@email.com";
        let role = Role::User;
        let token_validity_duration_h = 1;

        let claims = Claims {
            sub: email.into(),
            role,
            exp: (Utc::now() + Duration::hours(token_validity_duration_h)).timestamp(),
        };

        let secret = "testSecret";

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;

        let claims = decode_jwt(&token, secret)?;

        assert_eq!(
            claims.sub,
            String::from(email),
            "Incorrect email from claims"
        );
        assert_eq!(claims.role, role, "Incorrect role from claims");
        assert!(claims.exp > (Utc::now() + Duration::hours(1)).timestamp() - 1000);

        assert_date_in_range(
            claims.exp,
            (Utc::now() + Duration::hours(1)).timestamp(),
            1000,
            "Claims in time range",
        )?;

        Ok(())
    }
}
