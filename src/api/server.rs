use anyhow::Result;
use axum::{serve, Router};
use std::mem::replace;
use tokio::{self, net::TcpListener};

pub struct Server {}

impl Server {
    pub async fn new(host: &str, routers: Vec<Router>) -> Result<Server> {
        let router = routers.into_iter().fold(Router::new(), |mut acc, r| {
            let temp = replace(&mut acc, Router::new());
            acc = temp.merge(r);
            acc
        });

        let listener = TcpListener::bind(host).await?;
        println!("Listening on, {:?}", listener.local_addr());

        serve(listener, router.into_make_service()).await?;

        Ok(Server {})
    }
}
