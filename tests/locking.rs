#[test]
fn test_new_file_unlocked_by_default() -> anyhow::Result<()> {
    // Create a new cache instance
    let cache = fcache::new()?;

    // Create a file in the cache
    let cache_file = cache.get("file.txt", |_| Ok(()))?;

    // Verify file is unlocked
    assert!(!cache_file.is_locked(), "File should be initially unlocked");
    assert!(cache_file.is_unlocked(), "File should be initially unlocked");

    Ok(())
}
#[test]
fn test_file_locking() -> anyhow::Result<()> {
    // Create a new cache instance
    let cache = fcache::new()?;

    // Create a file in the cache
    let mut cache_file = cache.get("file.txt", |_| Ok(()))?;

    // Lock the file
    cache_file.lock()?;

    // Verify file is locked
    assert!(cache_file.is_locked(), "File should be locked");
    assert!(!cache_file.is_unlocked(), "File should be locked");

    // Unlock the file
    cache_file.unlock()?;

    // Verify file is unlocked
    assert!(!cache_file.is_locked(), "File should be unlocked");
    assert!(cache_file.is_unlocked(), "File should be unlocked");

    Ok(())
}
