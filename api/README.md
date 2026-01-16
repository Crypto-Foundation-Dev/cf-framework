# CF Framework - API Crate

This crate (`api`) serves as the entry point for the CF Framework REST API application. It handles HTTP requests, authentication, and responses.

## Structure

- `src/main.rs`: Entry point, server configuration, service wiring.
- `src/handlers/`: API Request handlers (Controllers).
- `src/services/`: Business logic layer.
- `src/routes/`: Route definitions and resource mapping.
- `src/dto/`: Data Transfer Objects (Request/Response structs).
- `src/config/`: Configuration modules (DB, Errors).
- `src/structs/`: Shared structs (Response wrapper, Pagination).

## Adding a New Endpoint

1. Create DTOs in `src/dto/`.
2. Implement service logic in `src/services/`.
3. Create handler function in `src/handlers/` with Utoipa annotations.
4. Register route in `src/routes/`.
5. Update `main.rs` to include the service and handlers.
