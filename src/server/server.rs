use anyhow::Result;
use axum::{serve, Router};
use tokio::{self, net::TcpListener};

pub struct Server {}

impl Server {
    pub async fn new(host: &str, heart_beat_router: Router) -> Result<Server> {
        let listener = TcpListener::bind(host).await?;

        println!("Listening on, {:?}", listener.local_addr());

        serve(listener, heart_beat_router.into_make_service()).await?;

        Ok(Server {})
    }
}
