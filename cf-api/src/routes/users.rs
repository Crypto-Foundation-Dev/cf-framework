use crate::handlers::users::*;
use actix_web::web;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(get_all_users)
    .service(get_user)
    .service(create_user)
    .service(update_user)
    .service(delete_user);
}
