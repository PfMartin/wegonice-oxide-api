use anyhow::Result;
use axum::{serve, Router};
use std::mem::replace;
use tokio::{self, net::TcpListener};

pub struct Server {}

impl Server {
    pub async fn new(host: &str, routers: Vec<Router>) -> Result<Self> {
        let router = routers.into_iter().fold(Router::new(), |mut acc, r| {
            let temp_acc = replace(&mut acc, Router::new());
            acc = temp_acc.merge(r);
            acc
        });

        let listener = TcpListener::bind(host).await?;
        println!("Started server on, {host}");

        serve(listener, router.into_make_service()).await?;

        Ok(Self {})
    }
}
