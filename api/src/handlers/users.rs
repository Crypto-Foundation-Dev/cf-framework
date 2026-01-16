use crate::config::custom_error::AppError;
use crate::dto::users::{CreateUserRequest, UpdateUserRequest};
use crate::services::users::UserService;
use crate::structs::generals::{Empty, PaginationParam};
use crate::structs::response_api::ResponseApi;
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};

#[utoipa::path(
    get,
    path = "/api/users",
    params(PaginationParam),
    tag = "user",
    responses(
        (status = 200, description = "List users", body = ResponseApi<Vec<entity::entity::users::Model>>),
    )
)]
#[get("")]
pub async fn get_all_users(
  service: web::Data<UserService>,
  params: web::Query<PaginationParam>,
) -> Result<impl Responder, AppError> {
  let page = params.page.unwrap_or(1);
  let per_page = params.per_page.unwrap_or(10);

  let (users, _total_pages) = service.get_all_users(page, per_page).await?;
  Ok(HttpResponse::Ok().json(ResponseApi {
    status: true,
    message: "Users retrieved successfully".to_string(),
    data: Some(users),
  }))
}

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    tag = "user",
    responses(
        (status = 200, description = "Get user", body = ResponseApi<entity::entity::users::Model>),
        (status = 404, description = "User not found")
    )
)]
#[get("/{id}")]
pub async fn get_user(
  service: web::Data<UserService>,
  id: web::Path<uuid::Uuid>,
) -> Result<impl Responder, AppError> {
  let user = service.get_user(*id).await?;
  Ok(HttpResponse::Ok().json(ResponseApi {
    status: true,
    message: "User retrieved successfully".to_string(),
    data: Some(user),
  }))
}

#[utoipa::path(
    post,
    path = "/api/users",
    request_body = CreateUserRequest,
    tag = "user",
    responses(
        (status = 201, description = "User created", body = ResponseApi<entity::entity::users::Model>),
    )
)]
#[post("")]
pub async fn create_user(
  service: web::Data<UserService>,
  req: web::Json<CreateUserRequest>,
) -> Result<impl Responder, AppError> {
  let user = service.create_user(req.into_inner()).await?;
  Ok(HttpResponse::Created().json(ResponseApi {
    status: true,
    message: "User created successfully".to_string(),
    data: Some(user),
  }))
}

#[utoipa::path(
    put,
    path = "/api/users/{id}",
    tag = "user",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated", body = ResponseApi<entity::entity::users::Model>),
    )
)]
#[put("/{id}")]
pub async fn update_user(
  service: web::Data<UserService>,
  id: web::Path<uuid::Uuid>,
  req: web::Json<UpdateUserRequest>,
) -> Result<impl Responder, AppError> {
  let user = service.update_user(*id, req.into_inner()).await?;
  Ok(HttpResponse::Ok().json(ResponseApi {
    status: true,
    message: "User updated successfully".to_string(),
    data: Some(user),
  }))
}

#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    tag = "user",
    responses(
        (status = 200, description = "User deleted", body = ResponseApi<Empty>),
    )
)]
#[delete("/{id}")]
pub async fn delete_user(
  service: web::Data<UserService>,
  id: web::Path<uuid::Uuid>,
) -> Result<impl Responder, AppError> {
  service.delete_user(*id).await?;
  Ok(HttpResponse::Ok().json(ResponseApi {
    status: true,
    message: "User deleted successfully".to_string(),
    data: Some(Empty {}),
  }))
}
