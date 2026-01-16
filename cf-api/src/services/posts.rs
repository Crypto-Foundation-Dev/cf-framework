use crate::config::custom_error::AppError;
use crate::dto::posts::{CreatePostRequest, UpdatePostRequest};
use cf_entity::entity::posts;
use cf_entity::entity::sea_orm_active_enums::PostStatus;
use cf_repository::repositories::PostsRepository;
use sea_orm::{DatabaseConnection, IntoActiveModel, Set};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostService {
  db: Arc<DatabaseConnection>,
}

impl PostService {
  pub fn new(db: Arc<DatabaseConnection>) -> Self {
    Self { db }
  }

  pub async fn get_all_posts(
    &self,
    page: u64,
    per_page: u64,
  ) -> Result<(Vec<posts::Model>, u64), AppError> {
    PostsRepository::find_all(&self.db, page, per_page)
      .await
      .map_err(AppError::from)
  }

  pub async fn get_post(&self, id: Uuid) -> Result<posts::Model, AppError> {
    PostsRepository::find_by_id(&self.db, id)
      .await
      .map_err(AppError::from)?
      .ok_or_else(|| AppError::NotFound("Post not found".to_string()))
  }

  pub async fn create_post(&self, req: CreatePostRequest) -> Result<posts::Model, AppError> {
    let new_post = posts::ActiveModel {
      id: Set(Uuid::new_v4()),
      user_id: Set(req.user_id),
      title: Set(req.title),
      content: Set(req.content),
      status: Set(PostStatus::Draft),
      ..Default::default()
    };

    PostsRepository::create(&self.db, new_post)
      .await
      .map_err(AppError::from)
  }

  pub async fn update_post(
    &self,
    id: Uuid,
    req: UpdatePostRequest,
  ) -> Result<posts::Model, AppError> {
    let mut post_model: posts::ActiveModel = PostsRepository::find_by_id(&self.db, id)
      .await
      .map_err(AppError::from)?
      .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?
      .into_active_model();

    if let Some(title) = req.title {
      post_model.title = Set(title);
    }

    if let Some(content) = req.content {
      post_model.content = Set(content);
    }

    if let Some(status) = req.status {
      post_model.status = Set(status);
    }

    post_model.updated_at = Set(chrono::Utc::now().into());

    PostsRepository::update(&self.db, id, post_model)
      .await
      .map_err(AppError::from)
  }

  pub async fn delete_post(&self, id: Uuid) -> Result<(), AppError> {
    PostsRepository::delete(&self.db, id)
      .await
      .map(|_| ())
      .map_err(AppError::from)
  }
}
