use crate::config::actix_error::{json_error_handler, path_error_handler};
use crate::config::database::connect_db;
use crate::routes::routes::main_routes;
use crate::services::posts::PostService;
use crate::services::users::UserService;
use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, web};
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use std::sync::Arc;
use std::{env, process};
use util::intro;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod dto;
mod handlers;
mod routes;
mod services;
mod structs;

use entity::entity::posts::Model as PostModel;
use entity::entity::sea_orm_active_enums::PostStatus;
use entity::entity::users::Model as UserModel;

// Define OpenAPI spec
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Rust API Framework Template",
        description = "A clean, modular Rust API framework template using Actix-web, SeaORM, and Utoipa."
    ),
    paths(
        crate::handlers::users::get_all_users,
        crate::handlers::users::get_user,
        crate::handlers::users::create_user,
        crate::handlers::users::update_user,
        crate::handlers::users::delete_user,
        crate::handlers::posts::get_all_posts,
        crate::handlers::posts::get_post,
        crate::handlers::posts::create_post,
        crate::handlers::posts::update_post,
        crate::handlers::posts::delete_post,
    ),
    components(
        schemas(
            crate::dto::users::CreateUserRequest,
            crate::dto::users::UpdateUserRequest,
            crate::dto::posts::CreatePostRequest,
            crate::dto::posts::UpdatePostRequest,
            crate::structs::generals::PaginationParam,
            UserModel,
            PostModel,
            PostStatus,
        )
    ),
    tags(
        (name = "user", description = "User management endpoints"),
        (name = "post", description = "Post management endpoints"),
    ),
)]
struct ApiDoc;

#[actix_web::main]
async fn main() {
  intro("Starting API Server");

  dotenv().ok();

  // Tracing
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .with_env_filter("sea_orm=debug")
    .init();

  let db_host = env::var("DATABASE_HOST").expect("DATABASE_HOST not set");
  let db_username = env::var("DATABASE_USERNAME").expect("DATABASE_USERNAME not set");
  let db_password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD not set");
  let db_port = env::var("DATABASE_PORT").expect("DATABASE_PORT not set");
  let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME not set");

  let api_host = env::var("API_HOST").unwrap_or(String::from("0.0.0.0"));
  let api_port = env::var("API_port").unwrap_or(String::from("3000"));

  // Connect DB
  let db = match connect_db(db_host, db_username, db_password, db_port, db_name).await {
    Ok(db) => {
      println!("Successfully connect to db!");
      db
    }
    Err(err) => {
      eprintln!("Failed connect to DB: {}", err);
      process::exit(1);
    }
  };

  // Run migrations
  match Migrator::up(&db, None).await {
    Ok(_) => {
      println!("Successfully migrated to database!")
    }
    Err(err) => {
      eprintln!("Failed connect to migrate: {}", err);
      process::exit(1);
    }
  };

  let db = Arc::new(db);

  // Initial services
  let user_service = UserService::new(db.clone());
  let post_service = PostService::new(db.clone());

  // Run web api
  println!("\nStarting server to {}:{}", api_host, api_port);
  HttpServer::new(move || {
    // Config cors
    let cors = Cors::default()
      .allow_any_origin()
      .allow_any_method()
      .allow_any_header()
      .max_age(3600);

    App::new()
      .wrap(cors)
      .service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
      )
      .app_data(web::Data::new(user_service.clone()))
      .app_data(web::Data::new(post_service.clone()))
      .app_data(web::PathConfig::default().error_handler(path_error_handler))
      .app_data(web::JsonConfig::default().error_handler(json_error_handler))
      .route(
        "/",
        web::get().to(|| async { HttpResponse::Ok().body("API framework template is running!") }),
      )
      .route(
        "/health",
        web::get().to(|| async {
          HttpResponse::Ok().json(serde_json::json!({
              "status": "ok",
              "timestamp": chrono::Utc::now().to_rfc3339()
          }))
        }),
      )
      .configure(|cfg| {
        cfg.service(web::scope("/api").configure(main_routes));
      })
      .default_service(
        web::route().to(|| async { HttpResponse::NotFound().body("404 page not found") }),
      )
  })
  .bind(format!("{}:{}", api_host, api_port))
  .unwrap()
  .run()
  .await
  .unwrap();
}
