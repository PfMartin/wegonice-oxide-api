mod api;
mod config;
mod db;
mod model;

#[cfg(test)]
mod test_utils;

use anyhow::{Error, Result};
use api::{heart_beat_router::HeartBeatRouter, server::Server, users_router::UsersRouter};
use config::Config;
use db::{
    generic_handler::GenericHandler, mongo_db_handler::MongoDbHandler, user_handler::UserHandler,
};
use model::user::{User, UserCreate, UserMongoDb, UserPatch};

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

    let routers = vec![HeartBeatRouter::new()?.router, UsersRouter::new()?.router];

    let _ = Server::new(&config.server_host, routers).await?;

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
