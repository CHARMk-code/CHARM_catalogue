use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::errors::MyError;
use crate::routes::map::MapWeb;

use super::is_valid_required_field;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct MapDB {
    pub id: i32,
    pub name: String,
    pub image: String,
    pub reference: i32,
}

pub async fn create(db: Pool<Postgres>, data: MapWeb) -> Result<i32, actix_web::Error> {
    let name = is_valid_required_field(&data.name)?;
    let image = is_valid_required_field(&data.image)?;
    let reference = is_valid_required_field(&data.reference)?;

    let query_result = sqlx::query!(
        "INSERT INTO maps (name, image, reference) VALUES ($1, $2, $3) returning id;",
        name,
        image,
        reference
    )
    .fetch_one(&db)
    .await
    .map_err(MyError::SQLxError)?;

    Ok(query_result.id)
}

pub async fn update(db: Pool<Postgres>, data: MapWeb) -> Result<i32, actix_web::Error> {
    let id = data.id.expect("Should have id to update");

    // In an optimal world we shouldn't need this query
    // (TODO change the second query to only use the data values that will be updated)
    let map = sqlx::query_as!(MapDB, "SELECT * FROM maps where id = $1", id)
        .fetch_one(&db)
        .await
        .map_err(MyError::SQLxError)?;

    let name = data.name.as_ref();
    let image = data.image.as_ref();
    let reference = data.reference.as_ref();

    let query_result = sqlx::query!(
        "UPDATE maps SET name = $1, image = $2, reference = $3 where id = $4 returning id",
        if name.is_some() {
            name.unwrap()
        } else {
            &map.name
        },
        if image.is_some() {
            image.unwrap()
        } else {
            &map.image
        },
        if reference.is_some() {
            reference.unwrap()
        } else {
            &map.reference
        },
        data.id
    )
    .fetch_one(&db)
    .await
    .map_err(MyError::SQLxError)?;

    Ok(query_result.id)
}

pub async fn delete(db: Pool<Postgres>, id: i32) -> Result<u64, actix_web::Error> {
    let query_result = sqlx::query!("DELETE FROM maps WHERE id = $1", id)
        .execute(&db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result.rows_affected())
}

pub async fn get_all(db: Pool<Postgres>) -> Result<Vec<MapDB>, actix_web::Error> {
    let query_result = sqlx::query_as!(MapDB, "SELECT * FROM maps")
        .fetch_all(&db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result)
}

pub async fn get_by_id(db: Pool<Postgres>, id: i32) -> Result<MapDB, actix_web::Error> {
    let query_result = sqlx::query_as!(MapDB, "SELECT * FROM maps where id = $1", id)
        .fetch_one(&db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result)
}
