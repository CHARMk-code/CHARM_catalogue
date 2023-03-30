use actix_web::web;

pub mod shortcut;

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v2")
            .configure(shortcut::routes),
    );
}


