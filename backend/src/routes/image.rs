use std::path::PathBuf;

use actix_multipart::Multipart;
use actix_web::{
    get, post, put,
    web::{self, Json},
    HttpResponse, Responder, Result, delete,
};
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, PgPool};

use crate::services::{self, auth::AuthedUser, file::save_files};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/image").service(get_by_id_handler));
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct FileWeb {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub image: Option<bool>,
}

#[get("/")]
async fn get_by_id_handler(db: web::Data<PgPool>, path: web::Path<Uuid>) -> Result<impl Responder> {
    let id = path.into_inner();
    let image = services::image::get_by_id(&db, &id).await?;

    Ok(HttpResponse::Ok().json(image))
}

#[put("/")]
async fn update_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    data: Json<FileWeb>,
) -> Result<impl Responder> {
    let input_file = data.into_inner();

    let response = match input_file.id {
        Some(_) => {
            let name = input_file.name.as_ref();
            let namespace = input_file.namespace.as_ref();
            let image = input_file.image.as_ref();

            if name.and(namespace).and(image).is_none() {
                HttpResponse::UnprocessableEntity().finish()
            } else {
                let map = services::image::update(&db, input_file).await?;
                HttpResponse::Ok().json(map)
            }
        }
        None => HttpResponse::UnprocessableEntity().finish(),
    };

    Ok(response)
}

#[post("/")]
async fn create_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    payload: Multipart,
) -> Result<impl Responder> {
    let original_path: PathBuf = "upload/".into(); // TODO: Move this into a configuration file
    let final_path: PathBuf = "storage/".into(); // TODO: Move this into a configuration file

    let saved_files = save_files(payload, &original_path).await?;

    let uuids = services::image::create(&db, "images", saved_files, &final_path).await?;

    Ok(HttpResponse::Ok().json(uuids))
}

#[delete("/{id}")]
async fn delete_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let affected_rows = services::image::delete(&db, &id).await?;

    Ok(HttpResponse::Ok().json(affected_rows))
}
