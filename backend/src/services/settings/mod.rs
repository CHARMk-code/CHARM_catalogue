use std::path::Path;

use actix_web::HttpResponse;
use sqlx::{query, Pool, Postgres};

use crate::errors::MyError;

pub mod blob;

pub(crate) async fn reset_database(
    db: &Pool<Postgres>,
    upload_path: &Path,
    storage_path: &Path,
) -> Result<(), actix_web::Error> {
    //Remove all files
    let working_dir = std::env::current_dir()?;
    if upload_path.is_absolute() && !upload_path.starts_with(&working_dir) {
        return Err(MyError::FileDeletionError.into());
    }

    if storage_path.is_absolute() && !storage_path.starts_with(&working_dir) {
        return Err(MyError::FileDeletionError.into());
    }

    std::fs::remove_dir_all(upload_path)?;
    std::fs::create_dir(upload_path)?;
    std::fs::remove_dir_all(storage_path)?;
    std::fs::create_dir(storage_path)?;

    // Empty database
    let rows_affected = query!("TRUNCATE blobs, companies, companies_tags, company_cards, files, layouts, maps, prepages, shortcuts, tags, tag_categories")
        .execute(db)
        .await
        .map_err(MyError::SQLxError)?;


    Ok(())
}
