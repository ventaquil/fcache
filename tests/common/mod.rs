//! Common utilities and imports for fcache tests

#![allow(dead_code)] // Allow unused constants for shared utilities
#![allow(unused_imports)] // Allow unused imports as they may be used in different test files

pub use std::fs::File;
pub use std::io::{Read, Write};
pub use std::time::Duration;

pub use tempfile::TempDir;

/// Test data content
pub const TEST_CONTENT: &[u8] = include_bytes!("test_content.txt");
pub const TEST_LARGE_CONTENT: &[u8] = include_bytes!("test_large_content.txt");
