use std::{collections::HashSet, path::PathBuf};

use strum::IntoEnumIterator;

use crate::services::batch::SheetNames;

use super::{
    company_processor::CompanyProcessor, layout_processor::LayoutProcessor,
    map_processor::MapProcessor, prepage_processor::PrepageProcessor,
    shortcut_processor::ShortcutProcessor, tag_processor::TagProcessor, BatchProcessError,
    ProcessedSheets, XlsxSheetProcessor, tag_category_processor::TagCategoryProcessor,
};

pub fn check_file_dependencies(
    processed_values: &ProcessedSheets,
    provided_files: &[PathBuf],
) -> Result<(), BatchProcessError> {
    fn get_missing_files<T>(
        values: &[(T, Vec<PathBuf>)],
        provided_files_set: &HashSet<PathBuf>,
    ) -> Vec<PathBuf> {
        let required_files_set = values
            .iter()
            .flat_map(|(_, req_files)| req_files)
            .cloned()
            .collect::<HashSet<PathBuf>>();
        required_files_set
            .difference(provided_files_set)
            .cloned()
            .collect()
    }

    let provided_files_set: HashSet<PathBuf> = provided_files.iter().cloned().collect();

    SheetNames::iter()
        .map(|sheet_name| match sheet_name {
            SheetNames::Companies => (
                sheet_name,
                get_missing_files(&processed_values.companies.rows, &provided_files_set),
            ),
            SheetNames::Tags => (
                sheet_name,
                get_missing_files(&processed_values.tags.rows, &provided_files_set),
            ),
            SheetNames::TagCategories => (
                sheet_name,
                get_missing_files(&processed_values.tag_categories.rows, &provided_files_set),
            ),
            SheetNames::Prepages => (
                sheet_name,
                get_missing_files(&processed_values.prepages.rows, &provided_files_set),
            ),
            SheetNames::Layouts => (
                sheet_name,
                get_missing_files(&processed_values.layouts.rows, &provided_files_set),
            ),
            SheetNames::Maps => (
                sheet_name,
                get_missing_files(&processed_values.maps.rows, &provided_files_set),
            ),
            SheetNames::Shortcuts => (
                sheet_name,
                get_missing_files(&processed_values.shortcuts.rows, &provided_files_set),
            ),
        })
        .try_for_each(|(sheet_name, missing_files)| match missing_files.len() {
            0 => Ok(()),
            _ => Err(BatchProcessError::MissingRequiredFiles {
                sheet_name,
                missing_files,
            }),
        })
}

pub fn check_foreign_key_deps(processed_values: &ProcessedSheets) -> Result<(), BatchProcessError> {
    SheetNames::iter().try_for_each(|sheet_name| match sheet_name {
        SheetNames::Companies => CompanyProcessor::check_foreign_key_deps(processed_values),
        SheetNames::Tags => TagProcessor::check_foreign_key_deps(processed_values),
        SheetNames::TagCategories => TagCategoryProcessor::check_foreign_key_deps(processed_values),
        SheetNames::Prepages => PrepageProcessor::check_foreign_key_deps(processed_values),
        SheetNames::Layouts => LayoutProcessor::check_foreign_key_deps(processed_values),
        SheetNames::Maps => MapProcessor::check_foreign_key_deps(processed_values),
        SheetNames::Shortcuts => ShortcutProcessor::check_foreign_key_deps(processed_values),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::company::CompanyWeb,
        services::batch::{BatchProcessError, ProcessStage, ProcessedSheet, ProcessedSheets},
    };

    #[test]
    fn check_file_dependencie_should_succeed_on_empty_inputs() -> Result<(), BatchProcessError> {
        let processed_values = ProcessedSheets::default(); // Default gives empty lists
        let provided_files: Vec<PathBuf> = Vec::new();

        let res = check_file_dependencies(&processed_values, &provided_files);

        assert!(
            res.is_ok(),
            "Check file dependencies should not fail on empty inputs"
        );

        Ok(())
    }

    #[test]
    fn check_file_dependencies_should_fail_when_file_is_in_processed_but_not_provided(
    ) -> Result<(), BatchProcessError> {
        let processed_values = ProcessedSheets {
            companies: ProcessedSheet {
                rows: vec![(CompanyWeb::default(), vec![PathBuf::from("FILENAME")])],
                process_stage: ProcessStage::NotStarted,
            },
            tags: ProcessedSheet::default(),
            tag_categories: ProcessedSheet::default(),
            prepages: ProcessedSheet::default(),
            layouts: ProcessedSheet::default(),
            maps: ProcessedSheet::default(),
            shortcuts: ProcessedSheet::default(),
        };
        let provided_files = vec![PathBuf::from("OTHERFILE")];

        let res = check_file_dependencies(&processed_values, &provided_files);

        assert!(
            res.is_err(),
            "Check file dependencies should fail when a required file is not in the provided files"
        );

        Ok(())
    }

    #[test]
    fn check_file_dependencies_should_succeed_when_file_is_in_processed_and_provided(
    ) -> Result<(), BatchProcessError> {
        let processed_values = ProcessedSheets {
            companies: ProcessedSheet {
                rows: vec![(CompanyWeb::default(), vec![PathBuf::from("FILENAME")])],
                process_stage: ProcessStage::NotStarted,
            },
            tags: ProcessedSheet::default(),
            tag_categories: ProcessedSheet::default(),
            prepages: ProcessedSheet::default(),
            layouts: ProcessedSheet::default(),
            maps: ProcessedSheet::default(),
            shortcuts: ProcessedSheet::default(),
        };
        let provided_files = vec![PathBuf::from("OTHERFILE"), PathBuf::from("FILENAME")];

        let res = check_file_dependencies(&processed_values, &provided_files);

        assert!(
            res.is_ok(),
            "Check file dependencies should fail when a required file is not in the provided files"
        );

        Ok(())
    }

    //     #[test]
    //     fn check_tag_exist_for_company_tags_should_succeed_on_empty_input(
    //     ) -> Result<(), BatchProcessError> {
    //         let processed_values = ProcessedSheets::default();
    //
    //         let res = check_tag_exist_for_company_tags(&processed_values);
    //
    //         assert!(
    //             res.is_ok(),
    //             "Check tag exist for company tags should not fail on empty inputs"
    //         );
    //
    //         Ok(())
    //     }
    //
    //     #[test]
    //     fn check_tag_exist_for_company_tags_should_fail_if_missing_tags(
    //     ) -> Result<(), BatchProcessError> {
    //         let processed_values = ProcessedSheets {
    //             companies: ProcessedSheet {
    //                 rows: vec![
    //                     (
    //                         CompanyWeb {
    //                             tags: Some(vec![1, 2]),
    //                             ..CompanyWeb::default()
    //                         },
    //                         Vec::new(),
    //                     ),
    //                     (
    //                         CompanyWeb {
    //                             tags: Some(vec![2, 3]),
    //                             ..CompanyWeb::default()
    //                         },
    //                         Vec::new(),
    //                     ),
    //                 ],
    //                 process_stage: ProcessStage::NotStarted,
    //             },
    //             tags: ProcessedSheet::default(),
    //             prepages: ProcessedSheet::default(),
    //             layouts: ProcessedSheet::default(),
    //             maps: ProcessedSheet::default(),
    //             shortcuts: ProcessedSheet::default(),
    //         };
    //
    //         let res = check_tag_exist_for_company_tags(&processed_values);
    //
    //         assert!(
    //             res.is_err(),
    //             "Check tag exist for company tags should fail if missing tags"
    //         );
    //
    //         Ok(())
    //     }
    //
    //     #[test]
    //     fn check_tag_exist_for_company_tags_should_fail_if_missing_subset_of_tags(
    //     ) -> Result<(), BatchProcessError> {
    //         let processed_values = ProcessedSheets {
    //             companies: ProcessedSheet {
    //                 rows: vec![
    //                     (
    //                         CompanyWeb {
    //                             tags: Some(vec![1, 2]),
    //                             ..CompanyWeb::default()
    //                         },
    //                         Vec::new(),
    //                     ),
    //                     (
    //                         CompanyWeb {
    //                             tags: Some(vec![2, 3]),
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
    //                         Vec::new(),
    //                     ),
    //                     (
    //                         TagWeb {
    //                             id: Some(2),
    //                             ..TagWeb::default()
    //                         },
    //                         Vec::new(),
    //                     ),
    //                 ],
    //                 process_stage: ProcessStage::NotStarted,
    //             },
    //             prepages: ProcessedSheet::default(),
    //             layouts: ProcessedSheet::default(),
    //             maps: ProcessedSheet::default(),
    //             shortcuts: ProcessedSheet::default(),
    //         };
    //
    //         let res = check_tag_exist_for_company_tags(&processed_values);
    //
    //         assert!(
    //             res.is_err(),
    //             "Check tag exist for company tags should fail if missing subset of tags"
    //         );
    //
    //         Ok(())
    //     }
    //
    //     #[test]
    //     fn check_tag_exist_for_company_tags_should_succeed_if_required_tags_are_provided(
    //     ) -> Result<(), BatchProcessError> {
    //         let processed_values = ProcessedSheets {
    //             companies: ProcessedSheet {
    //                 rows: vec![
    //                     (
    //                         CompanyWeb {
    //                             tags: Some(vec![1, 2]),
    //                             ..CompanyWeb::default()
    //                         },
    //                         Vec::new(),
    //                     ),
    //                     (
    //                         CompanyWeb {
    //                             tags: Some(vec![2, 3]),
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
    //                         Vec::new(),
    //                     ),
    //                     (
    //                         TagWeb {
    //                             id: Some(2),
    //                             ..TagWeb::default()
    //                         },
    //                         Vec::new(),
    //                     ),
    //                     (
    //                         TagWeb {
    //                             id: Some(3),
    //                             ..TagWeb::default()
    //                         },
    //                         Vec::new(),
    //                     ),
    //                 ],
    //                 process_stage: ProcessStage::NotStarted,
    //             },
    //             prepages: ProcessedSheet::default(),
    //             layouts: ProcessedSheet::default(),
    //             maps: ProcessedSheet::default(),
    //             shortcuts: ProcessedSheet::default(),
    //         };
    //
    //         let res = check_tag_exist_for_company_tags(&processed_values);
    //
    //         assert!(
    //             res.is_ok(),
    //             "Check tag exist for company tags should succeed if required tags are provided"
    //         );
    //
    //         Ok(())
    //     }
}
