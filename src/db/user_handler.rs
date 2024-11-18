use super::mongo_db_handler::MongoDbHandler;

use crate::model::user::{Role, User, UserCreate, UserDb};
use anyhow::{anyhow, Result};
use bson::{doc, oid::ObjectId};
use std::str::FromStr;

pub trait UserHandler {
    async fn get_user_by_id(&self, id: &str) -> Result<User>;
    async fn create_user(&self, user: UserCreate) -> Result<String>;
}

impl UserHandler for MongoDbHandler {
    async fn get_user_by_id(&self, id: &str) -> Result<User> {
        let object_id = ObjectId::from_str(id)?;

        let user_db = self
            .users_collection
            .find_one(doc! {"_id": object_id})
            .await?;

        let user: User = match user_db {
            Some(u) => u.into(),
            None => return Err(anyhow!(format!("Failed to find user with id: {id}"))),
        };

        Ok(user)
    }

    async fn create_user(&self, user: UserCreate) -> Result<String> {
        let user_db = UserDb {
            id: None,
            email: user.email,
            password_hash: user.password_hash,
            role: Role::User,
            is_activated: false,
            created_at: bson::DateTime::now(),
            modified_at: bson::DateTime::now(),
        };

        let insert_result = self.users_collection.insert_one(&user_db).await?;

        Ok(insert_result.inserted_id.to_string())
    }
}

#[cfg(test)]
pub mod unit_tests_users_handler {
    use crate::test_utils::get_test_config;

    use super::*;
    use anyhow::Result;
    use tokio::test;

    #[test]
    async fn create_user() -> Result<()> {
        struct TestCase {
            title: String,
            test_user: UserCreate,
            expected_insert_success: bool,
            expected_role: Role,
        }

        let test_cases = vec![TestCase {
            title: String::from("Successfully creates user"),
            test_user: UserCreate {
                email: String::from("test@user.com"),
                password_hash: String::from("testpassword"),
            },
            expected_insert_success: true,
            expected_role: Role::User,
        }];

        async fn run_test(t: &TestCase) -> Result<()> {
            let config = get_test_config()?;
            let db_handler = MongoDbHandler::new(
                &config.db_name,
                &config.db_user_name,
                &config.db_user_password,
                &config.db_host,
            )
            .await?;

            Ok(())
        }

        Ok(())
    }
}
