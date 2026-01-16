use super::{posts::post_routes, users::user_routes};
use actix_web::web;

pub fn main_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(web::scope("/users").configure(user_routes))
    .service(web::scope("/posts").configure(post_routes));
}
