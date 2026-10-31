# CF Framework

A production-ready, modular Rust API framework designed for scalability and maintainability. Built with **Actix-web**, **SeaORM**, and **Utoipa**.

## 🚀 Features

- **Modular Architecture**: Split into `cf-api`, `cf-entity`, `cf-migration`, `cf-repository`, and `cf-util` crates.
- **REST API**: Built with Actix-web.
- **ORM**: SeaORM for type-safe database interactions (PostgreSQL).
- **Migrations**: SeaORM Migration with a CLI (`up`, `down`, `status`, `fresh`, ...), similar to `php artisan migrate`.
- **Documentation**: Auto-generated Swagger/OpenAPI documentation (`/swagger-ui`).
- **Configuration**: Environment variable based configuration.
- **Pinned Toolchain**: Rust version locked via `rust-toolchain.toml`.
- **Containerization**: Optimized Dockerfile for production.
- **CI/CD**: Jenkinsfile and Kubernetes manifests included.

## 📂 Project Structure

```
├── cf-api              # Main API server, handlers, services, routes, DTOs
├── cf-entity           # Database entities (SeaORM models)
├── cf-migration        # Database migrations + migration CLI
├── cf-repository       # Data access layer (Repositories)
├── cf-util             # Shared utilities (S3, Sanitization, etc.)
├── k8s                 # Kubernetes manifests
├── Cargo.toml          # Workspace configuration
├── rust-toolchain.toml # Pinned Rust version
└── Dockerfile          # Multi-stage Docker build
```

## 🛠 Prerequisites

- **Rust**: 1.94.0 (installed automatically by `rustup` from `rust-toolchain.toml`)
- **PostgreSQL**: 14+
- **Docker** (optional)

## 🚦 Quick Start

### 1. Configure Environment

Copy `.env.example` to `.env` and configure your database credentials:

```bash
cp .env.example .env
```

### 2. Create the Database

Create an empty database matching `DATABASE_NAME`. No extensions are required; tables are created by the migrations.

```sql
CREATE DATABASE your_database;
```

### 3. Run Database Migrations

```bash
cargo run -p cf-migration
```

### 4. Run the API Server

```bash
cargo run -p cf-api
```

The API also runs pending migrations on startup, so step 3 is optional for local development.

The server will start at `http://localhost:3000` (configurable with `API_HOST` and `API_PORT`).

- **Health Check**: `http://localhost:3000/health`
- **Swagger UI**: `http://localhost:3000/swagger-ui/`

## 🗄 Migrations

| Command                                  | Description                               |
| ---------------------------------------- | ----------------------------------------- |
| `cargo run -p cf-migration`              | Apply all pending migrations (same as `up`) |
| `cargo run -p cf-migration -- status`    | Show applied / pending migrations         |
| `cargo run -p cf-migration -- down`      | Roll back the last migration              |
| `cargo run -p cf-migration -- refresh`   | Roll back all, then apply all             |
| `cargo run -p cf-migration -- reset`     | Roll back all migrations                  |
| `cargo run -p cf-migration -- fresh`     | Drop all tables, then apply all (⚠️ deletes data) |

The CLI builds the connection URL from the `DATABASE_*` variables in `.env`.

## 👨‍💻 Development

### Adding a New Entity

1. Create a new migration in `cf-migration/src/` and register it in `cf-migration/src/lib.rs`.
   - Use `gen_random_uuid()` for UUID defaults (built into PostgreSQL 13+). Do not use `uuid_generate_v4()`, which requires the `uuid-ossp` extension.
2. Create the entity file in `cf-entity/src/entity/`.
3. Register the entity in `cf-entity/src/entity/mod.rs` and `prelude.rs`.

### Adding a New API Resource

1. Create a Repository in `cf-repository/src/repositories/` and re-export it in `mod.rs`.
2. Create DTOs in `cf-api/src/dto/`.
3. Create a Service in `cf-api/src/services/`.
4. Create a Handler in `cf-api/src/handlers/`.
5. Register Routes in `cf-api/src/routes/` and add the scope in `routes/routes.rs`.
6. In `cf-api/src/main.rs`, register the handlers in `paths(...)`, the DTOs/models in `schemas(...)`, and the service with `app_data(...)`.

### Upgrading Rust

Update the version in `rust-toolchain.toml`, `rust-version` in the root `Cargo.toml`, and the `FROM rust:` image in `Dockerfile` in the same commit.

## 🐳 Docker Build

```bash
docker build -t cf-framework .
docker run -p 3000:3000 --env-file .env cf-framework
```

The container runs `cf-migration` and then starts `cf-api` (see `entrypoint.sh`).

## 📦 Deployment

The `k8s/` directory contains standard Kubernetes manifests:

- `deployment.yaml`
- `service.yaml`
- `ingress.yaml`
- `configmap.yaml`
- `secret.yaml`

Update `configmap.yaml` and `secret.yaml` with your production configuration before applying.

## 📄 License

[MIT License](LICENSE)
