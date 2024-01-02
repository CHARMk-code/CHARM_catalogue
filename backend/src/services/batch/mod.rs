use actix_web::ResponseError;
use async_trait::async_trait;
use futures::{future::join_all, FutureExt};
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
    models::{
        company::CompanyWeb, layout::LayoutWeb, map::FairMapWeb, prepage::PrepageWeb,
        shortcut::ShortcutWeb, tag::TagWeb,
    },
    services::{
        self,
        batch::{
            check_functions::{check_file_dependencies, check_tag_exist_for_company_tags},
            company_processor::CompanyProcessor,
            layout_processor::LayoutProcessor,
            map_processor::MapProcessor,
            prepage_processor::PrepageProcessor,
            shortcut_processor::ShortcutProcessor,
            tag_category_processor::TagCategoryProcessor,
            tag_processor::TagProcessor,
        },
    },
};

use self::{
    check_functions::{check_file_dependencies, check_foreign_key_deps},
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
mod tag_category_processor;
mod tag_processor;

#[derive(EnumIter, Display, Debug)]
pub enum SheetNames {
    Companies,
    Tags,
    TagCategories,
    Prepages,
    Layouts,
    Maps,
    Shortcuts,
}

#[derive(Default, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ProcessStage {
    #[default]
    NotStarted,
    ForeignKeysUpdated,
    Finished,
}

#[derive(Default, Clone, Debug)]
pub struct ProcessedSheet<T> {
    pub rows: Vec<(T, Vec<PathBuf>)>,
    pub process_stage: ProcessStage,
}

impl<T> ProcessedSheet<T> {
    fn extend(&mut self, additional_rows: ProcessedSheet<T>) -> &Self {
        self.rows.extend(additional_rows.rows);
        self
    }
}

#[derive(Default, Clone, Debug)]
pub struct ProcessedSheets {
    pub companies: ProcessedSheet<CompanyWeb>,
    pub tags: ProcessedSheet<TagWeb>,
    pub prepages: ProcessedSheet<PrepageWeb>,
    pub layouts: ProcessedSheet<LayoutWeb>,
    pub maps: ProcessedSheet<FairMapWeb>,
    pub shortcuts: ProcessedSheet<ShortcutWeb>,
}

impl ProcessedSheets {
    fn extend(&mut self, additional_rows: ProcessedSheets) -> &Self {
        self.companies.extend(additional_rows.companies);
        self.tags.extend(additional_rows.tags);
        self.prepages.extend(additional_rows.prepages);
        self.layouts.extend(additional_rows.layouts);
        self.maps.extend(additional_rows.maps);
        self.shortcuts.extend(additional_rows.shortcuts);
        self
    }

    fn get_min_process_stage(&self) -> &ProcessStage {
        SheetNames::iter().fold(
            &ProcessStage::Finished,
            |min_stage, sheet_name| match sheet_name {
                SheetNames::Companies => std::cmp::min(min_stage, &self.companies.process_stage),
                SheetNames::Tags => std::cmp::min(min_stage, &self.tags.process_stage),
                SheetNames::Prepages => std::cmp::min(min_stage, &self.prepages.process_stage),
                SheetNames::Layouts => std::cmp::min(min_stage, &self.layouts.process_stage),
                SheetNames::Maps => std::cmp::min(min_stage, &self.maps.process_stage),
                SheetNames::Shortcuts => std::cmp::min(min_stage, &self.shortcuts.process_stage),
            },
        )
    }
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
    let mut processed_sheets: ProcessedSheets = ProcessedSheets::default();
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
                let new_processed_sheets = process_xlsx_file(file, upload_path)?;
                processed_sheets.extend(new_processed_sheets);
            }

            // Ignore files without extension
            None => {}

            // if not excel file handle it as such
            _ => {
                println!("{:?}: {:?}", name, name.extension());
                provided_files.push(process_other_file(file, upload_path, storage_path, db).await?)
            }
        };
    }

    //Check that all files for processed_sheets exist
    check_file_dependencies(&processed_sheets, &provided_files)?;

    //Check that all foreign key dependencies are valid
    check_foreign_key_deps(&processed_sheets)?;

    apply_proccessed_sheets_to_db(db, &processed_sheets).await?;

    Ok(())
}

async fn apply_proccessed_sheets_to_db(
    db: &Pool<Postgres>,
    original_processed_sheets: &ProcessedSheets,
) -> Result<(), BatchProcessError> {
    // Helper function to handle ProcessedSheet instead of single row
    async fn apply_processed_sheet<'a, P: XlsxSheetProcessor>(
        db: &Pool<Postgres>,
        processed_sheet: ProcessedSheet<P::OutputType>,
    ) -> Result<ProcessedSheet<P::OutputType>, BatchProcessError> {
        println!("Updating {:?}, process_stage: {:?}", std::any::type_name::<P>(), processed_sheet.process_stage);
        if processed_sheet.process_stage != ProcessStage::ForeignKeysUpdated {
            return Ok(processed_sheet);
        }

        let updated_rows = join_all(processed_sheet.rows.iter().map(|(row, paths)| {
            P::apply_to_database(db, row).then(|new_row_result| async {
                new_row_result.map(|new_row| (new_row, paths.clone()))
            })
        }))
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

        Ok(ProcessedSheet {
            rows: updated_rows,
            process_stage: ProcessStage::Finished,
        })
    }

    async fn apply_processed_sheets(
        db: &Pool<Postgres>,
        processed_sheets: ProcessedSheets,
    ) -> Result<ProcessedSheets, BatchProcessError> {
        let updated_processed_sheets = ProcessedSheets {
            companies: apply_processed_sheet::<CompanyProcessor>(db, processed_sheets.companies)
                .await?,
            tags: apply_processed_sheet::<TagProcessor>(db, processed_sheets.tags).await?,
            prepages: apply_processed_sheet::<PrepageProcessor>(db, processed_sheets.prepages)
                .await?,
            layouts: apply_processed_sheet::<LayoutProcessor>(db, processed_sheets.layouts).await?,
            maps: apply_processed_sheet::<MapProcessor>(db, processed_sheets.maps).await?,
            shortcuts: apply_processed_sheet::<ShortcutProcessor>(db, processed_sheets.shortcuts)
                .await?,
        };
        Ok(updated_processed_sheets)
    }

    fn update_foreign_keys(
        mut processed_sheets: ProcessedSheets,
        original_processed_sheets: &ProcessedSheets,
    ) -> Result<ProcessedSheets, BatchProcessError> {

        SheetNames::iter().try_for_each(|name| match name {
            SheetNames::Companies => CompanyProcessor::update_foreign_keys(
                &mut processed_sheets,
                original_processed_sheets,
            ),
            SheetNames::Tags => {
                TagProcessor::update_foreign_keys(&mut processed_sheets, original_processed_sheets)
            }
            SheetNames::Prepages => PrepageProcessor::update_foreign_keys(
                &mut processed_sheets,
                original_processed_sheets,
            ),
            SheetNames::Layouts => LayoutProcessor::update_foreign_keys(
                &mut processed_sheets,
                original_processed_sheets,
            ),
            SheetNames::Maps => {
                MapProcessor::update_foreign_keys(&mut processed_sheets, original_processed_sheets)
            }
            SheetNames::Shortcuts => ShortcutProcessor::update_foreign_keys(
                &mut processed_sheets,
                original_processed_sheets,
            ),
        })?;

        Ok(processed_sheets)
    }

    async fn apply_to_db_then_update_foreign_keys(
        db: &Pool<Postgres>,
        original_processed_sheets: &ProcessedSheets,
        updated_processed_sheets: ProcessedSheets,
    ) -> Result<ProcessedSheets, BatchProcessError> {
        let after_apply_sheets = apply_processed_sheets(db, updated_processed_sheets).await?;

        update_foreign_keys(after_apply_sheets, original_processed_sheets)
    }

    // Initalize a transaction so we can reset if anything goes wrong.
    let tx = db
        .begin()
        .await
        .map_err(|_| BatchProcessError::InitializeDbTransaction)?;

    let mut updated_processed_sheets = Ok(original_processed_sheets.clone());

    while updated_processed_sheets
        .as_ref()
        .is_ok_and(|processed_sheets| {
            processed_sheets.get_min_process_stage() != &ProcessStage::Finished
        })
    {
        updated_processed_sheets = apply_to_db_then_update_foreign_keys(
            db,
            original_processed_sheets,
            updated_processed_sheets.expect("Already checked to be ok"),
        )
        .await
    }

    match updated_processed_sheets.is_ok() {
        true => tx
            .commit()
            .await
            .map_err(|_| BatchProcessError::CommitDbTransaction)?,
        false => {
            tx.rollback()
                .await
                .map_err(|_| BatchProcessError::RollbackDbTransaction)?;

            updated_processed_sheets?;
        }
    }

    Ok(())
}

fn process_xlsx_file(
    mut file: ZipFile,
    base_file_path: &Path,
) -> Result<ProcessedSheets, BatchProcessError> {
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
        sheets: &'a [(String, Range<DataType>)],
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

    Ok(ProcessedSheets {
        tags: TagProcessor::process_sheet(get_sheet("tags", &sheets)?, "tags", base_file_path)?,
        tag_categories: TagCategoryProcessor::process_sheet(
            get_sheet("tag categories", &sheets)?,
            "tag categories",
            base_file_path,
        )?,
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
        .ok_or(BatchProcessError::FileNameError {
            file_name: file.name().to_string(),
        })?
        .iter();

    let file_name = full_path
        .next_back()
        .ok_or(BatchProcessError::FileNameError {
            file_name: file.name().to_string(),
        })?;

    let parent_dir = full_path
        .next_back()
        .ok_or(BatchProcessError::FileNameError {
            file_name: file.name().to_string(),
        })?;

    let new_path = upload_path
        .join(Path::new(&parent_dir))
        .join(Path::new(&file_name));

    let parent_as_string = parent_dir
        .to_str()
        .ok_or(BatchProcessError::FileNameError {
            file_name: file.name().to_string(),
        })?
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
        vec![upload_path.join(parent_dir).join(file_name)],
        upload_path,
        storage_path,
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
    );
    // checks ids of uploaded sheets. And makes sure there are valid foreign keys. Each sheet
    // processor needs to define it's own checking from a ProcessedValues
    fn check_foreign_key_deps(processed_sheets: &ProcessedSheets) -> Result<(), BatchProcessError>;

    // How should circular dependencies
    fn update_foreign_keys(
        updated_sheets: &mut ProcessedSheets,
        original_sheets: &ProcessedSheets,
    ) -> Result<(), BatchProcessError>;

    fn process_sheet(
        sheet: &Range<DataType>,
        name: &str,
        base_file_path: &Path,
    ) -> Result<ProcessedSheet<Self::OutputType>, BatchProcessError> {
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
        let mut output_rows: ProcessedSheet<Self::OutputType> = ProcessedSheet::default();
        for row in 1..height {
            let mut row_struct = Self::OutputType::default();
            let mut required_files = Vec::new();

            for (col_name, col_index) in valid_headers.iter() {
                let value = sheet.index((row, *col_index));
                Self::set_struct_value(
                    col_name,
                    value,
                    &mut row_struct,
                    &mut required_files,
                    &base_file_path.join(name),
                )
            }
            output_rows.rows.push((row_struct, required_files));
        }

        Ok(output_rows)
    }

    async fn apply_to_database(
        db: &Pool<Postgres>,
        row: &Self::OutputType,
    ) -> Result<Self::OutputType, BatchProcessError>;
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

    #[error("Invalid id specified: \ncolumn {used_key_column:?} in {used_key_sheet:?} must be a subset of ids \nin column {valid_key_column:?} in {valid_key_sheet:?} The following invalid keys were found: {invalid_keys:?}")]
    InvalidForeignKey {
        valid_key_sheet: String,
        valid_key_column: String,
        used_key_sheet: String,
        used_key_column: String,
        invalid_keys: Vec<i32>,
    },

    #[error("The Following files for '{sheet_name:?}' sheet is missing: {missing_files:?} ")]
    MissingRequiredFiles {
        sheet_name: SheetNames,
        missing_files: Vec<PathBuf>,
    },

    #[error(
        "Error retrieving valid name to file '{file_name:?}' inside zip archive. Try renaming it"
    )]
    FileNameError { file_name: String },

    #[error("The file dir (currently {name:?}) must be the same as the sheet using the file")]
    InvalidParentName { name: String },

    #[error("Error creating file from Zippedfile")]
    FileCreationError { source: std::io::Error },

    #[error("Error when handling uploaded file")]
    ImageCreationError { source: actix_web::Error },

    #[error("Error when updating foreign key values in column {key_column:?} of {key_sheet:?} sheet. Contact an administrator for more details")]
    ForeignKeyUpdateFailed {
        key_sheet: String,
        key_column: String,
    },

    #[error("Error when initializing transaction to database")]
    InitializeDbTransaction,

    #[error("Error when committing transaction to database")]
    CommitDbTransaction,

    #[error("Error occured when trying to rollback transaction on database")]
    RollbackDbTransaction,
}

impl ResponseError for BatchProcessError {}

#[cfg(test)]
mod tests {
    // use super::*;
    //
    // #[test]
    // fn update_tags_ids_should_update_company_tag_values() -> Result<(), BatchProcessError> {
    //     let processed_sheets = ProcessedSheets {
    //         companies: ProcessedSheet {
    //             rows: vec![
    //                 (
    //                     CompanyWeb {
    //                         tags: Some(vec![1, 2]),
    //                         ..CompanyWeb::default()
    //                     },
    //                     Vec::new(),
    //                 ),
    //                 (
    //                     CompanyWeb {
    //                         tags: Some(vec![1, 2]),
    //                         ..CompanyWeb::default()
    //                     },
    //                     Vec::new(),
    //                 ),
    //             ],
    //             process_stage: ProcessStage::NotStarted,
    //         },
    //         tags: ProcessedSheet {
    //             rows: vec![
    //                 (
    //                     TagWeb {
    //                         id: Some(1),
    //                         ..TagWeb::default()
    //                     },
    //                     vec![],
    //                 ),
    //                 (
    //                     TagWeb {
    //                         id: Some(2),
    //                         ..TagWeb::default()
    //                     },
    //                     vec![],
    //                 ),
    //             ],
    //             process_stage: ProcessStage::NotStarted,
    //         },
    //         prepages: ProcessedSheet::default(),
    //         layouts: ProcessedSheet::default(),
    //         maps: ProcessedSheet::default(),
    //         shortcuts: ProcessedSheet::default(),
    //     };
    //     let new_tag_ids = vec![4, 8];
    //
    //     let updated_processed_sheets = update_tag_ids(&processed_sheets, &new_tag_ids);
    //
    //     assert_eq!(
    //         updated_processed_sheets
    //             .companies
    //             .rows
    //             .iter()
    //             .map(|(c, _)| c.tags.clone().unwrap())
    //             .collect::<Vec<Vec<i32>>>(),
    //         vec![vec![4, 8], vec![4, 8]],
    //         "Tags should have beed updated in companies"
    //     );
    //
    //     Ok(())
    // }

    //     #[test]
    //     fn update_tags_ids_should_update_tag_id_values() -> Result<(), BatchProcessError> {
    //         let processed_sheets = ProcessedSheets {
    //             companies: ProcessedSheet {
    //                 rows: vec![
    //                     (
    //                         CompanyWeb {
    //                             tags: Some(vec![1, 3]),
    //                             ..CompanyWeb::default()
    //                         },
    //                         Vec::new(),
    //                     ),
    //                     (
    //                         CompanyWeb {
    //                             tags: Some(vec![1, 2]),
    //                             ..CompanyWeb::default()
    //                         },
    //                         Vec::new(),
    //                     ),
    //                 ],
    //                 process_stage: ProcessStage::NotStarted,
    //             },
    //             tags: ProcessedSheet {
    //                 rows: vec![
    //                     (
    //                         TagWeb {
    //                             id: Some(1),
    //                             ..TagWeb::default()
    //                         },
    //                         vec![],
    //                     ),
    //                     (
    //                         TagWeb {
    //                             id: Some(2),
    //                             ..TagWeb::default()
    //                         },
    //                         vec![],
    //                     ),
    //                     (
    //                         TagWeb {
    //                             id: Some(3),
    //                             ..TagWeb::default()
    //                         },
    //                         vec![],
    //                     ),
    //                 ],
    //                 process_stage: ProcessStage::NotStarted,
    //             },
    //             prepages: ProcessedSheet::default(),
    //             layouts: ProcessedSheet::default(),
    //             maps: ProcessedSheet::default(),
    //             shortcuts: ProcessedSheet::default(),
    //         };
    //         let new_tag_ids = vec![4, 8, 7];
    //
    //         let updated_processed_sheets = update_tag_ids(&processed_sheets, &new_tag_ids);
    //
    //         assert_eq!(
    //             updated_processed_sheets
    //                 .tags
    //                 .rows
    //                 .iter()
    //                 .map(|(t, _)| t.id.unwrap())
    //                 .collect::<Vec<i32>>(),
    //             new_tag_ids,
    //             "Tags should have beed updated in companies"
    //         );
    //
    //         Ok(())
    //     }
}
