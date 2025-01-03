use anyhow::Result;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

pub struct UsersRouter {
    pub router: Router,
}

impl UsersRouter {
    pub fn new() -> Result<Self> {
        let router = Router::new().route("/users", get(Self::get_users));

        Ok(Self { router })
    }

    async fn get_users() -> impl IntoResponse {
        Html("Hello Users")
    }
}
