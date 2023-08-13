use actix_web::web;

pub mod company_view;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/settings").configure(company_view::routes));
}
