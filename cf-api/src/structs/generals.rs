use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, Debug, IntoParams, ToSchema)]
pub struct PaginationParam {
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, ToSchema, Clone)]
pub struct Empty {}
