use std::path::{Path, PathBuf};

use async_trait::async_trait;
use calamine::DataType;

use super::{
    helper_functions::{value_to_file_path, value_to_i32, value_to_json, value_to_string},
    BatchProcessError, ProcessStage, ProcessedSheets, XlsxSheetProcessor,
};
use crate::{
    models::map::{FairMapWeb, RequiredField},
    services::map,
};
use sqlx::{Pool, Postgres};

pub struct MapProcessor();

#[async_trait]
impl XlsxSheetProcessor for MapProcessor {
    type OutputType = FairMapWeb;
    type RequiredField = RequiredField;

    async fn apply_to_database(
        db: &Pool<Postgres>,
        row: &Self::OutputType,
    ) -> Result<Self::OutputType, BatchProcessError> {
        let mut inserted_row = row.clone();

        inserted_row.id = Some(map::create(db, row).await.map_err(|source| {
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
            RequiredField::Name => row_struct.name = value_to_string(value),
            RequiredField::Background => {
                row_struct.background = value_to_file_path(value, required_files, base_file_path)
            }
            RequiredField::MapData => row_struct.map_data = value_to_json(value),
        };
    }

    fn check_foreign_key_deps(
        _processed_values: &ProcessedSheets,
    ) -> Result<(), BatchProcessError> {
        Ok(())
    }

    fn update_foreign_keys<'a>(
        updated_values: &'a mut ProcessedSheets,
        _original_values: &ProcessedSheets,
    ) -> Result<&'a mut ProcessedSheets, BatchProcessError> {
        updated_values.maps.process_stage = ProcessStage::ForeignKeysUpdated;
        Ok(updated_values)
    }
}
