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
    std::env::var("HOME").expect("HOME environment variable is not set.")
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
    std::env::current_dir().expect("Failed to get the current working directory.")
}

/// Get the last component of a path (file or folder name).
///
/// # Arguments
///
/// * `path` - Path (can be a `&str`, [`String`], [`Path`], or [`PathBuf`]).
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
/// use file_io::get_last_path_component;
///
/// let name = get_last_path_component("/some/path/to/file.txt");
/// assert_eq!(name, "file.txt");
/// ```
///
/// ## Folder path
///
/// ```
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

/// Get the file name (including any extension).
///
/// # Arguments
///
/// * `path` - The path to the file (can be a `&str`, [`String`], [`Path`], or [`PathBuf`]).
///
/// # Returns
///
/// The file name (including any extension).
///
/// # Panics
///
/// If the file name cannot be determined.
///
/// # Example
///
/// ```
/// use file_io::get_file_name;
///
/// let file_name = get_file_name("/some/path/to/file.txt");
/// assert_eq!(file_name, "file.txt");
/// ```
pub fn get_file_name<P: AsRef<Path>>(path: P) -> String {
    path.as_ref()
        .file_name()
        .and_then(|s| s.to_str())
        .map(String::from)
        .expect("Failed to get the file name.")
}

/// Get the file stem (i.e. file name without its extension).
///
/// # Arguments
///
/// * `path` - The path to the file (can be a `&str`, [`String`], [`Path`], or [`PathBuf`]).
///
/// # Returns
///
/// The file stem (i.e. the file name without its extension).
///
/// # Panics
///
/// If the file stem cannot be determined.
///
/// # Example
///
/// ```
/// use file_io::get_file_stem;
///
/// let file_stem = get_file_stem("/some/path/to/file.txt");
/// assert_eq!(file_stem, "file");
/// ```
pub fn get_file_stem<P: AsRef<Path>>(path: P) -> String {
    path.as_ref()
        .file_stem()
        .and_then(|s| s.to_str())
        .map(String::from)
        .expect("Failed to get the file stem.")
}

/// Get the file extension.
///
/// # Arguments
///     
/// * `path` - The path to the file (can be a `&str`, [`String`], [`Path`], or [`PathBuf`]).
///
/// # Returns
///
/// The file extension. If the file has no extension, or if the extension cannot be determined, this
/// function returns an empty string.
///
/// # Example
///
/// ```
/// use file_io::get_file_extension;
///
/// let file_extension = get_file_extension("/some/path/to/file.txt");
/// assert_eq!(file_extension, "txt");
/// ```
pub fn get_file_extension<P: AsRef<Path>>(path: P) -> String {
    path.as_ref()
        .extension()
        .and_then(|s| s.to_str())
        .map(String::from)
        .unwrap_or(String::from(""))
}

/// Converts a path to a `PathBuf`.
///
/// # Arguments
///
/// * `path` - The path to convert (can be a `&str`, [`String`], [`Path`], or [`PathBuf`]).
///
/// # Returns
///
/// A `PathBuf` representation of the path.
///
/// # Examples
///
/// ## Using a string literal
///
/// ```
/// use file_io::to_path_buf;
/// use std::path::PathBuf;
///
/// let path: &str = "folder/subfolder_9/file.txt";
/// let path_buf: PathBuf = to_path_buf(path);
/// ```
///
/// ## Using a `Path` reference
///
/// ```
/// use file_io::to_path_buf;
/// use std::path::{Path, PathBuf};
///
/// let path: &Path = Path::new("folder/subfolder_10/file.txt");
/// let path_buf: PathBuf = to_path_buf(path);
/// ```
pub fn to_path_buf<P: AsRef<Path>>(path: P) -> PathBuf {
    path.as_ref().to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_cwd;
    use serial_test::serial;
    use temp_env::with_var;

    #[test]
    fn test_get_home() {
        with_var("HOME", Some("/tmp/test_home"), || {
            let home = get_home();
            assert_eq!(home, "/tmp/test_home");
        });
    }

    #[test]
    #[serial]
    fn test_get_cwd() {
        assert_eq!(get_last_path_component(get_cwd()), "file_io");
    }

    #[test]
    fn test_get_last_path_component_str() {
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
    fn test_get_last_path_component_other_type_spot_checks() {
        // Spot check with `String`.
        assert_eq!(
            get_last_path_component(String::from("/some/path/to/file.txt")),
            "file.txt"
        );

        // Spot check with `Path`.
        assert_eq!(
            get_last_path_component(Path::new("/some/path/to/file.txt")),
            "file.txt"
        );

        // Spot check with `PathBuf`.
        assert_eq!(
            get_last_path_component(PathBuf::from("/some/path/to/file.txt")),
            "file.txt"
        );
    }

    #[test]
    fn test_get_file_name_str() {
        assert_eq!(get_file_name("/some/path/to/file.txt"), "file.txt");
        assert_eq!(get_file_name("some/path/to/file.txt"), "file.txt");
        assert_eq!(get_file_name("/file.txt"), "file.txt");
        assert_eq!(get_file_name("file.txt"), "file.txt");
        assert_eq!(get_file_name("/some/path/to/file"), "file");
        assert_eq!(get_file_name("some/path/to/file"), "file");
        assert_eq!(get_file_name("/file"), "file");
        assert_eq!(get_file_name("file"), "file");
    }

    #[test]
    fn test_get_file_name_other_type_spot_checks() {
        // Spot check with `String`.
        assert_eq!(
            get_file_name(String::from("/some/path/to/file.txt")),
            "file.txt"
        );

        // Spot check with `Path`.
        assert_eq!(
            get_file_name(Path::new("/some/path/to/file.txt")),
            "file.txt"
        );

        // Spot check with `PathBuf`.
        assert_eq!(
            get_file_name(PathBuf::from("/some/path/to/file.txt")),
            "file.txt"
        );
    }

    #[test]
    fn test_get_file_stem_str() {
        assert_eq!(get_file_stem("/some/path/to/file.txt"), "file");
        assert_eq!(get_file_stem("some/path/to/file.txt"), "file");
        assert_eq!(get_file_stem("/file.txt"), "file");
        assert_eq!(get_file_stem("file.txt"), "file");
        assert_eq!(get_file_stem("/some/path/to/file"), "file");
        assert_eq!(get_file_stem("some/path/to/file"), "file");
        assert_eq!(get_file_stem("/file"), "file");
        assert_eq!(get_file_stem("file"), "file");
    }

    #[test]
    fn test_get_file_stem_other_type_spot_checks() {
        // Spot check with `String`.
        assert_eq!(
            get_file_stem(String::from("/some/path/to/file.txt")),
            "file"
        );

        // Spot check with `Path`.
        assert_eq!(get_file_stem(Path::new("/some/path/to/file.txt")), "file");

        // Spot check with `PathBuf`.
        assert_eq!(
            get_file_stem(PathBuf::from("/some/path/to/file.txt")),
            "file"
        );
    }

    #[test]
    fn test_get_file_extension_str() {
        assert_eq!(get_file_extension("/some/path/to/file.txt"), "txt");
        assert_eq!(get_file_extension("some/path/to/file.txt"), "txt");
        assert_eq!(get_file_extension("/file.txt"), "txt");
        assert_eq!(get_file_extension("file.txt"), "txt");
        assert_eq!(get_file_extension("/some/path/to/file"), "");
        assert_eq!(get_file_extension("some/path/to/file"), "");
        assert_eq!(get_file_extension("/file"), "");
        assert_eq!(get_file_extension("file"), "");
    }

    #[test]
    fn test_get_file_extension_other_type_spot_checks() {
        // Spot check with `String`.
        assert_eq!(
            get_file_extension(String::from("/some/path/to/file.txt")),
            "txt"
        );

        // Spot check with `Path`.
        assert_eq!(
            get_file_extension(Path::new("/some/path/to/file.txt")),
            "txt"
        );

        // Spot check with `PathBuf`.
        assert_eq!(
            get_file_extension(PathBuf::from("/some/path/to/file.txt")),
            "txt"
        );
    }

    #[test]
    fn test_to_path_buf() {
        // Test with a `&str`.
        let path_str: &str = "folder/subfolder/file.txt";
        let path_buf: PathBuf = to_path_buf(path_str);
        assert_eq!(path_buf.to_str().unwrap(), path_str);

        // Test with a `String`.
        let path_string: String = String::from("folder/subfolder/file.txt");
        let path_buf: PathBuf = to_path_buf(path_string);
        assert_eq!(path_buf.to_str().unwrap(), "folder/subfolder/file.txt");

        // Test with a `Path`.
        let path: &Path = Path::new("folder/subfolder/file.txt");
        let path_buf: PathBuf = to_path_buf(path);
        assert_eq!(path_buf.to_str().unwrap(), "folder/subfolder/file.txt");

        // Test with a `PathBuf`.
        let path_buf_input: PathBuf = PathBuf::from("folder/subfolder/file.txt");
        let path_buf_output: PathBuf = to_path_buf(path_buf_input);
        assert_eq!(
            path_buf_output.to_str().unwrap(),
            "folder/subfolder/file.txt"
        );
    }
}
