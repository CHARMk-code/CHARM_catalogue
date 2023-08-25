use actix_web::web;

pub mod blob;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/settings")
            .configure(blob::routes),
    );
}
