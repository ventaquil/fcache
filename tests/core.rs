mod common;

use common::*;

#[test]
fn test_cache_new() -> anyhow::Result<()> {
    // Create a new cache instance
    let cache = fcache::new()?;

    // Verify cache directory exists
    assert!(cache.path().exists());
    assert!(cache.path().is_dir());

    // Verify default refresh interval
    assert_eq!(cache.refresh_interval(), fcache::DEFAULT_REFRESH_INTERVAL);

    Ok(())
}

#[test]
fn test_cache_with_prefix() -> anyhow::Result<()> {
    let prefix = "fcache_test_prefix";

    // Create a new cache instance with prefix
    let cache = fcache::with_prefix(prefix)?;

    // Verify cache directory exists and contains the prefix
    assert!(cache.path().exists());
    assert!(cache.path().is_dir());
    assert_eq!(
        cache
            .path()
            .file_name()
            .and_then(|file_name| file_name.to_str())
            .map(|file_name| file_name.starts_with(prefix)),
        Some(true)
    );

    Ok(())
}

#[test]
fn test_cache_with_dir() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;

    // Create a new cache instance
    let cache = fcache::with_dir(temp_dir.path())?;

    // Verify cache uses the specified directory
    assert_eq!(cache.path(), temp_dir.path());
    assert!(cache.path().exists());

    // Verify default refresh interval
    assert_eq!(cache.refresh_interval(), fcache::DEFAULT_REFRESH_INTERVAL);

    Ok(())
}

#[test]
fn test_cache_with_file() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;

    // Create a new file within the temporary directory
    let file_path = temp_dir.path().join("custom_cache_dir");
    let _ = File::create(&file_path)?;

    // Create a new cache instance
    assert!(
        matches!(fcache::with_dir(file_path), Err(fcache::Error::NotADirectory { .. }),),
        "Should return an error when providing a file path instead of directory"
    );

    Ok(())
}

#[test]
fn test_cache_with_refresh_interval() -> anyhow::Result<()> {
    let refresh_interval = Duration::from_secs(10);

    // Create a new cache instance
    let cache = fcache::new()?.with_refresh_interval(refresh_interval);

    // Verify custom refresh interval
    assert_eq!(cache.refresh_interval(), refresh_interval);

    Ok(())
}

#[test]
fn test_cache_with_default_refresh_interval() -> anyhow::Result<()> {
    let refresh_interval = Duration::from_secs(30);

    // Create a new cache instance
    let cache = fcache::new()?
        .with_refresh_interval(refresh_interval)
        .with_default_refresh_interval();

    // Verify default refresh interval is restored
    assert_eq!(cache.refresh_interval(), fcache::DEFAULT_REFRESH_INTERVAL);

    Ok(())
}
