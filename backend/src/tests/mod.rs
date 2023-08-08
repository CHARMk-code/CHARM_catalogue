use std::path::{Path, PathBuf};

use uuid::Uuid;

mod auth;
mod batch;
mod company;
mod feedback;
mod image;
mod layout;
mod map;
mod prepage;
mod settings;
mod shortcut;
mod tag;

static TEST_BASE_FOLDER: &str = "test_temp/";
static TEST_UPLOAD: &str = "upload/";
static TEST_STORAGE: &str = "storage/";

fn create_test_folders() -> Result<PathBuf, std::io::Error> {
    let test_uuid = Uuid::new_v4();
    let mut test_path = PathBuf::new();
    test_path.push(env!("CARGO_MANIFEST_DIR"));
    test_path.push(TEST_BASE_FOLDER);
    test_path.push(test_uuid.to_string());

    std::fs::create_dir_all(&test_path)?;

    std::fs::create_dir(&test_path.join(TEST_UPLOAD))?;
    std::fs::create_dir(&test_path.join(TEST_STORAGE))?;

    println!("Directory uuid: {:?}", test_uuid);
    Ok(test_path.to_path_buf())
}

fn remove_test_folders(test_path: &Path) -> Result<(), std::io::Error> {
    if Path::new(test_path).is_dir() {
        std::fs::remove_dir_all(test_path)?;
    }

    Ok(())
}

fn create_dummy_file(test_path: &Path, folder: &str, name: &str) -> Result<(), std::io::Error> {
    let mut path = test_path.to_path_buf();
    path.push(folder);
    path.push(name);

    println!("Created dummy file at: {:?}", path);
    std::fs::File::create(path)?;
    Ok(())
}
