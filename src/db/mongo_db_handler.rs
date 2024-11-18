use crate::model::user::UserDb;
use anyhow::Result;

use mongodb::{options::ClientOptions, Client, Collection};

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
