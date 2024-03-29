use std::path::{Path, PathBuf};

use async_trait::async_trait;
use calamine::DataType;

use super::{
    helper_functions::{value_to_i32, value_to_string},
    BatchProcessError, XlsxSheetProcessor,
};
use crate::{
    models::{shortcut::RequiredField, shortcut::ShortcutWeb},
    services::shortcut,
};
use sqlx::{Pool, Postgres};

pub struct ShortcutProcessor();

#[async_trait]
impl XlsxSheetProcessor for ShortcutProcessor {
    type OutputType = ShortcutWeb;
    type RequiredField = RequiredField;

    async fn apply_to_database(
        db: &Pool<Postgres>,
        row: &Self::OutputType,
    ) -> Result<i32, BatchProcessError> {
        Ok(shortcut::create(db, row).await.map_err(|source| {
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
        _required_files: &mut Vec<PathBuf>,
        _base_file_path: &Path,
    ) {
        match column_name {
            RequiredField::Id => row_struct.id = value_to_i32(value),
            RequiredField::Name => row_struct.name = value_to_string(value),
            RequiredField::Description => row_struct.description = value_to_string(value),
            RequiredField::Link => row_struct.link = value_to_string(value),
            RequiredField::Icon => row_struct.icon = value_to_string(value),
        };
    }
}
