use std::path::{Path, PathBuf};

use async_trait::async_trait;
use calamine::DataType;

use super::{
    helper_functions::{value_to_bool, value_to_file_path, value_to_i32, value_to_string},
    BatchProcessError, XlsxSheetProcessor,
};
use crate::{
    models::tag::{RequiredField, TagWeb},
    services::tag,
};
use sqlx::{Pool, Postgres};

pub struct TagProcessor();

#[async_trait]
impl XlsxSheetProcessor for TagProcessor {
    type OutputType = TagWeb;
    type RequiredField = RequiredField;

    async fn apply_to_database(
        db: &Pool<Postgres>,
        row: &Self::OutputType,
    ) -> Result<i32, BatchProcessError> {
        Ok(tag::create(db, row).await.map_err(|source| {
            BatchProcessError::ApplyToDatabaseError {
                source,
                row: format!("{:?}", row),
            }
        })?)
    }

    fn set_struct_value(
        column_name: &Self::RequiredField,
        value: &DataType,
        row_struct: &mut Self::OutputType,
        required_files: &mut Vec<PathBuf>,
        base_file_path: &Path,
    ) {
        match column_name {
            RequiredField::Id => row_struct.id = value_to_i32(value),
            RequiredField::Name => row_struct.name = value_to_string(value),
            RequiredField::Icon => {
                row_struct.icon = value_to_file_path(value, required_files, base_file_path)
            }
            RequiredField::Category => row_struct.category = value_to_i32(value),
        };
    }
}
