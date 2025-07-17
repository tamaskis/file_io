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

/// Deletes a file at the specified path if it exists.
///
/// # Arguments
///
/// * `path` - The path to the file to delete (can be a `&str`, `String`, `Path`, or `PathBuf`).
///
/// # Panics
///
/// If some error is encountered while deleting the file at `path`.
///
/// # Examples
///
/// ## Using a string literal
///
/// ```
/// use file_io::{delete_file, save_string_to_file};
/// use std::path::Path;
///
/// // Create a file to delete later.
/// let path: &str = "file_to_delete_1.txt";
/// save_string_to_file("Hello, world!", path);
///
/// // Verify that the file exists.
/// assert!(Path::new(path).exists());
///
/// // Now delete the file.
/// delete_file(path);
///
/// // Verify that the file no longer exists.
/// assert!(!Path::new(path).exists());
/// ```
///
/// ## Using a `Path` reference
///
/// ```
/// use file_io::{delete_file, save_string_to_file};
/// use std::path::Path;
///
/// // Create a file to delete later.
/// let path: &Path = Path::new("file_to_delete_2.txt");
/// save_string_to_file("Hello, world!", path);
///
/// // Verify that the file exists.
/// assert!(path.exists());
///
/// // Now delete the file.
/// delete_file(path);
///
/// // Verify that the file no longer exists.
/// assert!(!path.exists());
/// ```
pub fn delete_file<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();
    if path.exists() {
        std::fs::remove_file(path)
            .unwrap_or_else(|_| panic!("Failed to delete file at '{path:?}'."));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::save::save_string_to_file;
    use crate::test_utils::get_temp_dir_path;
    use crate::to_path_buf;
    use tempfile::tempdir;

    #[test]
    fn test_delete_file() {
        // Create a temporary directory.
        let temp_dir = tempdir().unwrap();

        // Get the path to the temporary directory.
        let temp_dir_path = get_temp_dir_path(&temp_dir);

        // Path to the file to copy.
        let file_path = temp_dir_path.join("file_to_copy.txt");

        // File path in different formats.
        let file_paths: Vec<Box<dyn AsRef<Path>>> = vec![
            Box::new(file_path.to_str().unwrap()),             // &str
            Box::new(file_path.to_str().unwrap().to_string()), // String
            Box::new(file_path.as_path()),                     // Path
            Box::new(file_path.clone()),                       // PathBuf
        ];

        // Test with all different path formats.
        for file_path in file_paths {
            // Get a reference to this path representation (i.e. "unbox").
            let file_path = file_path.as_ref();

            // File path as a pathbuf.
            let file_path_buf = to_path_buf(file_path);

            // Check that the file does not exist before creating it.
            assert!(!file_path_buf.exists());

            // Create a file at the specified path.
            save_string_to_file("Hello, world!", file_path);

            // Verify that the file exists.
            assert!(file_path_buf.exists());

            // Now delete the file.
            delete_file(file_path);

            // Verify that the file no longer exists.
            assert!(!file_path_buf.exists());
        }
    }
}
