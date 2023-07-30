use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::errors::MyError;
use crate::routes::layout::LayoutWeb;

use super::is_valid_required_field;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct LayoutDB {
    pub id: i32,
    pub image: String,
    pub active: bool,
    pub placement: i32,
}

pub async fn create(db: &Pool<Postgres>, data: &LayoutWeb) -> Result<i32, actix_web::Error> {
    let image = is_valid_required_field(&data.image)?;
    let active = is_valid_required_field(&data.active)?;
    let placement = is_valid_required_field(&data.placement)?;

    let query_result = sqlx::query!(
        "INSERT INTO layouts (image, active, placement) VALUES ($1, $2, $3) returning id",
        image,
        active,
        placement
    )
    .fetch_one(db)
    .await
    .map_err(MyError::SQLxError)?;

    Ok(query_result.id)
}

pub async fn update(db: &Pool<Postgres>, data: &LayoutWeb) -> Result<i32, actix_web::Error> {
    let id = data.id.expect("Should have id to update");

    // In an optimal world we shouldn't need this query
    // (TODO change the second query to only use the data values that will be updated)
    let layout = sqlx::query_as!(LayoutDB, "SELECT * FROM layouts where id = $1 ", id)
        .fetch_one(db)
        .await
        .map_err(MyError::SQLxError)?;

    let image = data.image.as_ref();
    let active = data.active.as_ref();
    let placement = data.placement.as_ref();

    let query_result = sqlx::query!(
        "UPDATE layouts SET image = $1, active = $2, placement = $3 where id = $4 returning id",
        if image.is_some() {
            image.unwrap()
        } else {
            &layout.image
        },
        if active.is_some() {
            active.unwrap()
        } else {
            &layout.active
        },
        if placement.is_some() {
            placement.unwrap()
        } else {
            &layout.placement
        },
        data.id
    )
    .fetch_one(db)
    .await
    .map_err(MyError::SQLxError)?;

    Ok(query_result.id)
}

pub async fn delete(db: &Pool<Postgres>, id: i32) -> Result<u64, actix_web::Error> {
    let query_result = sqlx::query!("DELETE FROM layouts WHERE id = $1", id)
        .execute(db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result.rows_affected())
}

pub async fn get_all(db: &Pool<Postgres>) -> Result<Vec<LayoutDB>, actix_web::Error> {
    let query_result = sqlx::query_as!(LayoutDB, "SELECT * FROM layouts")
        .fetch_all(db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result)
}

pub async fn get_by_id(db: &Pool<Postgres>, id: i32) -> Result<LayoutDB, actix_web::Error> {
    let query_result = sqlx::query_as!(LayoutDB, "SELECT * FROM layouts where id = $1 ", id)
        .fetch_one(db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result)
}
