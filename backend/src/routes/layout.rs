use actix_web::web::Json;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};

use crate::{
    models::layout::LayoutWeb,
    services::{self, auth::AuthedUser, database::Tenant},
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/layout")
            .service(get_all_handler)
            .service(get_by_id_handler)
            .service(update_handler)
            .service(create_handler)
            .service(delete_handler),
    );
}

#[get("/")]
async fn get_all_handler(tenant: Tenant) -> Result<impl Responder> {
    let layouts = services::layout::get_all(&tenant.db).await?;

    Ok(HttpResponse::Ok().json(layouts))
}

#[get("/{id}")]
async fn get_by_id_handler(tenant: Tenant, path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    let layout = services::layout::get_by_id(&tenant.db, id).await?;

    Ok(HttpResponse::Ok().json(layout))
}

#[put("/")]
async fn update_handler(
    _user: AuthedUser,
    tenant: Tenant,
    data: Json<LayoutWeb>,
) -> Result<impl Responder> {
    let input_layout = data.into_inner();

    let response = match input_layout.id {
        Some(_) => {
            let id = input_layout.id.as_ref();
            let image = input_layout.image.as_ref();
            let active = input_layout.active.as_ref();
            let placement = input_layout.placement.as_ref();

            if id.and(image).and(active).and(placement).is_none() {
                HttpResponse::UnprocessableEntity().finish()
            } else {
                let layout = services::layout::update(&tenant.db, &input_layout).await?;
                HttpResponse::Ok().json(layout)
            }
        }
        None => {
            let layout = services::layout::create(&tenant.db, &input_layout).await?;
            HttpResponse::Created().json(layout)
        }
    };

    Ok(response)
}

#[post("/")] // TODO Deprecatea in favor of put
async fn create_handler(
    _user: AuthedUser,
    tenant: Tenant,
    data: Json<LayoutWeb>,
) -> Result<impl Responder> {
    let input_layout = data.into_inner();
    let affected_rows = services::layout::create(&tenant.db, &input_layout).await?;

    Ok(HttpResponse::Created().json(affected_rows))
}

#[delete("/{id}")]
async fn delete_handler(
    _user: AuthedUser,
    tenant: Tenant,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let affected_rows = services::layout::delete(&tenant.db, id).await?;

    Ok(HttpResponse::Ok().json(affected_rows))
}
