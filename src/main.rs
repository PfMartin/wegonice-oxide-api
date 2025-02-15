mod api;
mod config;
mod db;
mod model;

#[cfg(test)]
mod test_utils;

use anyhow::{Error, Result};
use api::{
    routers::{
        auth_router::AuthRouter, heart_beat_router::HeartBeatRouter, recipes_router::RecipesRouter,
        users_router::UsersRouter,
    },
    server::Server,
};
use config::Config;
use db::mongo_db_handler::MongoDbHandler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().json().init();

    let config = Config::new(Some(".env"))?;

    let db_handler = MongoDbHandler::new(
        &config.db_user_name,
        &config.db_user_password,
        &config.db_name,
        &config.db_host,
    )
    .await?;

    let routers = vec![
        HeartBeatRouter::new().router,
        AuthRouter::new(db_handler.clone(), &config.jwt_secret).router,
        UsersRouter::new(db_handler.clone()).router,
        RecipesRouter::new(db_handler.clone(), &config.jwt_secret).router,
    ];

    let _ = Server::new(&config.server_host, routers).await?;

    Ok(())
}
