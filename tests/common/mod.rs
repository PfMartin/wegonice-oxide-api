use anyhow::Result;
use bson::{doc, oid::ObjectId, DateTime};
use dotenv;
use mongodb::{options::ClientOptions, Client, Collection};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
pub struct ResponseBody {
    pub data: Option<String>,
    pub error: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthPayload {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
enum Role {
    User,
    Admin,
}

#[derive(Serialize, Deserialize)]
struct UserMongoDb {
    _id: ObjectId,
    email: String,
    password_hash: String,
    role: Role,
    is_activated: bool,
    created_at: DateTime,
    modified_at: DateTime,
}

#[derive(Debug)]
pub struct Config {
    pub db_name: String,
    pub db_user_name: String,
    pub db_user_password: String,
    pub db_host: String,
    pub server_host: String,
    pub jwt_secret: String,
}

pub fn get_config(config_path: Option<&str>) -> Result<Config> {
    match config_path {
        Some(path) => {
            if dotenv::from_path(path).is_err() {
                println!("Using config from env variables")
            }
        }
        None => println!("Using config from env variables"),
    }

    Ok(Config {
        db_name: env::var("MONGO_WEGONICE_DB")?,
        db_user_name: env::var("MONGO_WEGONICE_USER")?,
        db_user_password: env::var("MONGO_WEGONICE_PASSWORD")?,
        db_host: env::var("MONGO_WEGONICE_HOST")?,
        server_host: env::var("SERVER_HOST")?,
        jwt_secret: env::var("JWT_SECRET")?,
    })
}

pub async fn get_users_collection() -> Result<Collection<UserMongoDb>> {
    let config = get_config(Some(".env"))?;

    let uri = format!(
        "mongodb://{}:{}@{}/{db_name}?authSource={db_name}",
        config.db_user_name,
        config.db_user_password,
        config.db_host,
        db_name = config.db_name,
    );

    let client_options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(client_options)?;

    client
        .database(&config.db_name)
        .run_command(doc! { "ping": 1 })
        .await?;

    let db = client.database(&config.db_name);
    let users_collection = db.collection::<UserMongoDb>("users");

    Ok(users_collection)
}

pub async fn register_user(is_activated: bool) -> Result<String> {
    let collection = get_users_collection().await?;

    // Password: test_password
    let user = UserMongoDb {
        _id: ObjectId::new(),
        email: "loginUser@email.com".into(),
        password_hash: "$argon2id$v=19$m=19456,t=2,p=1$8/I4UD0qo5joX/Vd5R9TfA$ryIDC5fe3axKWvC4qLsznYsEUjgqu+xvgV3aX/mCJOI".into(),
        role: Role::User,
        is_activated,
        created_at: DateTime::now(),
        modified_at: DateTime::now()
    };

    let insert_result = collection.insert_one(&user).await?;

    Ok(insert_result.inserted_id.to_string())
}

pub async fn delete_users() -> Result<u64> {
    let collection = get_users_collection().await?;

    let filter = doc! {};

    let delete_result = collection.delete_many(filter).await?;

    Ok(delete_result.deleted_count)
}
