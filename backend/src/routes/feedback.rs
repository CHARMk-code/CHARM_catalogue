use actix_web::web::Json;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use sqlx::PgPool;

use crate::models::feedback::FeedbackWeb;
use crate::services;
use crate::services::auth::AuthedUser;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/feedback")
            .service(get_all_handler)
            .service(get_by_id_handler)
            .service(create_user_handler)
            .service(update_admin_handler)
            .service(delete_handler),
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

#[put("/admin")]
async fn update_admin_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    data: Json<FeedbackWeb>,
) -> Result<impl Responder> {
    let input_feedback = data.into_inner();

    let response = match input_feedback.id {
        Some(_) => {
            let title = input_feedback.title.as_ref();
            let text = input_feedback.text.as_ref();
            let meta = input_feedback.meta.as_ref();
            let received = input_feedback.received.as_ref();
            let important = input_feedback.important.as_ref();
            let archived = input_feedback.archived.as_ref();

            if title
                .and(text)
                .and(meta)
                .and(received)
                .and(important)
                .and(archived)
                .is_none()
            {
                HttpResponse::UnprocessableEntity().finish()
            } else {
                let feedback =
                    services::feedback::update((*db).as_ref().clone(), input_feedback).await?;
                HttpResponse::Ok().json(feedback)
            }
        }
        None => {
            let feedback =
                services::feedback::create((*db).as_ref().clone(), input_feedback).await?;
            HttpResponse::Created().json(feedback)
        }
    };

    Ok(response)
}

#[put("/user")]
async fn create_user_handler(
    db: web::Data<PgPool>,
    data: Json<FeedbackWeb>,
) -> Result<impl Responder> {
    let input_feedback = data.into_inner();
    let feedback =
        services::feedback::create((*db).as_ref().clone(), input_feedback).await?;
    Ok(HttpResponse::Created().json(feedback))
}

#[delete("/{id}")]
async fn delete_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let affected_rows = services::feedback::delete((*db).as_ref().clone(), id).await?;

    Ok(HttpResponse::Ok().json(affected_rows))
}
