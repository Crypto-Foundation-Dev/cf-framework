use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "users")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  pub name: String,
  #[sea_orm(unique)]
  pub email: String,
  #[schema(value_type = String, format = DateTime)]
  pub created_at: DateTimeWithTimeZone,
  #[schema(value_type = String, format = DateTime)]
  pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::posts::Entity")]
  Posts,
}

impl Related<super::posts::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Posts.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}
