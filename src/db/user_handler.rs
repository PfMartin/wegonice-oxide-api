use super::mongo_db_handler::MongoDbHandler;

use crate::model::user::{Role, User, UserAuthInfo, UserCreate, UserMongoDb, UserPatch};
use anyhow::{anyhow, Result};
use bson::{doc, oid::ObjectId, to_bson, Bson, DateTime};
use futures_util::StreamExt;

pub trait UserHandler {
    async fn create_user(&self, user: UserCreate) -> Result<String>;
    async fn get_user_by_email(&self, email: &str) -> Result<User>;
    async fn patch_user_by_id(&self, id: &str, user_patch: UserPatch) -> Result<()>;
    async fn delete_user_by_id(&self, id: &str) -> Result<u64>;
    async fn get_user_auth_info(&self, email: &str) -> Result<UserAuthInfo>;
}

impl UserHandler for MongoDbHandler {
    async fn create_user(&self, user: UserCreate) -> Result<String> {
        let user_db = UserMongoDb {
            _id: ObjectId::new(),
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

    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let filter = doc! {"email": email};

        let find_result = self.users_collection.find_one(filter).await?;

        match find_result {
            Some(user) => Ok(user.into()),
            None => Err(anyhow!("Failed to find user with email {}", email)),
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
            let role_bson: Bson = to_bson(&role)?;
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

    async fn delete_user_by_id(&self, id: &str) -> Result<u64> {
        let object_id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": object_id};

        let delete_result = self.users_collection.delete_one(filter).await?;

        Ok(delete_result.deleted_count)
    }

    async fn get_user_auth_info(&self, email: &str) -> Result<UserAuthInfo> {
        let stage_match_email = doc! {
            "$match": {
                "email": email,
            }
        };

        let stage_project = doc! {
            "$project": { "email": 1, "password_hash": 1, "role": 1, "is_activated": 1}
        };

        let pipeline = vec![stage_match_email, stage_project];

        let mut results = self.users_collection.aggregate(pipeline).await?;

        match results.next().await {
            Some(document) => match document {
                Ok(document) => {
                    let auth_info: UserAuthInfo = bson::from_document(document)?;
                    Ok(auth_info)
                }
                Err(err) => Err(anyhow!("Failed to get user auth info: {err}")),
            },
            None => Err(anyhow!("Failed to find any matching documents")),
        }
    }
}

#[cfg(test)]
pub mod unit_tests_users_handler {
    use crate::{
        model::user::UserMongoDb,
        test_utils::{
            assert_date_is_current, db_clean_up, get_db_config, get_db_connection,
            get_random_user_db, print_assert_failed,
        },
    };

    use super::*;
    use anyhow::Result;
    use bson::{doc, oid::ObjectId};
    use pretty_assertions::assert_eq;

    #[tokio::test]
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
            let (db_name, db_user_name, db_user_password, db_host) = get_db_config(Some(".env"))?;
            let db_handler =
                MongoDbHandler::new(&db_user_name, &db_user_password, &db_name, &db_host).await?;

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

            let object_id = ObjectId::parse_str(&inserted_id)?;

            let db = get_db_connection().await?;
            let user_db = db
                .collection::<UserMongoDb>("users")
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

            Ok(())
        }

        for t in test_cases {
            run_test(&t).await?;
            db_clean_up().await?;
        }

        Ok(())
    }

    #[tokio::test]
    async fn get_user_by_email() -> Result<()> {
        struct TestCase {
            title: String,
            test_users: Vec<UserMongoDb>,
            test_email: Option<String>,
            is_success: bool,
        }

        let test_cases = vec![
            TestCase {
                title: "Successfully finds user by email".into(),
                test_users: vec![get_random_user_db(None), get_random_user_db(None)],
                test_email: None,
                is_success: true,
            },
            TestCase {
                title: "Fails to find user by email with non-existing email".into(),
                test_users: vec![get_random_user_db(None), get_random_user_db(None)],
                test_email: Some("non-existing".into()),
                is_success: false,
            },
        ];

        let (db_name, db_user_name, db_user_password, db_host) = get_db_config(Some(".env"))?;
        let db_handler =
            MongoDbHandler::new(&db_user_name, &db_user_password, &db_name, &db_host).await?;

        for t in test_cases {
            let db = get_db_connection().await?;
            let users_to_insert = t.test_users.clone();
            db.collection::<UserMongoDb>("users")
                .insert_many(users_to_insert)
                .await?;

            let db_user_to_find = match t.test_users.get(0) {
                Some(u) => u,
                None => return Err(anyhow!("Failed to get first user from test users")),
            };

            let email = match t.test_email {
                Some(e) => e,
                None => db_user_to_find.clone().email,
            };

            let get_result = db_handler.get_user_by_email(&email).await;

            if t.is_success {
                let got_user = get_result?;

                assert_eq!(got_user.email, db_user_to_find.email, "{}", &t.title);
                assert_eq!(got_user.role, db_user_to_find.role, "{}", &t.title);
                assert_eq!(
                    got_user.is_activated, db_user_to_find.is_activated,
                    "{}",
                    &t.title
                );

                assert_date_is_current(got_user.created_at, &t.title)?;
                assert_date_is_current(got_user.modified_at, &t.title)?;
            } else {
                assert!(get_result.is_err());
            }

            db_clean_up().await?;
        }

        Ok(())
    }

    #[tokio::test]
    async fn patch_user_by_id() -> Result<()> {
        struct TestCase {
            title: String,
            user_patch: UserPatch,
            is_success: bool,
        }

        let test_cases = vec![
            TestCase {
                title: "Successfully patches existing user".into(),
                user_patch: UserPatch {
                    email: Some("patched@user.com".into()),
                    password_hash: Some("patchedPassword".into()),
                    role: Some(Role::Admin),
                    is_activated: Some(false),
                },
                is_success: true,
            },
            TestCase {
                title: "Fails to patch existing user due to invalid objectId".into(),
                user_patch: UserPatch {
                    email: Some("patched@user.com".into()),
                    password_hash: Some("patchedPassword".into()),
                    role: Some(Role::Admin),
                    is_activated: Some(false),
                },
                is_success: false,
            },
        ];

        async fn run_test(t: &TestCase) -> Result<()> {
            let (db_name, db_user_name, db_user_password, db_host) = get_db_config(Some(".env"))?;
            let db_handler =
                MongoDbHandler::new(&db_user_name, &db_user_password, &db_name, &db_host).await?;

            let users_collection = get_db_connection()
                .await?
                .collection::<UserMongoDb>("users");

            let insert_result = users_collection
                .insert_one(get_random_user_db(None))
                .await?;

            let user_id = if t.is_success {
                match insert_result.inserted_id {
                    Bson::ObjectId(object_id) => object_id.to_hex(),
                    _ => String::from("invalidId"),
                }
            } else {
                String::from("invalidId")
            };

            let patch_result = db_handler
                .patch_user_by_id(&user_id, t.user_patch.clone())
                .await;

            if !t.is_success {
                assert!(patch_result.is_err());
                return Ok(());
            }

            let object_id = ObjectId::parse_str(&user_id)?;

            let user_db = users_collection.find_one(doc! {"_id": object_id}).await?;

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

            Ok(())
        }

        for t in test_cases {
            run_test(&t).await?;
            db_clean_up().await?;
        }

        Ok(())
    }

    #[tokio::test]
    async fn delete_user() -> Result<()> {
        struct TestCase {
            title: String,
            is_success: bool,
        }

        let test_cases = vec![TestCase {
            title: "Successfully deletes a user".into(),
            is_success: true,
        }];

        async fn run_test(t: &TestCase) -> Result<()> {
            let (db_name, db_user_name, db_user_password, db_host) = get_db_config(Some(".env"))?;
            let db_handler =
                MongoDbHandler::new(&db_user_name, &db_user_password, &db_name, &db_host).await?;

            // TODO: Create helper function for this
            let users_collection = get_db_connection()
                .await?
                .collection::<UserMongoDb>("users");

            let insert_result = users_collection
                .insert_one(get_random_user_db(None))
                .await?;

            let user_id = if t.is_success {
                match insert_result.inserted_id {
                    Bson::ObjectId(object_id) => object_id.to_hex(),
                    _ => String::from("invalidId"),
                }
            } else {
                String::from("invalidId")
            };

            let delete_count = db_handler.delete_user_by_id(&user_id).await?;

            if t.is_success {
                let expected_delete_count = 1;
                assert_eq!(
                    delete_count,
                    expected_delete_count,
                    "{}",
                    print_assert_failed(
                        &t.title,
                        &format!("{:?}", delete_count),
                        &format!("{:?}", expected_delete_count)
                    )
                );
            }

            Ok(())
        }

        for t in test_cases {
            run_test(&t).await?;
            db_clean_up().await?;
        }

        Ok(())
    }

    #[tokio::test]
    async fn get_user_auth_info() -> Result<()> {
        struct TestCase {
            title: String,
            test_email: Option<String>,
            is_success: bool,
        }

        let test_cases = vec![
            TestCase {
                title: "Successfully returns user auth info".into(),
                test_email: None,
                is_success: true,
            },
            TestCase {
                title: "Fails returns user auth info for non-existing email".into(),
                test_email: Some("not_existing@gmail.com".into()),
                is_success: false,
            },
        ];

        for t in test_cases {
            let (db_name, db_user_name, db_user_password, db_host) = get_db_config(Some(".env"))?;

            // TODO: Create helper function for this
            let users_collection = get_db_connection()
                .await?
                .collection::<UserMongoDb>("users");

            let user = get_random_user_db(None);
            let _ = users_collection.insert_one(&user).await?;

            let db_handler =
                MongoDbHandler::new(&db_user_name, &db_user_password, &db_name, &db_host).await?;

            let search_email = match &t.test_email {
                Some(email) => &email,
                None => &user.email,
            };

            let result = db_handler.get_user_auth_info(&search_email).await;

            if !t.is_success {
                assert!(result.is_err(), "{}", &t.title);
            } else {
                let user_auth_info = result?;

                assert_eq!(&user_auth_info.email, search_email, "{}", t.title);
                assert_eq!(
                    &user_auth_info.password_hash, &user.password_hash,
                    "{}",
                    &t.title
                );
                assert_eq!(&user_auth_info.role, &user.role, "{}", &t.title);
                assert_eq!(
                    &user_auth_info.is_activated, &user.is_activated,
                    "{}",
                    &t.title
                );
            }

            db_clean_up().await?;
        }

        Ok(())
    }
}
