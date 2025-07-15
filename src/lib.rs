//! [![github]](https://github.com/tamaskis/file_io)&ensp;[![crates-io]](https://crates.io/crates/file-io)&ensp;[![docs-rs]](https://docs.rs/file-io)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! Easy interfaces for file i/o.

// Linter setup.
#![warn(missing_docs)]

// Module declarations.
pub(crate) mod create;
pub(crate) mod load;
pub(crate) mod modify;
pub(crate) mod path;
pub(crate) mod save;

// Re-exports.
pub use create::{create_folder, create_folder_for_file};
pub use load::load_file_as_string;
pub use modify::{replace_str_in_file, replace_str_in_files};
pub use path::{
    cd, get_cwd, get_file_extension, get_file_name, get_file_stem, get_home,
    get_last_path_component,
};
pub use save::save_string_to_file;

// Helper functions for unit testing.
#[cfg(test)]
pub(crate) mod test_utils;
