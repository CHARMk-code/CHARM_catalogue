use actix_web::web::Json;
use actix_web::{delete, get, put, web, HttpResponse, Responder, Result};

use crate::{
    models::feedback::FeedbackWeb,
    services::{self, auth::AuthedUser, database::Tenant},
};

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
async fn get_all_handler(tenant: Tenant) -> Result<impl Responder> {
    let feedback = services::feedback::get_all(tenant.db).await?;

    Ok(HttpResponse::Ok().json(feedback))
}

#[get("/{id}")]
async fn get_by_id_handler(tenant: Tenant, path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    let feedback = services::feedback::get_by_id(tenant.db, id).await?;

    Ok(HttpResponse::Ok().json(feedback))
}

#[put("/admin")]
async fn update_admin_handler(
    _user: AuthedUser,
    tenant: Tenant,
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
                let feedback = services::feedback::update(tenant.db, input_feedback).await?;
                HttpResponse::Ok().json(feedback)
            }
        }
        None => {
            let feedback = services::feedback::create(tenant.db, input_feedback).await?;
            HttpResponse::Created().json(feedback)
        }
    };

    Ok(response)
}

#[put("/user")]
async fn create_user_handler(tenant: Tenant, data: Json<FeedbackWeb>) -> Result<impl Responder> {
    let input_feedback = data.into_inner();
    let feedback = services::feedback::create(tenant.db, input_feedback).await?;
    Ok(HttpResponse::Created().json(feedback))
}

#[delete("/{id}")]
async fn delete_handler(
    _user: AuthedUser,
    tenant: Tenant,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let affected_rows = services::feedback::delete(tenant.db, id).await?;

    Ok(HttpResponse::Ok().json(affected_rows))
}
