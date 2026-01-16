# CF Framework

A production-ready, modular Rust API framework designed for scalability and maintainability. Built with **Actix-web**, **SeaORM**, and **Utoipa**.

## 🚀 Features

- **Modular Architecture**: Split into `api`, `entity`, `migration`, `repository`, and `util` crates.
- **REST API**: Built with Actix-web.
- **ORM**: SeaORM for type-safe database interactions (PostgreSQL).
- **Migrations**: Database schema migrations managed via SeaORM Migration.
- **Documentation**: Auto-generated Swagger/OpenAPI documentation (`/swagger-ui`).
- **Configuration**: Environment variable based configuration.
- **Containerization**: Optimized Dockerfile for production.
- **CI/CD**: Jenkinsfile and Kubernetes manifests included.

## 📂 Project Structure

```
├── api             # Main API server, handlers, services, routes, DTOs
├── entity          # Database entities (SeaORM models)
├── migration       # Database migrations
├── repository      # Data access layer (Repositories)
├── util            # Shared utilities (S3, Sanitization, etc.)
├── k8s             # Kubernetes manifests
├── Cargo.toml      # Workspace configuration
└── Dockerfile      # Multi-stage Docker build
```

## 🛠 Prerequisites

- **Rust**: 1.75+
- **PostgreSQL**: 14+
- **Docker** (optional)

## 🚦 Quick Start

### 1. Configure Environment

Copy `.env.example` to `.env` and configure your database credentials:

```bash
cp .env.example .env
```

### 2. Run Database Migrations

```bash
cargo run -p migration
```

### 3. Run the API Server

```bash
cargo run -p api
```

The server will start at `http://localhost:3000`.

- **Health Check**: `http://localhost:3000/health`
- **Swagger UI**: `http://localhost:3000/swagger-ui/`

## 👨‍💻 Development

### Adding a New Entity

1. Create a new migration in `migration/src/`.
2. Create the entity file in `entity/src/entity/`.
3. Register the entity in `entity/src/entity/mod.rs`.

### Adding a New API Resource

1. Create a Repository in `repository/src/repositories/`.
2. Create DTOs in `api/src/dto/`.
3. Create a Service in `api/src/services/`.
4. Create a Handler in `api/src/handlers/`.
5. Register Routes in `api/src/routes/`.
6. Register the service in `api/src/main.rs`.

## 🐳 Docker Build

```bash
docker build -t cf-framework .
docker run -p 3000:3000 --env-file .env cf-framework
```

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
