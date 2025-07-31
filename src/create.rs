use std::path::Path;

/// Creates a new folder at the specified path if it does not already exist.
///
/// # Arguments
///
/// * `path` - The path where the folder should be created (can be a `&str`, [`String`], [`Path`],
///   or [`std::path::PathBuf`]).
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
///   [`String`], [`Path`], or [`std::path::PathBuf`]).
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::delete::delete_folder;
    use crate::path::to_path_buf;
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
}
