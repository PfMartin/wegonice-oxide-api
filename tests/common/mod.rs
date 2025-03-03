use anyhow::Result;
use bson::{doc, oid::ObjectId, DateTime};
use mongodb::{options::ClientOptions, Client};
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

pub async fn register_user() -> Result<String> {
    dotenv::from_path(".env")?;

    let db_name = env::var("MONGO_WEGONICE_DB")?;
    let db_user_name = env::var("MONGO_WEGONICE_USER")?;
    let db_user_password = env::var("MONGO_WEGONICE_PASSWORD")?;
    let db_host = env::var("MONGO_WEGONICE_HOST")?;

    let uri = format!(
        "mongodb://{db_user_name}:{db_user_password}@{db_host}/{db_name}?authSource={db_name}"
    );

    let client_options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(client_options)?;

    client
        .database(&db_name)
        .run_command(doc! { "ping": 1 })
        .await?;

    let db = client.database(&db_name);
    let users_collection = db.collection::<UserMongoDb>("users");

    // Password: test_password
    let user = UserMongoDb {
        _id: ObjectId::new(),
        email: "loginUser@email.com".into(),
        password_hash: "$argon2id$v=19$m=19456,t=2,p=1$8/I4UD0qo5joX/Vd5R9TfA$ryIDC5fe3axKWvC4qLsznYsEUjgqu+xvgV3aX/mCJOI".into(),
        role: Role::User,
        is_activated: true,
        created_at: DateTime::now(),
        modified_at: DateTime::now()
    };

    let insert_result = users_collection.insert_one(&user).await?;

    Ok(insert_result.inserted_id.to_string())
}
