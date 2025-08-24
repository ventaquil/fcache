use std::fmt::{self, Debug};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use crate::callback::CallbackFn;
use crate::result::{Error, Result};

/// A file in the cache that is lazily created when accessed.
///
/// Lazy files defer their creation until the first time they are opened,
/// allowing for more efficient resource usage when files may not be needed immediately.
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
/// // Create a lazy file that won't be created until accessed
/// let cached_file = cache.get_lazy("data.txt", |mut file| {
///     file.write_all(b"Lazy file content")?;
///     Ok(())
/// })?;
///
/// // File doesn't exist yet on disk
/// assert!(!cached_file.path().exists());
///
/// // Opening the file triggers its creation
/// let mut file = cached_file.open()?;
/// // Read data from the file
/// let mut content = String::new();
/// file.read_to_string(&mut content)?;
/// assert_eq!(content, "Lazy file content");
/// # Ok(())
/// # }
/// ```
pub struct CacheLazyFile<'a> {
    /// Path to the lazy file
    path: PathBuf,
    /// Callback function to initialize the file
    callback: Box<dyn CallbackFn>,
    /// Refresh interval for the file
    refresh_interval: Duration,
    /// Cache root directory
    cache_root: &'a Path,
    /// Cache refresh interval
    cache_refresh_interval: &'a Duration,
    /// Whether the file is locked
    locked: bool,
}

impl<'a> CacheLazyFile<'a> {
    /// Creates a new lazy file instance.
    pub(crate) fn new(
        path: impl AsRef<Path>,
        callback: impl CallbackFn + 'static,
        refresh_interval: Duration,
        cache_root: &'a Path,
        cache_refresh_interval: &'a Duration,
    ) -> Result<Self> {
        let path = path.as_ref();
        (!path.exists())
            .then(|| {
                let callback = Box::new(callback);
                let path = path.to_path_buf();
                let locked = false;
                Self {
                    path,
                    callback,
                    refresh_interval,
                    cache_root,
                    cache_refresh_interval,
                    locked,
                }
            })
            .ok_or_else(|| {
                let path = path.to_path_buf();
                Error::FileAlreadyExists { path }
            })
    }

    /// Sets the refresh interval for the lazy file.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::time::Duration;
    ///
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("data.txt", |mut f| {
    ///     f.write_all(b"content")?;
    ///     Ok(())
    /// })?;
    ///
    /// // Set custom refresh interval to 30 minutes
    /// let lazy_file = lazy_file.with_refresh_interval(Duration::from_secs(30 * 60));
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn with_refresh_interval(self, refresh_interval: Duration) -> Self {
        let Self {
            path,
            callback,
            cache_root,
            cache_refresh_interval,
            locked,
            ..
        } = self;
        Self {
            path,
            callback,
            refresh_interval,
            cache_root,
            cache_refresh_interval,
            locked,
        }
    }

    /// Sets the refresh interval to the default value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::time::Duration;
    ///
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("data.txt", |mut f| {
    ///     f.write_all(b"content")?;
    ///     Ok(())
    /// })?;
    ///
    /// // Set custom interval, then reset to default
    /// let lazy_file = lazy_file
    ///     .with_refresh_interval(Duration::from_secs(60))
    ///     .with_default_refresh_interval();
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn with_default_refresh_interval(self) -> Self {
        let Self {
            path,
            callback,
            cache_root,
            cache_refresh_interval,
            locked,
            ..
        } = self;
        let refresh_interval = *cache_refresh_interval;
        Self {
            path,
            callback,
            refresh_interval,
            cache_root,
            cache_refresh_interval,
            locked,
        }
    }

    /// Returns the path of the lazy file.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("config.txt", |mut f| {
    ///     f.write_all(b"config data")?;
    ///     Ok(())
    /// })?;
    ///
    /// // Get the file path
    /// let path = lazy_file.path();
    /// println!("File will be created at: {}", path.display());
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn path(&self) -> &Path {
        let Self { path, .. } = self;
        path
    }

    /// Returns the refresh interval of the lazy file.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::time::Duration;
    ///
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache
    ///     .get_lazy("data.txt", |mut f| {
    ///         f.write_all(b"content")?;
    ///         Ok(())
    ///     })?
    ///     .with_refresh_interval(Duration::from_secs(300));
    ///
    /// // Check the current refresh interval
    /// let interval = lazy_file.refresh_interval();
    /// println!("Refresh interval: {} seconds", interval.as_secs());
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn refresh_interval(&self) -> Duration {
        let Self { refresh_interval, .. } = self;
        *refresh_interval
    }

    /// Returns whether the lazy file is locked.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let mut lazy_file = cache.get_lazy("data.txt", |mut f| {
    ///     f.write_all(b"content")?;
    ///     Ok(())
    /// })?;
    ///
    /// // Check if the file is locked
    /// assert!(!lazy_file.is_locked());
    /// lazy_file.lock()?;
    /// assert!(lazy_file.is_locked());
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn is_locked(&self) -> bool {
        let Self { locked, .. } = self;
        *locked
    }

    /// Returns whether the lazy file is unlocked.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let mut lazy_file = cache.get_lazy("data.txt", |mut f| {
    ///     f.write_all(b"content")?;
    ///     Ok(())
    /// })?;
    ///
    /// // Check if the file is unlocked
    /// assert!(lazy_file.is_unlocked());
    /// lazy_file.lock()?;
    /// assert!(!lazy_file.is_unlocked());
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn is_unlocked(&self) -> bool {
        !self.is_locked()
    }

    /// Checks if the lazy file is valid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("data.txt", |mut f| {
    ///     f.write_all(b"content")?;
    ///     Ok(())
    /// })?;
    ///
    /// // Check if the file is still valid
    /// if lazy_file.is_valid()? {
    ///     println!("File is still fresh");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the file metadata cannot be read, modification time cannot be determined, or system time calculations fail.
    pub fn is_valid(&self) -> Result<bool> {
        let Self {
            path, refresh_interval, ..
        } = self;
        let metadata = fs::metadata(path)?;
        let modified = metadata.modified()?;
        let elapsed = modified.elapsed()?;
        Ok(elapsed < *refresh_interval)
    }

    /// Checks if the lazy file is invalid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("data.txt", |mut f| {
    ///     f.write_all(b"content")?;
    ///     Ok(())
    /// })?;
    ///
    /// // Check if the file needs refreshing
    /// if lazy_file.is_invalid()? {
    ///     println!("File needs to be refreshed");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the file metadata cannot be read, modification time cannot be determined, or system time calculations fail.
    pub fn is_invalid(&self) -> Result<bool> {
        self.is_valid().map(|valid| !valid)
    }

    /// Returns the time until the lazy file is valid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("data.txt", |mut f| {
    ///     f.write_all(b"content")?;
    ///     Ok(())
    /// })?;
    ///
    /// // Get when the file will expire
    /// let valid_until = lazy_file.valid_until()?;
    /// println!("File valid until: {:?}", valid_until);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the file metadata cannot be read or the file's modification time cannot be determined.
    pub fn valid_until(&self) -> Result<SystemTime> {
        let Self {
            path, refresh_interval, ..
        } = self;
        let metadata = fs::metadata(path)?;
        let modified = metadata.modified()?;
        Ok(modified + *refresh_interval)
    }

    /// Locks this file to prevent other processes from reading or writing to it.
    ///
    /// For more details about the locking mechanism see [`CacheFile::lock`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let mut lazy_file = cache.get_lazy("shared.txt", |mut f| {
    ///     f.write_all(b"shared data")?;
    ///     Ok(())
    /// })?;
    ///
    /// // Lock the file to prevent concurrent access
    /// lazy_file.lock()?;
    /// // ... perform critical operations ...
    /// lazy_file.unlock()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the file is already locked by another process, system file locking mechanisms fail, or the underlying file cannot be accessed.
    pub fn lock(&mut self) -> Result<()> {
        self.is_unlocked()
            .then(|| {
                self.locked = true;
            })
            .ok_or_else(|| Error::FileAlreadyLocked)
    }

    /// Unlocks the lazy file to allow refreshing.
    ///
    /// For more details about the locking mechanism see [`CacheFile::unlock`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let mut lazy_file = cache.get_lazy("shared.txt", |mut f| {
    ///     f.write_all(b"shared data")?;
    ///     Ok(())
    /// })?;
    ///
    /// // Lock and then unlock the file
    /// lazy_file.lock()?;
    /// // ... critical operations complete ...
    /// lazy_file.unlock()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the file is already unlocked.
    pub fn unlock(&mut self) -> Result<()> {
        self.is_locked()
            .then(|| {
                self.locked = false;
            })
            .ok_or_else(|| Error::FileAlreadyUnlocked)
    }

    /// Creates the lazy file.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::io::Write;
    ///
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("document.txt", |mut f| {
    ///     f.write_all(b"Document content")?;
    ///     Ok(())
    /// })?;
    ///
    /// // Explicitly create the file if it doesn't exist
    /// let file = lazy_file.create()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the file already exists, file creation fails due to permissions or disk space, the callback function returns an error, or the file cannot be reopened for reading.
    pub fn create(&self) -> Result<File> {
        // FIXME: Refactor
        let Self { path, callback, .. } = self;
        File::options()
            .create_new(true)
            .read(false)
            .write(true)
            .open(path)
            .map_err(Error::IO)
            .and_then(|file| callback(file).map_err(Error::Callback))
            .and_then(|()| File::options().read(true).write(false).open(path).map_err(Error::IO))
    }

    /// Opens the lazy file, creating it if it doesn't exist.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::io::Read;
    ///
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("config.txt", |mut f| {
    ///     f.write_all(b"config data")?;
    ///     Ok(())
    /// })?;
    ///
    /// // Open and read the file content
    /// let mut file = lazy_file.open()?;
    /// let mut content = String::new();
    /// file.read_to_string(&mut content)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if file creation fails (if the file doesn't exist), file refresh fails (if the file exists), the file cannot be opened for reading, or the callback function returns an error during creation.
    pub fn open(&self) -> Result<File> {
        let Self { path, .. } = self;
        if path.exists() {
            self.refresh()?;
            File::options().read(true).write(false).open(path).map_err(Error::IO)
        } else {
            self.create()
        }
    }

    /// Refreshes the lazy file if it is invalid.
    ///
    /// This method only refreshes the file when it has expired. For unconditional refresh, see [`force_refresh`](Self::force_refresh).
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("cache.txt", |mut f| {
    ///     f.write_all(b"cached data")?;
    ///     Ok(())
    /// })?;
    ///
    /// // Refresh only if the file is invalid
    /// lazy_file.refresh()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if file validity cannot be determined or force refresh fails when the file is invalid.
    pub fn refresh(&self) -> Result<()> {
        self.is_invalid()
            .and_then(|invalid| if invalid { self.force_refresh() } else { Ok(()) })
    }

    /// Forces a refresh of the lazy file.
    ///
    /// This method refreshes the file regardless of its validity. For conditional refresh, see [`refresh`](Self::refresh).
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("data.txt", |mut f| {
    ///     f.write_all(b"fresh data")?;
    ///     Ok(())
    /// })?;
    ///
    /// // Force refresh regardless of validity
    /// lazy_file.force_refresh()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the file cannot be opened for writing, the callback function returns an error, or file truncation fails.
    pub fn force_refresh(&self) -> Result<()> {
        let Self { path, callback, .. } = self;
        File::options()
            .read(false)
            .write(true)
            .truncate(true)
            .open(path)
            .map_err(Error::IO)
            .and_then(|file| callback(file).map_err(Error::Callback))
    }

    /// Removes the lazy file.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("temp.txt", |mut f| {
    ///     f.write_all(b"temporary data")?;
    ///     Ok(())
    /// })?;
    ///
    /// // Create the file first
    /// lazy_file.open()?;
    ///
    /// // Remove the file when no longer needed
    /// lazy_file.remove()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the file exists but cannot be removed due to permissions or file system operations fail.
    pub fn remove(&self) -> Result<()> {
        let Self { path, cache_root, .. } = self;
        if path.exists() {
            fs::remove_file(path)?;

            // Remove empty parent directories up to cache root
            let mut current_parent = path.parent();
            while let Some(parent_dir) = current_parent
                && parent_dir != *cache_root
                && fs::read_dir(parent_dir)?.next().is_none()
            {
                // Try to remove the directory if it's empty
                fs::remove_dir(parent_dir)?;
                current_parent = parent_dir.parent();
            }
        }
        Ok(())
    }

    /// Initializes the lazy file, converting it to a [`CacheFile`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("settings.txt", |mut f| {
    ///     f.write_all(b"default settings")?;
    ///     Ok(())
    /// })?;
    ///
    /// // Initialize and convert to CacheFile
    /// let cache_file = lazy_file.init()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the file creation fails, the callback function returns an error, or file system operations fail.
    pub fn init(self) -> Result<CacheFile<'a>> {
        let Self { path, .. } = &self;
        if !path.exists() {
            let _ = self.create()?;
        }
        let cache_file = CacheFile(self);
        Ok(cache_file)
    }
}

impl Debug for CacheLazyFile<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            path,
            refresh_interval,
            locked,
            ..
        } = self;
        f.debug_struct("LazyFile")
            .field("path", &path)
            .field("callback", &"...")
            .field("refresh_interval", &refresh_interval)
            .field("locked", &locked)
            .finish()
    }
}

/// A file in the cache.
///
/// Files are created immediately and can be accessed right away through the cache.
pub struct CacheFile<'a>(CacheLazyFile<'a>);

impl CacheFile<'_> {
    /// Sets the refresh interval for the file.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::time::Duration;
    ///
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("data.txt", |mut f| {
    ///     f.write_all(b"content")?;
    ///     Ok(())
    /// })?;
    /// let cache_file = lazy_file.init()?;
    ///
    /// // Set custom refresh interval to 10 minutes
    /// let cache_file = cache_file.with_refresh_interval(Duration::from_secs(10 * 60));
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn with_refresh_interval(self, refresh_interval: Duration) -> Self {
        let Self(inner) = self;
        let inner = inner.with_refresh_interval(refresh_interval);
        Self(inner)
    }

    /// Sets the refresh interval to the default value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::time::Duration;
    ///
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("data.txt", |mut f| {
    ///     f.write_all(b"content")?;
    ///     Ok(())
    /// })?;
    /// let cache_file = lazy_file.init()?;
    ///
    /// // Set custom interval, then reset to default
    /// let cache_file = cache_file
    ///     .with_refresh_interval(Duration::from_secs(120))
    ///     .with_default_refresh_interval();
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn with_default_refresh_interval(self) -> Self {
        let Self(inner) = self;
        let inner = inner.with_default_refresh_interval();
        Self(inner)
    }

    /// Returns the path of the file.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("settings.txt", |mut f| {
    ///     f.write_all(b"settings data")?;
    ///     Ok(())
    /// })?;
    /// let cache_file = lazy_file.init()?;
    ///
    /// // Get the file path
    /// let path = cache_file.path();
    /// println!("Cache file located at: {}", path.display());
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn path(&self) -> &Path {
        let Self(inner) = self;
        inner.path()
    }

    /// Returns the refresh interval of the file.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::time::Duration;
    ///
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache
    ///     .get_lazy("data.txt", |mut f| {
    ///         f.write_all(b"content")?;
    ///         Ok(())
    ///     })?
    ///     .with_refresh_interval(Duration::from_secs(600));
    /// let cache_file = lazy_file.init()?;
    ///
    /// // Check the current refresh interval
    /// let interval = cache_file.refresh_interval();
    /// println!("Cache refresh interval: {} seconds", interval.as_secs());
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn refresh_interval(&self) -> Duration {
        let Self(inner) = self;
        inner.refresh_interval()
    }

    /// Returns whether the file is locked.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("data.txt", |mut f| {
    ///     f.write_all(b"content")?;
    ///     Ok(())
    /// })?;
    /// let mut cache_file = lazy_file.init()?;
    ///
    /// // Check if the file is locked
    /// assert!(!cache_file.is_locked());
    /// cache_file.lock()?;
    /// assert!(cache_file.is_locked());
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn is_locked(&self) -> bool {
        let Self(inner) = self;
        inner.is_locked()
    }

    /// Returns whether the file is unlocked.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("data.txt", |mut f| {
    ///     f.write_all(b"content")?;
    ///     Ok(())
    /// })?;
    /// let mut cache_file = lazy_file.init()?;
    ///
    /// // Check if the file is unlocked
    /// assert!(cache_file.is_unlocked());
    /// cache_file.lock()?;
    /// assert!(!cache_file.is_unlocked());
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn is_unlocked(&self) -> bool {
        let Self(inner) = self;
        inner.is_unlocked()
    }

    /// Checks if the file is valid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("config.txt", |mut f| {
    ///     f.write_all(b"config data")?;
    ///     Ok(())
    /// })?;
    /// let cache_file = lazy_file.init()?;
    ///
    /// // Check if the cache file is still valid
    /// if cache_file.is_valid()? {
    ///     println!("File is valid, using cached content");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the file metadata cannot be read, modification time cannot be determined, or system time calculations fail.
    pub fn is_valid(&self) -> Result<bool> {
        let Self(inner) = self;
        inner.is_valid()
    }

    /// Checks if the file is invalid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("data.txt", |mut f| {
    ///     f.write_all(b"cached data")?;
    ///     Ok(())
    /// })?;
    /// let cache_file = lazy_file.init()?;
    ///
    /// // Check if the cache file needs refreshing
    /// if cache_file.is_invalid()? {
    ///     println!("File is invalid, needs refresh");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the file metadata cannot be read, modification time cannot be determined, or system time calculations fail.
    pub fn is_invalid(&self) -> Result<bool> {
        let Self(inner) = self;
        inner.is_invalid()
    }

    /// Returns the time until the file is valid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("data.txt", |mut f| {
    ///     f.write_all(b"content")?;
    ///     Ok(())
    /// })?;
    /// let cache_file = lazy_file.init()?;
    ///
    /// // Get when the file will expire
    /// let valid_until = cache_file.valid_until()?;
    /// println!("File valid until: {:?}", valid_until);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the file metadata cannot be read or the file's modification time cannot be determined.
    pub fn valid_until(&self) -> Result<SystemTime> {
        let Self(inner) = self;
        inner.valid_until()
    }

    /// Locks the file to prevent refreshing.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("shared.txt", |mut f| {
    ///     f.write_all(b"shared data")?;
    ///     Ok(())
    /// })?;
    /// let mut cache_file = lazy_file.init()?;
    ///
    /// // Lock the file to prevent concurrent access
    /// cache_file.lock()?;
    /// // ... perform critical operations ...
    /// cache_file.unlock()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the file is already locked.
    pub fn lock(&mut self) -> Result<()> {
        let Self(inner) = self;
        inner.lock()
    }

    /// Unlocks the file to allow refreshing.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("shared.txt", |mut f| {
    ///     f.write_all(b"shared data")?;
    ///     Ok(())
    /// })?;
    /// let mut cache_file = lazy_file.init()?;
    ///
    /// // Lock and then unlock the file
    /// cache_file.lock()?;
    /// // ... critical operations complete ...
    /// cache_file.unlock()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the file is already unlocked.
    pub fn unlock(&mut self) -> Result<()> {
        let Self(inner) = self;
        inner.unlock()
    }

    /// Opens the file.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::io::Read;
    ///
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("config.txt", |mut f| {
    ///     f.write_all(b"config data")?;
    ///     Ok(())
    /// })?;
    /// let cache_file = lazy_file.init()?;
    ///
    /// // Open and read the file content
    /// let mut file = cache_file.open()?;
    /// let mut content = String::new();
    /// file.read_to_string(&mut content)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if file creation fails (if the file doesn't exist), file refresh fails (if the file exists), the file cannot be opened for reading, or the callback function returns an error during creation.
    pub fn open(&self) -> Result<File> {
        let Self(inner) = self;
        inner.open()
    }

    /// Refreshes the file if it is invalid.
    ///
    /// This method only refreshes the file when it has expired. For unconditional refresh, see [`force_refresh`](Self::force_refresh).
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("cache.txt", |mut f| {
    ///     f.write_all(b"cached data")?;
    ///     Ok(())
    /// })?;
    /// let cache_file = lazy_file.init()?;
    ///
    /// // Refresh only if the file is invalid
    /// cache_file.refresh()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if file validity cannot be determined or force refresh fails when the file is invalid.
    pub fn refresh(&self) -> Result<()> {
        let Self(inner) = self;
        inner.refresh()
    }

    /// Forces a refresh of the file.
    ///
    /// This method refreshes the file regardless of its validity. For conditional refresh, see [`refresh`](Self::refresh).
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("data.txt", |mut f| {
    ///     f.write_all(b"fresh data")?;
    ///     Ok(())
    /// })?;
    /// let cache_file = lazy_file.init()?;
    ///
    /// // Force refresh regardless of validity
    /// cache_file.force_refresh()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the file cannot be opened for writing, the callback function returns an error, or file truncation fails.
    pub fn force_refresh(&self) -> Result<()> {
        let Self(inner) = self;
        inner.force_refresh()
    }

    /// Removes the file.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fcache::prelude::*;
    ///
    /// # fn wrapper() -> fcache::Result<()> {
    /// let cache = fcache::new()?;
    /// let lazy_file = cache.get_lazy("temp.txt", |mut f| {
    ///     f.write_all(b"temporary data")?;
    ///     Ok(())
    /// })?;
    /// let cache_file = lazy_file.init()?;
    ///
    /// // Remove the file when no longer needed
    /// cache_file.remove()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the file exists but cannot be removed due to permissions or file system operations fail.
    pub fn remove(&self) -> Result<()> {
        let Self(inner) = self;
        inner.remove()
    }
}

impl Debug for CacheFile<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(lazy_file) = self;
        let CacheLazyFile {
            path,
            refresh_interval,
            locked,
            ..
        } = lazy_file;
        f.debug_struct("File")
            .field("path", &path)
            .field("callback", &"...")
            .field("refresh_interval", &refresh_interval)
            .field("locked", &locked)
            .finish()
    }
}
