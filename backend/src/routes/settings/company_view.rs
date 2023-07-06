use actix_web::{
    get, post, put,
    web::{self, Json},
    HttpResponse, Responder, Result,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::services;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/company_view")
            .service(get_all_handler)
            .service(reset_company_view_handler)
            .service(update_company_view_handler)
    );
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct CompanyCardWeb {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub text: Option<String>,
    pub active: Option<bool>,
}

#[get("/")]
async fn get_all_handler(db: web::Data<PgPool>) -> Result<impl Responder> {
    let cards = services::settings::company_view::get_all((*db).as_ref().clone()).await?;

    Ok(HttpResponse::Ok().json(cards))
}

#[post("/reset")]
async fn reset_company_view_handler(db: web::Data<PgPool>) -> Result<impl Responder> {
    let response = services::settings::company_view::reset((*db).as_ref().clone()).await?;

    Ok(HttpResponse::Ok())
}

#[put("/")]
async fn update_company_view_handler(
    db: web::Data<PgPool>,
    data: Json<CompanyCardWeb>,
) -> Result<impl Responder> {
    let response = services::settings::company_view::reset((*db).as_ref().clone()).await?;

    Ok(HttpResponse::Ok())
}
