use std::path::{Path, PathBuf};

use async_trait::async_trait;
use calamine::DataType;

use super::{
    helper_functions::{value_to_i32, value_to_string},
    BatchProcessError, XlsxSheetProcessor,
};
use crate::{
    models::tag_category::{RequiredField, TagCategoryWeb},
    services::tag_category,
};
use sqlx::{Pool, Postgres};

pub struct TagCategoryProcessor();

#[async_trait]
impl XlsxSheetProcessor for TagCategoryProcessor {
    type OutputType = TagCategoryWeb;
    type RequiredField = RequiredField;

    async fn apply_to_database(
        db: &Pool<Postgres>,
        row: &Self::OutputType,
    ) -> Result<i32, BatchProcessError> {
        Ok(tag_category::create(db, row).await.map_err(|source| {
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
        };
    }
}
