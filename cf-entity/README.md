# CF Framework - Entity Crate

This crate (`entity`) contains the SeaORM entity definitions that map to the database tables.

## Structure

- `src/entity/`: Entity definitions (Models).
- `src/entity/mod.rs`: Module registration.
- `src/entity/prelude.rs`: Common imports.
- `src/entity/sea_orm_active_enums.rs`: Database enums.

## Generating Entities

You can manually create entities or use `sea-orm-cli` to generate them from an existing database:

```bash
sea-orm-cli generate entity -o entity/src/entity
```
