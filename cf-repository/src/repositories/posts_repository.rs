use ::cf_entity::entity::posts;
use sea_orm::prelude::Uuid;
use sea_orm::*;

pub struct PostsRepository;

impl PostsRepository {
  pub async fn find_by_id(db: &DbConn, id: Uuid) -> Result<Option<posts::Model>, DbErr> {
    posts::Entity::find_by_id(id).one(db).await
  }

  pub async fn find_all(
    db: &DbConn,
    page: u64,
    per_page: u64,
  ) -> Result<(Vec<posts::Model>, u64), DbErr> {
    let paginator = posts::Entity::find()
      .order_by_desc(posts::Column::CreatedAt)
      .paginate(db, per_page);
    let num_pages = paginator.num_pages().await?;
    let items = paginator.fetch_page(page - 1).await?;
    Ok((items, num_pages))
  }

  pub async fn create(db: &DbConn, form_data: posts::ActiveModel) -> Result<posts::Model, DbErr> {
    form_data.insert(db).await
  }

  pub async fn update(
    db: &DbConn,
    id: Uuid,
    form_data: posts::ActiveModel,
  ) -> Result<posts::Model, DbErr> {
    // Ensure exists
    posts::Entity::find_by_id(id)
      .one(db)
      .await?
      .ok_or(DbErr::RecordNotFound("Post not found".to_owned()))?;

    let mut form_data = form_data;
    form_data.id = Set(id);

    form_data.update(db).await
  }

  pub async fn delete(db: &DbConn, id: Uuid) -> Result<DeleteResult, DbErr> {
    posts::Entity::delete_by_id(id).exec(db).await
  }
}
