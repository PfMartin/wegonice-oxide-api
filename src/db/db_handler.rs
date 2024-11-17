use crate::model::user::{User, UserCreate};
use anyhow::Result;

pub trait DbHandler {
    async fn get_user_by_id(&self, id: &str) -> Result<User>;
    async fn create_user(&self, user: UserCreate) -> Result<String>;
}
