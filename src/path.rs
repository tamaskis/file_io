use std::path::{Path, PathBuf};

/// Retrieves the user's home directory from the `$HOME` environment variable.
///
/// # Returns
///
/// Path to the user's home directory.
///
/// # Panics
///
/// If the `$HOME` environment variable is not set.
///
/// # Example
///
/// ```
/// use file_io::get_home;
///
/// let home: String = get_home();
/// ```
pub fn get_home() -> String {
    std::env::var("HOME").unwrap()
}

/// Get the current working directory.
///
/// # Returns
///
/// Current working directory.
///
/// # Panics
///
/// If the current directory cannot be determined.
///
/// # Example
///
/// ```
/// use file_io::get_cwd;
/// use std::path::PathBuf;
///
/// let cwd: PathBuf = get_cwd();
/// ```
pub fn get_cwd() -> PathBuf {
    std::env::current_dir().unwrap()
}

/// Get the last component of a path (file or folder name).
///
/// # Arguments
///
/// * `path` - Path (can be a `&str`, `String`, `Path`, or `PathBuf`).
///
/// # Returns
///
/// The last component of the path.
///
/// # Panics
///
/// If the last path component cannot be determined.
///
/// # Examples
///
/// ## File path
///
/// ```
/// use std::path::Path;
/// use file_io::get_last_path_component;
///
/// let name = get_last_path_component("/some/path/to/file.txt");
/// assert_eq!(name, "file.txt");
/// ```
///
/// ## Folder path
///
/// ```
/// use std::path::Path;
/// use file_io::get_last_path_component;
///
/// let name = get_last_path_component("some/path/to/folder");
/// assert_eq!(name, "folder");
/// ```
pub fn get_last_path_component<P: AsRef<Path>>(path: P) -> String {
    path.as_ref()
        .components()
        .next_back()
        .map(|comp| comp.as_os_str().to_string_lossy().into_owned())
        .unwrap()
}

/// Change the current working directory.
///
/// # Arguments
///
/// * `path` - The path to change the current working directory to (can be a `&str`, `String`,
///   `Path`, or `PathBuf`).
///
/// # Panics
///
/// If `path` does not exist or cannot be accessed.
///
/// # Example
///
/// ```
/// use file_io::{cd, get_cwd};
/// use std::path::Path;
///
/// // Store the current directory before changing it.
/// let original_dir = get_cwd();
///
/// // Verify we are in the `file-io` directory.
/// assert!(original_dir.ends_with("file-io"));
///
/// // Change to the `src` directory.
/// let new_dir = original_dir.join("src");
/// cd(&new_dir);
///
/// // Verify the current directory has changed.
/// assert_eq!(get_cwd(), new_dir);
///
/// // Change back to the original directory.
/// cd(&original_dir);
///
/// // Verify we are back in the original directory.
/// assert_eq!(get_cwd(), original_dir);
/// ```
pub fn cd<P: AsRef<Path>>(path: P) {
    std::env::set_current_dir(path).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_cwd;
    use temp_env::with_var;

    #[test]
    fn test_get_home() {
        with_var("HOME", Some("/tmp/test_home"), || {
            let home = get_home();
            assert_eq!(home, "/tmp/test_home");
        });
    }

    #[test]
    fn test_get_cwd() {
        assert_eq!(get_last_path_component(get_cwd()), "file-io");
    }

    #[test]
    fn test_get_last_path_component() {
        assert_eq!(
            get_last_path_component("/some/path/to/file.txt"),
            "file.txt"
        );
        assert_eq!(get_last_path_component("some/path/to/file.txt"), "file.txt");
        assert_eq!(get_last_path_component("/some/path/to/folder/"), "folder");
        assert_eq!(get_last_path_component("/some/path/to/folder"), "folder");
        assert_eq!(get_last_path_component("some/path/to/folder/"), "folder");
        assert_eq!(get_last_path_component("some/path/to/folder"), "folder");
        assert_eq!(get_last_path_component("/file.txt"), "file.txt");
        assert_eq!(get_last_path_component("file.txt"), "file.txt");
        assert_eq!(get_last_path_component("/folder/"), "folder");
        assert_eq!(get_last_path_component("/folder"), "folder");
        assert_eq!(get_last_path_component("folder/"), "folder");
        assert_eq!(get_last_path_component("folder"), "folder");
    }

    #[test]
    fn test_cd() {
        // Store the current directory before changing it.
        let original_dir = get_cwd();

        // Verify we are in the `file-io` directory.
        assert!(original_dir.ends_with("file-io"));

        // Change to the `src` directory.
        let new_dir = original_dir.join("src");
        cd(&new_dir);

        // Verify the current directory has changed.
        assert_eq!(get_cwd(), new_dir);

        // Change back to the original directory.
        cd(&original_dir);

        // Verify we are back in the original directory.
        assert_eq!(get_cwd(), original_dir);
    }
}
