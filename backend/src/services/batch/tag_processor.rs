use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

use async_trait::async_trait;
use calamine::DataType;

use super::{
    helper_functions::{value_to_file_path, value_to_i32, value_to_string},
    BatchProcessError, ProcessStage, ProcessedSheets, XlsxSheetProcessor,
};
use crate::{
    get_column_in_sheet, get_id_mapper,
    models::tag::{RequiredField, TagWeb},
    services::tag,
    update_id_values,
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
    ) -> Result<Self::OutputType, BatchProcessError> {
        let mut inserted_row = row.clone();

        inserted_row.id = Some(tag::create(db, row).await.map_err(|source| {
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
            RequiredField::Icon => {
                row_struct.icon = value_to_file_path(value, required_files, base_file_path)
            }
            RequiredField::Category => row_struct.category = value_to_i32(value),
        };
    }

    fn check_foreign_key_deps(processed_values: &ProcessedSheets) -> Result<(), BatchProcessError> {
        println!("{:?}", processed_values.tags);
        println!("{:?}", processed_values.tag_categories);

        let valid_tag_category_ids: HashSet<i32> =
            get_column_in_sheet!(processed_values, tag_categories, id)
                .flatten()
                .collect();

        println!("{:?}", processed_values.tag_categories);

        let used_tag_category_ids: HashSet<i32> =
            get_column_in_sheet!(processed_values, tags, category)
                .flatten()
                .collect();

        if !valid_tag_category_ids.is_superset(&used_tag_category_ids) {
            let invalid_tag_categories: Vec<i32> = used_tag_category_ids
                .difference(&valid_tag_category_ids)
                .cloned()
                .collect();

            return Err(BatchProcessError::InvalidForeignKey {
                valid_key_sheet: "tag_categories".to_string(),
                valid_key_column: "id".to_string(),
                used_key_sheet: "tags".to_string(),
                used_key_column: "category".to_string(),
                available_keys: valid_tag_category_ids.into_iter().collect(),
                invalid_keys: invalid_tag_categories,
            });
        }

        Ok(())
    }

    fn update_foreign_keys(
        updated_values: &mut ProcessedSheets,
        original_values: &ProcessedSheets,
    ) -> Result<(), BatchProcessError> {
        if updated_values.tags.process_stage >= ProcessStage::ForeignKeysUpdated {
            return Ok(());
        }
        if updated_values.tag_categories.process_stage == ProcessStage::Finished {
            // Try updating map_image id to db values
            let tag_category_id_mapper: HashMap<i32, i32> =
                get_id_mapper!(original_values, updated_values, tag_categories, id);
            update_id_values!(tag_category_id_mapper, updated_values, tags, category)?;

            updated_values.tags.process_stage = ProcessStage::ForeignKeysUpdated;
        }

        Ok(())
    }
}
