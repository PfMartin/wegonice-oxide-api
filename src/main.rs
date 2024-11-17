mod db;
mod model;

use anyhow::{Error, Result};
use db::{db_handler::DbHandler, mongo_db_handler::MongoDbHandler};
use model::user::UserCreate;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let db_handler = MongoDbHandler::new("test", "hello", "he", "127.0.0.1:27017").await?;

    db_handler.get_user_by_id("testId").await?;
    db_handler
        .create_user(UserCreate {
            email: String::from("Test"),
            password_hash: String::from("hello"),
        })
        .await?;

    Ok(())
}
