use crate::path::get_cwd;
use std::path::{Path, PathBuf};

/// A struct that changes the current working directory to a specified path.
///
/// When an instance of this struct goes out of scope (i.e. it is dropped), it automatically
/// restores the original current working directory.
#[must_use]
pub struct CdGuard {
    /// Path to the original current working directory.
    original_cwd: PathBuf,
}

impl CdGuard {
    /// Constructor.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to change the current working directory to (can be a `&str`, [`String`],
    ///   [`Path`], or [`PathBuf`]).
    ///
    /// # Returns
    ///
    /// An instance of [`CdGuard`] that will restore the original directory when dropped.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();
        let original_cwd = get_cwd();
        std::env::set_current_dir(path)
            .unwrap_or_else(|_| panic!("Failed to change directory to '{path:?}'."));
        Self { original_cwd }
    }
}

// Restore the original directory when `cd` goes out of scope.
impl Drop for CdGuard {
    fn drop(&mut self) {
        let original_cwd = self.original_cwd.clone();
        std::env::set_current_dir(&original_cwd)
            .unwrap_or_else(|_| panic!("Failed to change directory to '{original_cwd:?}'."))
    }
}

/// Change the current working directory.
///
/// This function works by creating a [`CdGuard`] instance. When the [`CdGuard`] instance goes out
/// of scope (i.e. when it is dropped), the original current working directory is automatically
/// restored.
///
/// # Arguments
///
/// * `path` - The path to change the current working directory to (can be a `&str`, [`String`],
///   [`Path`], or [`PathBuf`]).
///
/// # Returns
///
/// A [`CdGuard`] instance that will automatically restore the original current working directory
/// when it goes out of scope (i.e. when it is dropped).
///
/// # Panics
///
/// If `path` does not exist or cannot be accessed.
///
/// # Example
///
/// ```
/// use file_io::{cd, get_cwd};
///
/// // Get the path to the original current working directory.
/// let original_cwd_path = get_cwd();
///
/// // Verify we are in the `file_io` directory.
/// assert!(original_cwd_path.ends_with("file_io"));
///
/// // Define the directory to change to.
/// let src_path = original_cwd_path.join("src");
///
/// // Enter a new scope.
/// {
///     // Change to the `src` directory within this limited scope.
///     let _cd = cd(&src_path);
///
///     // Verify the current directory has changed.
///     assert_eq!(get_cwd(), src_path);
/// }
///
/// // Verify that outside the scope, we are back in the original directory.
/// assert_eq!(get_cwd(), original_cwd_path);
/// ```
pub fn cd<P: AsRef<Path>>(path: P) -> CdGuard {
    CdGuard::new(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create::create_folder;
    use crate::path::to_path_buf;
    use crate::test_utils::get_temp_dir_path;
    use serial_test::serial;
    use tempfile::tempdir;

    #[test]
    #[serial]
    fn test_cd_without_scope() {
        // Get the path to the original current working directory.
        let original_cwd_path = get_cwd();

        // Verify we are in the `file_io` directory.
        assert!(original_cwd_path.ends_with("file_io"));

        // Define the directory to change to.
        let src_path = original_cwd_path.join("src");

        // src directory path in different formats.
        let src_paths: Vec<Box<dyn AsRef<Path>>> = vec![
            Box::new(src_path.to_str().unwrap()),             // &str
            Box::new(src_path.to_str().unwrap().to_string()), // String
            Box::new(src_path.as_path()),                     // Path
            Box::new(src_path.clone()),                       // PathBuf
        ];

        // Test with all different path formats.
        for src_path in src_paths {
            // Get a reference to this path representation (i.e. "unbox").
            let src_path = src_path.as_ref();

            // Change to the `src` directory.
            let _cd = cd(src_path);

            // Verify the current directory has changed.
            assert_eq!(get_cwd(), to_path_buf(src_path));

            // Change back to the original directory.
            let _cd = cd(&original_cwd_path);

            // Verify that we are back in the original directory.
            assert_eq!(get_cwd(), original_cwd_path);
        }
    }

    #[test]
    #[serial]
    fn test_cd_with_scope() {
        // Get the path to the original current working directory.
        let original_cwd_path = get_cwd();

        // Verify we are in the `file_io` directory.
        assert!(original_cwd_path.ends_with("file_io"));

        // Define the directory to change to.
        let src_path = original_cwd_path.join("src");

        // src directory path in different formats.
        let src_paths: Vec<Box<dyn AsRef<Path>>> = vec![
            Box::new(src_path.to_str().unwrap()),             // &str
            Box::new(src_path.to_str().unwrap().to_string()), // String
            Box::new(src_path.as_path()),                     // Path
            Box::new(src_path.clone()),                       // PathBuf
        ];

        // Test with all different path formats.
        for src_path in src_paths {
            // Get a reference to this path representation (i.e. "unbox").
            let src_path = src_path.as_ref();

            // Enter a new scope.
            {
                // Change to the `src` directory within this limited scope.
                let _cd = cd(src_path);

                // Verify the current directory has changed.
                assert_eq!(get_cwd(), to_path_buf(src_path));
            }

            // Verify that outside the scope, we are back in the original directory.
            assert_eq!(get_cwd(), original_cwd_path);
        }
    }

    #[test]
    #[serial]
    fn test_cd_with_panic() {
        // Create a temporary directory to work in and get its path.
        let temp_dir = tempdir().unwrap();
        let temp_dir_path = get_temp_dir_path(&temp_dir);

        // Get the path to the original current working directory.
        let original_cwd_path = get_cwd();

        // Create a folder within the temporary directory to move into.
        let new_cwd_path = temp_dir_path.join("subfolder");
        create_folder(&new_cwd_path);

        // Catch the panic inside this scope.
        let result = std::panic::catch_unwind(|| {
            // Change to the new directory.
            let _cd = cd(&temp_dir);

            // Ensure we changed into the new directory.
            assert_eq!(get_cwd(), new_cwd_path);

            // Simulate failure.
            panic!("Simulated failure.");
        });

        // Make sure a panic actually occurred.
        assert!(result.is_err());

        // Ensure we are back in the original directory.
        assert_eq!(get_cwd(), original_cwd_path);
    }
}
