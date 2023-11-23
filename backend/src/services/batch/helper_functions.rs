use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use calamine::DataType;
use chrono::{DateTime, Utc};

pub fn value_to_bool(value: &DataType) -> Option<bool> {
    value.get_bool()
}

pub fn value_to_string(value: &DataType) -> Option<String> {
    value.get_string().map(|str| str.to_string())
}

pub fn value_to_i32(value: &DataType) -> Option<i32> {
    value.as_i64().map(|v| v as i32)
}

pub fn value_to_json(value: &DataType) -> Option<serde_json::Value> {
    serde_json::from_str(value.to_string().as_str()).ok()?
}

pub fn value_to_file_path(
    value: &DataType,
    required_files: &mut Vec<PathBuf>,
    base_file_path: &Path,
) -> Option<String> {
    match value.get_string() {
        None => None,
        Some(str) => {
            let mut path = base_file_path.clone().to_path_buf();
            path.push(str);
            required_files.push(path);
            Some(str.to_string())
        }
    }
}

pub fn value_to_vec<T: FromStr>(value: &DataType) -> Option<Vec<T>> {
    value.as_string().map(|str| {
        str.split(',')
            .filter_map(|x| x.parse::<T>().ok())
            .collect::<Vec<T>>()
    })
}

pub fn value_to_chrono_date(value: &DataType) -> Option<DateTime<Utc>> {
    value.as_datetime().map(|naive_dt| naive_dt.and_utc())
}


#[macro_export]
macro_rules! get_column_in_sheet {
    ($all_values:expr, $sheet:ident, $column:ident) => {
        $all_values.$sheet.rows.iter().map(|(row, _)| row.$column.clone())
    };
}

#[macro_export]
macro_rules! get_id_mapper {
    ($original_values:expr, $updated_values:expr, $sheet:ident, $column:ident) => {{
        let uploaded_ids = get_column_in_sheet!($original_values, $sheet, $column).flatten();
        let db_ids = get_column_in_sheet!($updated_values, $sheet, $column).flatten();

        uploaded_ids.zip(db_ids).collect()
    }};
}

#[macro_export]
macro_rules! update_id_values {
    ($id_mapper:expr, $updated_values:expr, $sheet:ident, $column:ident) => {
        $updated_values
            .$sheet
            .rows
            .iter_mut()
            .try_for_each(|(value, _)| {
                let original_id =
                    value
                        .$column
                        .ok_or(BatchProcessError::ForeignKeyUpdateFailed {
                            key_sheet: stringify!($sheet).to_string(),
                            key_column: stringify!($column).to_string(),
                        })?;
                let updated_id = $id_mapper.get(&original_id).cloned();
                value.map_image = updated_id;
                Ok(())
            })
    };
}

#[macro_export]
macro_rules! update_vec_id_values {
    ($id_mapper:expr, $updated_values:expr, $sheet:ident, $column:ident) => {
        $updated_values
            .$sheet
            .rows
            .iter_mut()
            .try_for_each(|(value, _)| {
                let original_ids =
                    value
                        .$column 
                        .clone()
                        .ok_or(BatchProcessError::ForeignKeyUpdateFailed {
                            key_sheet: stringify!($sheet).to_string(),
                            key_column: stringify!($column).to_string(),
                        })?;
                let updated_ids: Option<Vec<i32>> = original_ids
                    .iter()
                    .map(|id| $id_mapper.get(id).cloned())
                    .collect();
                value.tags = updated_ids;
                Ok(())
            })
    };
}
