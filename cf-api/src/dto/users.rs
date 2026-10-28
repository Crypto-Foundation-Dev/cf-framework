use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate, ToSchema)]
pub struct CreateUserRequest {
  #[validate(length(min = 1, max = 255, message = "Name must be 1-255 characters"))]
  pub name: String,
  #[validate(email(message = "Invalid email format"), length(max = 255))]
  pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate, ToSchema)]
pub struct UpdateUserRequest {
  #[validate(length(min = 1, max = 255, message = "Name must be 1-255 characters"))]
  pub name: Option<String>,
  #[validate(email(message = "Invalid email format"), length(max = 255))]
  pub email: Option<String>,
}
