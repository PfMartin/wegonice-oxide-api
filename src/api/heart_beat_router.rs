use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

pub struct HeartBeatRouter {
    pub router: Router,
}

impl HeartBeatRouter {
    pub fn new() -> Self {
        let router = Router::new().route("/heart_beat", get(Self::get_heart_beat));

        Self { router }
    }

    async fn get_heart_beat() -> impl IntoResponse {
        Html("Hello world")
    }
}

#[cfg(test)]
mod unit_tests_heart_beat_router {
    use super::*;
    use anyhow::Result;
    use axum_test::TestServer;
    use tokio::test;

    #[test]
    async fn get_heart_beat() -> Result<()> {
        let heart_beat_router = HeartBeatRouter::new();

        let app = heart_beat_router.router;
        let server = TestServer::new(app).unwrap();

        let response = server.get("/heart_beat").await;

        response.assert_status_ok();
        response.assert_text("Hello world");

        Ok(())
    }
}
