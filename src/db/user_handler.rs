use super::mongo_db_handler::MongoDbHandler;

use crate::model::user::{Role, User, UserCreate, UserDb};
use anyhow::{anyhow, Result};
use bson::{doc, oid::ObjectId, Bson, DateTime};

pub trait UserHandler {
    async fn get_user_by_id(&self, id: &str) -> Result<User>;
    async fn create_user(&self, user: UserCreate) -> Result<String>;
}

impl UserHandler for MongoDbHandler {
    async fn get_user_by_id(&self, id: &str) -> Result<User> {
        let object_id = ObjectId::parse_str(id)?;

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
            created_at: DateTime::now(),
            modified_at: DateTime::now(),
        };

        let insert_result = self.users_collection.insert_one(&user_db).await?;

        match insert_result.inserted_id {
            Bson::ObjectId(object_id) => Ok(object_id.to_hex()),
            _ => Err(anyhow!(
                "Failed to convert inserted Id to string, {}",
                insert_result.inserted_id.to_string()
            )),
        }
    }
}

#[cfg(test)]
pub mod unit_tests_users_handler {
    use crate::{
        config::Config,
        test_utils::{assert_date_is_current, db_clean_up, print_assert_failed},
    };

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
            let config = Config::new()?;

            println!("{:?}", config);

            let db_handler = MongoDbHandler::new(
                &config.db_user_name,
                &config.db_user_password,
                &config.db_name,
                &config.db_host,
            )
            .await?;

            let insert_result = db_handler.create_user(t.test_user.clone()).await;
            assert_eq!(
                insert_result.is_ok(),
                t.expected_insert_success,
                "{}",
                print_assert_failed(
                    &t.title,
                    &format!("{:?}", t.expected_insert_success),
                    &format!("{:?}", insert_result)
                )
            );
            let inserted_id = insert_result?;

            let user = db_handler.get_user_by_id(&inserted_id).await?;
            assert_eq!(
                user.role,
                t.expected_role,
                "{}",
                print_assert_failed(
                    &t.title,
                    &format!("{:?}", &t.expected_role),
                    &format!("{:?}", &user.role)
                )
            );
            assert_date_is_current(user.created_at, &t.title)?;
            assert_date_is_current(user.modified_at, &t.title)?;

            db_clean_up(&db_handler).await?;

            Ok(())
        }

        for t in test_cases {
            run_test(&t).await?;
        }

        Ok(())
    }
}
