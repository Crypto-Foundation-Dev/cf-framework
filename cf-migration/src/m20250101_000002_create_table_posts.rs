use sea_orm_migration::prelude::*;

use super::m20250101_000001_create_table_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Posts::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(Posts::Id)
              .uuid()
              .not_null()
              .primary_key()
              .default(Expr::cust("uuid_generate_v4()")),
          )
          .col(ColumnDef::new(Posts::UserId).uuid().not_null())
          .col(ColumnDef::new(Posts::Title).string().not_null())
          .col(ColumnDef::new(Posts::Content).text().not_null())
          .col(
            ColumnDef::new(Posts::Status)
              .string()
              .not_null()
              .default("DRAFT"),
          )
          .col(
            ColumnDef::new(Posts::CreatedAt)
              .timestamp_with_time_zone()
              .not_null()
              .default(Expr::current_timestamp()),
          )
          .col(
            ColumnDef::new(Posts::UpdatedAt)
              .timestamp_with_time_zone()
              .not_null()
              .default(Expr::current_timestamp()),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk_posts_user_id")
              .from(Posts::Table, Posts::UserId)
              .to(Users::Table, Users::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Posts::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum Posts {
  Table,
  Id,
  UserId,
  Title,
  Content,
  Status,
  CreatedAt,
  UpdatedAt,
}
