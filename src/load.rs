use std::path::Path;

/// Loads the content of a file as a string.
///
/// # Arguments
///
/// * `path` - The path to the file to load (can be a `&str`, [`String`], [`Path`], or
///   [`std::path::PathBuf`]).
///
/// # Returns
///
/// The contents of the file as a string.
///
/// # Panics
///
/// If the file cannot be read.
///
/// # Examples
///
/// ## Using a string literal
///
/// ```
/// use file_io::{load_file_as_string, save_string_to_file};
///
/// // Define the content and the path.
/// let content: &str = "Hello, world!";
/// let path: &str = "folder/subfolder_6/file_3.txt";
///
/// // First, save the content to the file.
/// save_string_to_file(content, path);
///
/// // Now, load the content back from the file.
/// let loaded_content = load_file_as_string(path);
///
/// // Verify that the loaded content matches the original content.
/// assert_eq!(loaded_content, content);
/// ```
///
/// ## Using a `Path` reference
///
/// ```
/// use file_io::{load_file_as_string, save_string_to_file};
/// use std::path::Path;
///
/// // Define the content and the path.
/// let content: &str = "Hello, world!";
/// let path: &Path = Path::new("folder/subfolder_7/file_4.txt");
///
/// // First, save the content to the file.
/// save_string_to_file(content, path);
///
/// // Now, load the content back from the file.
/// let loaded_content = load_file_as_string(path);
///
/// // Verify that the loaded content matches the original content.
/// assert_eq!(loaded_content, content);
/// ```
pub fn load_file_as_string<P: AsRef<Path>>(path: P) -> String {
    let path = path.as_ref();
    std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read file at '{path:?}'."))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::save::save_string_to_file;
    use crate::test_utils::get_temp_dir_path;
    use tempfile::tempdir;

    #[test]
    fn test_save_load_file_string() {
        // Create a temporary directory.
        let temp_dir = tempdir().unwrap();

        // Get the path to the temporary directory.
        let temp_dir_path = get_temp_dir_path(&temp_dir);

        // Path to the file.
        let file_path = temp_dir_path.join("test_file.txt");

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

            // Content to save in the file.
            let content = "Hello, world!";

            // Save the content to the file.
            save_string_to_file(content, file_path);

            // Load the content from the file.
            let loaded_content = load_file_as_string(file_path);

            // Verify that the loaded content matches the original content.
            assert_eq!(loaded_content, content);
        }
    }
}
