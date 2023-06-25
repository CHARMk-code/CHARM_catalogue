use actix_web::web::Json;
use actix_web::{web, get, put, Result, Responder, post, delete, HttpResponse, error};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool};

use crate::errors::MyError;
use crate::services;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct MapWeb {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub image: Option<String>,
    pub reference: Option<i32>
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/map")
            .service(get_all_handler)
            .service(get_by_id_handler)
            .service(update_handler)
            .service(create_handler)
            .service(delete_handler)
    );
}

#[get("/")]
    async fn get_all_handler(db: web::Data<PgPool>) -> Result<impl Responder> {
    let maps = services::map::get_all((*db).as_ref().clone()).await?;

    Ok(HttpResponse::Ok().json(maps))
}

#[get("/{id}")]
async fn get_by_id_handler(db: web::Data<PgPool>, path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    let map = services::map::get_by_id((*db).as_ref().clone(), id).await?;

    Ok(HttpResponse::Ok().json(map))
}

#[put("/")]
async fn update_handler(db: web::Data<PgPool>, data: Json<MapWeb>) -> Result<impl Responder> {
    let input_map = data.into_inner();

    let response = match input_map.id {
        | Some(_) => {

            let name = input_map.name.as_ref();
            let image = input_map.image.as_ref();
            let reference = input_map.reference.as_ref();

            if name.and(image).and(reference).is_none() {
                HttpResponse::UnprocessableEntity().finish()
            } else {
                let map = services::map::update((*db).as_ref().clone(), input_map).await?;
                HttpResponse::Ok().json(map)
            }


        },
        | None => {
            let map = services::map::create((*db).as_ref().clone(), input_map).await?;
            HttpResponse::Created().json(map)
        }
    };

    Ok(response)
}

#[post("/")] // TODO Deprecatea in favor of put
async fn create_handler(db: web::Data<PgPool>, data: Json<MapWeb>) -> Result<impl Responder> {
    let input_map = data.into_inner();
    let affected_rows = services::map::create((*db).as_ref().clone(), input_map).await?;

    Ok(HttpResponse::Created().json(affected_rows))
}

#[delete("/{id}")]
async fn delete_handler(db: web::Data<PgPool>, path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    let affected_rows = services::map::delete((*db).as_ref().clone(), id).await?;

    Ok(HttpResponse::Ok().json(affected_rows))
}

