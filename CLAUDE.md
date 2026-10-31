# CLAUDE.md

Rust REST API template: Actix-web + SeaORM (PostgreSQL) + Utoipa (Swagger). Cargo workspace, edition 2024.

## Commands

```bash
cp .env.example .env          # DATABASE_* wajib, API_HOST/API_PORT opsional
cargo build
cargo run -p cf-api           # http://localhost:3000, auto-run migrations saat start
cargo fmt                     # pakai rustfmt.toml (2 spasi, max_width 100)
cargo clippy --workspace
```

- Package names pakai prefix `cf-` (`cf-api`, `cf-migration`, ...). README masih tulis `-p api` / `-p migration`, itu salah.
- Migration CLI (SeaORM `run_cli`, URL dirakit dari `DATABASE_*`): `cargo run -p cf-migration -- [up|down|status|fresh|refresh|reset]`. Tanpa argumen = `up`. `cf-api` juga auto-run `Migrator::up` saat start.
- Belum ada test di crate workspace.
- `cf-cli` (binary `cf`, `cf new <nama>`) ada di luar workspace (`exclude`), jadi jalankan dengan `--manifest-path cf-cli/Cargo.toml`. Baris yang mengandung kata `cf-cli` di `Cargo.toml`, `README.md`, dan `CLAUDE.md` dihapus otomatis dari project hasil scaffold, jadi tulis satu baris saja.
- `cf-api` punya dependency `cf-auth` (tag v0.3.5) dan `cf-email` (tag v0.1.0) dari repo publik `Crypto-Foundation-Dev`, tapi keduanya belum dipakai di kode.

## Workspace

| Crate | Isi |
|---|---|
| `cf-api` | Binary. `main.rs` (wiring, OpenAPI doc, CORS), `handlers/`, `services/`, `routes/`, `dto/`, `config/` (DB, `AppError`), `structs/` (`ResponseApi`, pagination), `validators/` |
| `cf-entity` | SeaORM models di `src/entity/`, enum DB di `sea_orm_active_enums.rs`. `util/post_generate.rs` = post-process hasil `sea-orm-cli` |
| `cf-migration` | `mYYYYMMDD_HHMMSS_name.rs`, didaftarkan di `src/lib.rs` |
| `cf-repository` | Static repo struct per tabel (`UsersRepository::find_by_id(&db, id)`), re-export di `repositories/mod.rs` |
| `cf-util` | `s3`, `sanitize` (ammonia), `tiptap`, `intro` |

## Request flow

`routes/` → `handlers/` (Actix macro + `#[utoipa::path]`) → `services/` (struct `Clone` berisi `Arc<DatabaseConnection>`, return `Result<_, AppError>`) → `cf-repository` (return `DbErr`) → `cf-entity`.

- Semua response pakai `ResponseApi { status, message, data }`. Error otomatis dibungkus format sama lewat `impl ResponseError for AppError`.
- Semua route di bawah `/api`. Juga ada `/health` dan `/swagger-ui/`.

## Tambah resource baru (checklist)

1. Migration di `cf-migration/src/`, daftarkan di `lib.rs`. Default UUID pakai `gen_random_uuid()` (bawaan PG 13+), jangan `uuid_generate_v4()` (butuh extension `uuid-ossp`).
2. Entity di `cf-entity/src/entity/`, daftarkan di `mod.rs` + `prelude.rs`.
3. Repository di `cf-repository/src/repositories/`, re-export di `mod.rs`.
4. DTO di `cf-api/src/dto/` (`Deserialize`, `Validate`, `ToSchema`).
5. Service di `cf-api/src/services/`.
6. Handler di `cf-api/src/handlers/`, route di `cf-api/src/routes/<name>.rs`, scope di `routes/routes.rs`.
7. Di `main.rs`: daftarkan handler di `paths(...)`, DTO/model di `schemas(...)`, tag, dan `app_data(web::Data::new(service))`.

## Gotchas

- `main.rs` baca `env::var("API_port")` (huruf kecil), jadi `API_PORT` di `.env` tidak terbaca; selalu default 3000.
- `cf-repository/src/helpers/generator.rs` dan `templates/repository.tera` (generator repository via Tera) belum dipakai di mana pun.
- `Dockerfile` copy binary `cf-entity`, padahal `cf-entity` tidak punya `main.rs`, jadi binary itu cuma dummy `fn main() {}` dari stage cache.
- Deploy: `Jenkinsfile` (agent `staging`, k3s) + manifest di `k8s/`.
