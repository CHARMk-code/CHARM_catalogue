use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

use async_trait::async_trait;
use calamine::DataType;
use sqlx::{Pool, Postgres};

use super::{
    helper_functions::{
        value_to_bool, value_to_chrono_date, value_to_file_path, value_to_i32, value_to_string,
        value_to_vec,
    },
    BatchProcessError, ProcessStage, ProcessedSheets, XlsxSheetProcessor,
};
use crate::{
    get_column_in_sheet, get_id_mapper,
    models::company::{CompanyWeb, RequiredField},
    services::company,
    update_id_values, update_vec_id_values,
};

pub struct CompanyProcessor();

#[async_trait]
impl XlsxSheetProcessor for CompanyProcessor {
    type OutputType = CompanyWeb;
    type RequiredField = RequiredField;

    fn set_struct_value(
        column_name: &Self::RequiredField,
        value: &DataType,
        row_struct: &mut Self::OutputType,
        required_files: &mut Vec<PathBuf>,
        file_base_path: &Path,
    ) {
        match column_name {
            RequiredField::Active => row_struct.active = value_to_bool(value),
            RequiredField::Charmtalk => row_struct.charmtalk = value_to_bool(value),
            RequiredField::Name => row_struct.name = value_to_string(value),
            RequiredField::Description => row_struct.description = value_to_string(value),
            RequiredField::Uniquesellingpoint => {
                row_struct.unique_selling_point = value_to_string(value)
            }
            RequiredField::Contacts => row_struct.contacts = value_to_string(value),
            RequiredField::Contactemail => row_struct.contact_email = value_to_string(value),
            RequiredField::Employeesworld => row_struct.employees_world = value_to_i32(value),
            RequiredField::Employeessweden => row_struct.employees_sweden = value_to_i32(value),
            RequiredField::Website => row_struct.website = value_to_string(value),
            RequiredField::Talktousabout => row_struct.talk_to_us_about = value_to_string(value),
            RequiredField::Logo => {
                row_struct.logo = value_to_file_path(value, required_files, file_base_path)
            }
            RequiredField::Mapimage => row_struct.map_image = value_to_i32(value),
            RequiredField::Boothnumber => row_struct.booth_number = value_to_i32(value),
            RequiredField::Tags => row_struct.tags = value_to_vec::<i32>(value),
			RequiredField::Imageoffice => row_struct.image_office = value_to_string(value),
			RequiredField::Imageproduct => row_struct.image_product = value_to_string(value),
			RequiredField::Founded => row_struct.founded = value_to_i32(value),
			RequiredField::Officelocation => row_struct.office_location = value_to_string(value),
			RequiredField::Maleboardshare => row_struct.male_board_share = value_to_i32(value),
			RequiredField::Femaleboardshare => row_struct.female_board_share = value_to_i32(value),
			RequiredField::Nonbinaryboardshare => row_struct.nonbinary_board_share = value_to_i32(value),
			RequiredField::Qrlink => row_struct.qr_link = value_to_string(value),
            _ => (),
        };
    }

    async fn apply_to_database(
        db: &Pool<Postgres>,
        row: &Self::OutputType,
    ) -> Result<Self::OutputType, BatchProcessError> {
        let mut inserted_row = row.clone();

        inserted_row.id = Some(company::create(db, row).await.map_err(|source| {
            BatchProcessError::ApplyToDatabaseError {
                source,
                row: format!("{:?}", row),
            }
        })?);

        Ok(inserted_row)
    }

    fn check_foreign_key_deps(processed_values: &ProcessedSheets) -> Result<(), BatchProcessError> {
        // check that map ids referenced in company map_image are a subset of available maps
        let valid_map_ids: HashSet<i32> = get_column_in_sheet!(processed_values, maps, id)
            .flatten()
            .collect();
        let used_map_ids: HashSet<i32> =
            get_column_in_sheet!(processed_values, companies, map_image)
                .flatten()
                .collect();

        if !valid_map_ids.is_superset(&used_map_ids) {
            let invalid_maps: Vec<i32> = used_map_ids
                .difference(&valid_map_ids)
                .cloned()
                .collect();

            return Err(BatchProcessError::InvalidForeignKey {
                valid_key_sheet: "maps".to_string(),
                valid_key_column: "id".to_string(),
                used_key_sheet: "companies".to_string(),
                used_key_column: "map_image".to_string(),
                available_keys: valid_map_ids.into_iter().collect(),
                invalid_keys: invalid_maps,
            });
        }

        // check that tags referenced in companies are a subset of available tags
        let valid_tag_ids: HashSet<i32> = get_column_in_sheet!(processed_values, tags, id)
            .flatten()
            .collect();
        let used_tag_ids: HashSet<i32> = get_column_in_sheet!(processed_values, companies, tags)
            .flatten()
            .flatten()
            .collect();
        if !valid_tag_ids.is_superset(&used_tag_ids) {
            let invalid_tags = used_tag_ids.difference(&valid_tag_ids).cloned().collect();

            return Err(BatchProcessError::InvalidForeignKey {
                valid_key_sheet: "tags".to_string(),
                valid_key_column: "id".to_string(),
                used_key_sheet: "companies".to_string(),
                used_key_column: "tags".to_string(),
                available_keys: valid_tag_ids.into_iter().collect(),
                invalid_keys: invalid_tags,
            });
        }

        Ok(())
    }

    fn update_foreign_keys(
        updated_values: &mut ProcessedSheets,
        original_values: &ProcessedSheets,
    ) -> Result<(), BatchProcessError> {
        if updated_values.companies.process_stage >= ProcessStage::ForeignKeysUpdated {
            return Ok(());
        }

        if updated_values.maps.process_stage == ProcessStage::Finished
            && updated_values.tags.process_stage == ProcessStage::Finished
        {
            // Try updating map_image id to db values
            let map_id_mapper: HashMap<i32, i32> =
                get_id_mapper!(original_values, updated_values, maps, id);
            update_id_values!(map_id_mapper, updated_values, companies, map_image)?;

            println!("map_id_mapper: {:?}", map_id_mapper);

            // Try updating tag ids to db values
            let tag_id_mapper: HashMap<i32, i32> =
                get_id_mapper!(original_values, updated_values, tags, id);

            update_vec_id_values!(tag_id_mapper, updated_values, companies, tags)?;

            updated_values.companies.process_stage = ProcessStage::ForeignKeysUpdated;
        }

        Ok(())
    }
}
