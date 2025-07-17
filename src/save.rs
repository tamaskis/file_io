use crate::create::create_folder_for_file;
use std::path::Path;

/// Saves a string to a file at the specified path.
///
/// # Arguments
///
/// * `content` - The string content to save to the file.
/// * `path` - The path where the file should be saved (can be a `&str`, `String`, `Path`, or
///   `PathBuf`).
///
/// # Panics
///
/// If some error is encountered while creating the file or writing to it.
///
/// # Examples
///
/// ## Using a string literal
///
/// ```
/// use file_io::save_string_to_file;
///
/// let content: &str = "Hello, world!";
/// let path: &str = "folder/subfolder_11/file_6.txt";
///
/// save_string_to_file(content, path);
/// ```
///
/// ## Using a `Path` reference
///
/// ```
/// use file_io::save_string_to_file;
/// use std::path::Path;
///
/// let content: &str = "Hello, world!";
/// let path: &Path = Path::new("folder/subfolder_12/file_7.txt");
///
/// save_string_to_file(content, path);
/// ```
pub fn save_string_to_file<P: AsRef<Path>>(content: &str, path: P) {
    let path = path.as_ref();
    create_folder_for_file(path);
    std::fs::write(path, content).unwrap_or_else(|_| panic!("Failed to write to file '{path:?}'."));
}
