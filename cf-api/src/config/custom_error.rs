use crate::structs::response_api::ResponseApi;
use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use sea_orm::DbErr;
use std::fmt;
use validator::ValidationErrors;

#[derive(Debug)]
pub enum AppError {
  ValidationError(String),
  Validator(ValidationErrors),
  NotFound(String),
  DatabaseError(String),
  InternalError(String),
  S3Error(String),
}

impl fmt::Display for AppError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
      AppError::Validator(msg) => write!(f, "{}", msg),
      AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
      AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
      AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
      AppError::S3Error(msg) => write!(f, "S3 error: {}", msg), // Add this
    }
  }
}

impl ResponseError for AppError {
  fn status_code(&self) -> StatusCode {
    match self {
      AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
      AppError::Validator(_) => StatusCode::BAD_REQUEST,
      AppError::NotFound(_) => StatusCode::NOT_FOUND,
      AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      AppError::S3Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }

  fn error_response(&self) -> HttpResponse {
    let message = match self {
      AppError::ValidationError(msg) => msg.clone(),
      AppError::Validator(err) => err.to_string(),
      AppError::NotFound(msg) => msg.clone(),
      AppError::DatabaseError(msg) => msg.clone(),
      AppError::InternalError(msg) => msg.clone(),
      AppError::S3Error(msg) => msg.clone(),
    };

    HttpResponse::build(self.status_code()).json(ResponseApi {
      status: false,
      message,
      data: None::<()>,
    })
  }
}

// Convert DbErr to AppError
impl From<DbErr> for AppError {
  fn from(err: DbErr) -> Self {
    match err {
      DbErr::RecordNotFound(msg) => AppError::NotFound(msg),
      DbErr::Custom(msg) => {
        if msg == "Author ID is required" {
          return AppError::ValidationError(msg);
        }
        AppError::DatabaseError(msg)
      }
      _ => AppError::InternalError(format!("{}", err)),
    }
  }
}

impl From<aws_sdk_s3::Error> for AppError {
  fn from(err: aws_sdk_s3::Error) -> Self {
    // Convert the AWS S3 error into your AppError variant
    AppError::S3Error(err.to_string()) // Adjust based on your AppError enum
  }
}

impl From<std::io::Error> for AppError {
  fn from(err: std::io::Error) -> Self {
    AppError::InternalError(err.to_string())
  }
}

impl From<serde_json::Error> for AppError {
  fn from(err: serde_json::Error) -> Self {
    AppError::ValidationError(format!("Invalid JSON format: {}", err))
  }
}

impl From<ValidationErrors> for AppError {
  fn from(err: ValidationErrors) -> Self {
    AppError::Validator(err)
  }
}
