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
            assert_date_is_current, assert_users_match, db_clean_up, get_db_connection,
            get_random_user_db, print_assert_failed,
        },
    };
    use anyhow::Result;
    use tokio::test;

    #[test]
    async fn get_multiple_users() -> Result<()> {
        struct TestCase {
            title: String,
            test_users_db: Vec<UserDb>,
            is_success: bool,
        }

        let test_cases = vec![TestCase {
            title: "Successfully gets all users".into(),
            test_users_db: vec![get_random_user_db(None), get_random_user_db(None)],
            is_success: true,
        }];

        let config = Config::new()?;

        let db_handler = MongoDbHandler::new(
            &config.db_user_name,
            &config.db_user_password,
            &config.db_name,
            &config.db_host,
        )
        .await?;

        for t in test_cases {
            let db = get_db_connection().await?;
            let cloned_db_users = t.test_users_db.clone();

            db.collection::<UserDb>("users")
                .insert_many(t.test_users_db)
                .await?;

            let mut got_users = db_handler.get_multiple::<UserDb, User>("users").await?;
            got_users.sort_by_key(|user| user.email.clone());

            if t.is_success {
                assert_eq!(
                    got_users.len(),
                    2,
                    "{}",
                    print_assert_failed(&t.title, "2", &format!("{:?}", got_users.len()))
                );

                let mut test_users = cloned_db_users
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<User>>();
                test_users.sort_by_key(|user| user.email.clone());

                for (idx, user) in got_users.iter().enumerate() {
                    assert_users_match(&t.title, &user, &test_users[idx]);
                }
            } else {
            }
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
                test_users: vec![get_random_user_db(None), get_random_user_db(None)],
                test_id: None,
                is_success: true,
            },
            TestCase {
                title: "Fails to get a user by id with non-existing id".into(),
                test_users: vec![get_random_user_db(None), get_random_user_db(None)],
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

        for t in test_cases {
            let db = get_db_connection().await?;

            let users_to_insert = t.test_users.clone();
            db.collection::<UserDb>("users")
                .insert_many(users_to_insert)
                .await?;

            let db_user_to_find = match t.test_users.get(0) {
                Some(u) => u,
                None => return Err(anyhow!("Failed to get first user from test users")),
            };

            let user_to_find: User = db_user_to_find.clone().into();
            let id = match t.test_id {
                Some(id) => id,
                None => match db_user_to_find.id {
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
                assert_users_match(&t.title, &got_user, &user_to_find);

                assert_date_is_current(got_user.created_at, &t.title)?;
                assert_date_is_current(got_user.modified_at, &t.title)?;
            } else {
                assert!(get_result.is_err())
            }
        }

        db_clean_up(&db_handler).await?;

        Ok(())
    }

    // #[test]
    // async fn fails_to_get_user_by_id() -> Result<()> {
    //     mock! {
    //         pub Collection<T> {
    //             fn find_one(&self, filter: mongodb::bson::Document) -> Result<Option<T>, MongoError>
    //             where
    //                 T: DeserializeOwned + 'static;
    //         }

    //         trait Clone {
    //             fn clone(&self) -> Self;
    //         }
    //     }

    //     Ok(())
    // }
}
