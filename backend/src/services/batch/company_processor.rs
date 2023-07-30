use std::path::{Path, PathBuf};

use async_trait::async_trait;
use calamine::DataType;
use sqlx::{Pool, Postgres};

use super::{
    helper_functions::{
        value_to_bool, value_to_chrono_date, value_to_file_path, value_to_i32, value_to_string,
        value_to_vec,
    },
    BatchProcessError, XlsxSheetProcessor,
};
use crate::{
    routes::company::{CompanyWeb, RequiredField},
    services::company,
};

pub struct CompanyProcessor();

#[async_trait]
impl XlsxSheetProcessor for CompanyProcessor {
    type OutputType = CompanyWeb;
    type RequiredField = RequiredField;

    async fn apply_to_database(
        db: &Pool<Postgres>,
        row: &Self::OutputType,
    ) -> Result<i32, BatchProcessError> {
        Ok(company::create(db, row)
            .await
            .map_err(|source| BatchProcessError::ApplyToDatabaseError { source, row: format!("{:?}",row) })?)
    }

    fn move_associated_files(file_names: Vec<&str>) -> Result<(), BatchProcessError> {
        todo!()
    }

    fn set_struct_value(
        column_name: &Self::RequiredField,
        value: &DataType,
        row_struct: &mut Self::OutputType,
        required_files: &mut Vec<PathBuf>,
        file_base_path: &Path,
    ) -> () {
        match column_name {
            RequiredField::Active => row_struct.active = value_to_bool(value),
            RequiredField::Charmtalk => row_struct.charmtalk = value_to_bool(value),
            RequiredField::Name => row_struct.name = value_to_string(value),
            RequiredField::Description => row_struct.description = value_to_string(value),
            RequiredField::Uniquesellingpoint => {
                row_struct.unique_selling_point = value_to_string(value)
            }
            RequiredField::Summerjobdescription => {
                row_struct.summer_job_description = value_to_string(value)
            }
            RequiredField::Summerjoblink => row_struct.summer_job_link = value_to_string(value),
            RequiredField::Summerjobdeadline => {
                row_struct.summer_job_deadline = value_to_chrono_date(value)
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
            _ => (),
        };
    }
}
