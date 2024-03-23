use std::path::PathBuf;

use actix_web::{delete, web, HttpResponse, Responder, Result};

use crate::{
    config::Config,
    services::{self, auth::AuthedUser, database::Tenant},
};

pub mod blob;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/settings")
            .configure(blob::routes)
            .service(reset_database),
    );
}

#[delete("/reset")]
async fn reset_database(
    tenant: Tenant,
    _user: AuthedUser,
    config: web::Data<Config>,
) -> Result<impl Responder> {
    let upload_path: PathBuf = config.upload_path.clone().into();
    let storage_path: PathBuf = config.storage_path.clone().into();

    services::settings::reset_database(&tenant.db, &upload_path, &storage_path).await?;
    Ok(HttpResponse::Ok())
}
