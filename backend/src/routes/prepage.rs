use actix_web::web::Json;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use sqlx::PgPool;

use crate::models::prepage::PrepageWeb;
use crate::services;
use crate::services::auth::AuthedUser;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/prepage")
            .service(get_all_handler)
            .service(get_by_id_handler)
            .service(update_handler)
            .service(create_handler)
            .service(delete_handler),
    );
}

#[get("/")]
async fn get_all_handler(db: web::Data<PgPool>) -> Result<impl Responder> {
    let prepages = services::prepage::get_all(&db).await?;

    Ok(HttpResponse::Ok().json(prepages))
}

#[get("/{id}")]
async fn get_by_id_handler(db: web::Data<PgPool>, path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    let prepage = services::prepage::get_by_id(&db, id).await?;

    Ok(HttpResponse::Ok().json(prepage))
}

#[put("/")]
async fn update_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    data: Json<PrepageWeb>,
) -> Result<impl Responder> {
    let input_prepage = data.into_inner();

    let response = match input_prepage.id {
        Some(_) => {
            let name = input_prepage.name.as_ref();
            let image = input_prepage.image.as_ref();
            let active = input_prepage.active.as_ref();
            let mobile = input_prepage.mobile.as_ref();
            let side = input_prepage.side.as_ref();
            let page = input_prepage.page.as_ref();

            if name
                .and(image)
                .and(active)
                .and(mobile)
                .and(side)
                .and(page)
                .is_none()
            {
                HttpResponse::UnprocessableEntity().finish()
            } else {
                let prepage = services::prepage::update(&db, &input_prepage).await?;
                HttpResponse::Ok().json(prepage)
            }
        }
        None => {
            let prepage = services::prepage::create(&db, &input_prepage).await?;
            HttpResponse::Created().json(prepage)
        }
    };

    Ok(response)
}

#[post("/")] // TODO: Deprecatea in favor of put
async fn create_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    data: Json<PrepageWeb>,
) -> Result<impl Responder> {
    let input_prepage = data.into_inner();
    let affected_rows = services::prepage::create(&db, &input_prepage).await?;

    Ok(HttpResponse::Created().json(affected_rows))
}

#[delete("/{id}")]
async fn delete_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let affected_rows = services::prepage::delete(&db, id).await?;

    Ok(HttpResponse::Ok().json(affected_rows))
}
