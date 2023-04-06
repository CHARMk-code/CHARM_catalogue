use actix_web::web::Json;
use actix_web::{web, get, put, Result, Responder, post, delete, HttpResponse, error};
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, MySql, Pool};

use crate::errors::MyError;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ShortcutDB {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub link: String,
    pub icon: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct ShortcutWeb {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub link: Option<String>,
    pub icon: Option<String>,
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
    let input_shortcut = data.into_inner();
    let response = match input_shortcut.id {
        | Some(_) => { 
            let shortcut = update_shortcut((*db).as_ref().clone(), input_shortcut).await?;
            HttpResponse::Ok().json(shortcut)
        },
        | None => {
            let shortcut = create_shortcut((*db).as_ref().clone(), input_shortcut).await?;
            HttpResponse::Created().json(shortcut)
        }
    };

    Ok(response)
}

#[post("/")]
async fn create_handler(db: web::Data<MySqlPool>, data: Json<ShortcutWeb>) -> Result<impl Responder> {
    let input_shortcut = data.into_inner();
    let response = HttpResponse::Created()
        .json(create_shortcut((*db).as_ref().clone(), input_shortcut).await?);

    Ok(response)
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

pub async fn create_shortcut(db: Pool<MySql>, data: ShortcutWeb) -> Result<Json<u64>> {
    let name = is_valid_required_field(&data.name)?;
    let desc = is_valid_required_field(&data.desc)?;
    let icon = is_valid_required_field(&data.icon)?;
    let link = is_valid_required_field(&data.link)?;
    
    let query_result = sqlx::query!("INSERT INTO shortcuts (`name`, `desc`, `link`, `icon`) VALUES (?, ?, ?, ?)",
        name, desc, link, icon)
        .execute(&db).await.map_err(MyError::SQLxError)?;

    Ok(Json(query_result.last_insert_id()))

}

pub async fn update_shortcut(db: Pool<MySql>, data: ShortcutWeb) -> Result<Json<u64>> {
    let id = data.id.expect("Should have id to update");
    
    // In an optimal world we shouldn't need this query 
    // (TODO change the second query to only use the data values that will be updated)
    let shortcut = sqlx::query_as!(ShortcutDB, "SELECT * FROM shortcuts where id= ? ", id)
        .fetch_one(&db)
        .await.map_err(MyError::SQLxError)?;

    let name = data.name.as_ref();
    let desc = data.desc.as_ref();
    let link = data.link.as_ref();
    let icon = data.icon.as_ref();

    if name.or(desc).or(link).or(icon).is_none() {
        return Err(error::ErrorUnprocessableEntity("No field to update"));
    }

    let query_result = sqlx::query!("UPDATE shortcuts SET `name` = ?, `desc` = ?, `link` = ?, `icon` = ? where `id` = ?",
        if name.is_some() {name.unwrap()} else {&shortcut.name}, 
        if desc.is_some() {desc.unwrap()} else {&shortcut.desc}, 
        if link.is_some() {link.unwrap()} else {&shortcut.link}, 
        if icon.is_some() {icon.unwrap()} else {&shortcut.icon}, 
        data.id)
        .execute(&db).await.map_err(MyError::SQLxError)?;

    let affected_rows = query_result.rows_affected();

    return Ok(Json(affected_rows))

}
