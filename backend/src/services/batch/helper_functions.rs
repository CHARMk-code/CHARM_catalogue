use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use serde_json::json;
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
