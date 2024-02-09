use actix_web::web::Json;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use sqlx::PgPool;

use crate::models::tag::TagWeb;
use crate::services;
use crate::services::auth::AuthedUser;
use crate::services::database::Tenant;

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
async fn get_all_handler(tenant: Tenant) -> Result<impl Responder> {
    let tags = services::tag::get_all(&tenant.db).await?;

    Ok(HttpResponse::Ok().json(tags))
}

#[get("/{id}")]
async fn get_by_id_handler(db: web::Data<PgPool>, path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    let tag = services::tag::get_by_id(&db, id).await?;

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
            let icon = input_tag.icon.as_ref();
            let category = input_tag.category.as_ref();

            if name
                .and(icon)
                .and(category)
                .is_none()
            {
                HttpResponse::UnprocessableEntity().finish()
            } else {
                let tag = services::tag::update(&db, input_tag).await?;
                HttpResponse::Ok().json(tag)
            }
        }
        None => {
            let tag = services::tag::create(&db, &input_tag).await?;
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
    let affected_rows = services::tag::create(&db, &input_tag).await?;

    Ok(HttpResponse::Created().json(affected_rows))
}

#[delete("/{id}")]
async fn delete_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let affected_rows = services::tag::delete(&db, id).await?;

    Ok(HttpResponse::Ok().json(affected_rows))
}
