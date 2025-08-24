use fcache::prelude::*;

fn main() -> anyhow::Result<()> {
    const FILE_CONTENT: &str = "Hello from fcache";

    println!("=== Basic Temporary Cache Usage ===");

    // Create a new cache instance in temporary directory
    let cache = fcache::new()?;
    println!("Cache directory: {}", cache.path().display());

    // Create a file with content
    let file = cache.get("example.txt", |mut file| {
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

    println!("✓ Basic usage successful!");

    Ok(())
}
