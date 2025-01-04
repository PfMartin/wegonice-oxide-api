use anyhow::Result;
use axum::{serve, Router};
use tokio::{self, net::TcpListener};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};

pub struct Server {}

impl Server {
    pub async fn new(host: &str, routers: Vec<Router>) -> Result<Self> {
        tracing_subscriber::fmt().init();

        let router = routers
            .into_iter()
            .fold(Router::new(), |mut acc, r| {
                acc = acc.merge(r);
                acc
            })
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO)),
            );

        let listener = TcpListener::bind(host).await?;
        info!("Started server on, {host}");

        serve(listener, router).await?;

        Ok(Self {})
    }
}
