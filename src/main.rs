mod config;
mod db;
mod model;

#[cfg(test)]
mod test_utils;

use anyhow::{Error, Result};
use config::Config;
use db::{mongo_db_handler::MongoDbHandler, user_handler::UserHandler};
use model::user::UserCreate;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::new()?;

    let db_handler = MongoDbHandler::new(
        &config.db_name,
        &config.db_user_name,
        &config.db_user_password,
        &config.db_host,
    )
    .await?;

    db_handler.get_user_by_id("testId").await?;
    db_handler
        .create_user(UserCreate {
            email: String::from("Test"),
            password_hash: String::from("hello"),
        })
        .await?;

    Ok(())
}
