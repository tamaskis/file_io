use std::path::Path;

/// Deletes a folder at the specified path if it exists.
///
/// # Arguments
///
/// * `path` - The path to the folder to delete (can be a `&str`, `String`, `Path`, or `PathBuf`).
///
/// # Panics
///
/// If some error is encountered while deleting the folder at `path`.
///
/// # Examples
///
/// ## Using a string literal
///
/// ```
/// use file_io::{create_folder, delete_folder};
/// use std::path::Path;
///
/// // Create a folder to delete later.
/// let path: &str = "folder/subfolder_5";
/// create_folder(path);
///
/// // Verify that the folder exists.
/// assert!(Path::new(path).exists());
///
/// // Now delete the folder.
/// delete_folder(path);
///
/// // Verify that the folder no longer exists.
/// assert!(!Path::new(path).exists());
/// ```
///
/// ## Using a `Path` reference
///
/// ```
/// use file_io::{create_folder, delete_folder};
/// use std::path::Path;
///
/// // Create a folder to delete later.
/// let path: &Path = Path::new("folder/subfolder_5");
/// create_folder(path);
///
/// // Verify that the folder exists.
/// assert!(path.exists());
///
/// // Now delete the folder.
/// delete_folder(path);
///
/// // Verify that the folder no longer exists.
/// assert!(!path.exists());
/// ```
pub fn delete_folder<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();
    if path.exists() {
        std::fs::remove_dir_all(path)
            .unwrap_or_else(|_| panic!("Failed to delete folder at '{path:?}'."));
    }
}
