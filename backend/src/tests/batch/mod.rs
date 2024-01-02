use std::{collections::HashMap, fs, path::PathBuf};

use chrono::DateTime;
use sqlx::{Pool, Postgres};
use serde_json::json;

use crate::{
    models::{
        company::CompanyDB, layout::LayoutDB, map::FairMapDB, prepage::PrepageDB, shortcut::ShortcutDB,
        tag::TagDB, tag_category::TagCategoryDB,
    },
    services::{
        self,
        batch::{process_batch_zip, BatchProcessError},
    },
    tests::{create_test_folders, TEST_STORAGE, TEST_UPLOAD},
};

#[sqlx::test()]
async fn full_parsing_of_zip_file_should_populate_db(
    db: Pool<Postgres>,
) -> Result<(), BatchProcessError> {
    let zip_path: PathBuf =
        (env!("CARGO_MANIFEST_DIR").to_owned() + "/src/tests/batch/batch_test.zip").into();

    let base_path = create_test_folders().unwrap();

    let upload_path = base_path.join(TEST_UPLOAD);
    let storage_path = base_path.join(TEST_STORAGE);

    process_batch_zip(&db, &zip_path, &upload_path, &storage_path).await?;

    // Expected results

    // Doesn't verify all data entered only the amount of rows 
    let expected_companies_amount = 160;
    let expected_layouts_amount = 6;
    let expected_maps_amount = 4;
    let expected_prepages_amount = 49;
    let expected_shortcuts_amount = 3;
    let expected_tags_amount = 41;
    let expected_tag_categories_amount = 6;
    let expected_images_amount = 257;


    // Companies
    let companies = services::company::get_all(&db)
        .await
        .expect("to have been tested elsewhere");
    assert_eq!(
        companies.len(),
        expected_companies_amount,
        "Different amount of companies than expected"
    );

    // Tags
    let tags = services::tag::get_all(&db)
        .await
        .expect("to have been tested elsewhere");
    assert_eq!(
        tags.len(),
        expected_tags_amount,
        "Different amount of tags than expected"
    );
    
    // Tag category
    let tag_categories = services::tag_category::get_all(&db)
        .await
        .expect("to have been tested elsewhere");
    assert_eq!(
        tag_categories.len(),
        expected_tag_categories_amount,
        "Different amount of tag categories than expected"
    );


    // Prepages
    let prepages = services::prepage::get_all(&db)
        .await
        .expect("to have been tested elsewhere");
    assert_eq!(
        prepages.len(),
        expected_prepages_amount,
        "Different amount of prepages than expected"
    );

    // Shortcuts
    let shortcuts = services::shortcut::get_all(&db)
        .await
        .expect("to have been tested elsewhere");
    assert_eq!(
        shortcuts.len(),
        expected_shortcuts_amount,
        "Different amount of shortcuts than expected"
    );

    // Layouts
    let layouts = services::layout::get_all(&db)
        .await
        .expect("to have been tested elsewhere");
    assert_eq!(
        layouts.len(),
        expected_layouts_amount,
        "Different amount of layouts than expected"
    );

    // Maps
    let maps = services::map::get_all(&db)
        .await
        .expect("to have been tested elsewhere");
    assert_eq!(
        maps.len(),
        expected_maps_amount,
        "Different amount of maps than expected"
    );

    // Images
    let images = services::image::get_all(&db)
        .await
        .expect("to have been tested elsewhere");
    assert_eq!(
        images.len(),
        expected_images_amount,
        "Different amount of images than expected"
    );

    let files_in_dir = fs::read_dir(storage_path).unwrap().fold(0, |c, _| c + 1);
    assert_eq!(files_in_dir, expected_images_amount);

    Ok(())
}
