use std::path::PathBuf;

use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Responder, Result};
use sqlx::PgPool;

use crate::services::{self, auth::AuthedUser};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/batch").service(upload_batchfile));
}

#[post("/")]
async fn upload_batchfile(
    db: web::Data<PgPool>,
    _user: AuthedUser,
    payload: Multipart,
) -> Result<impl Responder> {
    // TODO: Don't allow files to be saved if they have the wrong file_type
    
    let base_path: PathBuf = "upload/".into(); // TODO: Move this into a configuration file
    let file_paths: Vec<PathBuf> = services::file::save_files(payload, &base_path).await?;

    for file_path in file_paths.into_iter() {
        if file_path.extension().unwrap().to_str() == Some("zip") {
            let process_res = services::batch::process_batch_zip(&db, &file_path, &base_path).await;
            process_res?;
        }
        let file_removal_res = services::file::remove_file(file_path.as_path()).await;
        file_removal_res?;
    }

    Ok(HttpResponse::Ok())
}
