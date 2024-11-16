use anyhow::Result;

pub trait DbHandler {
    async fn get_user_by_id(&self, id: &str) -> Result<String>;
}
