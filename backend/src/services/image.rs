use std::path::{Path, PathBuf};

use sqlx::Pool;
use sqlx::{types::Uuid, Postgres};

use crate::errors::MyError;
use crate::models::file::{FileDB, FileWeb};
use crate::services;

use super::file::move_file;

pub async fn create(
    db: &Pool<Postgres>,
    namespace: &str,
    saved_files: Vec<PathBuf>,
    upload_path: &PathBuf,
    storage_path: &PathBuf,
) -> Result<Vec<Uuid>, actix_web::Error> {
    let mut uuids: Vec<Uuid> = Vec::new();

    for saved_file in saved_files {
        let file_name: String = saved_file
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        let query_result = sqlx::query!(
            "INSERT INTO files (name, namespace, image) VALUES ($1, $2, true) returning id",
            file_name,
            namespace
        )
        .fetch_one(db)
        .await
        .map_err(MyError::SQLxError)?;

        uuids.push(query_result.id);

        move_file(
            upload_path.join(saved_file),
            storage_path.join(query_result.id.to_string()),
        )
        .await?;
    }

    Ok(uuids)
}

pub async fn update(db: &Pool<Postgres>, data: &FileWeb) -> Result<Uuid, actix_web::Error> {
    let id = data.id.expect("Should have id to update");

    // In an optimal world we shouldn't need this query
    // (TODO change the second query to only use the data values that will be updated)
    let queried_file = sqlx::query_as!(FileDB, "SELECT * FROM files where id = $1", id)
        .fetch_one(db)
        .await
        .map_err(MyError::SQLxError)?;

    let name = data.name.as_ref();
    let namespace = data.namespace.as_ref();
    let image = data.image.as_ref();

    let query_result = sqlx::query!(
        "UPDATE files SET name = $1, namespace = $2, image = $3 where id = $4 returning id",
        if name.is_some() {
            name.unwrap()
        } else {
            &queried_file.name
        },
        if namespace.is_some() {
            namespace.unwrap()
        } else {
            &queried_file.namespace
        },
        if image.is_some() {
            image.unwrap()
        } else {
            &queried_file.image
        },
        data.id
    )
    .fetch_one(db)
    .await
    .map_err(MyError::SQLxError)?;

    Ok(query_result.id)
}

pub async fn get_by_id(db: &Pool<Postgres>, id: &Uuid) -> Result<FileDB, actix_web::Error> {
    let query_result = sqlx::query_as!(FileDB, "SELECT * FROM files where id = $1", id)
        .fetch_one(db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result)
}

pub async fn get_by_name(db: &Pool<Postgres>, name: &str) -> Result<FileDB, actix_web::Error> {
    let query_result = sqlx::query_as!(FileDB, "SELECT * FROM files where name = $1", name)
        .fetch_one(db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result)
}

pub async fn get_all(db: &Pool<Postgres>) -> Result<Vec<FileDB>, actix_web::Error> {
    let query_result = sqlx::query_as!(FileDB, "SELECT * FROM files where image = true")
        .fetch_all(db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result)
}

pub async fn delete(
    db: &Pool<Postgres>,
    id: &Uuid,
    storage_path: &Path,
) -> Result<u64, actix_web::Error> {
    services::file::remove_file(&storage_path.join(id.to_string())).await?;

    let query_result = sqlx::query!("DELETE FROM files WHERE id = $1", id)
        .execute(db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result.rows_affected())
}
