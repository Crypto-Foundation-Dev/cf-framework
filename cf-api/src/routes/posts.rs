use crate::handlers::posts::*;
use actix_web::web;

pub fn post_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_posts)
        .service(get_post)
        .service(create_post)
        .service(update_post)
        .service(delete_post);
}
