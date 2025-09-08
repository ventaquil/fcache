use anyhow::Result;
use fcache::prelude::*;

fn main() -> Result<()> {
    println!("=== Lazy File Creation ===");

    // Create a new cache instance
    let cache = fcache::new()?;

    // Create a lazy file - not created until first access
    let cache_file = cache.get_lazy("expensive_computation.txt", |mut file| {
        println!("  → Callback executed: Creating file content...");
        file.write_all(b"Result of expensive computation")?;
        Ok(())
    })?;

    println!("Lazy file object created");
    println!("File exists on disk: {}", cache_file.path().exists());

    if cache_file.path().exists() {
        anyhow::bail!("Lazy file should not exist yet");
    }

    println!("Opening lazy file for the first time...");
    // Opening the file triggers creation
    let mut content1 = String::new();
    cache_file.open()?.read_to_string(&mut content1)?;

    println!("File now exists on disk: {}", cache_file.path().exists());
    println!("File content: {content1}");

    // Verify content
    if content1 != "Result of expensive computation" {
        anyhow::bail!("Content mismatch");
    }

    println!("Opening file again (no callback this time)...");
    // Second access doesn't trigger callback again
    let mut content2 = String::new();
    cache_file.open()?.read_to_string(&mut content2)?;
    println!("Second read content: {}", content2);

    println!("✓ Lazy file creation successful!");

    Ok(())
}
