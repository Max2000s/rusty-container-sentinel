use bollard::Docker;
use bollard::{
    container::{ListContainersOptions, LogsOptions},
    secret::ContainerSummary,
};
use futures_util::StreamExt;
use serde::Serialize;

#[derive(Serialize)]
pub struct DockerVersion {
    pub version: String,
    pub api_version: String,
}

#[derive(Serialize, Clone)]
pub struct ContainerInfo {
    pub id: String,
    pub names: Vec<String>,
    pub image: String,
    pub state: String,
    pub status: String,
}

pub async fn get_docker_version() -> Result<DockerVersion, bollard::errors::Error> {
    let docker = Docker::connect_with_unix_defaults()?;
    let version = docker.version().await?;

    Ok(DockerVersion {
        version: version.version.unwrap(),
        api_version: version.api_version.unwrap(),
    })
}

pub async fn get_containers() -> Result<Vec<ContainerInfo>, bollard::errors::Error> {
    let docker = Docker::connect_with_unix_defaults()?;

    let containers: Vec<ContainerSummary> = docker
        .list_containers(Some(ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await?;

    let container_info: Vec<ContainerInfo> = containers
        .into_iter()
        .map(|c| ContainerInfo {
            id: c.id.unwrap_or_default(),
            names: c.names.unwrap_or_default(),
            image: c.image.unwrap_or_default(),
            state: c.state.unwrap_or_default(),
            status: c.status.unwrap_or_default(),
        })
        .collect();

    Ok(container_info)
}

pub async fn stream_container_logs(
    container_id: &str,
) -> impl futures_util::Stream<Item = Result<String, bollard::errors::Error>> {
    let docker = Docker::connect_with_unix_defaults().unwrap();

    docker
        .logs(
            container_id,
            Some(LogsOptions::<String> {
                follow: true,
                stdout: true,
                stderr: true,
                tail: "250".to_string(),
                ..Default::default()
            }),
        )
        .map(|log_result| log_result.map(|log| log.to_string()))
}
