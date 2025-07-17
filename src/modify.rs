use crate::load::load_file_as_string;
use crate::save::save_string_to_file;
use std::panic;
use std::path::Path;
use walkdir::WalkDir;

/// Replaces all occurrences of a string in a file.
///
/// # Arguments
///
/// * `path` - Path to the file where the replacements will be performed (can be a `&str`, `String`,
///   `Path`, or `PathBuf`).
/// * `old_string` - The substring to find and replace in all files.
/// * `new_string` - The replacement string.
///
/// # Panics
///
/// If some error is encountered while reading from or writing to the file.
///
/// # Example
///
/// ```
/// use file_io::{load_file_as_string, replace_str_in_file, save_string_to_file};
/// use std::path::Path;
///
/// // Path to file.
/// let path: &Path = Path::new("folder/subfolder_8/file_5.txt");
///
/// // Create a file with some content.
/// save_string_to_file("Hello, world!", path);
///
/// // Replace "Hello" with "Goodbye".
/// replace_str_in_file(path, "Hello", "Goodbye");
///
/// // Verify that the content was replaced.
/// let content = load_file_as_string(path);
/// assert_eq!(content, "Goodbye, world!");
/// ```
pub fn replace_str_in_file<P: AsRef<Path>>(path: P, old_string: &str, new_string: &str) {
    // Load the file into a string.
    let content = load_file_as_string(&path);

    // Replace all instances of `old_string` with `new_string`.
    if content.contains(old_string) {
        let new_content = content.replace(old_string, new_string);
        save_string_to_file(&new_content, path);
    }
}

/// Replaces all occurrences of a string in all files within a directory (including subdirectories).
///
/// # Arguments
///
/// * `path` - Path to the directory or file where the replacements will be performed (can be a
///   `&str`, `String`, `Path`, or `PathBuf`).
/// * `old_string` - The substring to find and replace in all files.
/// * `new_string` - The replacement string.
///
/// # Note
///
/// This function will not panic if a single read/write fails (since this function may pull in
/// private, inaccessible files). However, a warning will be printed to `stderr`.
///
/// # Examples
///
/// ```ignore
/// use file_io::replace_str_in_files;
///
/// let dir = Path::new("/path/to/folder");
///
/// // Replace "foo" with "bar" in all files within the "/path/to/folder/" directory (including
/// // subdirectories).
/// replace_str_in_files(dir, "foo", "bar");
/// ```
pub fn replace_str_in_files<P: AsRef<Path>>(path: P, old_string: &str, new_string: &str) {
    // Iterate over all entries (files and folders) in the directory and its subdirectories.
    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        // Get the path of the current entry.
        let entry_path = entry.path();

        // If the entry is a file, replace any instances of `old_string` with `new_string`.
        if entry_path.is_file() {
            // We use `panic::catch_unwind` to handle any potential panics gracefully (since some
            // folders could have private, inaccessible files).
            let result =
                panic::catch_unwind(|| replace_str_in_file(entry_path, old_string, new_string));

            // If the replacement failed, print an error message to `stderr`.
            if result.is_err() {
                eprintln!(
                    "Failed to replace string in file '{}'.",
                    entry_path.display(),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::get_temp_dir_path;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_replace_str_in_file() {
        // Create a temporary directory.
        let temp_dir = tempdir().unwrap();

        // Get the path to the temporary directory.
        let temp_dir_path = get_temp_dir_path(&temp_dir);

        // File path.
        let file_path: PathBuf = temp_dir_path.join("test_file.txt");

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

            // Create a file with some content.
            save_string_to_file("Hello, world, hello, Hello!", file_path);

            // Replace "Hello" with "Goodbye".
            replace_str_in_file(file_path, "Hello", "Goodbye");

            // Verify that the content was replaced.
            let content = load_file_as_string(file_path);
            assert_eq!(content, "Goodbye, world, hello, Goodbye!");
        }
    }

    #[test]
    fn test_replace_str_in_files_basic() {
        // Create a temporary directory.
        let temp_dir = tempdir().unwrap();

        // Get the path to the temporary directory.
        let temp_dir_path = get_temp_dir_path(&temp_dir);

        // Paths to files.
        let file_1_path = temp_dir_path.join("file_1.txt");
        let file_2_path = temp_dir_path.join("file_2.txt");
        let file_3_path = temp_dir_path.join("file_3.txt");

        // File paths in different formats.
        let file_1_paths: Vec<Box<dyn AsRef<Path>>> = vec![
            Box::new(file_1_path.to_str().unwrap()),             // &str
            Box::new(file_1_path.to_str().unwrap().to_string()), // String
            Box::new(file_1_path.as_path()),                     // Path
            Box::new(file_1_path.clone()),                       // PathBuf
        ];
        let file_2_paths: Vec<Box<dyn AsRef<Path>>> = vec![
            Box::new(file_2_path.to_str().unwrap()),             // &str
            Box::new(file_2_path.to_str().unwrap().to_string()), // String
            Box::new(file_2_path.as_path()),                     // Path
            Box::new(file_2_path.clone()),                       // PathBuf
        ];
        let file_3_paths: Vec<Box<dyn AsRef<Path>>> = vec![
            Box::new(file_3_path.to_str().unwrap()),             // &str
            Box::new(file_3_path.to_str().unwrap().to_string()), // String
            Box::new(file_3_path.as_path()),                     // Path
            Box::new(file_3_path.clone()),                       // PathBuf
        ];

        // Contents of the files.
        let file_1_contents = "hello foo world";
        let file_2_contents = "no foo here";
        let file_3_contents = "nothing to replace";

        // Test with all different path formats.
        for ((file_1_path, file_2_path), file_3_path) in
            file_1_paths.into_iter().zip(file_2_paths).zip(file_3_paths)
        {
            // Get a reference to the path representations (i.e. "unbox").
            let file_1_path = file_1_path.as_ref();
            let file_2_path = file_2_path.as_ref();
            let file_3_path = file_3_path.as_ref();

            // Create files with known content.
            save_string_to_file(file_1_contents, file_1_path);
            save_string_to_file(file_2_contents, file_2_path);
            save_string_to_file(file_3_contents, file_3_path);

            // Run the replacement function.
            replace_str_in_files(&temp_dir_path, "foo", "bar");

            // Check that file 1 content changed.
            let content1 = load_file_as_string(file_1_path);
            assert_eq!(content1, "hello bar world");

            // Check that file 2 content changed.
            let content2 = load_file_as_string(file_2_path);
            assert_eq!(content2, "no bar here");

            // Check that file 3 content is unchanged.
            let content3 = load_file_as_string(file_3_path);
            assert_eq!(content3, "nothing to replace");
        }
    }

    #[test]
    fn test_replace_str_in_files_nested() {
        // Create a temporary directory.
        let temp_dir = tempdir().unwrap();

        // Get the path to the temporary directory.
        let temp_dir_path = get_temp_dir_path(&temp_dir);

        // File paths.
        let root_file_path = temp_dir_path.join("root.txt");
        let nested_file_path = temp_dir_path.join("nested/nested.txt");

        // File paths in different formats.
        let root_file_paths: Vec<Box<dyn AsRef<Path>>> = vec![
            Box::new(root_file_path.to_str().unwrap()), // &str
            Box::new(root_file_path.to_str().unwrap().to_string()), // String
            Box::new(root_file_path.as_path()),         // Path
            Box::new(root_file_path.clone()),           // PathBuf
        ];
        let nested_file_paths: Vec<Box<dyn AsRef<Path>>> = vec![
            Box::new(nested_file_path.to_str().unwrap()), // &str
            Box::new(nested_file_path.to_str().unwrap().to_string()), // String
            Box::new(nested_file_path.as_path()),         // Path
            Box::new(nested_file_path.clone()),           // PathBuf
        ];

        // Test with all different path formats.
        for (root_file_path, nested_file_path) in root_file_paths.into_iter().zip(nested_file_paths)
        {
            // Get a reference to the path representations (i.e. "unbox").
            let root_file_path = root_file_path.as_ref();
            let nested_file_path = nested_file_path.as_ref();

            // Create files in the root and nested directories.
            save_string_to_file("replace me", root_file_path);
            save_string_to_file("replace me too", nested_file_path);

            // Replace "replace" with "changed".
            replace_str_in_files(temp_dir.path(), "replace", "changed");

            // Check root file content.
            let root_content = load_file_as_string(root_file_path);
            assert_eq!(root_content, "changed me");

            // Check nested file content.
            let nested_content = load_file_as_string(nested_file_path);
            assert_eq!(nested_content, "changed me too");
        }
    }
}
