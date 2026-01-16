use crate::config::custom_error::AppError;
use crate::dto::posts::{CreatePostRequest, UpdatePostRequest};
use crate::services::posts::PostService;
use crate::structs::generals::{Empty, PaginationParam};
use crate::structs::response_api::ResponseApi;
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};

#[utoipa::path(
    get,
    path = "/api/posts",
    params(PaginationParam),
    tag = "post",
    responses(
        (status = 200, description = "List posts", body = ResponseApi<Vec<cf_entity::entity::posts::Model>>),
    )
)]
#[get("")]
pub async fn get_all_posts(
  service: web::Data<PostService>,
  params: web::Query<PaginationParam>,
) -> Result<impl Responder, AppError> {
  let page = params.page.unwrap_or(1);
  let per_page = params.per_page.unwrap_or(10);

  let (posts, _total_pages) = service.get_all_posts(page, per_page).await?;
  Ok(HttpResponse::Ok().json(ResponseApi {
    status: true,
    message: "Posts retrieved successfully".to_string(),
    data: Some(posts),
  }))
}

#[utoipa::path(
    get,
    path = "/api/posts/{id}",
    tag = "post",
    responses(
        (status = 200, description = "Get post", body = ResponseApi<cf_entity::entity::posts::Model>),
        (status = 404, description = "Post not found")
    )
)]
#[get("/{id}")]
pub async fn get_post(
  service: web::Data<PostService>,
  id: web::Path<uuid::Uuid>,
) -> Result<impl Responder, AppError> {
  let post = service.get_post(*id).await?;
  Ok(HttpResponse::Ok().json(ResponseApi {
    status: true,
    message: "Post retrieved successfully".to_string(),
    data: Some(post),
  }))
}

#[utoipa::path(
    post,
    path = "/api/posts",
    request_body = CreatePostRequest,
    tag = "post",
    responses(
        (status = 201, description = "Post created", body = ResponseApi<cf_entity::entity::posts::Model>),
    )
)]
#[post("")]
pub async fn create_post(
  service: web::Data<PostService>,
  req: web::Json<CreatePostRequest>,
) -> Result<impl Responder, AppError> {
  let post = service.create_post(req.into_inner()).await?;
  Ok(HttpResponse::Created().json(ResponseApi {
    status: true,
    message: "Post created successfully".to_string(),
    data: Some(post),
  }))
}

#[utoipa::path(
    put,
    path = "/api/posts/{id}",
    tag = "post",
    request_body = UpdatePostRequest,
    responses(
        (status = 200, description = "Post updated", body = ResponseApi<cf_entity::entity::posts::Model>),
    )
)]
#[put("/{id}")]
pub async fn update_post(
  service: web::Data<PostService>,
  id: web::Path<uuid::Uuid>,
  req: web::Json<UpdatePostRequest>,
) -> Result<impl Responder, AppError> {
  let post = service.update_post(*id, req.into_inner()).await?;
  Ok(HttpResponse::Ok().json(ResponseApi {
    status: true,
    message: "Post updated successfully".to_string(),
    data: Some(post),
  }))
}

#[utoipa::path(
    delete,
    path = "/api/posts/{id}",
    tag = "post",
    responses(
        (status = 200, description = "Post deleted", body = ResponseApi<Empty>),
    )
)]
#[delete("/{id}")]
pub async fn delete_post(
  service: web::Data<PostService>,
  id: web::Path<uuid::Uuid>,
) -> Result<impl Responder, AppError> {
  service.delete_post(*id).await?;
  Ok(HttpResponse::Ok().json(ResponseApi {
    status: true,
    message: "Post deleted successfully".to_string(),
    data: Some(Empty {}),
  }))
}
