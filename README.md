# Rusty Container Sentinel

<p align="center">
  <img src="static/logo.png" width="200" alt="Rusty Container Sentinel Logo">
</p>

Rusty Container Sentinel is a lightweight, efficient monitoring tool built in Rust, designed to provide real-time visibility into Docker container logs, statuses, and metrics.

## 🚀 Features

- **Real-time Docker Container Monitoring** (logs and statistics)
- **Simple Web UI** with live updates via WebSockets
- **Built with Rust & Axum** for performance, efficiency, and reliability

## 📌 Requirements

- Docker
- Docker Compose

## 🚀 Quick Start

Clone and launch with Docker Compose:

```bash
git clone https://github.com/your-username/rusty-container-sentinel.git
cd rusty-container-sentinel
docker-compose up --build
```

Open your browser:

```
http://localhost:8080
```

## 🛠 Development

Build and run manually:

```bash
cargo run
```

Build Docker image locally:

```bash
docker-compose build
docker-compose up
```

## 📂 Project Structure

```
rusty-container-sentinel
├── src                 # Rust backend logic
├── static              # Frontend HTML, JS, CSS
├── Dockerfile          # Docker build instructions
└── docker-compose.yml  # Easy Docker setup
```

## 📖 Logging

Set the logging level using `RUST_LOG`:

```bash
RUST_LOG=info cargo run
```

## 🔐 Security Considerations

Ensure Docker socket is securely mounted with read-only permissions in production.

## 📃 License

[MIT License](LICENSE)
