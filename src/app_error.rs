use chrono::{DateTime, Local, LocalResult, NaiveDateTime, TimeZone};
use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum AppError {
    ParseDate {
        input: String,
        source: chrono::ParseError,
    },
    InvalidDateRange {
        start: NaiveDateTime,
        end: NaiveDateTime,
    },
    DateConstruction {
        context: String,
    },
    DateConversion {
        context: String,
    },
    DateOverflow {
        context: String,
    },
    InvalidMode {
        mode: String,
    },
    OutputPathInaccessible {
        path: PathBuf,
        existing_parent: Option<PathBuf>,
        missing_path: Option<PathBuf>,
        source: io::Error,
    },
    OutputPathNotDirectory {
        path: PathBuf,
    },
    OutputPathNotWritable {
        path: PathBuf,
        source: io::Error,
    },
    Api {
        context: String,
        source: String,
    },
}

impl AppError {
    pub fn parse_date(input: &str, source: chrono::ParseError) -> Self {
        Self::ParseDate {
            input: input.to_string(),
            source,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ParseDate { input, source } => {
                write!(f, "failed to parse date '{}': {}", input, source)
            }
            AppError::InvalidDateRange { start, end } => {
                write!(
                    f,
                    "invalid date range: end date/time '{}' is before start date/time '{}'",
                    end, start
                )
            }
            AppError::DateConstruction { context } => write!(f, "invalid date/time: {}", context),
            AppError::DateConversion { context } => {
                write!(f, "time zone conversion failed: {}", context)
            }
            AppError::DateOverflow { context } => write!(f, "date arithmetic failed: {}", context),
            AppError::InvalidMode { mode } => write!(f, "invalid download mode '{}'", mode),
            AppError::OutputPathInaccessible {
                path,
                existing_parent: Some(existing_parent),
                missing_path: Some(missing_path),
                source,
            } => write!(
                f,
                "pre-download output path check failed: '{}' is not accessible: path exists up to '{}', but '{}' does not exist ({})",
                path.display(),
                existing_parent.display(),
                missing_path.display(),
                source
            ),
            AppError::OutputPathInaccessible { path, source, .. } => write!(
                f,
                "pre-download output path check failed: '{}' is not accessible: {}",
                path.display(),
                source
            ),
            AppError::OutputPathNotDirectory { path } => {
                write!(
                    f,
                    "pre-download output path check failed: '{}' is not a directory",
                    path.display()
                )
            }
            AppError::OutputPathNotWritable { path, source } => write!(
                f,
                "pre-download output path check failed: '{}' is not writable: {}",
                path.display(),
                source
            ),
            AppError::Api { context, source } => write!(f, "{}: {}", context, source),
        }
    }
}

impl std::error::Error for AppError {}

pub fn to_local(naive: NaiveDateTime, context: String) -> Result<DateTime<Local>, AppError> {
    match Local.from_local_datetime(&naive) {
        LocalResult::Single(dt) => Ok(dt),
        LocalResult::Ambiguous(first, second) => Err(AppError::DateConversion {
            context: format!(
                "{} ({}) is ambiguous between '{}' and '{}'",
                context, naive, first, second
            ),
        }),
        LocalResult::None => Err(AppError::DateConversion {
            context: format!("{} ({}) does not exist in local timezone", context, naive),
        }),
    }
}

pub fn api_error(context: impl Into<String>, source: impl Into<String>) -> AppError {
    AppError::Api {
        context: context.into(),
        source: source.into(),
    }
}
