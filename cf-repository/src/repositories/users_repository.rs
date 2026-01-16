use ::cf_entity::entity::users;
use sea_orm::prelude::Uuid;
use sea_orm::*;

pub struct UsersRepository;

impl UsersRepository {
  pub async fn find_by_id(db: &DbConn, id: Uuid) -> Result<Option<users::Model>, DbErr> {
    users::Entity::find_by_id(id).one(db).await
  }

  pub async fn find_by_email(db: &DbConn, email: &str) -> Result<Option<users::Model>, DbErr> {
    users::Entity::find()
      .filter(users::Column::Email.eq(email))
      .one(db)
      .await
  }

  pub async fn find_all(
    db: &DbConn,
    page: u64,
    per_page: u64,
  ) -> Result<(Vec<users::Model>, u64), DbErr> {
    let paginator = users::Entity::find().paginate(db, per_page);
    let num_pages = paginator.num_pages().await?;
    let items = paginator.fetch_page(page - 1).await?;
    Ok((items, num_pages))
  }

  pub async fn create(db: &DbConn, form_data: users::ActiveModel) -> Result<users::Model, DbErr> {
    form_data.insert(db).await
  }

  pub async fn update(
    db: &DbConn,
    id: Uuid,
    form_data: users::ActiveModel,
  ) -> Result<users::Model, DbErr> {
    let _user: users::ActiveModel = users::Entity::find_by_id(id)
      .one(db)
      .await?
      .ok_or(DbErr::RecordNotFound("User not found".to_owned()))?
      .into();

    // Merge updates here if needed, or assume form_data has the ID set
    // But typically we fetch, then update fields.
    // For simplicity, let's assume form_data is prepared.
    // Actually, let's just use the form_data directly but ensure ID is correct
    let mut form_data = form_data;
    form_data.id = Set(id);

    form_data.update(db).await
  }

  pub async fn delete(db: &DbConn, id: Uuid) -> Result<DeleteResult, DbErr> {
    users::Entity::delete_by_id(id).exec(db).await
  }
}
