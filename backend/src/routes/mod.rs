pub mod auth;
pub mod company;
pub mod feedback;
pub mod layout;
pub mod map;
pub mod prepage;
pub mod settings;
pub mod shortcut;
pub mod tag;
pub mod batch;
pub mod image;

use actix_web::web;

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v2")
            .configure(shortcut::routes)
            .configure(tag::routes)
            .configure(map::routes)
            .configure(layout::routes)
            .configure(prepage::routes)
            .configure(feedback::routes)
            .configure(company::routes)
            .configure(auth::routes)
            .configure(batch::routes)
            .configure(image::routes)
            .configure(settings::scope),
    );
}