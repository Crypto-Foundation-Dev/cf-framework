pub use sea_orm_migration::prelude::*;

mod m20250101_000001_create_table_users;
mod m20250101_000002_create_table_posts;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
      Box::new(m20250101_000001_create_table_users::Migration),
      Box::new(m20250101_000002_create_table_posts::Migration),
    ]
  }
}
