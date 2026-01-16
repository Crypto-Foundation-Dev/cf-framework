use hub_be_entity::entity::news_posts;
use hub_be_entity::entity::sea_orm_active_enums::PostStatus;
use sea_orm::FromQueryResult;
use sea_orm::prelude::{DateTime, Uuid};
use serde::Serialize;
use utoipa::ToSchema;
// Create struct for custom select, for the type please specify same as the original model
// Below is the example
// #[derive(FromQueryResult, Serialize)]
// pub struct NewsListCustom {
//     pub id: Uuid,
//     pub title: String,
//     pub slug: String,
// }

#[derive(FromQueryResult, Serialize, ToSchema)]
pub struct AuthorResponse {
    #[sea_orm(from_alias = "author_id")]
    pub id: String,
    #[sea_orm(from_alias = "author_first")]
    pub first_name: String,
    #[sea_orm(from_alias = "author_last")]
    pub last_name: String,
}

/// Response Section
#[derive(FromQueryResult, Serialize, ToSchema)]
pub struct CategoryResponse {
    #[sea_orm(from_alias = "category_id")]
    pub id: Uuid,
    #[sea_orm(from_alias = "category_name")]
    pub name: String,
}

#[derive(Serialize, ToSchema)]
pub struct NewsOneResponse {
    #[serde(flatten)]
    pub post: news_posts::Model,
    pub author: Option<AuthorResponse>,
    pub category: Option<CategoryResponse>,
    pub tags: Vec<String>,
}

#[derive(FromQueryResult, Serialize, ToSchema)]
pub struct NewsListCustom {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub status: PostStatus,
    pub excerpt: String,
    pub header_image: String,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,

    #[sea_orm(nested)]
    pub author: Option<AuthorResponse>,

    #[sea_orm(nested)]
    pub category: Option<CategoryResponse>,
}

#[derive(FromQueryResult, Serialize, ToSchema)]
pub struct NewsListPreview {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub header_image: String,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
}
