use {
    core::{
        fmt::Display,
        ops::{Range, RangeFrom},
    },
    lazy_string_replace::{LazyReplace, LazyReplaceDisplay},
    std::{
        env::current_dir,
        fs::{File, OpenOptions},
        io::{Error as ioError, ErrorKind as ioErrorKind},
        path::PathBuf,
    },
    time::OffsetDateTime,
};

mod parser;

const TIMESTAMP_FORMAT: &str = "%_Y-%_m-%_d %_H:%M:%S %z";

struct Entry {
    span: Span,
    tags: Vec<String>,
    comments: Vec<String>,
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.span))?;
        if !self.tags.is_empty() {
            let mut iter = self.tags.iter();
            f.write_fmt(format_args!("({}", iter.next().unwrap()))?;
            iter.try_for_each(|tag| f.write_fmt(format_args!(",{}", tag)))?;
            f.write_str(")")?;
        }
        self.comments.iter().try_for_each(|comment| {
            f.write_fmt(format_args!(
                "#{}",
                comment
                    .lazy_replace('\\', "\\\\")
                    .replace_display("#", "\\#")
                    .replace_display("\r", "\\r")
                    .replace_display("\n", "\\n")
            ))
        })?;
        Ok(())
    }
}

enum Span {
    Active(RangeFrom<OffsetDateTime>),
    Closed(Range<OffsetDateTime>),
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Span::Active(RangeFrom { start }) => {
                f.write_fmt(format_args!("{}..", start.lazy_format(TIMESTAMP_FORMAT)))
            }
            Span::Closed(Range { start, end }) => f.write_fmt(format_args!(
                "{}..{}",
                start.lazy_format(TIMESTAMP_FORMAT),
                end.lazy_format(TIMESTAMP_FORMAT)
            )),
        }
    }
}

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
