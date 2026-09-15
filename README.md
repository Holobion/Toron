# Toron
From the french "toronner" this app help you create the main thread of your life.

## Description
A local first todo app.


## Architecture & Services

The stack is composed of 4 coordinated Docker services:

| Service | Technology | Default Port | Description |
|---|---|---|---|
| **`frontend`** | Dioxus (Rust / WASM) + Nginx / Dev Server | `8000` (prod) / `8080` (dev) | Local-first Web UI with live reload |
| **`backend`** | Axum (Rust) + Cargo Watch | `3000` | REST API, authentication & JWKS provider |
| **`powersync`** | PowerSync Service | `8080` | Real-time database sync stream service |
| **`postgres`** | PostgreSQL 16 (`wal_level=logical`) | `5432` | Source database with logical replication enabled |

## Getting Started

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/) & [Docker Compose](https://docs.docker.com/compose/) (v2.22+)

### 1. Environment Setup

Copy the example environment variables:
```bash
cp .env.example .env
```

### 2. Running in Production Mode

Start all services in detached mode:
```bash
docker compose up -d
```

Check service status and logs:
```bash
docker compose ps
docker compose logs -f
```

### 3. Running in Development Mode (with Watch / Hot-Reload)

Use [docker-compose.dev.yml](file:///home/viveli/Dev/Toron/docker-compose.dev.yml) to enable hot-reloading and file synchronization with `docker compose watch`:

```bash
docker compose -f docker-compose.yml -f docker-compose.dev.yml up --watch
```

- **Frontend (`src/`, `Dioxus.toml`)**: Synchronized in real-time with Dioxus dev server live reload.
- **Backend (`src/`)**: Synchronized in real-time with incremental `cargo-watch` re-compilation.
- **PowerSync / Database configs (`powersync/`, `init-db/`)**: Automatically synced with container restart.
- **Dependencies (`Cargo.toml`, `Cargo.lock`)**: Automatically triggers a container rebuild.

### Access Endpoints

- **Frontend UI**: [http://localhost:8000](http://localhost:8000) (Production) / [http://localhost:8080](http://localhost:8080) (Development)
- **Backend API**: [http://localhost:3000](http://localhost:3000)
- **PowerSync Service**: [http://localhost:8080](http://localhost:8080)


## Technical stack

### Frontend

- rust
- [dioxus](https://dioxuslabs.com/)
- [powersync-sdk](https://docs.powersync.com/client-sdks/reference/rust)

### Backend

- rust
- [axum](https://docs.rs/axum/latest/axum/)

### Database Sync

- [powersync](https://docs.powersync.com/intro/powersync-overview)

### Database

- Postgres

### CI / CD

- github
- docker
- docker compose
