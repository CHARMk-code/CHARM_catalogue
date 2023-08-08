use std::{collections::HashSet, path::PathBuf};

use strum::IntoEnumIterator;

use crate::services::batch::SheetNames;

use super::{BatchProcessError, ProcessedValues};

pub fn check_file_dependencies(
    processed_values: &ProcessedValues,
    provided_files: &Vec<PathBuf>,
) -> Result<(), BatchProcessError> {
    fn get_missing_files<T>(
        values: &Vec<(T, Vec<PathBuf>)>,
        provided_files_set: &HashSet<PathBuf>,
    ) -> Vec<PathBuf> {
        let required_files_set = values
            .iter()
            .map(|(_, req_files)| req_files)
            .flatten()
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
                get_missing_files(&processed_values.companies, &provided_files_set),
            ),
            SheetNames::Tags => (
                sheet_name,
                get_missing_files(&processed_values.tags, &provided_files_set),
            ),
            SheetNames::Prepages => (
                sheet_name,
                get_missing_files(&processed_values.prepages, &provided_files_set),
            ),
            SheetNames::Layouts => (
                sheet_name,
                get_missing_files(&processed_values.layouts, &provided_files_set),
            ),
            SheetNames::Maps => (
                sheet_name,
                get_missing_files(&processed_values.maps, &provided_files_set),
            ),
            SheetNames::Shortcuts => (
                sheet_name,
                get_missing_files(&processed_values.shortcuts, &provided_files_set),
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

pub fn check_tag_exist_for_company_tags(
    processed_values: &ProcessedValues,
) -> Result<(), BatchProcessError> {
    let companies = processed_values.companies.iter().map(|x| &x.0);
    let tags = processed_values.tags.iter().map(|x| &x.0);

    let tag_ids: HashSet<i32> = HashSet::from_iter(
        tags.map(|tag| {
            tag.id.ok_or(BatchProcessError::MissingIdError {
                value: format!("{:?}", tag),
            })
        })
        .collect::<Result<Vec<i32>, BatchProcessError>>()?
        .into_iter(),
    );
    let company_tag_ids: HashSet<i32> = HashSet::from_iter(
        companies.flat_map(|company| company.clone().tags.unwrap_or(Vec::new())),
    );

    if company_tag_ids.is_subset(&tag_ids) {
        Ok(())
    } else {
        let missing_tag_ids = company_tag_ids.difference(&tag_ids);
        Err(BatchProcessError::MissingTagIdsForCompanyTags {
            tag_ids: missing_tag_ids.cloned().collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        routes::{company::CompanyWeb, tag::TagWeb},
        services::batch::{BatchProcessError, ProcessedValues},
    };

    #[test]
    fn check_file_dependencie_should_succeed_on_empty_inputs() -> Result<(), BatchProcessError> {
        let processed_values = ProcessedValues::default(); // Default gives empty lists
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
        let processed_values = ProcessedValues {
            companies: vec![(CompanyWeb::default(), vec![PathBuf::from("FILENAME")])],
            tags: Vec::new(),
            prepages: Vec::new(),
            layouts: Vec::new(),
            maps: Vec::new(),
            shortcuts: Vec::new(),
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
        let processed_values = ProcessedValues {
            companies: vec![(CompanyWeb::default(), vec![PathBuf::from("FILENAME")])],
            tags: Vec::new(),
            prepages: Vec::new(),
            layouts: Vec::new(),
            maps: Vec::new(),
            shortcuts: Vec::new(),
        };
        let provided_files = vec![PathBuf::from("OTHERFILE"), PathBuf::from("FILENAME")];

        let res = check_file_dependencies(&processed_values, &provided_files);

        assert!(
            res.is_ok(),
            "Check file dependencies should fail when a required file is not in the provided files"
        );

        Ok(())
    }

    #[test]
    fn check_tag_exist_for_company_tags_should_succeed_on_empty_input(
    ) -> Result<(), BatchProcessError> {
        let processed_values = ProcessedValues::default();

        let res = check_tag_exist_for_company_tags(&processed_values);

        assert!(
            res.is_ok(),
            "Check tag exist for company tags should not fail on empty inputs"
        );

        Ok(())
    }

    #[test]
    fn check_tag_exist_for_company_tags_should_fail_if_missing_tags(
    ) -> Result<(), BatchProcessError> {
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
                        tags: Some(vec![2, 3]),
                        ..CompanyWeb::default()
                    },
                    Vec::new(),
                ),
            ],
            tags: vec![],
            prepages: vec![],
            layouts: vec![],
            maps: vec![],
            shortcuts: vec![],
        };

        let res = check_tag_exist_for_company_tags(&processed_values);

        assert!(
            res.is_err(),
            "Check tag exist for company tags should fail if missing tags"
        );

        Ok(())
    }

    #[test]
    fn check_tag_exist_for_company_tags_should_fail_if_missing_subset_of_tags(
    ) -> Result<(), BatchProcessError> {
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
                        tags: Some(vec![2, 3]),
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
                    Vec::new(),
                ),
                (
                    TagWeb {
                        id: Some(2),
                        ..TagWeb::default()
                    },
                    Vec::new(),
                ),
            ],
            prepages: vec![],
            layouts: vec![],
            maps: vec![],
            shortcuts: vec![],
        };

        let res = check_tag_exist_for_company_tags(&processed_values);

        assert!(
            res.is_err(),
            "Check tag exist for company tags should fail if missing subset of tags"
        );

        Ok(())
    }

    #[test]
    fn check_tag_exist_for_company_tags_should_succeed_if_required_tags_are_provided(
    ) -> Result<(), BatchProcessError> {
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
                        tags: Some(vec![2, 3]),
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
                    Vec::new(),
                ),
                (
                    TagWeb {
                        id: Some(2),
                        ..TagWeb::default()
                    },
                    Vec::new(),
                ),
                (
                    TagWeb {
                        id: Some(3),
                        ..TagWeb::default()
                    },
                    Vec::new(),
                ),
            ],
            prepages: vec![],
            layouts: vec![],
            maps: vec![],
            shortcuts: vec![],
        };

        let res = check_tag_exist_for_company_tags(&processed_values);

        assert!(
            res.is_ok(),
            "Check tag exist for company tags should succeed if required tags are provided"
        );

        Ok(())
    }
}
