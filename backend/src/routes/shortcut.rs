use actix_web::web::Json;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};

use crate::{
    models::shortcut::ShortcutWeb,
    services::{self, auth::AuthedUser, database::Tenant},
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/shortcut")
            .service(get_all_handler)
            .service(get_by_id_handler)
            .service(update_handler)
            .service(create_handler)
            .service(delete_handler),
    );
}

#[get("/")]
async fn get_all_handler(tenant: Tenant) -> Result<impl Responder> {
    let shortcuts = services::shortcut::get_all(&tenant.db).await?;

    Ok(HttpResponse::Ok().json(shortcuts))
}

#[get("/{id}")]
async fn get_by_id_handler(tenant: Tenant, path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    let shortcut = services::shortcut::get_by_id(&tenant.db, id).await?;

    Ok(HttpResponse::Ok().json(shortcut))
}

#[put("/")]
async fn update_handler(
    _user: AuthedUser,
    tenant: Tenant,
    data: Json<ShortcutWeb>,
) -> Result<impl Responder> {
    let input_shortcut = data.into_inner();

    let response = match input_shortcut.id {
        Some(_) => {
            let name = input_shortcut.name.as_ref();
            let description = input_shortcut.description.as_ref();
            let link = input_shortcut.link.as_ref();
            let icon = input_shortcut.icon.as_ref();

            if name.and(description).and(link).and(icon).is_none() {
                HttpResponse::UnprocessableEntity().finish()
            } else {
                let shortcut = services::shortcut::update(&tenant.db, &input_shortcut).await?;
                HttpResponse::Ok().json(shortcut)
            }
        }
        None => {
            let shortcut = services::shortcut::create(&tenant.db, &input_shortcut).await?;
            HttpResponse::Created().json(shortcut)
        }
    };

    Ok(response)
}

#[post("/")] // TODO Deprecatea in favor of put
async fn create_handler(
    _user: AuthedUser,
    tenant: Tenant,
    data: Json<ShortcutWeb>,
) -> Result<impl Responder> {
    let input_shortcut = data.into_inner();
    let affected_rows = services::shortcut::create(&tenant.db, &input_shortcut).await?;

    Ok(HttpResponse::Created().json(affected_rows))
}

#[delete("/{id}")]
async fn delete_handler(
    _user: AuthedUser,
    tenant: Tenant,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let affected_rows = services::shortcut::delete(&tenant.db, id).await?;

    Ok(HttpResponse::Ok().json(affected_rows))
}
