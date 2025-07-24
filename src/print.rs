use crate::list::list_folder_contents;
use crate::path::get_last_path_component;
use std::path::Path;

/// Helper function to recursively print the folder tree.
///
/// # Arguments
///
/// * `path` - The current path to print.
/// * `prefix` - The prefix string to use for the current level of indentation.
/// * `is_last` - A boolean indicating if this is the last entry at the current level.
/// * `output` - The output stream to write the tree structure to.
fn helper<W: std::io::Write>(path: &Path, prefix: String, is_last: bool, output: &mut W) {
    // Get the name of the file or folder (i.e. the last component of the path).
    let name = get_last_path_component(path);

    // Print the current file or folder with the appropriate prefix.
    let connector = if is_last { "└── " } else { "├── " };
    writeln!(output, "{prefix}{connector}{name}").unwrap();

    // Special handling for folders (we need to recurse into them and update the prefix).
    if path.is_dir() {
        // Create a new prefix for the children. If this is the last entry, we use spaces to avoid
        // drawing the vertical line.
        let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

        // Read the directory entries into a vector and sort them.
        let entries = list_folder_contents(path);

        // Call the helper function recursively for each entry.
        for (i, entry) in entries.iter().enumerate() {
            let is_last = i == entries.len() - 1;
            helper(entry, new_prefix.clone(), is_last, output);
        }
    }
}

/// Write the folder tree structure starting from the specified path.
///
/// # Arguments
///
/// * `path` - The path to the folder to print (can be a `&str`, `String`, `Path`, or `PathBuf`).
/// * `output` - The output stream to write the tree structure to.
fn write_folder_tree<P: AsRef<Path>, W: std::io::Write>(path: P, output: &mut W) {
    // Convert the input path to a Path reference.
    let path = path.as_ref();

    // Print the full top-level path once.
    writeln!(output, "{}", path.display()).unwrap();

    // List and sort children.
    let entries = list_folder_contents(path);

    // Recurse only into children.
    //  --> The first entry is the top-level path, so we don't need to print it again.
    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        helper(entry, "".to_string(), is_last, output);
    }
}

/// Print the folder tree structure starting from the specified path.
///
/// # Arguments
///
/// * `path` - The path to the folder to print (can be a `&str`, `String`, `Path`, or `PathBuf`).
///
/// # Examples
///
/// ## Using a string literal
///
/// ```
/// use file_io::print_folder_tree;
///
/// print_folder_tree("src");
/// ```
///
/// ## Using a `Path` reference
///
/// ```
/// use file_io::print_folder_tree;
/// use std::path::Path;
///
/// print_folder_tree(Path::new("src"));
/// ```
pub fn print_folder_tree<P: AsRef<Path>>(path: P) {
    write_folder_tree(path, &mut std::io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::save_string_to_file;
    use crate::test_utils::get_temp_dir_path;
    use tempfile::tempdir;

    #[test]
    fn test_write_folder_tree() {
        // Create a temporary directory to work in.
        let temp_dir = tempdir().unwrap();

        // Get the path to the temporary directory.
        let temp_dir_path = get_temp_dir_path(&temp_dir);

        // Create some test files and folders.
        save_string_to_file("Content 1", temp_dir_path.join("file1.txt"));
        save_string_to_file("Content 2", temp_dir_path.join("file2.txt"));
        save_string_to_file("Content 3", temp_dir_path.join("subfolder/file3.txt"));

        // Vector of bytes to capture the output (would be redirected to stdout using
        // `print_folder_tree`).
        let mut stdout: Vec<u8> = Vec::new();

        // Call the function to print the folder tree.
        write_folder_tree(&temp_dir_path, &mut stdout);

        // Check the output.
        let output = String::from_utf8(stdout).unwrap();
        assert_eq!(
            output,
            format!(
                "{}\n├── file1.txt\n├── file2.txt\n└── subfolder\n    └── file3.txt\n",
                temp_dir_path.display()
            )
        );
    }
}
