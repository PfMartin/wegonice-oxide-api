use super::mongo_db_handler::MongoDbHandler;
use anyhow::{anyhow, Result};
use bson::{doc, oid::ObjectId};
use futures_util::TryStreamExt;
use serde::de::DeserializeOwned;
use std::{convert::Into, marker::Sync};

pub trait GenericHandler {
    async fn get_multiple<T, S>(&self, collection_name: &str) -> Result<Vec<S>>
    where
        T: Sync + Send + DeserializeOwned + Into<S> + Copy;
    async fn get_by_id<T, S>(&self, id: &str, collection_name: &str) -> Result<S>
    where
        T: Sync + Send + DeserializeOwned + Into<S>;
}

impl GenericHandler for MongoDbHandler {
    async fn get_multiple<T, S>(&self, collection_name: &str) -> Result<Vec<S>>
    where
        T: Sync + Send + DeserializeOwned + Into<S> + Copy,
    {
        let cursor = self
            .db
            .collection::<T>(collection_name)
            .find(doc! {})
            .await?;

        let db_documents = cursor.try_collect::<Vec<T>>().await?;
        let documents = db_documents.iter().map(|&d| d.into()).collect::<Vec<S>>();

        Ok(documents)
    }

    async fn get_by_id<T, S>(&self, id: &str, collection_name: &str) -> Result<S>
    where
        T: Sync + Send + DeserializeOwned + Into<S>,
    {
        let object_id = ObjectId::parse_str(id)?;

        let find_result = self
            .db
            .collection::<T>(collection_name)
            .find_one(doc! {"_id": object_id})
            .await?;

        match find_result {
            Some(document) => Ok(document.into()),
            None => Err(anyhow!(
                "Failed to find document in {collection_name} collection with id {id}"
            )),
        }
    }
}
