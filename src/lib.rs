use std::{
    env::current_dir,
    io::{Error as ioError, ErrorKind as ioErrorKind},
    path::PathBuf,
};

pub enum FindTrackingFileError {
    IoError(ioError),
}

/// Finds the nearest `.tok-tracker` file starting at the current working directory.
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
                "Not found in current working directory",
            ));
        }
    }
}
