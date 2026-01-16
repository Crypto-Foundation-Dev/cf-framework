use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate, ToSchema)]
pub struct CreateUserRequest {
  #[validate(length(min = 1, message = "Name is required"))]
  pub name: String,
  #[validate(email(message = "Invalid email format"))]
  pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate, ToSchema)]
pub struct UpdateUserRequest {
  pub name: Option<String>,
  #[validate(email(message = "Invalid email format"))]
  pub email: Option<String>,
}
