use crate::model::user::{Role, User, UserMongoDb};
use anyhow::{anyhow, Result};
use bson::{doc, oid::ObjectId, DateTime};
use mongodb::{options::ClientOptions, Client, Database};
use rand::{distributions::Alphanumeric, Rng};
use std::env;

#[cfg(test)]
pub fn print_assert_failed(title: &str, expected: &str, got: &str) -> String {
    format!(
        "{} failed: Expected '{:?}', but Got '{:?}'",
        title, expected, got
    )
}

#[cfg(test)]
pub fn assert_date_is_current(date: DateTime, title: &str) -> Result<()> {
    let puffer_ms = 2000;

    let date_ms = date.timestamp_millis();

    let start_ms = DateTime::now().timestamp_millis() - puffer_ms;
    let end_ms = DateTime::now().timestamp_millis() + puffer_ms;

    assert!(
        date_ms >= start_ms && date_ms <= end_ms,
        "{}",
        print_assert_failed(
            title,
            &format!("Between {start_ms} and {end_ms}"),
            &format!("{date_ms}")
        )
    );

    Ok(())
}

#[cfg(test)]
pub fn assert_users_match(title: &str, user_1: &User, user_2: &User) {
    assert_eq!(
        user_1.email,
        user_2.email,
        "{}",
        print_assert_failed(title, &user_1.email, &user_2.email)
    );
    assert_eq!(
        user_1.role,
        user_2.role,
        "{}",
        print_assert_failed(
            title,
            &format!("{:?}", user_1.role),
            &format!("{:?}", user_2.role)
        )
    );
    assert_eq!(
        user_1.is_activated,
        user_2.is_activated,
        "{}",
        print_assert_failed(
            title,
            &format!("{:?}", user_1.is_activated),
            &format!("{:?}", user_2.is_activated)
        )
    );
}

#[cfg(test)]
pub fn get_db_config(config_path: Option<&str>) -> Result<(String, String, String, String)> {
    match config_path {
        Some(path) => {
            if dotenv::from_path(path).is_err() {
                println!("Using config from env variables");
            };
        }
        None => println!("Using config from env variables"),
    }

    let db_name = env::var("MONGO_WEGONICE_DB")?;
    let db_user_name = env::var("MONGO_WEGONICE_USER")?;
    let db_user_password = env::var("MONGO_WEGONICE_PASSWORD")?;
    let db_host = env::var("MONGO_WEGONICE_HOST")?;

    Ok((db_name, db_user_name, db_user_password, db_host))
}

#[cfg(test)]
pub async fn get_db_connection() -> Result<Database> {
    let (db_name, db_user_name, db_user_password, db_host) = get_db_config(Some(".env"))?;

    println!("GET DB CONNECTION: {db_name}, {db_user_name}, {db_user_password}, {db_host}");

    let uri = format!(
        "mongodb://{db_user_name}:{db_user_password}@{db_host}/{db_name}?authSource={db_name}"
    );
    let client_options = ClientOptions::parse(uri).await?;

    let client = Client::with_options(client_options)?;

    Ok(client.database(&db_name))
}

#[cfg(test)]
pub async fn db_clean_up() -> Result<()> {
    let database = get_db_connection().await?;

    match database
        .collection::<UserMongoDb>("users")
        .delete_many(doc! {})
        .await
    {
        Ok(_) => Ok(()),
        Err(error) => Err(anyhow!(
            "Failed to delete users in clean up step: {}",
            error
        )),
    }
}

#[cfg(test)]
pub fn get_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

#[cfg(test)]
pub fn get_random_email() -> String {
    format!("{}@{}.com", get_random_string(10), get_random_string(5))
}

#[cfg(test)]
pub fn get_random_user_db(id: Option<ObjectId>) -> UserMongoDb {
    let user_id = match id {
        Some(object_id) => object_id,
        None => ObjectId::new(),
    };

    UserMongoDb {
        _id: user_id,
        email: get_random_email(),
        password_hash: get_random_string(10),
        role: Role::User,
        is_activated: true,
        created_at: DateTime::now(),
        modified_at: DateTime::now(),
    }
}
