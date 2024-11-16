use crate::db::db_handler::DbHandler;
use anyhow::Result;
use mongodb::{options::ClientOptions, Client, Database};

pub struct MongoDbHandler {
    pub users_db: Database,
}

impl MongoDbHandler {
    pub async fn new(user: &str, password: &str, db_name: &str, db_host: &str) -> Result<Self> {
        let uri = format!("mongodb://{user}:{password}@{db_host}/{db_name}?authSource={db_name}");
        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;

        let users_db = client.database("users");

        let db_handler = MongoDbHandler { users_db: users_db };

        Ok(db_handler)
    }
}

impl DbHandler for MongoDbHandler {
    async fn get_user_by_id(&self, id: &str) -> Result<String> {
        let new_id = format!("{id}");

        Ok(new_id.into())
    }
}
