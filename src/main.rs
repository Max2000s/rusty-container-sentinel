use axum::Router;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing_subscriber::EnvFilter;

mod docker;
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting rusty-container-sentinel!");

    let api_routes = Router::new().merge(routes::docker_routes());

    let app = Router::new()
        .nest("/api", api_routes)
        .layer(CorsLayer::permissive())
        .fallback_service(ServeDir::new("static"));

    let host = "0.0.0.0:8080";

    tracing::info!("Starting backen service on {}.", host);
    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
