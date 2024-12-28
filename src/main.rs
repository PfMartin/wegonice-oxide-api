mod config;
mod db;
mod model;
mod server;

#[cfg(test)]
mod test_utils;

use anyhow::{Error, Result};
use config::Config;
use db::{
    generic_handler::GenericHandler, mongo_db_handler::MongoDbHandler, user_handler::UserHandler,
};
use model::user::{User, UserCreate, UserMongoDb, UserPatch};
use server::{heart_beat_router::HeartBeatRouter, server::Server};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::new(Some(".env"))?;

    let db_handler = MongoDbHandler::new(
        &config.db_name,
        &config.db_user_name,
        &config.db_user_password,
        &config.db_host,
    )
    .await?;

    let heart_beat_router = HeartBeatRouter::new()?;

    let _ = Server::new(&config.server_host, heart_beat_router.router).await?;

    db_handler
        .create_user(UserCreate {
            email: String::from("Test"),
            password_hash: String::from("hello"),
        })
        .await?;

    db_handler
        .get_multiple::<UserMongoDb, User>("users")
        .await?;

    db_handler
        .get_by_id::<UserMongoDb, User>("1", "users")
        .await?;

    db_handler.get_user_by_email("test").await?;
    db_handler
        .patch_user_by_id(
            "id",
            UserPatch {
                email: None,
                password_hash: None,
                role: None,
                is_activated: None,
            },
        )
        .await?;
    db_handler.delete_user_by_id("test").await?;

    Ok(())
}
