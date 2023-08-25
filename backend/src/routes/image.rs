use std::path::{Path, PathBuf};

use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    HttpResponse, Responder, Result,
};
use sqlx::{types::Uuid, PgPool};

use crate::{
    config::Config,
    models::file::FileWeb,
    services::{self, auth::AuthedUser, file::save_files},
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/image")
            // .service(get_by_uuid_handler)
            .service(get_by_name_handler),
    );
}

#[get("/{name}")]
async fn get_by_name_handler(
    db: web::Data<PgPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> Result<impl Responder> {
    let name = path.into_inner();
    let image_meta = services::image::get_by_name(&db, &name).await?;

    let image_path = Path::new(&config.storage_path).join(image_meta.id.to_string());

    let file = NamedFile::open_async(image_path)
        .await
        .map_err(|e| Into::<actix_web::Error>::into(e))?
        .file()
        .try_clone()?;

    Ok(NamedFile::from_file(file, image_meta.name))
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
                let map = services::image::update(&db, &input_file).await?;
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
    config: web::Data<Config>,
    payload: Multipart,
) -> Result<impl Responder> {
    let upload_path: PathBuf = config.upload_path.clone().into();
    let storage_path: PathBuf = config.storage_path.clone().into();

    let saved_files = save_files(payload, &upload_path).await?;

    let uuids =
        services::image::create(&db, "images", saved_files, &upload_path, &storage_path).await?;

    Ok(HttpResponse::Ok().json(uuids))
}

#[delete("/{id}")]
async fn delete_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    config: web::Data<Config>,
    path: web::Path<Uuid>,
) -> Result<impl Responder> {
    let id = path.into_inner();

    let affected_rows = services::image::delete(&db, &id, Path::new(&config.storage_path)).await?;

    Ok(HttpResponse::Ok().json(affected_rows))
}
