use anyhow::Result;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

pub struct HeartBeatRouter {
    pub router: Router,
}

impl HeartBeatRouter {
    pub fn new() -> Result<HeartBeatRouter> {
        let router = Router::new().route("/heart_beat", get(Self::get_heart_beat));

        Ok(HeartBeatRouter { router })
    }

    async fn get_heart_beat() -> impl IntoResponse {
        Html("Hello world")
    }
}
