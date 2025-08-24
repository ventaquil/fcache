use fcache::prelude::*;

fn main() -> anyhow::Result<()> {
    println!("=== File Refresh Intervals ===");

    // Create cache with default refresh interval
    let cache = fcache::new()?;
    println!("Cache default refresh interval: {:?}", cache.refresh_interval());

    // Create a file with default refresh interval
    let file1 = cache.get("default_refresh.txt", |mut file| {
        const FILE_CONTENT: &str = "File with default refresh";
        file.write_all(FILE_CONTENT.as_bytes())?;
        Ok(())
    })?;

    println!("File 1 uses cache default refresh interval");

    // Create a file and change its refresh interval
    let file2 = cache
        .get("custom_refresh.txt", |mut file| {
            const FILE_CONTENT: &str = "File with custom refresh";
            file.write_all(FILE_CONTENT.as_bytes())?;
            Ok(())
        })?
        .with_refresh_interval(Duration::from_secs(60));

    println!("File 2 uses custom 60-second refresh interval");

    // Create another file and reset to cache defaults
    let file3 = cache.get("reset_refresh.txt", |mut file| {
        const FILE_CONTENT: &str = "File with reset refresh";
        file.write_all(FILE_CONTENT.as_bytes())?;
        Ok(())
    })?.with_refresh_interval(Duration::from_secs(120)) // First set custom
     .with_default_refresh_interval(); // Then reset to cache default

    println!("File 3 refresh interval reset to cache default");

    // Demonstrate reading from all files
    let mut content1 = String::new();
    file1.open()?.read_to_string(&mut content1)?;
    println!("File 1 content: {content1}");

    let mut content2 = String::new();
    file2.open()?.read_to_string(&mut content2)?;
    println!("File 2 content: {content2}");

    let mut content3 = String::new();
    file3.open()?.read_to_string(&mut content3)?;
    println!("File 3 content: {content3}");

    println!("✓ File refresh intervals demonstration successful!");

    Ok(())
}
