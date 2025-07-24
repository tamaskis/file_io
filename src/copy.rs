use crate::create::create_folder_for_file;
use std::path::Path;
use walkdir::WalkDir;

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
/// * The parent folder for the destination file will be created if it does not already exist.
/// * If the destination file already exists, it will be overwritten.
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

/// Copies a folder and its contents from one location to another.
///
/// # Arguments
///
/// * `from` - The source folder path (can be a `&str`, `String`, `Path`, or `PathBuf`).
/// * `to` - The destination folder path (can be a `&str`, `String`, `Path`, or `PathBuf`).
///
/// # Panics
///
/// If any error occurs while copying the folder or its contents.
///
/// # Note
///
/// * The desination folder and/or any of its subdirectories will be created if they do not already
///   exist.
/// * Any existing files in the destination folder will be overwritten.
///
/// # Examples
///
/// ## Using string literals
///
/// ```
/// use file_io::copy_folder;
///
/// // Copy 'src/' to 'folder/src/'.
/// let from: &str = "src";
/// let to: &str = "folder/src";
/// copy_folder(from, to);
/// ```
///
/// ## Using `Path` references
///
/// ```
/// use file_io::copy_folder;
/// use std::path::Path;
///
/// // Copy 'src/' to 'folder/src/'.
/// let from: &Path = Path::new("src");
/// let to: &Path = Path::new("folder/src");
/// copy_folder(from, to);
/// ```
pub fn copy_folder<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) {
    // Convert the input paths to `Path` references.
    let from = from.as_ref();
    let to = to.as_ref();

    // Traverse over all entries (files and folders) in the directory and its subdirectories.
    for entry in WalkDir::new(from).into_iter().filter_map(Result::ok) {
        // Get the path of the current entry.
        let entry_path = entry.path();

        // Construct the destination path.
        let destination_path = to.join(entry_path.strip_prefix(from).unwrap());

        // Copy any files (note that `WalkDir` will also traverse subdirectories, and we don't need
        // to manually create subdirectories since `copy_file` will handle that for us).
        if entry_path.is_file() {
            copy_file(entry_path, &destination_path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::delete::{delete_file, delete_folder};
    use crate::load::load_file_as_string;
    use crate::path::to_path_buf;
    use crate::save::save_string_to_file;
    use crate::test_utils::get_temp_dir_path;
    use std::path::Path;
    use tempfile::tempdir;

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

        // Create the source file.
        save_string_to_file("Hello, world!", &source_path);

        // Test with all different path formats.
        for (source_path, destination_path) in source_paths.iter().zip(destination_paths) {
            // Get a reference to these path representations (i.e. "unbox").
            let source_path: &dyn AsRef<Path> = source_path.as_ref();
            let destination_path: &dyn AsRef<Path> = destination_path.as_ref();

            // The destination file shouldn't exist yet.
            assert!(!to_path_buf(destination_path).exists());

            // Copy the file.
            copy_file(source_path, destination_path);

            // The destination file should now exist.
            assert!(to_path_buf(destination_path).exists());

            // Check that the contents of the copied file are identical.
            assert_eq!(load_file_as_string(destination_path), "Hello, world!");

            // Delete the destination file.
            delete_file(destination_path);

            // Verify that the destination file no longer exists.
            assert!(!to_path_buf(destination_path).exists());
        }
    }

    #[test]
    fn test_copy_file_with_existing_destination() {
        // Create a temporary directory to work in.
        let temp_dir = tempdir().unwrap();

        // Define the source file path.
        let source_path = get_temp_dir_path(&temp_dir).join("source.txt");

        // Create the source file.
        save_string_to_file("Hello, world!", &source_path);

        // Define the destination file path.
        let destination_path = get_temp_dir_path(&temp_dir).join("destination.txt");

        // Create the destination file with different content.
        save_string_to_file("Old content", &destination_path);

        // Copy the source file to the destination file.
        copy_file(&source_path, &destination_path);

        // Verify that the contents of the destination file have been overwritten.
        assert_eq!(load_file_as_string(&destination_path), "Hello, world!");
    }

    #[test]
    fn test_copy_folder_flat() {
        // Create a temporary directory to work in.
        let temp_dir = tempdir().unwrap();

        // Define the paths for the source folder.
        let source_path = get_temp_dir_path(&temp_dir).join("source_folder");
        let source_paths: Vec<Box<dyn AsRef<Path>>> = vec![
            Box::new(source_path.to_str().unwrap()),             // &str
            Box::new(source_path.to_str().unwrap().to_string()), // String
            Box::new(source_path.as_path()),                     // Path
            Box::new(source_path.clone()),                       // PathBuf
        ];

        // Define the paths for the destination folder.
        let destination_path = get_temp_dir_path(&temp_dir).join("destination_folder");
        let destination_paths: Vec<Box<dyn AsRef<Path>>> = vec![
            Box::new(destination_path.clone()),   // PathBuf
            Box::new(destination_path.as_path()), // Path
            Box::new(destination_path.to_str().unwrap().to_string()), // String
            Box::new(destination_path.to_str().unwrap()), // &str
        ];

        // Define the source folder path.
        let source_path = get_temp_dir_path(&temp_dir).join("source_folder");

        // Create files in the source folder.
        save_string_to_file("Hello, world!", source_path.join("file_1.txt"));
        save_string_to_file("hello world", source_path.join("file_2.txt"));

        // Test with all different path formats.
        for (source_path, destination_path) in source_paths.iter().zip(destination_paths) {
            // Get a reference to these path representations (i.e. "unbox").
            let source_path: &dyn AsRef<Path> = source_path.as_ref();
            let destination_path: &dyn AsRef<Path> = destination_path.as_ref();

            // The destination folder shouldn't exist yet.
            assert!(!to_path_buf(destination_path).exists());

            // Copy the source folder to the destination folder.
            copy_folder(source_path, destination_path);

            // The destination folder should now exist.
            assert!(to_path_buf(destination_path).exists());

            // Verify that the files were copied correctly.
            assert_eq!(
                load_file_as_string(to_path_buf(destination_path).join("file_1.txt")),
                "Hello, world!"
            );
            assert_eq!(
                load_file_as_string(to_path_buf(destination_path).join("file_2.txt")),
                "hello world"
            );

            // Delete the desination folder.
            delete_folder(destination_path);

            // Verify that the destination folder no longer exists.
            assert!(!to_path_buf(destination_path).exists());
        }
    }

    #[test]
    fn test_copy_folder_nested() {
        // Create a temporary directory to work in.
        let temp_dir = tempdir().unwrap();

        // Define the source folder path.
        let source_folder = get_temp_dir_path(&temp_dir).join("source_folder");

        // Create a file in the source folder.
        save_string_to_file("Hello, world!", source_folder.join("file.txt"));

        // Create a file in the subfolder.
        save_string_to_file(
            "Hello from subfolder!",
            source_folder.join("subfolder/subfile.txt"),
        );

        // Define the destination folder path.
        let destination_folder = get_temp_dir_path(&temp_dir).join("destination_folder");

        // Copy the source folder to the destination folder.
        copy_folder(&source_folder, &destination_folder);

        // Verify that the files were copied correctly.
        assert_eq!(
            load_file_as_string(destination_folder.join("file.txt")),
            "Hello, world!"
        );
        assert_eq!(
            load_file_as_string(destination_folder.join("subfolder/subfile.txt")),
            "Hello from subfolder!"
        );
    }

    #[test]
    fn test_copy_folder_with_existing_destination() {
        // Create a temporary directory to work in.
        let temp_dir = tempdir().unwrap();

        // Define the source folder path.
        let source_folder = get_temp_dir_path(&temp_dir).join("source_folder");

        // Create files in the source folder.
        save_string_to_file("Hello, world!", source_folder.join("file.txt"));
        save_string_to_file(
            "Overwrite existing file",
            source_folder.join("existing_file.txt"),
        );

        // Create a file in a subfolder.
        save_string_to_file(
            "Hello from subfolder!",
            source_folder.join("subfolder/subfile.txt"),
        );

        // Define the destination folder path.
        let destination_folder = get_temp_dir_path(&temp_dir).join("destination_folder");

        // Create the destination folder and a file in it.
        save_string_to_file(
            "Existing file",
            destination_folder.join("existing_file.txt"),
        );

        // Copy the source folder to the destination folder.
        copy_folder(&source_folder, &destination_folder);

        // Verify that the files were copied correctly. Note that the existing file should be
        // overwritten.
        assert_eq!(
            load_file_as_string(destination_folder.join("file.txt")),
            "Hello, world!"
        );
        assert_eq!(
            load_file_as_string(destination_folder.join("existing_file.txt")),
            "Overwrite existing file"
        );
        assert_eq!(
            load_file_as_string(destination_folder.join("subfolder/subfile.txt")),
            "Hello from subfolder!"
        );
    }
}
