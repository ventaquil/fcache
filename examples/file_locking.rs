use std::thread::sleep;

use chrono::Local;
use fcache::prelude::*;

fn main() -> anyhow::Result<()> {
    println!("=== File Locking Demonstration ===");

    // Create cache with short refresh interval for demonstration
    let cache = fcache::new()?.with_refresh_interval(Duration::from_millis(100));

    // Create a file
    let mut file = cache.get("locked_file.txt", |mut file| {
        let datetime = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let content = format!("Generated at: {datetime}");
        file.write_all(content.as_bytes())?;
        Ok(())
    })?;

    println!("File created with initial content");

    // Read initial content
    let mut content = String::new();
    file.open()?.read_to_string(&mut content)?;
    println!("Initial content: {content}");

    // Lock the file to prevent refreshing
    println!("\nLocking file...");
    file.lock()?;
    println!("File is now locked");

    // Try to lock again (should fail)
    println!("Trying to lock already locked file:");
    match file.lock() {
        Ok(_) => anyhow::bail!("Should not be able to lock already locked file"),
        Err(error) => println!("  Expected error: {error}"),
    }

    // Wait longer than refresh interval to show lock prevents refresh
    println!("\nWaiting 200ms (longer than refresh interval)...");
    sleep(Duration::from_millis(200));

    // File should still have old content due to lock
    let mut locked_content = String::new();
    file.open()?.read_to_string(&mut locked_content)?;
    println!("Content while locked: {locked_content}");

    // Unlock the file
    println!("\nUnlocking file...");
    file.unlock()?;
    println!("File is now unlocked");

    // Try to unlock again (should fail)
    println!("Trying to unlock already unlocked file:");
    match file.unlock() {
        Ok(_) => anyhow::bail!("Should not be able to unlock already unlocked file"),
        Err(error) => println!("  Expected error: {error}"),
    }

    // Now file can be refreshed normally
    println!("\nFile operations after unlock work normally");
    let mut unlocked_content = String::new();
    file.open()?.read_to_string(&mut unlocked_content)?;
    println!("Content after unlock: {unlocked_content}");

    println!("✓ File locking demonstration successful!");
    println!("Note: Locking prevents file refresh but allows reading");

    Ok(())
}
