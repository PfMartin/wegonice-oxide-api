use crate::{
    db::mongo_db_handler::MongoDbHandler,
    model::user::{Role, UserDb},
};
use anyhow::{anyhow, Result};
use bson::{doc, oid::ObjectId, DateTime};
use rand::{distributions::Alphanumeric, Rng};

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
pub async fn db_clean_up(db_handler: &MongoDbHandler) -> Result<()> {
    match db_handler.users_collection.delete_many(doc! {}).await {
        Ok(_) => Ok(()),
        Err(error) => Err(anyhow!(
            "Failed to delete users in clean up step: {}",
            error
        )),
    }
}

#[cfg(test)]
fn get_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

#[cfg(test)]
pub fn get_random_user_db() -> UserDb {
    UserDb {
        id: Some(ObjectId::new()),
        email: get_random_string(10),
        password_hash: get_random_string(10),
        role: Role::User,
        is_activated: true,
        created_at: DateTime::now(),
        modified_at: DateTime::now(),
    }
}
