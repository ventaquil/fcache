use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use fcache::prelude::*;

fn main() -> anyhow::Result<()> {
    println!("=== Force Refresh Demonstration ===");

    // Counter to track callback executions
    let counter = Arc::new(AtomicU32::new(0));

    // Create cache with longer refresh interval to demonstrate force refresh
    let cache = fcache::new()?.with_refresh_interval(Duration::from_secs(3600)); // 1 hour
    println!("Cache refresh interval: {:?}", cache.refresh_interval());

    // Create a file that generates dynamic content
    let counter_clone = Arc::clone(&counter);
    let cache_file = cache.get("dynamic_data.txt", move |mut file| {
        let count = counter_clone.fetch_add(1, Ordering::SeqCst) + 1;
        let content = format!("Generated content #{}", count);
        file.write_all(content.as_bytes())?;
        println!("  → Callback executed #{}: {}", count, content);
        Ok(())
    })?;

    // Read initial content
    let mut content1 = String::new();
    cache_file.open()?.read_to_string(&mut content1)?;
    println!("Initial content: {content1}");

    // Try reading again - should use cached version (no callback)
    println!("\nReading again (should use cache):");
    let mut content2 = String::new();
    cache_file.open()?.read_to_string(&mut content2)?;
    println!("Cached content: {content2}");

    // Force refresh - should trigger callback even though cache is valid
    println!("\nForce refreshing file:");
    cache_file.force_refresh()?;

    // Read after force refresh
    let mut content3 = String::new();
    cache_file.open()?.read_to_string(&mut content3)?;
    println!("Content after force refresh: {content3}");

    // Verify content changed
    if content1 == content3 {
        anyhow::bail!("Content should have changed after force refresh");
    }

    println!("✓ Force refresh demonstration successful!");
    println!("Total callback executions: {}", counter.load(Ordering::SeqCst));

    Ok(())
}
