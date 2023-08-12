use actix_web::{
    get, post, put,
    web::{self, Json},
    HttpResponse, Responder, Result,
};
use sqlx::PgPool;

use crate::{
    models::company_card::CompanyCardWeb,
    services::{self, auth::AuthedUser},
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/company_view")
            .service(get_all_handler)
            .service(reset_company_view_handler)
            .service(update_company_view_handler),
    );
}

#[get("/")]
async fn get_all_handler(db: web::Data<PgPool>) -> Result<impl Responder> {
    let cards = services::settings::company_view::get_all(db.as_ref()).await?;

    Ok(HttpResponse::Ok().json(cards))
}

#[post("/reset")]
async fn reset_company_view_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
) -> Result<impl Responder> {
    let _response = services::settings::company_view::reset(db.as_ref()).await?;

    Ok(HttpResponse::Ok())
}

#[put("/")]
async fn update_company_view_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    data: Json<CompanyCardWeb>,
) -> Result<impl Responder> {
    let response = services::settings::company_view::update(db.as_ref(), data.into_inner()).await?;

    Ok(HttpResponse::Ok().json(response))
}
