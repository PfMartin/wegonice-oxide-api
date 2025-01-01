use super::api_response::ApiResponse;
use axum::{response::IntoResponse, routing::get, Json, Router};

pub struct HeartBeatRouter {
    pub router: Router,
}

impl HeartBeatRouter {
    pub fn new() -> Self {
        let router = Router::new().route("/heart_beat", get(Self::get_heart_beat));

        Self { router }
    }

    async fn get_heart_beat() -> impl IntoResponse {
        Json(ApiResponse::<String> {
            data: "Ok".into(),
            error: "".into(),
        })
    }
}

#[cfg(test)]
mod unit_tests_heart_beat_router {
    use super::*;
    use anyhow::Result;
    use axum_test::TestServer;
    use pretty_assertions::assert_eq;
    use tokio::test;

    #[test]
    async fn get_heart_beat() -> Result<()> {
        let heart_beat_router = HeartBeatRouter::new();

        let app = heart_beat_router.router;
        let server = TestServer::new(app).unwrap();

        let response = server.get("/heart_beat").await;

        response.assert_status_ok();
        let body = response.json::<ApiResponse<String>>();
        assert_eq!(body.data, "Ok");
        assert_eq!(body.error, "");

        Ok(())
    }
}
