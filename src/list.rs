use std::path::{Path, PathBuf};

/// Lists the contents of a folder at the specified path.
///
/// # Arguments
///
/// * `path` - The path to the folder (can be a `&str`, `String`, `Path`, or `PathBuf`).
///
/// # Returns
///
/// Paths of the files and folders in the specified directory (in alphabetical order). Note that
/// folders are included in the list, but their contents are not recursively listed.
///
/// # Panics
///
/// If the provided path is not a folder or if an error occurs while reading the folder.
///
/// # Examples
///
/// ## Using a string literal
///
/// ```
/// use file_io::list_folder_contents;
/// use std::path::PathBuf;
///     
/// let contents: Vec<PathBuf> = list_folder_contents(".vscode");
///
/// assert_eq!(
///     contents,
///     vec![PathBuf::from(".vscode/extensions.json"), PathBuf::from(".vscode/settings.json")]
/// );
/// ```
///
/// ## Using a `Path` reference
///
/// ```
/// use file_io::list_folder_contents;
/// use std::path::{Path, PathBuf};
///
/// let contents: Vec<PathBuf> = list_folder_contents(Path::new(".vscode"));
///
/// assert_eq!(
///    contents,
///    vec![PathBuf::from(".vscode/extensions.json"), PathBuf::from(".vscode/settings.json")]
/// );
/// ```
pub fn list_folder_contents<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    // Convert the input path to a Path reference.
    let path = path.as_ref();

    // Ensure the path is a folder.
    if !path.is_dir() {
        panic!("The provided path is not a folder: {path:?}");
    }

    // Read the folder entries into a vector.
    let mut entries = match std::fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .map(|e| e.path())
            .collect::<Vec<PathBuf>>(),
        Err(_) => panic!("Failed to read directory: {path:?}"),
    };

    // Sort the entries alphabetically.
    entries.sort();

    entries
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::save::save_string_to_file;
    use crate::test_utils::get_temp_dir_path;
    use tempfile::tempdir;

    #[test]
    fn test_list_folder_contents() {
        // Create a temporary directory to work in.
        let temp_dir = tempdir().unwrap();

        // Get the path to the temporary directory.
        let temp_dir_path = get_temp_dir_path(&temp_dir);

        // Create some test files and folders.
        save_string_to_file("Content 1", temp_dir_path.join("file1.txt"));
        save_string_to_file("Content 2", temp_dir_path.join("file2.txt"));
        save_string_to_file("Content 3", temp_dir_path.join("subfolder/file3.txt"));

        // List the contents of the temporary directory.
        let contents = list_folder_contents(&temp_dir_path);

        // Check that the contents are as expected.
        assert_eq!(
            contents,
            vec![
                temp_dir_path.join("file1.txt"),
                temp_dir_path.join("file2.txt"),
                temp_dir_path.join("subfolder")
            ]
        );
    }
}
