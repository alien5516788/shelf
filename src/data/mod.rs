pub mod db;

use std::{fs, path::PathBuf};
use std::sync::OnceLock;
use directories::BaseDirs;


const DATA_DIR_NAME: &str = ".shelf";

// Set once at runtime
static DATA_DIR_PATH: OnceLock<PathBuf> = OnceLock::new();


// Returns the data directory for the application
pub fn ensure_data_dir() -> Result<PathBuf, String> {
    // Data dir already initialized, return cached value
    if let Some(data_dir) = DATA_DIR_PATH.get() {
        println!("Shelf: Using data directory: {:?}", data_dir);
        return Ok(data_dir.into());
    }

    // Subsequent code will only be executed once after caching

    // User directory
    let user_dir: PathBuf = BaseDirs::new()
        .ok_or("Failed to determine user directory".to_string())?
        .home_dir()
        .to_path_buf();

    // Canonicalize (fully resolve) the user directory path
    let user_dir = user_dir
        .canonicalize()
        .map_err(|_| "Failed to canonicalize user directory".to_string())?;

    // Obtain data directory path
    let data_dir = user_dir.join(DATA_DIR_NAME);
    println!("Shelf: Using data directory: {:?}", data_dir);

    // Create data directory if it does not exist
    if !data_dir.exists() {
        fs::create_dir(&data_dir)
            .map_err(|_| "Failed to create data directory".to_string())?;
    }

    DATA_DIR_PATH.set(PathBuf::from(data_dir.clone())).ok();
    Ok(data_dir)
}
