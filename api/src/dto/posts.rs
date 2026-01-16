use entity::entity::sea_orm_active_enums::PostStatus;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate, ToSchema)]
pub struct CreatePostRequest {
  pub user_id: Uuid,
  #[validate(length(min = 1, message = "Title is required"))]
  pub title: String,
  #[validate(length(min = 1, message = "Content is required"))]
  pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate, ToSchema)]
pub struct UpdatePostRequest {
  pub title: Option<String>,
  pub content: Option<String>,
  pub status: Option<PostStatus>,
}
