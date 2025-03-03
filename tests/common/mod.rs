use serde::Deserialize;

#[derive(Deserialize)]
pub struct ResponseBody {
    pub data: Option<String>,
    pub error: String,
}
