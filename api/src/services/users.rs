use crate::config::custom_error::AppError;
use crate::dto::users::{CreateUserRequest, UpdateUserRequest};
use entity::entity::users;
use repository::repositories::UsersRepository;
use sea_orm::{DatabaseConnection, IntoActiveModel, Set};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
  db: Arc<DatabaseConnection>,
}

impl UserService {
  pub fn new(db: Arc<DatabaseConnection>) -> Self {
    Self { db }
  }

  pub async fn get_all_users(
    &self,
    page: u64,
    per_page: u64,
  ) -> Result<(Vec<users::Model>, u64), AppError> {
    UsersRepository::find_all(&self.db, page, per_page)
      .await
      .map_err(AppError::from)
  }

  pub async fn get_user(&self, id: Uuid) -> Result<users::Model, AppError> {
    UsersRepository::find_by_id(&self.db, id)
      .await
      .map_err(AppError::from)?
      .ok_or_else(|| AppError::NotFound("User not found".to_string()))
  }

  pub async fn create_user(&self, req: CreateUserRequest) -> Result<users::Model, AppError> {
    // Check if email exists
    if let Ok(Some(_)) = UsersRepository::find_by_email(&self.db, &req.email).await {
      return Err(AppError::ValidationError(
        "Email already exists".to_string(),
      ));
    }

    let new_user = users::ActiveModel {
      id: Set(Uuid::new_v4()),
      name: Set(req.name),
      email: Set(req.email),
      ..Default::default()
    };

    UsersRepository::create(&self.db, new_user)
      .await
      .map_err(AppError::from)
  }

  pub async fn update_user(
    &self,
    id: Uuid,
    req: UpdateUserRequest,
  ) -> Result<users::Model, AppError> {
    let mut user_model: users::ActiveModel = UsersRepository::find_by_id(&self.db, id)
      .await
      .map_err(AppError::from)?
      .ok_or_else(|| AppError::NotFound("User not found".to_string()))?
      .into_active_model();

    if let Some(name) = req.name {
      user_model.name = Set(name);
    }

    if let Some(email) = req.email {
      user_model.email = Set(email);
    }

    user_model.updated_at = Set(chrono::Utc::now().into());

    UsersRepository::update(&self.db, id, user_model)
      .await
      .map_err(AppError::from)
  }

  pub async fn delete_user(&self, id: Uuid) -> Result<(), AppError> {
    UsersRepository::delete(&self.db, id)
      .await
      .map(|_| ())
      .map_err(AppError::from)
  }
}
