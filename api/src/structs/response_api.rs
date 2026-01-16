use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ResponseApi<T> {
  pub(crate) status: bool,
  pub(crate) message: String,
  pub(crate) data: Option<T>,
}
