use super::mongo_db_handler::MongoDbHandler;

use crate::model::user::{Role, UserCreate, UserDb, UserPatch};
use anyhow::{anyhow, Result};
use bson::{doc, oid::ObjectId, to_bson, Bson, DateTime};

pub trait UserHandler {
    async fn create_user(&self, user: UserCreate) -> Result<String>;
    async fn patch_user_by_id(&self, id: &str, user_patch: UserPatch) -> Result<()>;
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

    async fn patch_user_by_id(&self, id: &str, user_patch: UserPatch) -> Result<()> {
        let mut update_doc = doc! {};

        if let Some(email) = user_patch.email {
            update_doc.insert("email", email);
        }

        if let Some(password_hash) = user_patch.password_hash {
            update_doc.insert("password_hash", password_hash);
        }
        if let Some(role) = user_patch.role {
            // Serialize the enum to BSON
            let role_bson: Bson = to_bson(&role).map_err(|e| mongodb::error::Error::from(e))?;
            update_doc.insert("role", role_bson);
        }

        if let Some(is_activated) = user_patch.is_activated {
            update_doc.insert("is_activated", is_activated);
        }

        let object_id = ObjectId::parse_str(id)?;

        if update_doc.is_empty() {
            return Ok(());
        }

        let filter = doc! {"_id": object_id};
        let update = doc! { "$set": update_doc };

        self.users_collection.update_one(filter, update).await?;

        Ok(())
    }
}

#[cfg(test)]
pub mod unit_tests_users_handler {
    use crate::{
        config::Config,
        test_utils::{
            assert_date_is_current, db_clean_up, get_db_connection, get_random_user_db,
            print_assert_failed,
        },
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

    #[test]
    async fn patch_user_by_id() -> Result<()> {
        struct TestCase {
            title: String,
            user_patch: UserPatch,
            is_success: bool,
        }

        let test_cases = vec![TestCase {
            title: "Successfully patches existing user".into(),
            user_patch: UserPatch {
                email: Some("patched@user.com".into()),
                password_hash: Some("patchedPassword".into()),
                role: Some(Role::Admin),
                is_activated: Some(false),
            },
            is_success: true,
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

            let insert_result = db_handler
                .create_user(get_random_user_db(None).into())
                .await;
            assert_eq!(
                insert_result.is_ok(),
                true,
                "{}",
                print_assert_failed(
                    &t.title,
                    &format!("{:?}", true),
                    &format!("{:?}", insert_result)
                )
            );
            let inserted_id = insert_result?;

            let patch_result = db_handler
                .patch_user_by_id(&inserted_id, t.user_patch.clone())
                .await;

            if t.is_success {
                assert!(patch_result.is_ok())
            }

            let db = get_db_connection().await?;
            let object_id = ObjectId::parse_str(&inserted_id)?;

            let user_db = db
                .collection::<UserDb>("users")
                .find_one(doc! {"_id": object_id})
                .await?;

            match user_db {
                Some(user) => {
                    let email = Some(user.email);
                    assert_eq!(
                        email,
                        t.user_patch.email,
                        "{}",
                        print_assert_failed(
                            &t.title,
                            &format!("{:?}", &t.user_patch.email),
                            &format!("{:?}", &email)
                        )
                    );
                    let password_hash = Some(user.password_hash);
                    assert_eq!(
                        password_hash,
                        t.user_patch.password_hash,
                        "{}",
                        print_assert_failed(
                            &t.title,
                            &format!("{:?}", &t.user_patch.password_hash),
                            &format!("{:?}", &password_hash)
                        )
                    );
                    assert_eq!(
                        Some(user.role),
                        t.user_patch.role,
                        "{}",
                        print_assert_failed(
                            &t.title,
                            &format!("{:?}", &t.user_patch.role),
                            &format!("{:?}", &user.role)
                        )
                    );
                    assert_eq!(
                        Some(user.is_activated),
                        t.user_patch.is_activated,
                        "{}",
                        print_assert_failed(
                            &t.title,
                            &format!("{:?}", &t.user_patch.is_activated),
                            &format!("{:?}", &user.is_activated)
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
