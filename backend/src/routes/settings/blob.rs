use actix_web::{
    delete, get, put,
    web::{self, Json},
    HttpResponse, Responder, Result,
};
use sqlx::PgPool;

use crate::{
    models::blob::JSONBlobWeb,
    services::{self, auth::AuthedUser},
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/blob")
            .service(get_by_name_handler)
            .service(update_handler)
            .service(delete_handler),
    );
}

#[get("/{name}")]
async fn get_by_name_handler(
    db: web::Data<PgPool>,
    path: web::Path<String>,
) -> Result<impl Responder> {
    let name = path.into_inner();
    let prepage = services::settings::blob::get_by_name(&db, &name).await?;

    Ok(HttpResponse::Ok().json(prepage))
}

#[put("/")]
async fn update_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    data: Json<JSONBlobWeb>,
) -> Result<impl Responder> {
    let jsonblob_id = services::settings::blob::update(&db, &data.into_inner()).await?;

    Ok(HttpResponse::Ok().json(jsonblob_id))
}

#[delete("/{name}")]
async fn delete_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    path: web::Path<String>,
) -> Result<impl Responder> {
    let name = path.into_inner();
    let affected_rows = services::settings::blob::delete_by_name(&db, &name).await?;

    Ok(HttpResponse::Ok().json(affected_rows))
}
