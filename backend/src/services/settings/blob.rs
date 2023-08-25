use sqlx::{Pool, Postgres};

use crate::{
    errors::MyError,
    models::blob::{JSONBlobDB, JSONBlobWeb},
    services::is_valid_required_field,
};

pub async fn get_by_name(db: &Pool<Postgres>, name: &str) -> Result<JSONBlobDB, actix_web::Error> {
    let query_result = sqlx::query_as!(JSONBlobDB, "select * from blobs where name = $1", name)
        .fetch_one(db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result)
}

pub(crate) async fn update(
    db: &Pool<Postgres>,
    data: &JSONBlobWeb,
) -> Result<i32, actix_web::Error> {
    let name = is_valid_required_field(&data.name)?;
    let blob = is_valid_required_field(&data.blob)?;

    let query_result = sqlx::query!(
        "UPDATE blobs SET blob = $2 where name = $1 returning id",
        name,
        blob
    )
    .fetch_one(db)
    .await;

    match query_result {
        Err(sqlx::Error::RowNotFound) => create(db, data).await,
        Err(e) => Err(MyError::SQLxError(e).into()),
        Ok(query) => Ok(query.id),
    }
}

pub(crate) async fn create(
    db: &Pool<Postgres>,
    data: &JSONBlobWeb,
) -> Result<i32, actix_web::Error> {
    let name = is_valid_required_field(&data.name)?;
    let blob = is_valid_required_field(&data.blob)?;

    let query_result = sqlx::query!(
        "INSERT INTO blobs (name, blob) values ($1, $2) returning id",
        name,
        blob
    )
    .fetch_one(db)
    .await
    .map_err(MyError::SQLxError)?;

    Ok(query_result.id)
}

pub(crate) async fn delete_by_name(
    db: &Pool<Postgres>,
    name: &str,
) -> Result<u64, actix_web::Error> {
    let query_result = sqlx::query!("DELETE FROM blobs WHERE name = $1", name)
        .execute(db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result.rows_affected())
}
