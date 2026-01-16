# CF Framework - Repository Crate

This crate (`repository`) handles direct database interactions. It abstracts the SeaORM queries from the service layer.

## Structure

- `src/repositories/`: Repository implementations.
- `src/repositories/mod.rs`: Module registration.

## Usage

Repositories should implement common CRUD operations:

- `find_by_id`
- `find_all`
- `create`
- `update`
- `delete`
