use sea_orm::FromQueryResult;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, FromQueryResult, Serialize, ToSchema)]
pub struct FollowUsersResponse {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
    pub is_verified: Option<bool>,
    pub is_affiliated: Option<bool>,
    pub affiliated_with_id: Option<Uuid>,
    pub affiliated_name: Option<String>,
}
