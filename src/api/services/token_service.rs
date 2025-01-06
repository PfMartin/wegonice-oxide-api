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
