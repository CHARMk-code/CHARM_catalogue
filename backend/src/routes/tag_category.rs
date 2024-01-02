use actix_web::web::Json;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use sqlx::PgPool;

use crate::models::tag_category::TagCategoryWeb;
use crate::services;
use crate::services::auth::AuthedUser;

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
    let tag_categories = services::tag_category::get_all(&db).await?;

    Ok(HttpResponse::Ok().json(tag_categories))
}

#[get("/{id}")]
async fn get_by_id_handler(db: web::Data<PgPool>, path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    let tag_category= services::tag_category::get_by_id(&db, id).await?;

    Ok(HttpResponse::Ok().json(tag))
}

#[put("/")]
async fn update_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    data: Json<TagCategory>,
) -> Result<impl Responder> {
    let input_tag_category = data.into_inner();

    let response = match input_tag_category .id {
        Some(_) => {
            let name = input_tag_category.name.as_ref();

            if name
                .is_none()
            {
                HttpResponse::UnprocessableEntity().finish()
            } else {
                let tag_category= services::tag_category::update(&db, input_tag_category).await?;
                HttpResponse::Ok().json(tag)
            }
        }
        None => {
            let tag_category= services::tag_category::create(&db, &input_tag_category).await?;
            HttpResponse::Created().json(tag)
        }
    };

    Ok(response)
}

#[post("/")] // TODO Deprecatea in favor of put
async fn create_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    data: Json<TagCategory>,
) -> Result<impl Responder> {
    let input_tag_category = data.into_inner();
    let affected_rows = services::tag_category::create(&db, &input_tag_category).await?;

    Ok(HttpResponse::Created().json(affected_rows))
}

#[delete("/{id}")]
async fn delete_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let affected_rows = services::tag_category::delete(&db, id).await?;

    Ok(HttpResponse::Ok().json(affected_rows))
}