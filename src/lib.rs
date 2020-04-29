use std::{
    env::current_dir,
    fs::{File, OpenOptions},
    io::{Error as ioError, ErrorKind as ioErrorKind},
    path::PathBuf,
};

// Creates a new .tok-tracker file in the current directory and opens it in read-write mode.
pub fn init() -> Result<File, ioError> {
    OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(".tok-tracker")
}

/// Finds the nearest `.tok-tracker` file starting at the current directory.
pub fn find_tracking_file(walk_parents: bool) -> Result<PathBuf, ioError> {
    let current_directory = current_dir()?;
    let mut current_directory = current_directory.as_path();
    loop {
        let file_path = current_directory.with_file_name(".tok-tracker");
        if file_path.is_file() {
            return Ok(file_path);
        } else if walk_parents {
            current_directory = if let Some(parent) = current_directory.parent() {
                parent
            } else {
                return Err(ioError::new(
                    ioErrorKind::NotFound,
                    "Reached file system root",
                ));
            }
        } else {
            return Err(ioError::new(
                ioErrorKind::NotFound,
                "Not found in current directory",
            ));
        }
    }
}
