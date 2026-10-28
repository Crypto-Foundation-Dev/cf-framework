use cf_entity::entity::sea_orm_active_enums::PostStatus;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate, ToSchema)]
pub struct CreatePostRequest {
  pub user_id: Uuid,
  #[validate(length(min = 1, max = 255, message = "Title must be 1-255 characters"))]
  pub title: String,
  #[validate(length(min = 1, max = 100000, message = "Content must be 1-100000 characters"))]
  pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate, ToSchema)]
pub struct UpdatePostRequest {
  #[validate(length(min = 1, max = 255, message = "Title must be 1-255 characters"))]
  pub title: Option<String>,
  #[validate(length(min = 1, max = 100000, message = "Content must be 1-100000 characters"))]
  pub content: Option<String>,
  pub status: Option<PostStatus>,
}
