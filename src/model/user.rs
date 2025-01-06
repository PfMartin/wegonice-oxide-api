use anyhow::{anyhow, Result};
use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::api::services::hash_service::hash_password;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub enum Role {
    Admin,
    User,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserMongoDb {
    pub _id: ObjectId,
    pub email: String,
    pub password_hash: String,
    pub role: Role,
    pub is_activated: bool,
    pub created_at: DateTime,
    pub modified_at: DateTime,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub email: String,
    pub role: Role,
    pub is_activated: bool,
    pub created_at: DateTime,
    pub modified_at: DateTime,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthPayload {
    pub email: String,
    pub password: String,
}

impl TryFrom<AuthPayload> for UserCreate {
    type Error = anyhow::Error;

    fn try_from(auth_payload: AuthPayload) -> Result<Self, Self::Error> {
        match hash_password(&auth_payload.password) {
            Ok(password_hash) => Ok(Self {
                email: auth_payload.email,
                password_hash,
            }),
            Err(err) => Err(anyhow!(err)),
        }
    }
}

#[derive(Deserialize)]
pub struct UserAuthInfo {
    pub password_hash: String,
    pub role: Role,
    pub is_activated: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserCreate {
    pub email: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserPatch {
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub role: Option<Role>,
    pub is_activated: Option<bool>,
}

impl From<UserMongoDb> for User {
    fn from(user_mongo_db: UserMongoDb) -> Self {
        Self {
            id: user_mongo_db._id.to_hex(),
            email: user_mongo_db.email,
            role: user_mongo_db.role,
            is_activated: user_mongo_db.is_activated,
            created_at: user_mongo_db.created_at,
            modified_at: user_mongo_db.modified_at,
        }
    }
}

#[cfg(test)]
mod unit_tests_user_model {
    use crate::test_utils::{get_random_email, get_random_string, get_random_user_db};

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn user_mongo_db_into_user() {
        struct TestCase {
            title: String,
        }

        let test_cases = vec![TestCase {
            title: "Successfully converts an UserMongoDb into an User".into(),
        }];

        for t in test_cases {
            let user_db = get_random_user_db(None);
            let cloned_user_db = user_db.clone();
            let user: User = user_db.into();

            assert_eq!(cloned_user_db._id.to_hex(), user.id, "{}", &t.title,);
            assert_eq!(cloned_user_db.email, user.email, "{}", &t.title);
            assert_eq!(cloned_user_db.role, user.role, "{}", &t.title,);
            assert_eq!(
                cloned_user_db.is_activated, user.is_activated,
                "{}",
                &t.title,
            );
            assert_eq!(cloned_user_db.created_at, user.created_at, "{}", &t.title,);
            assert_eq!(cloned_user_db.modified_at, user.modified_at, "{}", &t.title,);
        }
    }

    #[test]
    fn auth_payload_into_user_create() -> Result<()> {
        struct TestCase {
            title: String,
            auth_payload: AuthPayload,
            is_success: bool,
        }

        let test_cases = vec![TestCase {
            title: "Successfully transforms auth payload into user create".into(),
            auth_payload: AuthPayload {
                email: get_random_email(),
                password: get_random_string(10),
            },
            is_success: true,
        }];

        for t in test_cases {
            let user_create: Result<UserCreate> = t.auth_payload.try_into();

            assert_eq!(user_create.is_ok(), t.is_success, "{}", &t.title);
        }

        Ok(())
    }
}
