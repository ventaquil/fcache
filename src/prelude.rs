//! Convenience module that re-exports commonly used types and traits.
//!
//! The prelude module provides a convenient way to import all the essential
//! types and traits needed for most fcache operations in a single use statement.
//!
//! # Usage
//!
//! ```rust
//! use fcache::prelude::*;
//!
//! # fn wrapper() -> fcache::Result<()> {
//! // Create a cache and use it
//! let cache = fcache::new()?;
//! let cache_file = cache.get("example.txt", |mut file| {
//!     file.write_all(b"Hello, prelude!")?;
//!     Ok(())
//! })?;
//! # Ok(())
//! # }
//! ```

#[doc(no_inline)]
pub use std::fs::File;
#[doc(no_inline)]
pub use std::io::{Read, Write};
#[doc(no_inline)]
pub use std::time::Duration;

pub use crate::{Cache, CacheFile, CacheLazyFile};
