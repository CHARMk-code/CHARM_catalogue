use actix_web::web::Json;
use actix_web::{web, get, put, Result, Responder, post, delete, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::services;

use chrono::DateTime;
use chrono::offset::Utc;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct FeedbackWeb {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub text: Option<String>,
    pub meta: Option<String>,
    pub received: Option<DateTime<Utc>>,
    pub important: Option<bool>,
    pub archived: Option<bool>
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/feedback")
            .service(get_all_handler)
            .service(get_by_id_handler)
            .service(update_handler)
            .service(create_handler)
            .service(delete_handler)
    );
}

#[get("/")]
    async fn get_all_handler(db: web::Data<PgPool>) -> Result<impl Responder> {
    let feedback = services::feedback::get_all((*db).as_ref().clone()).await?;

    Ok(HttpResponse::Ok().json(feedback))
}

#[get("/{id}")]
async fn get_by_id_handler(db: web::Data<PgPool>, path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    let feedback = services::feedback::get_by_id((*db).as_ref().clone(), id).await?;

    Ok(HttpResponse::Ok().json(feedback))
}

#[put("/")]
async fn update_handler(db: web::Data<PgPool>, data: Json<FeedbackWeb>) -> Result<impl Responder> {
    let input_feedback = data.into_inner();

    let response = match input_feedback.id {
        | Some(_) => {

            let title = input_feedback.title.as_ref();
            let text = input_feedback.text.as_ref();
            let meta = input_feedback.meta.as_ref();
            let received = input_feedback.received.as_ref();
            let important = input_feedback.important.as_ref();
            let archived = input_feedback.archived.as_ref();

            if title.and(text).and(meta).and(received).and(important).and(archived).is_none() {
                HttpResponse::UnprocessableEntity().finish()
            } else {
                let feedback = services::feedback::update((*db).as_ref().clone(), input_feedback).await?;
                HttpResponse::Ok().json(feedback)
            }


        },
        | None => {
            let feedback = services::feedback::create((*db).as_ref().clone(), input_feedback).await?;
            HttpResponse::Created().json(feedback)
        }
    };

    Ok(response)
}

#[post("/")] // TODO Deprecatea in favor of put
async fn create_handler(db: web::Data<PgPool>, data: Json<FeedbackWeb>) -> Result<impl Responder> {
    let input_feedback = data.into_inner();
    let affected_rows = services::feedback::create((*db).as_ref().clone(), input_feedback).await?;

    Ok(HttpResponse::Created().json(affected_rows))
}

#[delete("/{id}")]
async fn delete_handler(db: web::Data<PgPool>, path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    let affected_rows = services::feedback::delete((*db).as_ref().clone(), id).await?;

    Ok(HttpResponse::Ok().json(affected_rows))
}

