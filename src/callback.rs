use std::fs::File;
use std::{error, result};

#[cfg(doc)]
use crate::Cache;

/// Trait alias for callback functions used in cache operations.
///
/// Check the [`Cache::get`] and [`Cache::get_lazy`] methods for more details on how to use this trait.
pub trait CallbackFn: Fn(File) -> result::Result<(), Box<dyn error::Error + Send + Sync>> + Send + Sync {}

impl<T> CallbackFn for T where T: Fn(File) -> result::Result<(), Box<dyn error::Error + Send + Sync>> + Send + Sync {}
