//! [![github]](https://github.com/tamaskis/easy-io)&ensp;[![crates-io]](https://crates.io/crates/easy-io)&ensp;[![docs-rs]](https://docs.rs/easy-io)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! Easy interfaces for file i/o.

// Linter setup.
#![warn(missing_docs)]

// Module declarations.
pub(crate) mod module;

// Re-exports.
pub use crate::module::example_function;
