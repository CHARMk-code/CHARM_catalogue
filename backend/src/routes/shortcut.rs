use actix_web::web::Json;
use actix_web::{web, get, put, Result, Responder, post, delete, HttpResponse, error};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::errors::MyError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ShortcutDB {
    id: i32,
    name: Option<String>,
    desc: Option<String>,
    link: Option<String>,
    icon: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct ShortcutWeb {
    id: Option<i32>,
    name: Option<String>,
    desc: Option<String>,
    link: Option<String>,
    icon: Option<String>,
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/shortcut")
            .service(get_all_handler)
            .service(get_one_handler)
    );
}

#[get("/")]
async fn get_all_handler(db: web::Data<MySqlPool>) -> Result<impl Responder> {
    let shortcuts = sqlx::query_as!(ShortcutDB, "SELECT * FROM shortcuts")
        .fetch_all(&*db.into_inner())
        .await.map_err(MyError::SQLxError)?;
        
    Ok(Json(shortcuts))
}

#[get("/{id}")]
async fn get_one_handler(db: web::Data<MySqlPool>, path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();

    let shortcut = sqlx::query_as!(ShortcutDB, "SELECT * FROM shortcuts where id= ? ", id)
       .fetch_one(&*db.into_inner())
       .await.map_err(MyError::SQLxError)?;

    Ok(Json(shortcut))
}

#[put("/")]
async fn update_handler(db: web::Data<MySqlPool>, data: Json<ShortcutWeb>) -> Result<impl Responder> {
    match data.id {
        | Some(id) => { 
            let shortcut = sqlx::query_as!(ShortcutDB, "SELECT * FROM shortcuts where id= ? ", id)
                .fetch_one(& *db.into_inner())
                .await.map_err(MyError::SQLxError)?;

            let affected_rows = sqlx::query!("UPDATE shortcuts SET `name` = ?, `desc` = ?, `link` = ?, `icon` = ? where `id` = ?",
                data.name.as_ref().or(shortcut.name.as_ref()).expect("Either updated or original name should exist"),
                data.desc.as_ref().or(shortcut.desc.as_ref()).expect("Either updated or original name should exist"),
                data.link.as_ref().or(shortcut.link.as_ref()).expect("Either updated or original name should exist"),
                data.icon.as_ref().or(shortcut.icon.as_ref()).expect("Either updated or original name should exist"),
                data.id)
                .execute(&*db.into_inner()).await.map_err(MyError::SQLxError)?.rows_affected();

            return Ok(Json(affected_rows.try_into().expect("Should have affected rows")))
        },
        | None => {
            return Ok(Json(create_shortcut(db, data).await?))
        },
    }
}

#[post("/")]
async fn create_handler(db: web::Data<MySqlPool>, data: Json<ShortcutWeb>) -> Result<impl Responder> {
    create_shortcut(db, data)
}

#[delete("/{id}")]
async fn delete_shortcut(db: web::Data<MySqlPool>, path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    let query_result = sqlx::query!("DELETE FROM shortcuts WHERE `id` = ?", id).execute(&*db.into_inner()).await.map_err(MyError::SQLxError)?;
    let affected_rows = query_result.rows_affected();

    Ok(Json(affected_rows))
}

fn is_valid_required_field<T>(val: &Option<T>) -> Result<&T> {
    match val.as_ref() {
        None => Err(error::ErrorUnprocessableEntity("Missing required field")),
        Some(v) => Ok(v)
    }
}

async fn create_shortcut(db: web::Data<MySqlPool>, data: Json<ShortcutWeb>) -> Result<u64> {
    let name = is_valid_required_field(&data.name)?;
    let desc = is_valid_required_field(&data.desc)?;
    let icon = is_valid_required_field(&data.icon)?;
    let link = is_valid_required_field(&data.link)?;
    
    let query_result = sqlx::query!("INSERT INTO shortcuts (`name`, `desc`, `link`, `icon`) VALUES (?, ?, ?, ?)",
        name, desc, link, icon)
        .execute(&*db.into_inner()).await.map_err(MyError::SQLxError)?;

    Ok(query_result.last_insert_id().try_into().expect("Should have id for last insert"))

}
