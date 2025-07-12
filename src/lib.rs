//! [![github]](https://github.com/tamaskis/TODO_REPO)&ensp;[![crates-io]](https://crates.io/crates/TODO_CRATE_NAME_WITH_DASHES)&ensp;[![docs-rs]](https://docs.rs/TODO_CRATE_NAME_WITH_DASHES)&ensp;[![mdbook]](https://tamaskis.github.io/TODO_CRATE_NAME_WITH_DASHES/)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//! [mdbook]: https://img.shields.io/badge/mdbook-ffd92f?style=for-the-badge&labelColor=555555&logo=bookstack
//!
//! TODO_DESCRIPTION

// Linter setup.
#![warn(missing_docs)]

// Module declarations.
pub(crate) mod module;

// Re-exports.
pub use crate::module::example_function;
