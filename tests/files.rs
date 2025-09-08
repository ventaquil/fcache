mod common;

use common::*;

#[test]
fn test_get_file() -> anyhow::Result<()> {
    // Create a new cache instance
    let cache = fcache::new()?;

    // Create a file in the cache
    let cache_file = cache.get("file.txt", |mut file| {
        file.write_all(TEST_CONTENT)?;
        Ok(())
    })?;

    // Verify file name matches
    assert_eq!(cache_file.name(), "file.txt");

    // Verify file path ends with name
    assert!(cache_file.path().ends_with(cache_file.name()));

    // Verify file exists on disk
    assert!(cache_file.path().exists());

    // Verify content matches
    let mut content = Vec::new();
    cache_file.open()?.read_to_end(&mut content)?;
    assert_eq!(content, TEST_CONTENT, "File content does not match");

    Ok(())
}

#[test]
fn test_get_lazy_file() -> anyhow::Result<()> {
    // Create a new cache instance
    let cache = fcache::new()?;

    // Create a lazy file in the cache (not created until accessed)
    let cache_file = cache.get_lazy("file.txt", |mut file| {
        file.write_all(TEST_CONTENT)?;
        Ok(())
    })?;

    // Verify file name matches
    assert_eq!(cache_file.name(), "file.txt");

    // Verify file path ends with name
    assert!(cache_file.path().ends_with(cache_file.name()));

    // Verify file doesn't exist yet
    assert!(!cache_file.path().exists());

    // Access the file (triggers creation)
    let mut file = cache_file.open()?;

    // Now file should exist
    assert!(cache_file.path().exists());

    // Verify content matches
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    assert_eq!(content, TEST_CONTENT, "File content does not match");

    Ok(())
}

#[test]
fn test_double_file_get() -> anyhow::Result<()> {
    // Create a new cache instance
    let cache = fcache::new()?;

    // Create a file in the cache
    let _ = cache.get("file.txt", |_| Ok(()))?;

    // Create a second reference to the same file
    assert!(
        matches!(
            cache.get("file.txt", |_| Ok(())),
            Err(fcache::Error::FileAlreadyExists { .. })
        ),
        "Should return an error when trying to create the same file twice"
    );

    Ok(())
}

#[test]
fn test_file_empty_name() -> anyhow::Result<()> {
    // Create a new cache instance
    let cache = fcache::new()?;

    // Create a file in the cache
    assert!(
        matches!(cache.get("", |_| Ok(())), Err(fcache::Error::InvalidPath { .. }),),
        "Should return an error when trying to create a file with empty name"
    );

    // Create a file in the cache
    assert!(
        matches!(cache.get(" ", |_| Ok(())), Err(fcache::Error::InvalidPath { .. }),),
        "Should return an error when trying to create a file with empty name"
    );

    // Create a file in the cache
    assert!(
        matches!(cache.get("\t", |_| Ok(())), Err(fcache::Error::InvalidPath { .. }),),
        "Should return an error when trying to create a file with empty name"
    );

    // Create a file in the cache
    assert!(
        matches!(cache.get("\n", |_| Ok(())), Err(fcache::Error::InvalidPath { .. }),),
        "Should return an error when trying to create a file with empty name"
    );

    Ok(())
}

#[test]
fn test_file_dir_name() -> anyhow::Result<()> {
    // Create a new cache instance
    let cache = fcache::new()?;

    // Create a file in a subdirectory
    assert!(
        matches!(cache.get("dir/", |_| Ok(())), Err(fcache::Error::InvalidPath { .. }),),
        "Should return an error when trying to create a file with a trailing slash"
    );

    Ok(())
}

#[test]
fn test_file_out_of_cache() -> anyhow::Result<()> {
    // Create a new cache instance
    let cache = fcache::new()?;

    // Create a file out of the cache
    assert!(
        matches!(
            cache.get("../file.txt", |_| Ok(())),
            Err(fcache::Error::PathTraversal { .. }),
        ),
        "Should return an error when trying to create a file outside the cache"
    );

    // Create a file out of the cache
    assert!(
        matches!(
            cache.get("a/../../file.txt", |_| Ok(())),
            Err(fcache::Error::PathTraversal { .. }),
        ),
        "Should return an error when trying to create a file outside the cache"
    );

    // Create a file out of the cache
    assert!(
        matches!(
            cache.get("a/b/../c/../../../d/file.txt", |_| Ok(())),
            Err(fcache::Error::PathTraversal { .. }),
        ),
        "Should return an error when trying to create a file outside the cache"
    );

    Ok(())
}

#[test]
fn test_file_callback_error() -> anyhow::Result<()> {
    // Create a new cache instance
    let cache = fcache::new()?;

    // Create a file in the cache
    assert!(
        matches!(
            cache.get("file.txt", |_| {
                let _ = "fail".parse::<i32>()?;
                Ok(())
            }),
            Err(fcache::Error::Callback { .. })
        ),
        "Should return an error when callback fails"
    );

    Ok(())
}

#[test]
fn test_file_removal() -> anyhow::Result<()> {
    // Create a new cache instance
    let cache = fcache::new()?;

    // Create a file in the cache
    let cache_file = cache.get("file.txt", |_| Ok(()))?;

    // Verify file exists
    assert!(cache_file.path().exists());

    // Remove the file
    cache_file.remove()?;

    // Verify file is gone
    assert!(!cache_file.path().exists());

    Ok(())
}

#[test]
fn test_nested_file_removal() -> anyhow::Result<()> {
    // Create a new cache instance
    let cache = fcache::new()?;

    // Create a file in the cache
    let cache_file = cache.get("a/b/c/d/file.txt", |_| Ok(()))?;

    // Verify file name matches
    assert_eq!(cache_file.name(), "file.txt");

    // Verify file path ends with name
    assert!(cache_file.path().ends_with(cache_file.name()));

    // Create a file
    let _ = cache.get("a/b/c/file.txt", |_| Ok(()))?;

    // Verify file exists
    assert!(cache_file.path().exists());

    // Remove the file
    cache_file.remove()?;

    // Verify file is gone
    assert!(!cache_file.path().exists());
    assert_eq!(
        cache_file.path().parent().map(|parent| parent.exists()),
        Some(false),
        "Parent directory should not exist"
    );
    assert_eq!(
        cache_file
            .path()
            .parent()
            .and_then(|parent| parent.parent())
            .map(|parent| parent.exists()),
        Some(true),
        "Grandparent directory should exist"
    );

    Ok(())
}

#[test]
fn test_large_file_content() -> anyhow::Result<()> {
    // Create a new cache instance
    let cache = fcache::new()?;

    // Create a file in the cache
    let cache_file = cache.get("file.txt", |mut file| {
        file.write_all(TEST_LARGE_CONTENT)?;
        Ok(())
    })?;

    // Verify file exists on disk
    assert!(cache_file.path().exists());

    // Verify content matches
    let mut content = Vec::new();
    cache_file.open()?.read_to_end(&mut content)?;
    assert_eq!(content, TEST_LARGE_CONTENT, "File content does not match");

    Ok(())
}

#[test]
fn test_file_with_refresh_interval() -> anyhow::Result<()> {
    let refresh_interval = Duration::from_secs(10);

    // Create a new cache instance
    let cache = fcache::new()?;

    // Create a file in the cache
    let cache_file = cache
        .get("file.txt", |_| Ok(()))?
        .with_refresh_interval(refresh_interval);

    // Verify refresh interval
    assert_eq!(
        cache_file.refresh_interval(),
        refresh_interval,
        "Refresh interval was not updated"
    );

    Ok(())
}

#[test]
fn test_file_with_default_refresh_interval() -> anyhow::Result<()> {
    let refresh_interval = Duration::from_secs(10);

    // Create a new cache instance
    let cache = fcache::new()?;

    // Create a file in the cache
    let cache_file = cache
        .get("file.txt", |_| Ok(()))?
        .with_refresh_interval(refresh_interval);

    // Update the cache file to use the default refresh interval
    let cache_file = cache_file.with_default_refresh_interval();

    // Verify the refresh interval is set to the default
    assert_eq!(
        cache_file.refresh_interval(),
        cache.refresh_interval(),
        "Refresh interval was not set to default"
    );

    Ok(())
}
