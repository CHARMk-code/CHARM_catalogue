use actix_web::web::Json;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use strum_macros::{EnumIter, EnumString, Display};

use crate::services;
use crate::services::auth::AuthedUser;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct ShortcutWeb {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub link: Option<String>,
    pub icon: Option<String>,
}

impl Default for ShortcutWeb {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            description: Default::default(),
            link: Default::default(),
            icon: Default::default(),
        }
    }
}

#[derive(EnumIter, EnumString, Display, Debug, PartialEq, Eq, Hash)]
pub enum RequiredField {
    Id,
    Name,
    Description,
    Link,
    Icon,
}

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
async fn get_all_handler(db: web::Data<PgPool>) -> Result<impl Responder> {
    let shortcuts = services::shortcut::get_all(&db).await?;

    Ok(HttpResponse::Ok().json(shortcuts))
}

#[get("/{id}")]
async fn get_by_id_handler(db: web::Data<PgPool>, path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    let shortcut = services::shortcut::get_by_id(&db, id).await?;

    Ok(HttpResponse::Ok().json(shortcut))
}

#[put("/")]
async fn update_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
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
                let shortcut =
                    services::shortcut::update(&db, &input_shortcut).await?;
                HttpResponse::Ok().json(shortcut)
            }
        }
        None => {
            let shortcut =
                services::shortcut::create(&db, &input_shortcut).await?;
            HttpResponse::Created().json(shortcut)
        }
    };

    Ok(response)
}

#[post("/")] // TODO Deprecatea in favor of put
async fn create_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    data: Json<ShortcutWeb>,
) -> Result<impl Responder> {
    let input_shortcut = data.into_inner();
    let affected_rows = services::shortcut::create(&db, &input_shortcut).await?;

    Ok(HttpResponse::Created().json(affected_rows))
}

#[delete("/{id}")]
async fn delete_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let affected_rows = services::shortcut::delete(&db, id).await?;

    Ok(HttpResponse::Ok().json(affected_rows))
}
