use std::path::{Path, PathBuf};

use async_trait::async_trait;
use calamine::DataType;
use sqlx::{Pool, Postgres};

use crate::{
    models::layout::{LayoutWeb, RequiredField},
    services::layout,
};

use super::{
    helper_functions::{value_to_bool, value_to_file_path, value_to_i32},
    BatchProcessError, ProcessStage, ProcessedSheets, XlsxSheetProcessor,
};

pub struct LayoutProcessor();

#[async_trait]
impl XlsxSheetProcessor for LayoutProcessor {
    type OutputType = LayoutWeb;
    type RequiredField = RequiredField;

    async fn apply_to_database(
        db: &Pool<Postgres>,
        row: &Self::OutputType,
    ) -> Result<Self::OutputType, BatchProcessError> {
        let mut inserted_row = row.clone();

        inserted_row.id = Some(layout::create(db, row).await.map_err(|source| {
            BatchProcessError::ApplyToDatabaseError {
                source,
                row: format!("{:?}", row),
            }
        })?);

        Ok(inserted_row)
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
            RequiredField::Image => {
                row_struct.image = value_to_file_path(value, required_files, base_file_path)
            }
            RequiredField::Active => row_struct.active = value_to_bool(value),
            RequiredField::Placement => row_struct.placement = value_to_i32(value),
        };
    }

    fn check_foreign_key_deps(_processed_values: &ProcessedSheets) -> Result<(), BatchProcessError> {
        Ok(())
    }

    fn update_foreign_keys<'a>(
        updated_values: &'a mut ProcessedSheets,
        _original_values: &ProcessedSheets,
    ) -> Result<&'a mut ProcessedSheets, BatchProcessError> {
        updated_values.layouts.process_stage = ProcessStage::ForeignKeysUpdated;
        Ok(updated_values)
    }
}
