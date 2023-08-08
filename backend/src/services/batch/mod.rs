use actix_web::ResponseError;
use async_trait::async_trait;
use futures::future::join_all;
use sqlx::{Pool, Postgres};
use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    fs::File,
    io::{Cursor, Read, Write},
    ops::Index,
    path::{Path, PathBuf},
    str::FromStr,
};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use thiserror::Error;

use calamine::{open_workbook_auto_from_rs, DataType, Range, Reader};
use zip::{read::ZipFile, ZipArchive};

use crate::{
    routes::{
        company::CompanyWeb, layout::LayoutWeb, map::MapWeb, prepage::PrepageWeb,
        shortcut::ShortcutWeb, tag::TagWeb,
    },
    services::{
        self,
        batch::{
            layout_processor::LayoutProcessor, map_processor::MapProcessor,
            prepage_processor::PrepageProcessor, shortcut_processor::ShortcutProcessor,
        },
    },
};

use self::{
    check_functions::{check_file_dependencies, check_tag_exist_for_company_tags},
    company_processor::CompanyProcessor,
    tag_processor::TagProcessor,
};

pub mod check_functions;
mod company_processor;
mod helper_functions;
mod layout_processor;
mod map_processor;
mod prepage_processor;
mod shortcut_processor;
mod tag_processor;

#[derive(EnumIter, Display, Debug)]
pub enum SheetNames {
    Companies,
    Tags,
    Prepages,
    Layouts,
    Maps,
    Shortcuts,
}

#[derive(Default, Clone, Debug)]
pub struct ProcessedValues {
    pub companies: Vec<(CompanyWeb, Vec<PathBuf>)>,
    pub tags: Vec<(TagWeb, Vec<PathBuf>)>,
    pub prepages: Vec<(PrepageWeb, Vec<PathBuf>)>,
    pub layouts: Vec<(LayoutWeb, Vec<PathBuf>)>,
    pub maps: Vec<(MapWeb, Vec<PathBuf>)>,
    pub shortcuts: Vec<(ShortcutWeb, Vec<PathBuf>)>,
}

pub async fn process_batch_zip(
    db: &Pool<Postgres>,
    zip_path: &Path,
    upload_path: &Path,
    storage_path: &Path,
) -> Result<(), BatchProcessError> {
    //Read the ZipArcive
    let zip_reader =
        File::open(zip_path).map_err(|source| BatchProcessError::FileError { source })?;
    let mut zip_archive =
        ZipArchive::new(zip_reader).map_err(|source| BatchProcessError::ZipError { source })?;

    //Setup values to populate from the excel files contained in the zip file
    let mut processed_values: ProcessedValues = ProcessedValues::default();
    let mut provided_files: Vec<PathBuf> = Vec::new();

    // Loop through all the files in the zipArchive
    for i in 0..zip_archive.len() {
        let file = zip_archive
            .by_index(i)
            .map_err(|source| BatchProcessError::ZipError { source })?;
        let name = file.enclosed_name().unwrap();

        match name.extension().and_then(|s| s.to_str()) {
            // If the file is an excel file process it as such
            Some("xlsx") | Some("xlsm") | Some("xlsb") | Some("xls") => {
                let new_processed_values = process_xlsx_file(file, upload_path)?;
                processed_values = ProcessedValues {
                    companies: [processed_values.companies, new_processed_values.companies]
                        .concat(),
                    tags: [processed_values.tags, new_processed_values.tags].concat(),
                    layouts: [processed_values.layouts, new_processed_values.layouts].concat(),
                    maps: [processed_values.maps, new_processed_values.maps].concat(),
                    prepages: [processed_values.prepages, new_processed_values.prepages].concat(),
                    shortcuts: [processed_values.shortcuts, new_processed_values.shortcuts]
                        .concat(),
                }
            }
            // if not excel file handle it as such
            _ => {
                provided_files.push(process_other_file(file, upload_path, storage_path, db).await?)
            }
        };
    }

    //Check that all files for processed_values exist
    check_file_dependencies(&processed_values, &provided_files)?;

    //Check internal table dependencies
    check_tag_exist_for_company_tags(&processed_values)?;

    apply_proccessed_values_to_db(db, &processed_values).await?;

    Ok(())
}

async fn apply_proccessed_values_to_db(
    db: &Pool<Postgres>,
    processed_values: &ProcessedValues,
) -> Result<(), BatchProcessError> {
    async fn apply_vec_to_database<'a, P: XlsxSheetProcessor>(
        db: &Pool<Postgres>,
        rows: &Vec<(P::OutputType, Vec<PathBuf>)>,
    ) -> Result<Vec<i32>, BatchProcessError> {
        join_all(rows.iter().map(|(row, _)| P::apply_to_database(db, row)))
            .await
            .into_iter()
            .collect()
    }
    let tag_ids = apply_vec_to_database::<TagProcessor>(db, &processed_values.tags).await?;

    let updated_processed_values = update_tag_ids(processed_values, &tag_ids);

    apply_vec_to_database::<CompanyProcessor>(db, &updated_processed_values.companies).await?;

    apply_vec_to_database::<LayoutProcessor>(db, &updated_processed_values.layouts).await?;
    apply_vec_to_database::<MapProcessor>(db, &updated_processed_values.maps).await?;
    apply_vec_to_database::<PrepageProcessor>(db, &updated_processed_values.prepages).await?;
    apply_vec_to_database::<ShortcutProcessor>(db, &updated_processed_values.shortcuts).await?;

    Ok(())
}

fn update_tag_ids(processed_values: &ProcessedValues, new_tag_ids: &Vec<i32>) -> ProcessedValues {
    let tag_id_mapper: HashMap<i32, i32> = HashMap::from_iter(
        processed_values
            .tags
            .clone()
            .iter()
            .filter_map(|(tag, _)| tag.id)
            .zip(new_tag_ids.iter().cloned()),
    );
    let updated_processed_values = ProcessedValues {
        companies: processed_values
            .companies
            .iter()
            .map(|(c, f)| {
                (
                    CompanyWeb {
                        tags: match &c.tags {
                            None => None,
                            Some(cts) => Some(
                                cts.iter()
                                    .map(|ct| tag_id_mapper.get(ct).unwrap().clone())
                                    .collect(),
                            ),
                        },
                        ..c.clone()
                    },
                    f.clone(),
                )
            })
            .collect(),
        tags: processed_values
            .tags
            .iter()
            .map(|(c, f)| {
                (
                    TagWeb {
                        id: Some(tag_id_mapper.get(&c.id.unwrap()).unwrap().clone()),
                        ..c.clone()
                    },
                    f.clone(),
                )
            })
            .collect(),
        ..processed_values.clone()
    };

    updated_processed_values
}

fn process_xlsx_file(
    mut file: ZipFile,
    base_file_path: &Path,
) -> Result<ProcessedValues, BatchProcessError> {
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)
        .map_err(|source| BatchProcessError::ZipIoError {
            source: zip::result::ZipError::Io(source),
        })?;
    let cursor = Cursor::new(buf);

    let sheets = open_workbook_auto_from_rs(cursor)
        .map_err(|source| BatchProcessError::CalamineError { source })?
        .worksheets();

    fn get_sheet<'a>(
        name: &str,
        sheets: &'a Vec<(String, Range<DataType>)>,
    ) -> Result<&'a Range<DataType>, BatchProcessError> {
        sheets
            .iter()
            .filter(|(sheet_name, _)| sheet_name.as_str().to_lowercase() == name)
            .map(|(_, range)| range)
            .next()
            .ok_or_else(|| BatchProcessError::MissingSheetError {
                name: name.to_string(),
            })
    }

    Ok(ProcessedValues {
        tags: TagProcessor::process_sheet(get_sheet("tags", &sheets)?, "tags", base_file_path)?,
        companies: CompanyProcessor::process_sheet(
            get_sheet("companies", &sheets)?,
            "companies",
            base_file_path,
        )?,
        prepages: PrepageProcessor::process_sheet(
            get_sheet("prepages", &sheets)?,
            "prepages",
            base_file_path,
        )?,
        layouts: LayoutProcessor::process_sheet(
            get_sheet("layouts", &sheets)?,
            "layouts",
            base_file_path,
        )?,
        maps: MapProcessor::process_sheet(get_sheet("maps", &sheets)?, "maps", base_file_path)?,
        shortcuts: ShortcutProcessor::process_sheet(
            get_sheet("shortcuts", &sheets)?,
            "shortcuts",
            base_file_path,
        )?,
    })
}

// TODO: Create a table that associates files with references (or maybe) just a table.
// would be good to have in order to clean unused files so we don't just accumulate old_files.
async fn process_other_file(
    mut file: ZipFile<'_>,
    upload_path: &Path,
    storage_path: &Path,
    db: &Pool<Postgres>,
) -> Result<PathBuf, BatchProcessError> {
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)
        .map_err(|source| BatchProcessError::ZipIoError {
            source: zip::result::ZipError::Io(source),
        })?;

    let mut full_path = file
        .enclosed_name()
        .ok_or(BatchProcessError::FileNameError)?
        .iter();

    let file_name = full_path
        .next_back()
        .ok_or(BatchProcessError::FileNameError)?;

    let parent_dir = full_path
        .next_back()
        .ok_or(BatchProcessError::FileNameError)?;

    let new_path = upload_path
        .join(Path::new(&parent_dir))
        .join(Path::new(&file_name));

    let parent_as_string = parent_dir
        .to_str()
        .ok_or(BatchProcessError::FileNameError)?
        .to_lowercase();

    // Error if parent is not the name of a sheet
    if SheetNames::iter()
        .all(|sheet_name| sheet_name.to_string().as_str().to_lowercase() != parent_as_string)
    {
        return Err(BatchProcessError::InvalidParentName {
            name: parent_as_string,
        });
    };

    // Temproarily save the file in the upload path
    let mut directory_path = new_path.clone();
    directory_path.pop();
    std::fs::create_dir_all(directory_path)
        .map_err(|source| BatchProcessError::FileCreationError { source })?;
    let mut f = std::fs::File::create(new_path.clone())
        .map_err(|source| BatchProcessError::FileCreationError { source })?;

    f.write_all(&buf)
        .map_err(|source| BatchProcessError::FileCreationError { source })?;

    services::image::create(
        db,
        "images",
        vec![Path::new(parent_dir).join(file_name)],
        &upload_path.to_path_buf(),
        &storage_path.to_path_buf(),
    )
    .await
    .map_err(|source| BatchProcessError::ImageCreationError { source })?;

    Ok(new_path)
}

#[async_trait]
trait XlsxSheetProcessor {
    type OutputType: Default + std::fmt::Debug;
    type RequiredField: IntoEnumIterator
        + FromStr
        + Display
        + Eq
        + std::hash::Hash
        + std::fmt::Debug;

    fn set_struct_value(
        column_name: &Self::RequiredField,
        value: &DataType,
        row_struct: &mut Self::OutputType,
        required_files: &mut Vec<PathBuf>,
        base_file_path: &Path,
    ) -> ();

    fn process_sheet(
        sheet: &Range<DataType>,
        name: &str,
        base_file_path: &Path,
    ) -> Result<Vec<(Self::OutputType, Vec<PathBuf>)>, BatchProcessError> {
        // (height, width)
        let height = sheet.height();
        let width = u32::try_from(sheet.width()).expect("Conversion from usize to u32 to work");

        let sheet_first_row = sheet.range((0, 0), (0, width));
        let header_row = sheet_first_row.cells().map(|(_, x, data)| {
            let formatted_header = format_header_string(data);
            (x, formatted_header)
        });
        let mut valid_headers: HashMap<Self::RequiredField, usize> = HashMap::new();
        header_row.for_each(|(x, h)| match Self::RequiredField::from_str(&h).ok() {
            None => (),
            Some(field) => {
                valid_headers.entry(field).or_insert(x);
            }
        });

        // Error if any required header is missing
        if Self::RequiredField::iter().any(|v| !valid_headers.contains_key(&v)) {
            let provided: Vec<String> = valid_headers.keys().map(|h| h.to_string()).collect();
            let needed: Vec<String> = Self::RequiredField::iter().map(|h| h.to_string()).collect();
            let missing = needed
                .iter()
                .cloned()
                .collect::<HashSet<String>>()
                .difference(&provided.iter().cloned().collect::<HashSet<String>>())
                .cloned()
                .collect();
            return Err(BatchProcessError::SheetMissingReqFields {
                name: name.to_string(),
                provided,
                needed,
                missing,
            });
        }

        // Check for duplicates
        if valid_headers.len()
            < Self::RequiredField::iter()
                .collect::<Vec<Self::RequiredField>>()
                .len()
        {
            return Err(BatchProcessError::DuplicateSheetHeaders {
                headers: valid_headers.keys().map(|h| h.to_string()).collect(),
            });
        }

        // Iterate through all values and add them correctly to output_rows
        let mut output_rows: Vec<(Self::OutputType, Vec<PathBuf>)> = Vec::new();
        for row in 1..height {
            let mut row_struct = Self::OutputType::default();
            let mut required_files = Vec::new();

            for (col_name, col_index) in valid_headers.iter() {
                let value = sheet.index((row, col_index.clone()));
                Self::set_struct_value(
                    &col_name,
                    value,
                    &mut row_struct,
                    &mut required_files,
                    &base_file_path.join(name),
                )
            }
            output_rows.push((row_struct, required_files));
        }

        Ok(output_rows)
    }

    async fn apply_to_database(
        db: &Pool<Postgres>,
        row: &Self::OutputType,
    ) -> Result<i32, BatchProcessError>;
}

fn format_header_string(data: &DataType) -> String {
    match data.as_string() {
        None => "".to_string(),
        Some(mut string) => {
            // Remove all '_', '-' and whitespaces
            string.retain(|c| !(c.is_whitespace() || c == '_' || c == '-'));

            // Capitalize only first letter
            let (first, remainder) = string.split_at_mut(1);
            first.make_ascii_uppercase();
            remainder.make_ascii_lowercase();

            string
        }
    }
}

#[derive(Error, Debug)]
pub enum BatchProcessError {
    #[error("Error when reading zip file")]
    FileError { source: std::io::Error },

    #[error("Error when parsing zip file")]
    ZipError { source: zip::result::ZipError },

    #[error("Error when parsing zip file")]
    ZipIoError { source: zip::result::ZipError },

    #[error("Wrapped excel (calamine) error")]
    CalamineError { source: calamine::Error },

    #[error("Sheet has duplicate headers: {headers:?}")]
    DuplicateSheetHeaders { headers: Vec<String> },

    #[error("Missing sheet with name: {name:?}")]
    MissingSheetError { name: String },

    #[error(
        "Missing Required fields for {name:?} sheet\nprovided: {provided:?}\nrequired: {needed:?} \nmissing: {missing:?}"
    )]
    SheetMissingReqFields {
        name: String,
        provided: Vec<String>,
        needed: Vec<String>,
        missing: Vec<String>,
    },

    #[error("Could not apply new row to database. Row: {row:?}")]
    ApplyToDatabaseError {
        source: actix_web::Error,
        row: String,
    },

    #[error("Missing Id in row: {value:?}")]
    MissingIdError { value: String },

    #[error("Missing tag ids ({tag_ids:?}) for company tags")]
    MissingTagIdsForCompanyTags { tag_ids: Vec<i32> },

    #[error("The Following files for '{sheet_name:?}' sheet is missing: {missing_files:?} ")]
    MissingRequiredFiles {
        sheet_name: SheetNames,
        missing_files: Vec<PathBuf>,
    },

    #[error("Error retrieving file name inside zip archive")]
    FileNameError,

    #[error("The file dir (currently {name:?}) must be the same as the sheet using the file")]
    InvalidParentName { name: String },

    #[error("Error creating file from Zippedfile")]
    FileCreationError { source: std::io::Error },

    #[error("Error when handling uploaded file")]
    ImageCreationError { source: actix_web::Error },
}

impl ResponseError for BatchProcessError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_tags_ids_should_update_company_tag_values() -> Result<(), BatchProcessError> {
        let processed_values = ProcessedValues {
            companies: vec![
                (
                    CompanyWeb {
                        tags: Some(vec![1, 2]),
                        ..CompanyWeb::default()
                    },
                    Vec::new(),
                ),
                (
                    CompanyWeb {
                        tags: Some(vec![1, 2]),
                        ..CompanyWeb::default()
                    },
                    Vec::new(),
                ),
            ],
            tags: vec![
                (
                    TagWeb {
                        id: Some(1),
                        ..TagWeb::default()
                    },
                    vec![],
                ),
                (
                    TagWeb {
                        id: Some(2),
                        ..TagWeb::default()
                    },
                    vec![],
                ),
            ],
            prepages: vec![],
            layouts: vec![],
            maps: vec![],
            shortcuts: vec![],
        };
        let new_tag_ids = vec![4, 8];

        let updated_processed_values = update_tag_ids(&processed_values, &new_tag_ids);

        assert_eq!(
            updated_processed_values
                .companies
                .iter()
                .map(|(c, _)| c.tags.clone().unwrap())
                .collect::<Vec<Vec<i32>>>(),
            vec![vec![4, 8], vec![4, 8]],
            "Tags should have beed updated in companies"
        );

        Ok(())
    }

    #[test]
    fn update_tags_ids_should_update_tag_id_values() -> Result<(), BatchProcessError> {
        let processed_values = ProcessedValues {
            companies: vec![
                (
                    CompanyWeb {
                        tags: Some(vec![1, 3]),
                        ..CompanyWeb::default()
                    },
                    Vec::new(),
                ),
                (
                    CompanyWeb {
                        tags: Some(vec![1, 2]),
                        ..CompanyWeb::default()
                    },
                    Vec::new(),
                ),
            ],
            tags: vec![
                (
                    TagWeb {
                        id: Some(1),
                        ..TagWeb::default()
                    },
                    vec![],
                ),
                (
                    TagWeb {
                        id: Some(2),
                        ..TagWeb::default()
                    },
                    vec![],
                ),
                (
                    TagWeb {
                        id: Some(3),
                        ..TagWeb::default()
                    },
                    vec![],
                ),
            ],
            prepages: vec![],
            layouts: vec![],
            maps: vec![],
            shortcuts: vec![],
        };
        let new_tag_ids = vec![4, 8, 7];

        let updated_processed_values = update_tag_ids(&processed_values, &new_tag_ids);

        assert_eq!(
            updated_processed_values
                .tags
                .iter()
                .map(|(t, _)| t.id.clone().unwrap())
                .collect::<Vec<i32>>(),
            new_tag_ids,
            "Tags should have beed updated in companies"
        );

        Ok(())
    }
}
