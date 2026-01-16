use super::sea_orm_active_enums::PostStatus;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "posts")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  pub user_id: Uuid,
  pub title: String,
  #[sea_orm(column_type = "Text")]
  pub content: String,
  pub status: PostStatus,
  #[schema(value_type = String, format = DateTime)]
  pub created_at: DateTimeWithTimeZone,
  #[schema(value_type = String, format = DateTime)]
  pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::users::Entity",
    from = "Column::UserId",
    to = "super::users::Column::Id",
    on_update = "Cascade",
    on_delete = "Cascade"
  )]
  Users,
}

impl Related<super::users::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Users.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}
