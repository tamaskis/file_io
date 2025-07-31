use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Get the path of a temporary directory.
///
/// # Arguments
///
/// * `temp_dir` - A reference to the temporary directory.
///
/// # Returns
///
/// The path of the temporary directory.
///
/// # Panics
///
/// If the canonicalization fails.
///
/// # Note
///
/// This function is useful for obtaining a stable path to a temporary directory that can be used in
/// unit tests (since temporary directories can have paths containing symlinks).
pub(crate) fn get_temp_dir_path(temp_dir: &TempDir) -> PathBuf {
    std::fs::canonicalize(temp_dir.path())
        .expect("Failed to get the canonical path of the temporary directory.")
}

/// Assert that a folder exists at the specified path.
///
/// # Arguments
///
/// * `path` - The path to the folder (can be a `&str`, [`String`], [`Path`], or [`PathBuf`]).
///
/// # Panics
///
/// If the path does not exist or is not a directory.
pub(crate) fn assert_folder_exists<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();
    assert!(path.exists(), "Path does not exist: {path:?}");
    assert!(path.is_dir(), "Path is not a directory: {path:?}");
}
