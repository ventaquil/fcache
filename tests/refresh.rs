mod common;

use std::sync::atomic::{AtomicUsize, Ordering};

use common::*;

#[test]
fn test_file_auto_refresh() -> anyhow::Result<()> {
    let i: AtomicUsize = AtomicUsize::new(0);

    // Create a new cache instance
    let cache = fcache::new()?.with_refresh_interval(Duration::ZERO); // Zero refresh interval to always refresh

    // Create a file in the cache
    let cache_file = cache.get_lazy("file.txt", move |mut file| {
        file.write_fmt(format_args!("{}", i.load(Ordering::SeqCst)))?;
        i.fetch_add(1, Ordering::SeqCst);
        Ok(())
    })?;

    // Read the initial content
    {
        let mut content = String::new();
        cache_file.open()?.read_to_string(&mut content)?;
        assert_eq!(content, "0");
    }

    // Refresh the file during the next access
    {
        let mut content = String::new();
        cache_file.open()?.read_to_string(&mut content)?;
        assert_eq!(content, "1");
    }

    Ok(())
}

#[test]
fn test_file_manual_refresh() -> anyhow::Result<()> {
    let i: AtomicUsize = AtomicUsize::new(0);

    // Create a new cache instance
    let cache = fcache::new()?.with_refresh_interval(Duration::MAX); // Max refresh interval to avoid auto-refresh

    // Create a file in the cache
    let cache_file = cache.get("file.txt", move |mut file| {
        file.write_fmt(format_args!("{}", i.load(Ordering::SeqCst)))?;
        i.fetch_add(1, Ordering::SeqCst);
        Ok(())
    })?;

    // Read the initial content
    {
        let mut content = String::new();
        cache_file.open()?.read_to_string(&mut content)?;
        assert_eq!(content, "0");
    }

    // Set to zero to allow immediate refresh
    let cache_file = cache_file.with_refresh_interval(Duration::ZERO);

    // Manually refresh the file
    cache_file.refresh()?;

    // Set to max to avoid auto-refresh
    let cache_file = cache_file.with_refresh_interval(Duration::MAX);

    // Read the content after manual refresh
    {
        let mut content = String::new();
        cache_file.open()?.read_to_string(&mut content)?;
        assert_eq!(content, "1");
    }

    Ok(())
}

#[test]
fn test_file_force_refresh() -> anyhow::Result<()> {
    let i: AtomicUsize = AtomicUsize::new(0);

    // Create a new cache instance
    let cache = fcache::new()?.with_refresh_interval(Duration::MAX); // Max refresh interval to avoid auto-refresh

    // Create a file in the cache
    let cache_file = cache.get("file.txt", move |mut file| {
        file.write_fmt(format_args!("{}", i.load(Ordering::SeqCst)))?;
        i.fetch_add(1, Ordering::SeqCst);
        Ok(())
    })?;

    // Read the initial content
    {
        let mut content = String::new();
        cache_file.open()?.read_to_string(&mut content)?;
        assert_eq!(content, "0");
    }

    // Force refresh the file
    cache_file.force_refresh()?;

    // Read the content after force refresh
    {
        let mut content = String::new();
        cache_file.open()?.read_to_string(&mut content)?;
        assert_eq!(content, "1");
    }

    Ok(())
}
