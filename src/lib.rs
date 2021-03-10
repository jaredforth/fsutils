// Copyright 2020 Jared Forth.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Utilities for common filesystem operations.
//!
//! **fsutils** provides an API based on Bash commands and includes a number
//! of utility functions to make interacting with the filesystem simpler and more
//! ergonomic.

#[macro_use]
extern crate log;

use std::fs;
use std::path::Path;
use std::io::{Write, Read};
use std::fs::{File, OpenOptions};

/// Creates a directory recursively at passed path
/// and returns a boolean based on success or failure.
///
/// ## Usage:
///
/// ```
/// assert_eq!(fsutils::mkdir("testdir"), true);
///
/// # // Cleanup
/// # fsutils::rmdir("testdir");
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

/// Removes a file at passed path
/// and returns a boolean based on success or failure.
///
/// ## Usage:
///
/// ```
/// fsutils::create_file("testfile.txt");
/// assert_eq!(fsutils::rm("testfile.txt"), true);
/// ```
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

/// Removes an empty directory
/// and returns a boolean based on success or failure.
///
/// This does not remove a directory recursively. Use `fsutils::rm_r`
/// if you need recursive directory deletion.
///
/// # Usage:
///
/// ```
/// use fsutils::rmdir;
/// fsutils::mkdir("testdir");
/// assert_eq!(rmdir("testdir"), true);
/// ```
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

/// Removes a directory recursively
/// and returns a boolean based on success or failure.
///
/// You should use this carefully.
///
/// ## Usage:
///
/// ```
/// fsutils::mkdir("testdir");
/// fsutils::create_file("testdir/testfile");
///
/// assert_eq!(fsutils::rm_r("testdir"), true);
/// ```
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

/// Checks if a path exists
/// and returns a boolean based on success or failure.
///
/// # Usage:
///
/// ```
/// fsutils::create_file("testfile");
/// assert_eq!(fsutils::path_exists("testfile"), true);
/// assert_eq!(fsutils::path_exists("a_very_1234_unlikely_9876_filename"), false);
/// 
/// # // Cleanup
/// # fsutils::rm("testfile");
/// ```
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

/// Checks if a directory is empty
/// and returns a boolean based on success or failure.
///
/// # Usage:
///
/// ```
/// fsutils::mkdir("empty_directory");
/// fsutils::mkdir("full_directory");
/// fsutils::create_file("full_directory/a_file.txt");
/// fsutils::create_file("full_directory/another_file.txt");
///
/// assert_eq!(fsutils::directory_is_empty("full_directory"), false);
/// assert_eq!(fsutils::directory_is_empty("empty_directory"), true);
///
/// # // Cleanup
/// # fsutils::rmdir("empty_directory");
/// # fsutils::rm_r("full_directory");
/// ```
pub fn directory_is_empty(path: &str) -> bool {
    // Turn str path into Path
    let new_path = Path::new(path);
    if new_path.exists() {
        if new_path.is_dir() {
            let mut i = 0;
            // iterate through entries and count them
            // `fs::read_dir` returns type `ReadDir`
            for entry in fs::read_dir(path) {
                // Iterating over `ReadDir` returns a Result<DirEntry>`
                // which is what we want to give us the count.
                for _ in entry {
                    i += 1;
                }
            }
            // if the count of directory entries is 1 (it counts itself), it is empty
            if i == 0 {
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

/// Moves a file from `path_one` to `path_two`
/// and returns a boolean based on success or failure.
///
/// # Usage:
///
/// ```
/// fsutils::mkdir("directory_one");
/// fsutils::mkdir("directory_two");
/// fsutils::create_file("directory_one/the_file");
///
/// assert_eq!(fsutils::mv("directory_one/the_file", "directory_two/the_file"), true);
///
/// # // Cleanup
/// # fsutils::rm_r("directory_one");
/// # fsutils::rm_r("directory_two");
/// ```
pub fn mv(path_one: &str, path_two: &str) -> bool {
    let p1 = Path::new(path_one);
    if p1.exists() {
        match fs::rename(path_one, path_two) {
            Ok(_) => {
                info!("Moved from {} to {}.", path_one, path_two);
                true
            },
            Err(e) => {
                info!("File moving error: {}", e);
                false
            }
        }
    } else {
        false
    }
}

/// Creates a file and returns a boolean based on success or failure.
///
/// ## Usage:
///
/// ```
/// assert_eq!(fsutils::create_file("the_file"), true);
///
/// # // Cleanup
/// # fsutils::rm("the_file");
/// ```
pub fn create_file(path: &str) -> bool {
    match fs::File::create(path) {
        Ok(_f) => {
            info!("Successfully wrote file to {}", path);
            true
        }
        Err(e) => {
            info!("{}", e);
            false
        }
    }
}

/// Creates a file from bytes
/// and returns a boolean based on success or failure.
///
/// # Usage:
///
/// ```
/// let binary_file: &'static [u8] = b"01001000 01100101 01101100 01101100 01101111 00100001";
///
/// assert_eq!(fsutils::create_file_bytes("a_binary_file", binary_file), true);
///
/// # // Cleanup
/// # fsutils::rm("a_binary_file");
/// ```
pub fn create_file_bytes(path: &str, bytes_to_write: &[u8]) -> bool {
    match fs::File::create(path) {
        Ok(mut buffer) => {
            match buffer.write_all(bytes_to_write) {
                Ok(_) => {
                    info!("Wrote buffer to {}", path);
                    true
                },
                Err(e) => {
                    info!("{}", e);
                    false
                }
            }
        }
        Err(e) => {
            info!("{}", e);
            false
        }
    }
}

/// Reads data to a file
/// and returns a `bool` on success
///
/// ## Usage:
///
/// ```
/// fsutils::write_file("text.txt", "Hello, world!");
///
/// assert_eq!(fsutils::read_file("text.txt"), "Hello, world!");
///
/// # // Cleanup
/// # fsutils::rm("text.txt");
/// ```
pub fn write_file(path: &str, contents: &str) -> bool {
    match File::create(path) {
        Ok(mut f) => {
            f.write_all(contents.as_ref()).unwrap();
            true
        }
        Err(e) => {
            info!("Cannot write file to location '{}' {}", path, e);
            false
        }
    }
}

/// Appends data to a file
/// and returns a `bool` on success
///
/// ## Usage:
///
/// ```
/// fsutils::write_file_append("text.txt", "Hello, world! ");
/// fsutils::write_file_append("text.txt", "Hi Again!");
///
/// assert_eq!(fsutils::read_file("text.txt"), "Hello, world! Hi Again!");
///
/// # // Cleanup
/// # fsutils::rm("text.txt");
/// ```
pub fn write_file_append(path: &str, contents: &str) -> bool {
    match OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path) {
        Ok(mut f) => {
            f.write_all(contents.as_ref()).unwrap();
            true
        }
        Err(e) => {
            info!("Cannot write file {}", e);
            false
        }
    }
}

/// Reads data from a file
/// and returns a `String` with the files's contents
///
/// ## Usage:
///
/// ```
/// fsutils::write_file("text.txt", "Hello, world!");
///
/// assert_eq!(fsutils::read_file("text.txt"), "Hello, world!");
///
/// # // Cleanup
/// # fsutils::rm("text.txt");
/// ```
pub fn read_file(path: &str) -> String {
    let mut contents = String::new();
    match File::open(path) {
        Ok(mut f) => {
            f.read_to_string(&mut contents).unwrap();
        }
        Err(e) => info!("Cannot read file {}", e)
    }
    contents
}