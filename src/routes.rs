use std::time::Duration;

use crate::docker::{get_containers, get_docker_version, stream_container_logs, DockerVersion};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path,
    },
    routing::get,
    Json, Router,
};
use futures_util::StreamExt;
use serde_json::to_string;
use tokio::time::interval;
use tracing::{debug, error, info};

pub fn docker_routes() -> Router {
    Router::new()
        .route("/version", get(docker_version_handler))
        .route("/ws/containers", get(ws_containers_handlers))
        .route("/ws/logs/:id", get(ws_logs_handler))
}

async fn ws_containers_handlers(ws: WebSocketUpgrade) -> impl axum::response::IntoResponse {
    ws.on_upgrade(stream_containers)
}

async fn ws_logs_handler(
    ws: WebSocketUpgrade,
    Path(id): Path<String>,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| stream_logs(socket, id))
}

async fn docker_version_handler() -> Json<DockerVersion> {
    info!("Got a request for docker version.");
    match get_docker_version().await {
        Ok(version) => Json(version),
        Err(_) => Json(DockerVersion {
            version: "unknown".to_string(),
            api_version: "unknown".to_string(),
        }),
    }
}

async fn stream_containers(mut socket: WebSocket) {
    let mut ticker = interval(Duration::from_secs(5));

    loop {
        ticker.tick().await;
        match get_containers().await {
            Ok(containers) => {
                let json = to_string(&containers).unwrap_or_default();
                if socket.send(Message::Text(json)).await.is_err() {
                    error!("Failed to send message via websocket!");
                    break;
                } else {
                    debug!("Sent containers list via websockets!")
                }
            }
            Err(e) => {
                error!(
                    "Retrieval of information form docker socket failed: {:?}",
                    e
                );
                break;
            }
        }
    }
}

async fn stream_logs(mut socket: WebSocket, container_id: String) {
    let mut logs_stream = stream_container_logs(&container_id).await;

    while let Some(log_result) = logs_stream.next().await {
        match log_result {
            Ok(log_line) => {
                if socket.send(Message::Text(log_line)).await.is_err() {
                    break;
                }
            }
            Err(e) => {
                error!("Error streaming logs: {:?}", e);
                break;
            }
        }
    }
}
