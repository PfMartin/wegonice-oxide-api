use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

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
        User {
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
    use crate::test_utils::{get_random_user_db, print_assert_failed};

    use super::*;

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

            assert_eq!(
                cloned_user_db._id.to_hex(),
                user.id,
                "{}",
                print_assert_failed(&t.title, &cloned_user_db._id.to_hex(), &user.id)
            );
            assert_eq!(
                cloned_user_db.email,
                user.email,
                "{}",
                print_assert_failed(&t.title, &cloned_user_db.email, &user.email)
            );
            assert_eq!(
                cloned_user_db.role,
                user.role,
                "{}",
                print_assert_failed(
                    &t.title,
                    &format!("{:?}", cloned_user_db.role),
                    &format!("{:?}", &user.role)
                )
            );
            assert_eq!(
                cloned_user_db.is_activated,
                user.is_activated,
                "{}",
                print_assert_failed(
                    &t.title,
                    &format!("{:?}", cloned_user_db.is_activated),
                    &format!("{:?}", &user.is_activated)
                )
            );
            assert_eq!(
                cloned_user_db.created_at,
                user.created_at,
                "{}",
                print_assert_failed(
                    &t.title,
                    &format!("{}", cloned_user_db.created_at),
                    &format!("{}", user.created_at)
                )
            );
            assert_eq!(
                cloned_user_db.modified_at,
                user.modified_at,
                "{}",
                print_assert_failed(
                    &t.title,
                    &format!("{}", cloned_user_db.modified_at),
                    &format!("{}", user.modified_at)
                )
            );
        }
    }
}
