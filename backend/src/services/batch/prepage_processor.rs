use std::path::{Path, PathBuf};

use async_trait::async_trait;
use calamine::DataType;
use sqlx::{Pool, Postgres};

use crate::{
    routes::prepage::{PrepageWeb, RequiredField},
    services::prepage,
};

use super::{
    helper_functions::{value_to_bool, value_to_file_path, value_to_i32, value_to_string},
    BatchProcessError, XlsxSheetProcessor,
};

pub struct PrepageProcessor();

#[async_trait]
impl XlsxSheetProcessor for PrepageProcessor {
    type OutputType = PrepageWeb;
    type RequiredField = RequiredField;

    fn set_struct_value(
        column_name: &Self::RequiredField,
        value: &DataType,
        row_struct: &mut Self::OutputType,
        required_files: &mut Vec<PathBuf>,
        base_file_path: &Path,
    ) -> () {
        match column_name {
            RequiredField::Id => row_struct.id = value_to_i32(value),
            RequiredField::Name => row_struct.name = value_to_string(value),
            RequiredField::Image => {
                row_struct.image = value_to_file_path(value, required_files, base_file_path)
            }
            RequiredField::Active => row_struct.active = value_to_bool(value),
            RequiredField::Mobile => row_struct.mobile = value_to_bool(value),
            RequiredField::Side => row_struct.side = value_to_string(value),
            RequiredField::Page => row_struct.page = value_to_i32(value),
        }
    }

    async fn apply_to_database(
        db: &Pool<Postgres>,
        row: &Self::OutputType,
    ) -> Result<i32, BatchProcessError> {
        Ok(prepage::create(db, row).await.map_err(|source| {
            BatchProcessError::ApplyToDatabaseError {
                source,
                row: format!("{:?}", row),
            }
        })?)
    }
}
