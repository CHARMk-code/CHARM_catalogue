use actix_web::error;
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};

use crate::errors::MyError;
use crate::routes::shortcut::ShortcutWeb;

use super::is_valid_required_field;


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ShortcutDB {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub link: String,
    pub icon: String,
}



pub async fn create(db: Pool<MySql>, data: ShortcutWeb) -> Result<u64, actix_web::Error> {
    let name = is_valid_required_field(&data.name)?;
    let desc = is_valid_required_field(&data.desc)?;
    let icon = is_valid_required_field(&data.icon)?;
    let link = is_valid_required_field(&data.link)?;
    
    let query_result = sqlx::query!("INSERT INTO shortcuts (`name`, `desc`, `link`, `icon`) VALUES (?, ?, ?, ?)",
        name, desc, link, icon)
        .execute(&db).await.map_err(MyError::SQLxError)?;

    Ok(query_result.last_insert_id())

}

pub async fn update(db: Pool<MySql>, data: ShortcutWeb) -> Result<u64, actix_web::Error> {
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

    let query_result = sqlx::query!("UPDATE shortcuts SET `name` = ?, `desc` = ?, `link` = ?, `icon` = ? where `id` = ?",
        if name.is_some() {name.unwrap()} else {&shortcut.name}, 
        if desc.is_some() {desc.unwrap()} else {&shortcut.desc}, 
        if link.is_some() {link.unwrap()} else {&shortcut.link}, 
        if icon.is_some() {icon.unwrap()} else {&shortcut.icon}, 
        data.id)
        .execute(&db).await.map_err(MyError::SQLxError)?;

    Ok(query_result.rows_affected())
}

pub async fn delete(db: Pool<MySql>, id: i32) -> Result<u64, actix_web::Error> {
    let query_result = sqlx::query!("DELETE FROM shortcuts WHERE `id` = ?", id).execute(&db).await.map_err(MyError::SQLxError)?;
    
    Ok(query_result.rows_affected())

}

pub async fn get_all(db: Pool<MySql>) ->  Result<Vec<ShortcutDB>, actix_web::Error> {
    let query_result = sqlx::query_as!(ShortcutDB, "SELECT * FROM shortcuts")
        .fetch_all(&db).await.map_err(MyError::SQLxError)?;

    Ok(query_result)
}

pub async fn get_by_id(db: Pool<MySql>, id: i32) -> Result<ShortcutDB, actix_web::Error> {
    let query_result = sqlx::query_as!(ShortcutDB, "SELECT * FROM shortcuts where id= ? ", id)
       .fetch_one(&db).await.map_err(MyError::SQLxError)?;

    Ok(query_result)
}
