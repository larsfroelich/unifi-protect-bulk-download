use chrono::{DateTime, Local, LocalResult, NaiveDateTime, TimeZone};
use std::fmt;

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
