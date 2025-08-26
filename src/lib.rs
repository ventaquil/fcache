//! This crate provides a simple file caching mechanism with a straightforward interface for creating, retrieving, and managing cached files efficiently.
//!
//! # Features
//!
//! - **Temporary and Persistent Caches**: Create caches in temporary directories or specified paths.
//! - **Lazy File Creation**: Files are created only when accessed, reducing unnecessary disk operations.
//! - **Automatic Refresh**: Files can be automatically refreshed based on a specified interval.
//! - **Callback Functions**: Custom logic can be executed when files are created or accessed.
//!
//! # Setup
//!
//! To use this crate, add the following entry to your `Cargo.toml` file in the `dependencies` section:
//!
//! ```toml
//! [dependencies]
//! fcache = "0.0.0"
//! ```
//!
//! Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:
//!
//! ```sh
//! cargo add fcache
//! ```
//!
//! # Usage
//!
//! Use the [`new`] function to create a new cache instance in a temporary directory.
//!
//! ```rust
//! use fcache::prelude::*;
//!
//! # fn wrapper() -> fcache::Result<()> {
//! // Create a new cache instance
//! let cache = fcache::new()?;
//!
//! // Get or create a cached file
//! let cache_file = cache.get("hello.txt", |mut file| {
//!     // Write data to the file
//!     file.write_all(b"Hello, world!")?;
//!     Ok(())
//! })?;
//! // File is created and can be used...
//!
//! // Open the cached file
//! let mut file = cache_file.open()?;
//!
//! // Read the content of the file
//! let mut content = String::new();
//! file.read_to_string(&mut content)?;
//! // Assert the content matches what was written
//! assert_eq!(content, "Hello, world!");
//! # Ok(())
//! # }
//! ```
//!
//! Use the [`with_dir`] function to create a cache in a specified directory.
//!
//! ```rust,no_run
//! use fcache::prelude::*;
//!
//! # fn wrapper() -> fcache::Result<()> {
//! // Create a new cache instance in a specified directory
//! let cache = fcache::with_dir("/path/to/cache")?;
//!
//! // Get or create a cached file
//! let cache_file = cache.get("hello.txt", |mut file| {
//!     // Write data to the file
//!     file.write_all(b"Hello, world!")?;
//!     Ok(())
//! })?;
//! // File is created and can be used...
//!
//! // Open the cached file
//! let mut file = cache_file.open()?;
//!
//! // Read the content of the file
//! let mut content = String::new();
//! file.read_to_string(&mut content)?;
//! // Assert the content matches what was written
//! assert_eq!(content, "Hello, world!");
//! # Ok(())
//! # }
//! ```
//!
//! ## Lazy files
//!
//! Lazy files are a special type of file that is not created until it is accessed. This can be useful for reducing unnecessary disk operations, especially when the file may not be needed immediately.
//!
//! See [`Cache::get_lazy`], and [`CacheLazyFile`] for more details on how to use lazy files.
//!
//! ```rust
//! use fcache::prelude::*;
//!
//! # fn wrapper() -> fcache::Result<()> {
//! // Create a new cache instance in a specified directory
//! let cache = fcache::new()?;
//!
//! // Get or create a lazy cached file
//! let cache_file = cache.get_lazy("hello.txt", |mut file| {
//!     // Write data to the file
//!     file.write_all(b"Hello, world!")?;
//!     Ok(())
//! })?;
//! // File isn't created until opened...
//!
//! // Open the cached file
//! let mut file = cache_file.open()?; // File is created here
//!
//! // Read the content of the file
//! let mut content = String::new();
//! file.read_to_string(&mut content)?;
//! // Assert the content matches what was written
//! assert_eq!(content, "Hello, world!");
//! # Ok(())
//! # }
//! ```
//!
//! ## Force Refresh
//!
//! Files in the cache can be forcefully refreshed to regenerate their content, even if they are still within their refresh interval. This is useful when you need to ensure the file content is updated immediately.
//!
//! See [`CacheFile::force_refresh`], and [`CacheLazyFile::force_refresh`] for more details.
//!
//! ```rust
//! use fcache::prelude::*;
//!
//! # fn wrapper() -> fcache::Result<()> {
//! // Create a new cache instance
//! let cache = fcache::new()?;
//!
//! // Create a file with initial content
//! let cache_file = cache.get("data.txt", |mut file| {
//!     file.write_all(b"Initial content")?;
//!     Ok(())
//! })?;
//!
//! // Force refresh the file with new content
//! let cache_file = cache.get("data.txt", |mut file| {
//!     file.write_all(b"Updated content")?;
//!     Ok(())
//! })?;
//! cache_file.force_refresh()?;
//!
//! // The file now contains the updated content
//! let mut file = cache_file.open()?;
//! let mut content = String::new();
//! file.read_to_string(&mut content)?;
//! assert_eq!(content, "Updated content");
//! # Ok(())
//! # }
//! ```
//!
//! ## Locking and unlocking
//!
//! Files can be locked and unlocked to prevent refreshing or modifying them while they are in use. This is useful for ensuring that the file remains consistent during operations.
//!
//! See [`CacheFile::lock`]/[`CacheFile::unlock`], and [`CacheLazyFile::lock`]/[`CacheLazyFile::unlock`] for more details.
//!
//! This is not a thread-safe operation.
//!
//! ```rust
//! use fcache::prelude::*;
//!
//! # fn wrapper() -> fcache::Result<()> {
//! // Create a new cache instance in a specified directory
//! let cache = fcache::new()?;
//!
//! // Get or create a cached file
//! let mut cache_file = cache.get("hello.txt", |mut file| {
//!     // Write data to the file
//!     file.write_all(b"Hello, world!")?;
//!     Ok(())
//! })?;
//! // File is created and can be used...
//!
//! // Lock the file to prevent refreshing
//! cache_file.lock()?; // Lock the file to prevent refreshing
//!
//! // Perform operations on the file
//!
//! // Unlock the file to allow refreshing
//! cache_file.unlock()?; // Unlock the file to allow refreshing
//!
//! // ...
//! # Ok(())
//! # }
//! ```
//!
//! ## Thread Safety
//!
//! The cache system is designed to be thread-safe for most operations. Cache instances can be safely shared across multiple threads using [`Arc`](std::sync::Arc) or similar synchronization primitives.
//!
//! ```rust
//! use std::sync::Arc;
//! use std::thread;
//!
//! use fcache::prelude::*;
//!
//! # fn wrapper() -> fcache::Result<()> {
//! // Create a cache instance that can be shared across threads
//! let cache = Arc::new(fcache::new()?);
//!
//! // Spawn multiple threads that use the same cache
//! let handles: Vec<_> = (0..4)
//!     .map(|i| {
//!         let cache = Arc::clone(&cache);
//!         thread::spawn(move || {
//!             // Each thread can safely create files in the cache
//!             let cache_file = cache.get(&format!("thread_{}.txt", i), move |mut file| {
//!                 file.write_all(format!("Content from thread {}", i).as_bytes())?;
//!                 Ok(())
//!             })?;
//!
//!             // And read from them
//!             let mut content = String::new();
//!             cache_file.open()?.read_to_string(&mut content)?;
//!             println!("Thread {}: {}", i, content);
//!
//!             Ok::<(), fcache::Error>(())
//!         })
//!     })
//!     .collect();
//!
//! // Wait for all threads to complete
//! for handle in handles {
//!     handle
//!         .join()
//!         .expect("Thread should complete successfully")?;
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Thread Safety Guarantees
//!
//! - **Cache instances**: Safe to share across threads using [`Arc`](std::sync::Arc).
//! - **File creation**: Multiple threads can safely create different files simultaneously.
//! - **File operations**: Reading and writing operations are thread-safe at the filesystem level.
//!
//! ### Thread Safety Limitations
//!
//! - **File locking**: The built-in locking mechanism is **not** thread-safe and should not be relied upon for inter-thread synchronization (see [Locking and unlocking](#locking-and-unlocking) for more details).
//! - **Concurrent access**: Users must implement their own synchronization using external mechanisms like [`Mutex`](std::sync::Mutex) or [`RwLock`](std::sync::RwLock) when multiple threads access the same file path, as the cache does not provide automatic protection against race conditions.
//!
//! # Tips and tricks
//!
//! ## Always refresh
//!
//! Use [`Duration::ZERO`] to ensure the cache is always refreshed.
//!
//! ```rust
//! use std::time::Duration;
//!
//! use fcache::prelude::*;
//!
//! # fn wrapper() -> fcache::Result<()> {
//! // Create a new cache instance
//! let cache = fcache::new()?.with_refresh_interval(Duration::ZERO);
//!
//! // Get or create a cached file
//! let mut cache_file = cache.get("hello.txt", |mut file| {
//!     // Write data to the file
//!     file.write_all(b"Hello, world!")?;
//!     // Inform about the refresh
//!     println!("Refreshing file");
//!     Ok(())
//! })?;
//!
//! // File refreshes on every access
//! let file = cache_file.open()?;
//! let file = cache_file.open()?;
//! let file = cache_file.open()?;
//! // ...
//! # Ok(())
//! # }
//! ```
//!
//! ## Never refresh
//!
//! Use [`Duration::MAX`] to disable automatic refresh. Use `force_refresh` for manual control.
//!
//! ```rust
//! use std::time::Duration;
//!
//! use fcache::prelude::*;
//!
//! # fn wrapper() -> fcache::Result<()> {
//! // Create a new cache instance
//! let cache = fcache::new()?.with_refresh_interval(Duration::MAX);
//!
//! // Get or create a cached file
//! let mut cache_file = cache.get("hello.txt", |mut file| {
//!     // Write data to the file
//!     file.write_all(b"Hello, world!")?;
//!     // Inform about the refresh
//!     println!("Refreshing file");
//!     Ok(())
//! })?;
//!
//! // File never refreshes automatically
//! let file = cache_file.open()?;
//! let file = cache_file.open()?;
//! let file = cache_file.open()?;
//! // ...
//!
//! // Manual refresh when needed
//! cache_file.force_refresh()?;
//! # Ok(())
//! # }
//! ```
//!
//! # License
//!
//! This crate is licensed under the MIT License.

#![forbid(unsafe_code)]

mod callback;
mod file;
pub mod prelude;
mod result;

use std::fmt::Debug;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::time::Duration;

use tempfile::TempDir;

pub use crate::callback::CallbackFn;
pub use crate::file::{CacheFile, CacheLazyFile};
use crate::result::Ok;
pub use crate::result::{Error, Result};

/// Default refresh interval for the cache.
pub const DEFAULT_REFRESH_INTERVAL: Duration = Duration::from_secs(5);

/// Creates a new cache instance within a temporary directory.
///
/// For more information on how to use the cache, refer to the [`Cache`] documentation.
///
/// # Example
///
/// ```rust
/// # fn wrapper() -> fcache::Result<()> {
/// // Create a new cache instance
/// let cache = fcache::new()?;
///
/// // Use the cache...
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// This function will return an error if the temporary directory cannot be created or if there are issues with the underlying filesystem operations.
pub fn new() -> Result<Cache> {
    Cache::new()
}

/// Creates a new cache instance within a temporary directory with a specified prefix.
///
/// For more information on how to use the cache, refer to the [`Cache`] documentation.
///
/// # Example
///
/// ```rust
/// # fn wrapper() -> fcache::Result<()> {
/// // Create a new cache instance with a custom prefix
/// let cache = fcache::with_prefix("my_cache")?;
/// assert!(cache.path().to_string_lossy().starts_with("/tmp/my_cache"));
///
/// // Use the cache...
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// This function will return an error if the temporary directory cannot be created or there are issues with the underlying filesystem operations.
pub fn with_prefix(prefix: &str) -> Result<Cache> {
    Cache::with_prefix(prefix)
}

/// Creates a new cache instance within a specified directory.
///
/// For more information on how to use the cache, refer to the [`Cache`] documentation.
///
/// # Example
///
/// ```rust,no_run
/// # fn wrapper() -> fcache::Result<()> {
/// // Create a new cache instance
/// let cache = fcache::with_dir("/path/to/cache")?;
///
/// // Use the cache...
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// This function will return an error if the specified path exists but is not a directory, the specified path does not exist and directory creation fails, or there are other underlying filesystem operation issues.
pub fn with_dir(dir: impl AsRef<Path>) -> Result<Cache> {
    Cache::with_dir(dir)
}

/// Represents a cache instance.
///
/// # Example
///
/// ```rust
/// use fcache::prelude::*;
///
/// # fn wrapper() -> fcache::Result<()> {
/// // Create a new cache instance
/// let cache = Cache::new()?;
///
/// // Create a new file in the cache
/// let cache_file = cache.get("example.txt", |mut file| {
///     file.write_all(b"Hello, Cache!")?;
///     Ok(())
/// })?;
///
/// // Open the cached file
/// let mut file = cache_file.open()?;
/// // Read the content of the file
/// let mut content = String::new();
/// file.read_to_string(&mut content)?;
///
/// // Assert the content matches what was written
/// assert_eq!(content, "Hello, Cache!");
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Cache(InnerCache);

impl Cache {
    /// Creates a new cache instance within a temporary directory.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// // Create a new cache instance
    /// let cache = Cache::new()?;
    ///
    /// // Use the cache...
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the temporary directory cannot be created or if there are issues with the underlying filesystem operations.
    pub fn new() -> Result<Self> {
        InnerCache::temp().map(Self)
    }

    /// Creates a new cache instance within a temporary directory with a specified prefix.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// // Create a new cache instance with a custom prefix
    /// let cache = Cache::with_prefix("my_cache")?;
    /// assert!(cache.path().to_string_lossy().starts_with("/tmp/my_cache"));
    ///
    /// // Use the cache...
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the temporary directory cannot be created or there are issues with the underlying filesystem operations.
    pub fn with_prefix(prefix: &str) -> Result<Self> {
        InnerCache::temp_with_prefix(prefix).map(Self)
    }

    /// Creates a new cache instance within a specified directory.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// // Create a new cache instance
    /// let cache = Cache::with_dir("/path/to/cache")?;
    ///
    /// // Use the cache...
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the specified path exists but is not a directory, the specified path does not exist and directory creation fails, or there are other underlying filesystem operation issues.
    pub fn with_dir(dir: impl AsRef<Path>) -> Result<Self> {
        InnerCache::dir(dir).map(Self)
    }

    /// Sets the refresh interval for the cache.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// // Create a new cache instance with a custom refresh interval
    /// let cache = Cache::new()?.with_refresh_interval(Duration::from_secs(10));
    ///
    /// // Use the cache...
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn with_refresh_interval(self, refresh_interval: Duration) -> Self {
        let Self(inner) = self;
        inner.with_refresh_interval(refresh_interval).into()
    }

    /// Sets the refresh interval to the default value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// // Create a new cache instance
    /// let cache = Cache::new()?.with_refresh_interval(Duration::from_secs(60));
    ///
    /// // Use the cache...
    ///
    /// // Reset to default refresh interval
    /// let cache = cache.with_default_refresh_interval();
    ///
    /// // Use the cache...
    /// # Ok(())
    /// # }
    #[must_use]
    pub fn with_default_refresh_interval(self) -> Self {
        let Self(inner) = self;
        inner.with_default_refresh_interval().into()
    }

    /// Returns the path of the cache directory.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// // Create a new cache instance
    /// let cache = Cache::new()?;
    ///
    /// // Print the cache path
    /// println!("Cache path: {}", cache.path().display());
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn path(&self) -> &Path {
        let Self(inner) = self;
        inner.path()
    }

    /// Returns the refresh interval of the cache.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// // Create a new cache instance
    /// let cache = Cache::new()?;
    ///
    /// // Print the refresh interval
    /// println!("Refresh interval: {:?}", cache.refresh_interval());
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn refresh_interval(&self) -> Duration {
        let Self(inner) = self;
        inner.refresh_interval()
    }

    /// Creates a file in the cache using a callback for initialization.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// // Create a new cache instance
    /// let cache = Cache::new()?;
    ///
    /// // Get or create a cached file
    /// let cache_file = cache.get("example.txt", |mut file| {
    ///     // Write data to the file
    ///     file.write_all(b"Hello, Cache!")?;
    ///     Ok(())
    /// })?;
    /// // File is created and can be used...
    ///
    /// // Open the cached file
    /// let mut file = cache_file.open()?;
    /// // Read data from the file
    /// let mut contents = String::new();
    /// file.read_to_string(&mut contents)?;
    /// println!("Cached file contents: {}", contents);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the file already exists, file creation fails due to permissions or disk space, the callback function returns an error, path traversal is detected outside the cache directory, or parent directory creation fails.
    pub fn get(&self, path: impl AsRef<Path>, callback: impl CallbackFn + 'static) -> Result<CacheFile<'_>> {
        let Self(inner) = self;
        inner.get(path, callback)
    }

    /// Creates a file in the cache that is lazily created when accessed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// // Create a new cache instance
    /// let cache = Cache::new()?;
    ///
    /// // Get or create a lazy cached file
    /// let cache_file = cache.get_lazy("lazy_file.txt", |mut file| {
    ///     // Write data to the file
    ///     file.write_all(b"Hello, Lazy Cache!")?;
    ///     Ok(())
    /// })?;
    ///
    /// // File isn't created until opened...
    /// assert!(!cache_file.path().exists());
    ///
    /// // Open the lazy cached file
    /// let mut file = cache_file.open()?;
    /// // Read data from the file
    /// let mut contents = String::new();
    /// file.read_to_string(&mut contents)?;
    /// println!("Lazy cached file contents: {}", contents);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the file already exists, path traversal is detected outside the cache directory, parent directory creation fails, or there are issues with path resolution or filesystem operations.
    pub fn get_lazy(&self, path: impl AsRef<Path>, callback: impl CallbackFn + 'static) -> Result<CacheLazyFile<'_>> {
        let Self(inner) = self;
        inner.get_lazy(path, callback)
    }
}

impl From<InnerCache> for Cache {
    fn from(inner: InnerCache) -> Self {
        Self(inner)
    }
}

/// Represents the inner cache implementation, either directory-based or temporary.
#[derive(Debug)]
enum InnerCache {
    /// Directory cache implementation
    Dir(InnerDirCache),
    /// Temporary cache implementation
    Temp(InnerTempCache),
}

impl InnerCache {
    /// Creates a new cache instance within a specified directory.
    fn dir(dir: impl AsRef<Path>) -> Result<Self> {
        InnerDirCache::new(dir).map(Self::Dir)
    }

    /// Creates a new cache instance within a temporary directory.
    fn temp() -> Result<Self> {
        InnerTempCache::new().map(Self::Temp)
    }

    /// Creates a new cache instance within a temporary directory with a specified prefix.
    fn temp_with_prefix(prefix: &str) -> Result<Self> {
        InnerTempCache::with_prefix(prefix).map(Self::Temp)
    }

    /// Sets the refresh interval for the cache.
    fn with_refresh_interval(self, refresh_interval: Duration) -> Self {
        match self {
            Self::Dir(dir_cache) => dir_cache.with_refresh_interval(refresh_interval).into(),
            Self::Temp(temp_cache) => temp_cache.with_refresh_interval(refresh_interval).into(),
        }
    }

    /// Sets the refresh interval to the default value.
    fn with_default_refresh_interval(self) -> Self {
        match self {
            Self::Dir(dir_cache) => dir_cache.with_default_refresh_interval().into(),
            Self::Temp(temp_cache) => temp_cache.with_default_refresh_interval().into(),
        }
    }

    /// Returns the path of the cache directory.
    fn path(&self) -> &Path {
        match self {
            Self::Dir(dir_cache) => dir_cache.path(),
            Self::Temp(temp_cache) => temp_cache.path(),
        }
    }

    /// Returns the refresh interval of the cache.
    fn refresh_interval(&self) -> Duration {
        match self {
            Self::Dir(dir_cache) => dir_cache.refresh_interval(),
            Self::Temp(temp_cache) => temp_cache.refresh_interval(),
        }
    }

    /// Creates a file in the cache using a callback for initialization.
    fn get(&self, path: impl AsRef<Path>, callback: impl CallbackFn + 'static) -> Result<CacheFile<'_>> {
        match self {
            Self::Dir(dir_cache) => dir_cache.get(path, callback),
            Self::Temp(temp_cache) => temp_cache.get(path, callback),
        }
    }

    /// Creates a file in the cache that is lazily created when accessed.
    fn get_lazy(&self, path: impl AsRef<Path>, callback: impl CallbackFn + 'static) -> Result<CacheLazyFile<'_>> {
        match self {
            Self::Dir(dir_cache) => dir_cache.get_lazy(path, callback),
            Self::Temp(temp_cache) => temp_cache.get_lazy(path, callback),
        }
    }
}

impl From<InnerDirCache> for InnerCache {
    fn from(dir_cache: InnerDirCache) -> Self {
        Self::Dir(dir_cache)
    }
}

impl From<InnerTempCache> for InnerCache {
    fn from(temp_cache: InnerTempCache) -> Self {
        Self::Temp(temp_cache)
    }
}

/// Inner cache implementation for a specified directory.
#[derive(Debug)]
struct InnerDirCache {
    /// Directory where the cache is stored
    root: PathBuf,
    /// Refresh interval for the cache
    refresh_interval: Duration,
}

impl InnerDirCache {
    /// Creates a new cache instance within a specified directory.
    fn new(dir: impl AsRef<Path>) -> Result<Self> {
        let dir = dir.as_ref().to_path_buf();

        if dir.exists() && !dir.is_dir() {
            return Err(Error::NotADirectory { path: dir });
        } else if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }

        // Canonicalize after ensuring the directory exists
        let root = dir.canonicalize()?;
        let refresh_interval = DEFAULT_REFRESH_INTERVAL;
        let inner_dir_cache = Self { root, refresh_interval };
        Ok(inner_dir_cache)
    }

    /// Sets the refresh interval for the cache.
    fn with_refresh_interval(self, refresh_interval: Duration) -> Self {
        let Self { root, .. } = self;
        Self { root, refresh_interval }
    }

    /// Sets the refresh interval to the default value.
    fn with_default_refresh_interval(self) -> Self {
        self.with_refresh_interval(DEFAULT_REFRESH_INTERVAL)
    }

    /// Returns the path of the cache directory.
    fn path(&self) -> &Path {
        let Self { root, .. } = self;
        root
    }

    /// Returns the refresh interval of the cache.
    fn refresh_interval(&self) -> Duration {
        let Self { refresh_interval, .. } = self;
        *refresh_interval
    }

    /// Creates a file in the cache using a callback for initialization.
    fn get(&self, path: impl AsRef<Path>, callback: impl CallbackFn + 'static) -> Result<CacheFile<'_>> {
        self.get_lazy(path, callback)?.init()
    }

    /// Creates a file in the cache that is lazily created when accessed.
    fn get_lazy(&self, path: impl AsRef<Path>, callback: impl CallbackFn + 'static) -> Result<CacheLazyFile<'_>> {
        let Self { root, refresh_interval } = self;
        let path = path.as_ref();

        // Ensure the path does not end with a slash
        if path.to_str().is_some_and(|path| path.ends_with('/')) {
            let path = path.to_path_buf();
            let error = Error::InvalidPath { path };
            return Err(error);
        }

        // Ensure the absolute path is within the cache directory to prevent path traversal attacks
        let mut components = path.components();
        let file_name = components.next_back().ok_or_else(|| {
            let path = path.to_path_buf();
            Error::InvalidPath { path }
        })?;
        let file_name = if let Component::Normal(name) = file_name
            && name.to_str().is_some_and(|file_name| file_name.trim() != "")
        {
            name
        } else {
            let path = path.to_path_buf();
            let error = Error::InvalidPath { path };
            return Err(error);
        };
        let mut path = root.clone();
        for component in components {
            path.push(component);
            if !path.exists() {
                fs::create_dir(&path)?;
            }
            let canonicalized_path = path.canonicalize()?;
            if !canonicalized_path.starts_with(root) {
                let cache_dir = root.clone();
                let error = Error::PathTraversal { path, cache_dir };
                return Err(error);
            }
        }

        let path = path.join(file_name);
        CacheLazyFile::new(path, callback, *refresh_interval, root, refresh_interval)
    }
}

/// Inner cache implementation for a temporary directory.
#[derive(Debug)]
struct InnerTempCache {
    /// Temporary directory for the cache
    temp_dir: TempDir, // Keep the temporary directory alive for the lifetime of the cache
    /// Directory cache implementation
    dir_cache: InnerDirCache,
}

impl InnerTempCache {
    const DEFAULT_PREFIX: &str = "fcache";

    /// Creates a new cache instance within a temporary directory.
    fn new() -> Result<Self> {
        Self::with_prefix(Self::DEFAULT_PREFIX)
    }

    /// Creates a new cache instance within a temporary directory with a specified prefix.
    fn with_prefix(prefix: &str) -> Result<Self> {
        let temp_dir = tempfile::Builder::new().prefix(prefix).tempdir()?;
        InnerDirCache::new(temp_dir.path()).map(|dir_cache| Self { temp_dir, dir_cache })
    }

    /// Sets the refresh interval for the cache.
    fn with_refresh_interval(self, refresh_interval: Duration) -> Self {
        let Self { temp_dir, dir_cache } = self;
        let dir_cache = dir_cache.with_refresh_interval(refresh_interval);
        Self { temp_dir, dir_cache }
    }

    /// Sets the refresh interval to the default value.
    fn with_default_refresh_interval(self) -> Self {
        self.with_refresh_interval(DEFAULT_REFRESH_INTERVAL)
    }

    /// Returns the path of the cache directory.
    fn path(&self) -> &Path {
        let Self { dir_cache, .. } = self;
        dir_cache.path()
    }

    /// Returns the refresh interval of the cache.
    fn refresh_interval(&self) -> Duration {
        let Self { dir_cache, .. } = self;
        dir_cache.refresh_interval()
    }

    /// Creates a file in the cache using a callback for initialization.
    fn get(&self, path: impl AsRef<Path>, callback: impl CallbackFn + 'static) -> Result<CacheFile<'_>> {
        let Self { dir_cache, .. } = self;
        dir_cache.get(path, callback)
    }

    /// Creates a file in the cache that is lazily created when accessed.
    fn get_lazy(&self, path: impl AsRef<Path>, callback: impl CallbackFn + 'static) -> Result<CacheLazyFile<'_>> {
        let Self { dir_cache, .. } = self;
        dir_cache.get_lazy(path, callback)
    }
}
