use std::fmt::Debug;
use std::path::PathBuf;
use std::time::SystemTimeError;
use std::{error, io, result};

use thiserror::Error;

/// Custom error types for the cache operations.
#[derive(Error, Debug)]
pub enum Error {
    /// The specified path exists but is not a directory.
    ///
    /// This error occurs when trying to create a cache in a location
    /// that already exists but is a file rather than a directory.
    #[error("Path is not a directory: {path}")]
    NotADirectory { path: PathBuf },

    /// Path traversal attempt detected outside the cache directory.
    ///
    /// This error occurs when a file path would escape the cache directory
    /// boundaries, which could be a security risk.
    #[error("Path traversal detected: {path} is not within cache directory {cache_dir}")]
    PathTraversal { path: PathBuf, cache_dir: PathBuf },

    /// The specified path is invalid.
    ///
    /// This error occurs when a file path is not valid, such as when it contains
    /// invalid characters or is otherwise malformed.
    #[error("Invalid path: {path}")]
    InvalidPath { path: PathBuf },

    /// The specified path has no parent directory.
    ///
    /// This error occurs when trying to create a file in a path that
    /// cannot be resolved to a valid parent directory.
    #[error("Invalid path: {path} has no parent directory")]
    NoParentDirectory { path: PathBuf },

    /// The file already exists when trying to create a new lazy file.
    ///
    /// This error occurs when attempting to create a lazy file that
    /// already exists on the filesystem.
    #[error("File already exists: {path}")]
    FileAlreadyExists { path: PathBuf },

    /// The file is already in a locked state.
    ///
    /// This error occurs when trying to lock a file that is already locked.
    #[error("File already locked")]
    FileAlreadyLocked,

    /// The file is already in an unlocked state.
    ///
    /// This error occurs when trying to unlock a file that is already unlocked.
    #[error("File already unlocked")]
    FileAlreadyUnlocked,

    /// Error from a user-provided callback function.
    ///
    /// This error wraps any error returned by callback functions
    /// used for file initialization or processing.
    #[error(transparent)]
    Callback(Box<dyn error::Error + Send + Sync>),

    /// System time calculation error.
    ///
    /// This error occurs when system time operations fail, typically
    /// during file validity checks or timestamp calculations.
    #[error(transparent)]
    SystemTime(#[from] SystemTimeError),

    /// Standard I/O operation error.
    ///
    /// This error wraps standard filesystem I/O errors such as
    /// file creation, reading, writing, or metadata access failures.
    #[error(transparent)]
    IO(#[from] io::Error),
}

/// Type alias for [`Result`](std::result::Result) with custom [`enum@Error`] type.
pub type Result<T> = result::Result<T, Error>;

/// Creates an [`Ok`](std::result::Result::Ok) result with the custom [`enum@Error`] type.
///
/// This is a convenience function that provides the same functionality as
/// [`std::result::Result::Ok`] but with the crate's custom error type.
///
/// # Example
///
/// ```rust
/// use fcache::{Error, Result};
///
/// // Using standard Result::Ok
/// let result: Result<i32> = std::result::Result::Ok(42);
/// assert!(result.is_ok());
/// assert_eq!(result.unwrap(), 42);
/// ```
#[allow(non_snake_case, clippy::unnecessary_wraps, clippy::missing_errors_doc)]
#[doc(hidden)]
pub(crate) fn Ok<T>(value: T) -> Result<T> {
    // TODO: Should be possible via `pub use Result::Ok` - see [GH-123131](https://github.com/rust-lang/rust/issues/123131)
    Result::Ok(value)
}
