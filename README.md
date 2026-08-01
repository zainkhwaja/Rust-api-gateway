# Rust API Gateway

A simplified API gateway scaffold in Rust using Axum, Tower, Redis, Prometheus, and OpenTelemetry.

## Features

- Reverse proxy handler
- JWT authentication support
- API key authentication support
- Rate limiting via Redis
- Middleware scaffolding for request/response lifecycle
- Prometheus metrics endpoint
- OpenTelemetry-ready tracing setup
- Hot config reload endpoint
- Docker and Docker Compose support

## Getting Started

1. Build locally:

```bash
cargo build
```

2. Run with sample config:

```bash
cargo run --release
```

3. Run with Docker Compose:

```bash
docker compose up --build
```

## Configuration

Edit `config.yaml` to configure Redis, authentication, routes, and telemetry.

## API Endpoints

- `GET /metrics` - Prometheus metrics
- `POST /admin/reload` - Reload configuration
- `GET /health` - Health check
- `/*` - Proxy to configured upstream
