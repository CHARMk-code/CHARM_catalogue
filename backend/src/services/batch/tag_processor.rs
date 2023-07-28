use std::path::{PathBuf, Path};

use async_trait::async_trait;
use calamine::DataType;

use super::{BatchProcessError, XlsxSheetProcessor, helper_functions::{value_to_i32, value_to_string, value_to_bool, value_to_file_path}};
use crate::{
    routes::tag::{RequiredField, TagWeb},
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
        
        Ok(tag::create(db, row)
            .await
            .map_err(|source| BatchProcessError::ApplyToDatabaseError { source })?)
    }

    fn move_associated_files(file_names: Vec<&str>) -> Result<(), BatchProcessError> {
        todo!()
    }

    fn set_struct_value(
        column_name: &Self::RequiredField,
        value: &DataType,
        row_struct: &mut Self::OutputType,
        required_files: &mut Vec<PathBuf>,
        base_file_path: &Path
    ) -> () {
        match column_name {
            RequiredField::Id => row_struct.id = value_to_i32(value),
            RequiredField::Name => row_struct.name = value_to_string(value),
            RequiredField::Division => row_struct.division = value_to_bool(value),
            RequiredField::Businessarea => row_struct.business_area = value_to_bool(value),
            RequiredField::Lookingfor => row_struct.looking_for = value_to_bool(value),
            RequiredField::Icon => row_struct.icon = value_to_file_path(value, required_files, base_file_path),
            RequiredField::Offering => {row_struct.offering = value_to_bool(value); println!("value from bool: {:?}", value_to_bool(value))},
            RequiredField::Language => row_struct.language = value_to_bool(value),
            RequiredField::Fairarea => row_struct.fair_area = value_to_bool(value),
        };
    }
}
