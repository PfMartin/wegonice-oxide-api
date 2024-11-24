use super::mongo_db_handler::MongoDbHandler;
use anyhow::{anyhow, Result};
use bson::{doc, oid::ObjectId};
use futures_util::TryStreamExt;
use serde::de::DeserializeOwned;
use std::{convert::Into, marker::Sync};

pub trait GenericHandler {
    async fn get_multiple<T, S>(&self, collection_name: &str) -> Result<Vec<S>>
    where
        T: Sync + Send + DeserializeOwned + Into<S> + Clone;
    async fn get_by_id<T, S>(&self, id: &str, collection_name: &str) -> Result<S>
    where
        T: Sync + Send + DeserializeOwned + Into<S>;
}

impl GenericHandler for MongoDbHandler {
    async fn get_multiple<T, S>(&self, collection_name: &str) -> Result<Vec<S>>
    where
        T: Sync + Send + DeserializeOwned + Into<S> + Clone,
    {
        let cursor = self
            .db
            .collection::<T>(collection_name)
            .find(doc! {})
            .await?;

        let db_documents = cursor.try_collect::<Vec<T>>().await?;
        let documents = db_documents
            .iter()
            .cloned()
            .map(|d| d.into())
            .collect::<Vec<S>>();

        Ok(documents)
    }

    async fn get_by_id<T, S>(&self, id: &str, collection_name: &str) -> Result<S>
    where
        T: Sync + Send + DeserializeOwned + Into<S>,
    {
        let object_id = ObjectId::parse_str(id)?;

        let find_result = self
            .db
            .collection::<T>(collection_name)
            .find_one(doc! {"id": object_id})
            .await?;

        match find_result {
            Some(document) => Ok(document.into()),
            None => Err(anyhow!(
                "Failed to find document in {collection_name} collection with id {id}"
            )),
        }
    }
}

#[cfg(test)]
pub mod unit_tests_generic_handler {
    use super::*;
    use crate::{
        config::Config,
        model::user::{User, UserDb},
        test_utils::{
            assert_date_is_current, db_clean_up, get_db_connection, get_random_user_db,
            print_assert_failed,
        },
    };
    use anyhow::Result;
    use tokio::test;

    #[test]
    async fn get_multiple_users() -> Result<()> {
        struct TestCase {
            title: String,
            test_users: Vec<UserDb>,
        }

        let test_cases = vec![TestCase {
            title: "Successfully gets all users".into(),
            test_users: vec![get_random_user_db(), get_random_user_db()],
        }];

        let config = Config::new()?;

        let db_handler = MongoDbHandler::new(
            &config.db_user_name,
            &config.db_user_password,
            &config.db_name,
            &config.db_host,
        )
        .await?;

        db_clean_up(&db_handler).await?;

        for t in test_cases {
            let db = get_db_connection().await?;
            db.collection::<UserDb>("users")
                .insert_many(t.test_users)
                .await?;

            let got_users = db_handler.get_multiple::<UserDb, User>("users").await?;
            assert_eq!(
                got_users.len(),
                2,
                "{}",
                print_assert_failed(&t.title, "2", &format!("{:?}", got_users.len()))
            );
        }

        db_clean_up(&db_handler).await?;

        Ok(())
    }

    #[test]
    async fn get_user_by_id() -> Result<()> {
        struct TestCase {
            title: String,
            test_users: Vec<UserDb>,
            test_id: Option<String>,
            is_success: bool,
        }

        let test_cases = vec![
            TestCase {
                title: "Successfully get a user by id".into(),
                test_users: vec![get_random_user_db(), get_random_user_db()],
                test_id: None,
                is_success: true,
            },
            TestCase {
                title: "Fails to get a user by id with non-existing id".into(),
                test_users: vec![get_random_user_db(), get_random_user_db()],
                test_id: Some("non-existent".into()),
                is_success: false,
            },
        ];

        let config = Config::new()?;

        let db_handler = MongoDbHandler::new(
            &config.db_user_name,
            &config.db_user_password,
            &config.db_name,
            &config.db_host,
        )
        .await?;

        db_clean_up(&db_handler).await?;

        for t in test_cases {
            let db = get_db_connection().await?;

            let users_to_insert = t.test_users.clone();
            db.collection::<UserDb>("users")
                .insert_many(users_to_insert)
                .await?;

            let user_to_find = match t.test_users.get(0) {
                Some(u) => u,
                None => return Err(anyhow!("Failed to get first user from test users")),
            };

            let id = match t.test_id {
                Some(id) => id,
                None => match user_to_find.id {
                    Some(i) => i.to_hex(),
                    None => "wrong".into(),
                },
            };

            let get_result = db_handler.get_by_id::<UserDb, User>(&id, "users").await;

            if t.is_success {
                let got_user = get_result?;

                assert_eq!(
                    got_user.id,
                    id,
                    "{}",
                    print_assert_failed(&t.title, &got_user.id, &id)
                );
                assert_eq!(
                    got_user.email,
                    user_to_find.email,
                    "{}",
                    print_assert_failed(&t.title, &got_user.email, &user_to_find.email)
                );
                assert_eq!(
                    got_user.role,
                    user_to_find.role,
                    "{}",
                    print_assert_failed(
                        &t.title,
                        &format!("{:?}", got_user.role),
                        &format!("{:?}", user_to_find.role)
                    )
                );
                assert_eq!(
                    got_user.is_activated,
                    user_to_find.is_activated,
                    "{}",
                    print_assert_failed(
                        &t.title,
                        &format!("{:?}", got_user.is_activated),
                        &format!("{:?}", user_to_find.is_activated)
                    )
                );
                assert_date_is_current(got_user.created_at, &t.title)?;
                assert_date_is_current(got_user.modified_at, &t.title)?;
            } else {
                assert!(get_result.is_err())
            }
        }

        db_clean_up(&db_handler).await?;

        Ok(())
    }
}
