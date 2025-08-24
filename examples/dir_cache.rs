use fcache::prelude::*;

fn main() -> anyhow::Result<()> {
    const FILE_CONTENT: &str = "Persistent cache data";

    println!("=== Directory-based Cache Usage ===");

    // Create cache in specific directory
    let cache = fcache::with_dir("./fcache_example")?;
    println!("Cache directory: {}", cache.path().display());

    // Create a file in the specified directory
    let file = cache.get("data.txt", |mut file| {
        file.write_all(FILE_CONTENT.as_bytes())?;
        Ok(())
    })?;

    println!("File created at: {}", file.path().display());

    // Read content back
    let mut content = String::new();
    file.open()?.read_to_string(&mut content)?;
    println!("File content: {content}");

    // Verify content
    if content != FILE_CONTENT {
        anyhow::bail!("Content mismatch");
    }

    println!("✓ Directory cache successful!");
    println!("Note: Files will persist in ./fcache_example/ directory");

    Ok(())
}
