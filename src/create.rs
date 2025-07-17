use std::path::Path;

/// Creates a new folder at the specified path if it does not already exist.
///
/// # Arguments
///
/// * `path` - The path where the folder should be created (can be a `&str`, `String`, `Path`, or
///   `PathBuf`).
///
/// # Panics
///
/// If some error is encountered while creating the folder at `path`.
///
/// # Examples
///
/// ## Using a string literal
///
/// ```
/// use file_io::create_folder;
///
/// let path: &str = "folder/subfolder_1";
/// create_folder(path);
/// ```
///
/// ## Using a `Path` reference
///
/// ```
/// use file_io::create_folder;
/// use std::path::Path;
///
/// let path: &Path = Path::new("folder/subfolder_2");
/// create_folder(path);
/// ```
pub fn create_folder<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();
    if !path.exists() {
        std::fs::create_dir_all(path)
            .unwrap_or_else(|_| panic!("Failed to create folder at '{path:?}'."));
    }
}

/// Creates the parent folder for a file at the specified path if it does not already exist.
///
/// # Arguments
///
/// * `path` - The path to the file for which the parent folder should be created (can be a `&str`,
///   `String`, `Path`, or `PathBuf`).
///
/// # Panics
///
/// If some error is encountered while creating the parent folder.
///
/// # Examples
///
/// ## Using a string literal
///
/// ```
/// use file_io::create_folder_for_file;
///     
/// let path: &str = "folder/subfolder_3/file_1.txt";
///
/// // This will create "folder/subfolder_3" if it does not exist.
/// create_folder_for_file(path);
/// ```
///
/// ## Using a `Path` reference
///
/// ```
/// use file_io::create_folder_for_file;
/// use std::path::Path;
///
/// let path: &Path = Path::new("folder/subfolder_4/file_2.txt");
///
/// // This will create "folder/subfolder_4" if it does not exist.
/// create_folder_for_file(path);
/// ```
pub fn create_folder_for_file<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        create_folder(parent);
    }
}

/// Copies a file from one location to another.
///
/// # Arguments
///
/// * `from` - The source file path (can be a `&str`, `String`, `Path`, or `PathBuf`).
/// * `to` - The destination file path (can be a `&str`, `String`, `Path`, or `PathBuf`).
///
/// # Panics
///
/// If the source file does not exist or cannot be accessed, or if the destination cannot be
/// created.
///
/// # Note
///
/// This function will create the parent folder for the destination file if it does not already
/// exist.
///
/// # Examples
///
/// ## Using string literals
///
/// ```
/// use file_io::copy_file;
///
/// // Copy 'Cargo.toml' to 'folder/Cargo_new_1.toml'.
/// let from: &str = "Cargo.toml";
/// let to: &str = "folder/Cargo_new_1.toml";
/// copy_file(from, to);
/// ```
///
/// ## Using `Path` references
///
/// ```
/// use file_io::copy_file;
/// use std::path::Path;
///
/// // Copy 'Cargo.toml' to 'folder/Cargo_new_2.toml'.
/// let from: &Path = Path::new("Cargo.toml");
/// let to: &Path = Path::new("folder/Cargo_new_2.toml");
/// copy_file(from, to);
/// ```
pub fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) {
    let from = from.as_ref();
    let to = to.as_ref();
    create_folder_for_file(to);
    std::fs::copy(from, to)
        .unwrap_or_else(|_| panic!("Failed to copy file from '{from:?}' to '{to:?}'."));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::delete::{delete_file, delete_folder};
    use crate::load::load_file_as_string;
    use crate::path::to_path_buf;
    use crate::save::save_string_to_file;
    use crate::test_utils::{assert_folder_exists, get_temp_dir_path};
    use tempfile::tempdir;

    #[test]
    fn test_create_delete_folder_basic() {
        // Create a temporary directory to work in.
        let temp_dir = tempdir().unwrap();

        // Define the new folder path.
        let new_folder_path = get_temp_dir_path(&temp_dir).join("new_folder");

        // New folder path in different formats.
        let new_folder_paths: Vec<Box<dyn AsRef<Path>>> = vec![
            Box::new(new_folder_path.to_str().unwrap()), // &str
            Box::new(new_folder_path.to_str().unwrap().to_string()), // String
            Box::new(new_folder_path.as_path()),         // Path
            Box::new(new_folder_path.clone()),           // PathBuf
        ];

        // Test with all different path formats.
        for new_folder_path in new_folder_paths {
            // Get a reference to this path representation (i.e. "unbox").
            let new_folder_path: &dyn AsRef<Path> = new_folder_path.as_ref();

            // The new folder should not exist yet.
            assert!(!to_path_buf(new_folder_path).exists());

            // Create the new folder.
            create_folder(new_folder_path);

            // Now the new folder should exist.
            assert_folder_exists(new_folder_path);

            // Try creating the folder again (should not panic or error).
            create_folder(new_folder_path);

            // The new folder should still exist.
            assert_folder_exists(new_folder_path);

            // Delete the new folder.
            delete_folder(new_folder_path);
        }
    }

    #[test]
    fn test_create_folder_nested() {
        // Create a temporary directory to work in.
        let temp_dir = tempdir().unwrap();

        // Define a nested folder path.
        let nested = get_temp_dir_path(&temp_dir).join("a/b/c");

        // Create the nested folder.
        create_folder(&nested);

        // Check that the deepest directory was successfully created.
        assert_folder_exists(nested);
    }

    #[test]
    fn test_create_folder_for_file() {
        // Create a temporary directory to work in.
        let temp_dir = tempdir().unwrap();

        // Define a file path that requires a parent directory.
        let file_path = get_temp_dir_path(&temp_dir).join("a/b/c/file.txt");

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

            // The parent directory should not exist yet.
            assert!(!file_path_buf.parent().unwrap().exists());

            // Create the parent directory for the file.
            create_folder_for_file(file_path);

            // Now the parent directory should exist.
            assert_folder_exists(file_path_buf.parent().unwrap());

            // The file itself should not exist yet.
            assert!(!file_path_buf.exists());

            // Call `create_folder_for_file` again (should not panic or error).
            create_folder_for_file(file_path);

            // The parent directory should still exist.
            assert_folder_exists(file_path_buf.parent().unwrap());

            // Delete the parent directory.
            delete_folder(file_path_buf.parent().unwrap());
        }
    }

    #[test]
    fn test_copy_file() {
        // Create a temporary directory to work in.
        let temp_dir = tempdir().unwrap();

        // Define the paths for the source file.
        let source_path = get_temp_dir_path(&temp_dir).join("source.txt");
        let source_paths: Vec<Box<dyn AsRef<Path>>> = vec![
            Box::new(source_path.to_str().unwrap()),             // &str
            Box::new(source_path.to_str().unwrap().to_string()), // String
            Box::new(source_path.as_path()),                     // Path
            Box::new(source_path.clone()),                       // PathBuf
        ];

        // Define the paths for the destination file.
        let destination_path = get_temp_dir_path(&temp_dir).join("destination.txt");
        let destination_paths: Vec<Box<dyn AsRef<Path>>> = vec![
            Box::new(destination_path.clone()),   // PathBuf
            Box::new(destination_path.as_path()), // Path
            Box::new(destination_path.to_str().unwrap().to_string()), // String
            Box::new(destination_path.to_str().unwrap()), // &str
        ];

        // Test with all different path formats.
        for (source_path, destination_path) in source_paths.iter().zip(destination_paths) {
            // Get a reference to these path representations (i.e. "unbox").
            let source_path: &dyn AsRef<Path> = source_path.as_ref();
            let destination_path: &dyn AsRef<Path> = destination_path.as_ref();

            // The source and destination files shouldn't exist yet.
            assert!(!to_path_buf(source_path).exists());
            assert!(!to_path_buf(destination_path).exists());

            // Create the source file.
            save_string_to_file("Hello, world!", source_path);

            // Now the source file should exist, but the destination file should not.
            assert!(to_path_buf(source_path).exists());
            assert!(!to_path_buf(destination_path).exists());

            // Copy the file.
            copy_file(source_path, destination_path);

            // The destination file should now exist.
            assert!(to_path_buf(destination_path).exists());

            // Check that the contents of the copied file are identical.
            assert_eq!(load_file_as_string(destination_path), "Hello, world!");

            // Delete the source and destination files.
            delete_file(source_path);
            delete_file(destination_path);

            // Verify that the source and destination files no longer exist.
            assert!(!to_path_buf(source_path).exists());
            assert!(!to_path_buf(destination_path).exists());
        }
    }
}
