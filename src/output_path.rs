use crate::app_error::AppError;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const PROBE_ATTEMPTS: usize = 10;

pub fn validate(out_path: &Path, probe_writable: bool) -> Result<(), AppError> {
    let metadata = fs::metadata(out_path).map_err(|source| {
        let (existing_parent, missing_path) = if source.kind() == io::ErrorKind::NotFound {
            first_missing_path(out_path)
        } else {
            (None, None)
        };

        AppError::OutputPathInaccessible {
            path: out_path.to_path_buf(),
            existing_parent,
            missing_path,
            source,
        }
    })?;

    if !metadata.is_dir() {
        return Err(AppError::OutputPathNotDirectory {
            path: out_path.to_path_buf(),
        });
    }

    if metadata.permissions().readonly() {
        return Err(AppError::OutputPathNotWritable {
            path: out_path.to_path_buf(),
            source: io::Error::new(io::ErrorKind::PermissionDenied, "path is read-only"),
        });
    }

    if probe_writable {
        probe(out_path)?;
    }

    Ok(())
}

fn first_missing_path(path: &Path) -> (Option<PathBuf>, Option<PathBuf>) {
    let mut existing_parent = None;
    let mut candidate = PathBuf::new();

    for component in path.components() {
        candidate.push(component.as_os_str());

        if candidate.exists() {
            existing_parent = Some(candidate.clone());
        } else {
            return (existing_parent, Some(candidate));
        }
    }

    (existing_parent, None)
}

fn probe(out_path: &Path) -> Result<(), AppError> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();

    for attempt in 0..PROBE_ATTEMPTS {
        let test_path = out_path.join(format!(
            ".unifi-protect-bulk-download-write-test-{}-{}-{}",
            std::process::id(),
            timestamp,
            attempt
        ));

        let mut file = match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&test_path)
        {
            Ok(file) => file,
            Err(source) if source.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(source) => {
                return Err(AppError::OutputPathNotWritable {
                    path: out_path.to_path_buf(),
                    source,
                });
            }
        };

        if let Err(source) = file.write_all(b"test") {
            let _ = fs::remove_file(&test_path);
            return Err(AppError::OutputPathNotWritable {
                path: out_path.to_path_buf(),
                source,
            });
        }
        drop(file);

        return fs::remove_file(&test_path).map_err(|source| AppError::OutputPathNotWritable {
            path: out_path.to_path_buf(),
            source,
        });
    }

    Err(AppError::OutputPathNotWritable {
        path: out_path.to_path_buf(),
        source: io::Error::new(
            io::ErrorKind::AlreadyExists,
            "could not create a unique output path probe file without overwriting an existing file",
        ),
    })
}
