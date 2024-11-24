use super::mongo_db_handler::MongoDbHandler;

use crate::model::user::{Role, UserCreate, UserDb};
use anyhow::{anyhow, Result};
use bson::{Bson, DateTime};

pub trait UserHandler {
    async fn create_user(&self, user: UserCreate) -> Result<String>;
}

impl UserHandler for MongoDbHandler {
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
        test_utils::{assert_date_is_current, db_clean_up, get_db_connection, print_assert_failed},
    };

    use super::*;
    use anyhow::Result;
    use bson::{doc, oid::ObjectId};
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
            title: "Successfully creates user".into(),
            test_user: UserCreate {
                email: "test@user.com".into(),
                password_hash: "testpassword".into(),
            },
            expected_insert_success: true,
            expected_role: Role::User,
        }];

        async fn run_test(t: &TestCase) -> Result<()> {
            let config = Config::new()?;

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

            let db = get_db_connection().await?;
            let object_id = ObjectId::parse_str(&inserted_id)?;

            let user_db = db
                .collection::<UserDb>("users")
                .find_one(doc! {"_id": object_id})
                .await?;

            match user_db {
                Some(user) => {
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
                }
                None => return Err(anyhow!("Failed to find user with object id: {object_id}")),
            }

            db_clean_up(&db_handler).await?;

            Ok(())
        }

        for t in test_cases {
            run_test(&t).await?;
        }

        Ok(())
    }
}
