use std::{fs, io::Write, path::{PathBuf, Path}};

use actix_multipart::Multipart;
use actix_web::{error::BlockingError, web, ResponseError};
use futures::StreamExt;

#[derive(thiserror::Error, Debug)]
pub enum FileServiceError {
    #[error("MultipartError received when reading incoming file data")]
    MultipartError {
        source: actix_multipart::MultipartError,
    },
    #[error("Blocking error on file creation")]
    BlockingError { source: BlockingError },

    #[error("Error on file creation")]
    FileHandlingError { source: std::io::Error },

    #[error("Error when moving file: {from:?} -> {to:?}")]
    FileMovingError { source: std::io::Error, from: PathBuf, to: PathBuf },
}

// TODO: Possibly add more and better error handling out via the API, this just sends a
// INTERNAL_SERVER_ERROR in the API response
impl ResponseError for FileServiceError {}

pub async fn save_files(
    mut payload: Multipart,
    base_path: &PathBuf,
) -> Result<Vec<PathBuf>, FileServiceError> {
    let mut file_paths = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|source| FileServiceError::MultipartError { source })?;

        let content_type = field.content_disposition();
        let file_path: PathBuf = base_path.join(content_type.get_filename().unwrap());

        file_paths.push(file_path.clone());

        let mut f = web::block(|| std::fs::File::create(file_path))
            .await
            .map_err(|source| FileServiceError::BlockingError { source })?
            .map_err(|source| FileServiceError::FileHandlingError { source })?;

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();

            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f))
                .await
                .map_err(|source| FileServiceError::BlockingError { source })?
                .map_err(|source| FileServiceError::FileHandlingError { source })?;
        }
        println!("FilePath: {:?}", &file_paths.last().unwrap().canonicalize());
    }
    Ok(file_paths)
}

pub async fn move_file(
    old_file_path: PathBuf,
    new_file_path: PathBuf,
) -> Result<(), FileServiceError> {
    std::fs::rename(&old_file_path, &new_file_path)
        .map_err(|source| FileServiceError::FileMovingError { source, from: old_file_path, to: new_file_path })
}

pub async fn remove_file(file_path: &Path) -> Result<(), FileServiceError> {
    fs::remove_file(file_path).map_err(|source| FileServiceError::FileHandlingError { source })
}
