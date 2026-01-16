use sea_orm::prelude::DateTime;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

/// Affiliate info with id and name only
#[derive(Serialize, ToSchema)]
pub struct AffiliateInfo {
    pub id: Uuid,
    pub name: String,
}

/// User affiliate response without sensitive data (no password)
#[derive(Serialize, ToSchema)]
pub struct UserAffiliateResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub bio: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    pub is_verified: bool,
    pub is_affiliated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliated_with: Option<AffiliateInfo>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
