## Frontend development

### Prerequisites

For standalone frontend development, install:

- Rust and Cargo;
- the WebAssembly target: `wasm32-unknown-unknown`;
- the Dioxus CLI (`dx`).

From the repository root, enter the frontend directory:

```bash
cd toron_frontend
```

### Run the frontend locally

Start the Dioxus web development server with live reload:

```bash
dx serve
```

The application is available at `http://localhost:8080` by default. To bind
the server to another interface or port, use the corresponding Dioxus CLI
options. The web platform is selected by default through `Cargo.toml`.

### Validate frontend changes

Run formatting, unit tests and compilation checks from `toron_frontend`:

```bash
cargo fmt --check
cargo test
cargo check
```

The task service tests use the in-memory repository and do not require the
backend, PostgreSQL or PowerSync services.

### Run the Docker development environment

From the repository root, copy the environment template if needed:

```bash
cp .env.example .env
```

Start the development stack with Compose Watch:

```bash
docker compose -f docker-compose.yml -f docker-compose.dev.yml up --watch
```

The frontend development container runs Dioxus on port `8080` and synchronizes
changes to `toron_frontend/src`, `Dioxus.toml` and its Cargo manifests. The
other services are started alongside it so backend and sync integration can be
tested through the reverse proxy.

### Build and run the production frontend

Build the frontend image from the repository root:

```bash
docker compose build frontend
```

Start the production stack:

```bash
docker compose up -d
```

The Nginx-served frontend is available at `http://localhost:8000`. Nginx also
proxies `/api/` to the backend and `/powersync/` to the PowerSync service.

The Compose files expect `powersync/` and database initialization
configuration referenced by `docker-compose.yml`. Those files must be present
before starting the complete stack; standalone `dx serve` and the frontend
Cargo checks do not depend on them.

## Task service

The frontend task feature follows a local-first architecture organized by
logical level:

```text
      Home / Dioxus components
                 |
            TaskService
                 |
        TaskRepository trait
           /          \
Mock repository   PowerSync repository (future)
```

### Domain

`toron_frontend/src/models/task.rs` contains the `Task` entity, the `NewTask`
command and the `SyncStatus` state. The task carries a stable identifier and
timestamps so a PowerSync-backed implementation can map it to a local table.

### Service and repository boundary

`toron_frontend/src/services/task_service.rs` contains task use cases and does
not depend on Dioxus, HTTP or a database SDK. `TaskRepository` is defined in
`toron_frontend/src/repositories/task_repository.rs`; it is the persistence
port where a PowerSync adapter will be added later without changing the
service or views.

### Mock behavior

`MockTaskRepository` in
`toron_frontend/src/repositories/mock_task_repository.rs` is currently used by
the home view. Reads come from its in-memory local store, and writes are
immediately visible locally. New and updated tasks are marked `Pending`, which
represents the write queue that PowerSync will own in the production adapter.
No backend request is made yet.

### PowerSync integration boundary

The current frontend intentionally does not add a PowerSync SDK dependency.
The repository trait keeps that dependency isolated because the compatible
PowerSync client and its WASM/browser initialization still need to be selected.
The future adapter should map repository operations to local PowerSync SQL
mutations and expose PowerSync watch results as `Task` values.

### Validation

The task service currently has unit tests covering local creation, completion,
deletion and invalid titles. They run with the frontend Cargo test suite.
