# CF Framework - Migration Crate

This crate (`migration`) manages database schema changes using SeaORM Migration.

## Structure

- `src/mYYYYMMDD_HHMMSS_name.rs`: Individual migration files.
- `src/lib.rs`: Registry of all migrations.

## Creating a Migration

To create a new migration:

```bash
sea-orm-cli migrate generate create_table_example
```

Then edit the generated file in `src/`.

Finally, register the new migration in `src/lib.rs`.
