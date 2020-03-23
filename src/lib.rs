/// Collection of utility functions

#[macro_use]
extern crate log;

use std::fs;
use std::path::Path;
use std::io::Write;

/// Creates a directory recursively at passed path
/// and returns a boolean based on success or failure.
///
/// ## Usage:
///
/// ```
/// assert_eq!(fsutils::mkdir("./testdir"), true);
/// ```
pub fn mkdir(path: &str) -> bool {
    if !path_exists(path) {
        match fs::create_dir_all(path) {
            Ok(_) => {
                info!("Created {}", path);
                true
            }
            Err(e) => {
                info!("Error creating file: {}", e);
                false
            }
        }
    } else {
        false
    }
}

/// Function to remove file
/// Returns bool
pub fn rm(path: &str) -> bool {
    // str to Path
    let new_path = Path::new(path);
    if new_path.exists() {
        match fs::remove_file(path) {
            Ok(_) => {
                info!("Removed file {}", path);
                true
            },
            Err(e) => {
                info!("Error removing {} {}", path, e);
                false
            }
        }
    } else {
        false
    }
}

/// Function to remove directory
/// Returns bool
pub fn rmdir(path: &str) -> bool {
    // Turn str path into Path
    let new_path = Path::new(path);
    if new_path.exists() {
        match fs::remove_dir(path) {
            Ok(_) => {
                info!("Removed directory at {}", path);
                true
            },
            Err(_e) => {
                info!("The directory {} is not empty", path);
                false
            }
        }
    } else {
        info!("Directory does not exist");
        true
    }
}

/// Function to remove directory recursively
/// Use carefully
/// Returns bool
pub fn rm_r(path: &str) -> bool {
    // Turn str path into Path
    let new_path = Path::new(path);
    if new_path.exists() {
        match fs::remove_dir_all(path) {
            Ok(_) => {
                info!("Removed directory at {}", path);
                true
            },
            Err(_e) => {
                info!("The directory {} is not empty", path);
                false
            }
        }
    } else {
        info!("Directory does not exist");
        true
    }
}

/// Check if path exists and return boolean
pub fn path_exists(path: &str) -> bool {
    // Turn str path into Path
    let new_path = Path::new(path);
    if new_path.exists() {
        info!("{} exists", path);
        true
    } else {
        info!("{} does not exist", path);
        false
    }
}

/// Check if directory is empty and return boolean
pub fn directory_is_empty(path: &str) -> bool {
    // Turn str path into Path
    let new_path = Path::new(path);
    if new_path.exists() {
        if new_path.is_dir() {
            let mut i = 0;
            // iterate through entries and count them
            for _ in fs::read_dir(path) {
                i += 1;
            }
            // if the count of directory entries is 1 (it counts itself), it is empty
            if i == 1 {
                true
            } else {
                false
            }
        } else {
            info!("The path {} passed is not a directory", path);
            false
        }
    } else {
        info!("The path {} passed does not exist.", path);
        false
    }
}

/// Move file from one location to another
pub fn mv(path_one: &str, path_two: &str) {
    let p1 = Path::new(path_one);
    if p1.exists() {
        match fs::rename(path_one, path_two) {
            Ok(_) => info!("Moved from {} to {}.", path_one, path_two),
            Err(e) => info!("File moving error: {}", e)
        }
    }
}

/// Create file
pub fn create_file(path: &str) {
    match fs::File::create(path) {
        Ok(_f) => {
            info!("Successfully wrote file to {}", path)
        }
        Err(e) => info!("{}", e)
    }
}

/// Create file from bytes
pub fn create_file_bytes(path: &str, bytes_to_write: &[u8]) {
    match fs::File::create(path) {
        Ok(mut buffer) => {
            match buffer.write_all(bytes_to_write) {
                Ok(_) => info!("Wrote buffer to {}", path),
                Err(e) => info!("{}", e)
            }
        }
        Err(e) => info!("{}", e)
    }
}