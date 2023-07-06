use actix_web::web::Json;
use actix_web::{delete, error, get, post, put, web, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::errors::MyError;
use crate::services;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct TagWeb {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub parent_tag: Option<i32>, //TODO Remove these 3 columns
    pub up_votes: Option<i32>,
    pub down_votes: Option<i32>,
    pub crowd_sourced: Option<bool>,
    pub icon: Option<String>,
    pub division: Option<bool>,
    pub business_area: Option<bool>,
    pub looking_for: Option<bool>,
    pub offering: Option<bool>,
    pub language: Option<bool>,
    pub fair_area: Option<bool>,
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tag")
            .service(get_all_handler)
            .service(get_by_id_handler)
            .service(update_handler)
            .service(create_handler)
            .service(delete_handler),
    );
}

#[get("/")]
async fn get_all_handler(db: web::Data<PgPool>) -> Result<impl Responder> {
    let tags = services::tag::get_all((*db).as_ref().clone()).await?;

    Ok(HttpResponse::Ok().json(tags))
}

#[get("/{id}")]
async fn get_by_id_handler(db: web::Data<PgPool>, path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    let tag = services::tag::get_by_id((*db).as_ref().clone(), id).await?;

    Ok(HttpResponse::Ok().json(tag))
}

#[put("/")]
async fn update_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    data: Json<TagWeb>,
) -> Result<impl Responder> {
    let input_tag = data.into_inner();

    let response = match input_tag.id {
        Some(_) => {
            let name = input_tag.name.as_ref();
            let parent_tag = input_tag.parent_tag.as_ref();
            let up_votes = input_tag.up_votes.as_ref();
            let down_votes = input_tag.down_votes.as_ref();
            let icon = input_tag.icon.as_ref();
            let division = input_tag.division.as_ref();
            let business_area = input_tag.business_area.as_ref();
            let looking_for = input_tag.looking_for.as_ref();
            let offering = input_tag.offering.as_ref();
            let language = input_tag.language.as_ref();
            let fair_area = input_tag.fair_area.as_ref();

            if name
                .and(parent_tag)
                .and(up_votes)
                .and(down_votes)
                .and(icon)
                .and(division)
                .and(business_area)
                .and(looking_for)
                .and(offering)
                .and(language)
                .and(fair_area)
                .is_none()
            {
                HttpResponse::UnprocessableEntity().finish()
            } else {
                let tag = services::tag::update((*db).as_ref().clone(), input_tag).await?;
                HttpResponse::Ok().json(tag)
            }
        }
        None => {
            let tag = services::tag::create((*db).as_ref().clone(), input_tag).await?;
            HttpResponse::Created().json(tag)
        }
    };

    Ok(response)
}

#[post("/")] // TODO Deprecatea in favor of put
async fn create_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    data: Json<TagWeb>,
) -> Result<impl Responder> {
    let input_tag = data.into_inner();
    let affected_rows = services::tag::create((*db).as_ref().clone(), input_tag).await?;

    Ok(HttpResponse::Created().json(affected_rows))
}

#[delete("/{id}")]
async fn delete_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let affected_rows = services::tag::delete((*db).as_ref().clone(), id).await?;

    Ok(HttpResponse::Ok().json(affected_rows))
}
