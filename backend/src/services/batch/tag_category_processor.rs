use std::path::{Path, PathBuf};

use async_trait::async_trait;
use calamine::DataType;

use super::{
    helper_functions::{value_to_i32, value_to_string},
    BatchProcessError, XlsxSheetProcessor, ProcessedSheets, ProcessStage,
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

    async fn apply_to_database(
        db: &Pool<Postgres>,
        row: &Self::OutputType,
    ) -> Result<Self::OutputType, BatchProcessError> {
        let mut inserted_row = row.clone();

        inserted_row.id = Some(tag_category::create(db, row).await.map_err(|source| {
            BatchProcessError::ApplyToDatabaseError {
                source,
                row: format!("{:?}", row),
            }
        })?);

        Ok(inserted_row)
    }

    fn check_foreign_key_deps(
        _processed_values: &ProcessedSheets,
    ) -> Result<(), BatchProcessError> {
        Ok(())
    }

    fn update_foreign_keys(
        updated_values: &mut ProcessedSheets,
        _original_values: &ProcessedSheets,
    ) -> Result<(), BatchProcessError> {
        if updated_values.tag_categories.process_stage >= ProcessStage::ForeignKeysUpdated {
            return Ok(());
        }
        updated_values.tag_categories.process_stage = ProcessStage::ForeignKeysUpdated;
        Ok(())
    }
}
