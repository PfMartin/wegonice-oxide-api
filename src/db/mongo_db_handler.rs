use crate::{
    db::db_handler::DbHandler,
    model::user::{Role, User, UserCreate, UserDb},
};
use anyhow::{anyhow, Result};
use bson::{doc, oid::ObjectId};
use mongodb::{options::ClientOptions, Client, Collection};
use std::str::FromStr;

pub struct MongoDbHandler {
    pub users_collection: Collection<UserDb>,
}

impl MongoDbHandler {
    pub async fn new(user: &str, password: &str, db_name: &str, db_host: &str) -> Result<Self> {
        let uri = format!("mongodb://{user}:{password}@{db_host}/{db_name}?authSource={db_name}");
        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;

        let db = client.database(db_name);
        let users_collection = db.collection("users");

        let db_handler = MongoDbHandler { users_collection };

        Ok(db_handler)
    }
}

impl DbHandler for MongoDbHandler {
    async fn get_user_by_id(&self, id: &str) -> Result<User> {
        let object_id = ObjectId::from_str(id)?;

        let user_db = self
            .users_collection
            .find_one(doc! {"_id": object_id})
            .await?;

        let user: User = match user_db {
            Some(u) => u.into(),
            None => return Err(anyhow!(format!("Failed to find user with id: {id}"))),
        };

        Ok(user)
    }

    async fn create_user(&self, user: UserCreate) -> Result<String> {
        let user_db = UserDb {
            id: None,
            email: user.email,
            password_hash: user.password_hash,
            role: Role::User,
            is_activated: false,
            created_at: bson::DateTime::now(),
            modified_at: bson::DateTime::now(),
        };

        let insert_result = self.users_collection.insert_one(&user_db).await?;

        Ok(insert_result.inserted_id.to_string())
    }
}
