use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Pool};

use crate::errors::MyError;
use crate::routes::prepage::PrepageWeb;

use super::is_valid_required_field;


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct PrepageDB {
    pub id: i32,
    pub name: String,
    pub image: String,
    pub active: bool,
    pub mobile: bool,
    pub side: String,
    pub page: i32
}



pub async fn create(db: Pool<Postgres>, data: PrepageWeb) -> Result<i32, actix_web::Error> {
    let name = is_valid_required_field(&data.name)?;
    let image = is_valid_required_field(&data.image)?;
    let active = is_valid_required_field(&data.active)?;
    let mobile = is_valid_required_field(&data.mobile)?;
    let side = is_valid_required_field(&data.side)?;
    let page = is_valid_required_field(&data.page)?;

    let query_result = sqlx::query!("INSERT INTO prepages (name, image, active, mobile, side, page) VALUES ($1, $2, $3, $4, $5, $6) returning id;",
name, image, active, mobile, side, page)
        .fetch_one(&db).await.map_err(MyError::SQLxError)?;

    Ok(query_result.id)

}

pub async fn update(db: Pool<Postgres>, data: PrepageWeb) -> Result<i32, actix_web::Error> {
    let id = data.id.expect("Should have id to update");

    // In an optimal world we shouldn't need this query
    // (TODO change the second query to only use the data values that will be updated)
    let prepage = sqlx::query_as!(PrepageDB, "SELECT * FROM prepages where id = $1", id)
        .fetch_one(&db)
        .await.map_err(MyError::SQLxError)?;

    let name = data.name.as_ref();
    let image = data.image.as_ref();
    let active = data.active.as_ref();
    let mobile = data.mobile.as_ref();
    let side = data.side.as_ref();
    let page = data.page.as_ref();

    let query_result =
        sqlx::query!("UPDATE prepages SET name = $1, image = $2, active = $3, mobile = $4, side = $5, page = $6 where id = $7 returning id",
                     if name.is_some() {name.unwrap()} else {&prepage.name},
                     if image.is_some() {image.unwrap()} else {&prepage.image},
                     if active.is_some() {active.unwrap()} else {&prepage.active},
                     if mobile.is_some() {mobile.unwrap()} else {&prepage.mobile},
                     if side.is_some() {side.unwrap()} else {&prepage.side},
                     if page.is_some() {page.unwrap()} else {&prepage.page},
                     data.id)
        .fetch_one(&db).await.map_err(MyError::SQLxError)?;

    Ok(query_result.id)
}

pub async fn delete(db: Pool<Postgres>, id: i32) -> Result<u64, actix_web::Error> {
    let query_result = sqlx::query!("DELETE FROM prepages WHERE id = $1", id).execute(&db).await.map_err(MyError::SQLxError)?;

    Ok(query_result.rows_affected())

}

pub async fn get_all(db: Pool<Postgres>) ->  Result<Vec<PrepageDB>, actix_web::Error> {
    let query_result = sqlx::query_as!(PrepageDB, "SELECT * FROM prepages")
        .fetch_all(&db).await.map_err(MyError::SQLxError)?;

    Ok(query_result)
}

pub async fn get_by_id(db: Pool<Postgres>, id: i32) -> Result<PrepageDB, actix_web::Error> {
    let query_result = sqlx::query_as!(PrepageDB, "SELECT * FROM prepages where id = $1", id)
        .fetch_one(&db).await.map_err(MyError::SQLxError)?;

    Ok(query_result)
}
