use crate::structs::response_api::ResponseApi;
use actix_web::{HttpResponse, error};

pub fn path_error_handler(err: error::PathError, _req: &actix_web::HttpRequest) -> error::Error {
  let response_json: ResponseApi<()> = match &err {
    error::PathError::Deserialize(de_err) => {
      // Check if the error message contains UUID-related keywords
      let err_msg = de_err.to_string();
      if err_msg.contains("UUID") || err_msg.contains("uuid") {
        ResponseApi {
          status: false,
          message: format!("Invalid UUID format: {}", err_msg),
          data: None,
        }
      } else {
        // If your response result error ended here
        // please create new if with contains your error.
        ResponseApi {
          status: false,
          message: format!("Invalid path parameter: {}", de_err),
          data: None,
        }
      }
    }
    _ => ResponseApi {
      status: false,
      message: format!("Path error: {}", err),
      data: None,
    },
  };

  error::InternalError::from_response(err, HttpResponse::BadRequest().json(response_json)).into()
}

pub fn json_error_handler(
  err: error::JsonPayloadError,
  _req: &actix_web::HttpRequest,
) -> error::Error {
  let response_json: ResponseApi<()> = match &err {
    error::JsonPayloadError::Deserialize(de_err) => {
      // Check if the error message contains UUID-related keywords
      let err_msg = de_err.to_string();
      if err_msg.contains("UUID") || err_msg.contains("uuid") {
        ResponseApi {
          status: false,
          message: format!("Invalid UUID format: {}", err_msg),
          data: None,
        }
      } else {
        // Remove "at line X column Y" from the error message
        let cleaned_msg = if let Some(pos) = err_msg.find(" at line ") {
          err_msg[..pos].to_string()
        } else {
          err_msg
        };

        ResponseApi {
          status: false,
          message: format!("Invalid json parameter: {}", cleaned_msg),
          data: None,
        }
      }
    }
    _ => ResponseApi {
      status: false,
      message: format!("Path error: {}", err),
      data: None,
    },
  };

  error::InternalError::from_response(err, HttpResponse::BadRequest().json(response_json)).into()
}
