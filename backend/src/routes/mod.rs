pub mod shortcut;
pub mod tag;

use actix_web::web;

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v2")
            .configure(shortcut::routes)
            .configure(tag::routes),
    );
}


