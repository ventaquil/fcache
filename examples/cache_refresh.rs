use fcache::prelude::*;

fn main() -> anyhow::Result<()> {
    println!("=== Cache Refresh Intervals ===");

    // Demonstrate default refresh interval
    {
        const FILE_CONTENT: &str = "Data with default refresh";

        let cache = fcache::new()?;
        println!("Default refresh interval: {:?}", cache.refresh_interval());

        let cache_file = cache.get("default.txt", |mut file| {
            file.write_all(FILE_CONTENT.as_bytes())?;
            Ok(())
        })?;

        let mut content = String::new();
        cache_file.open()?.read_to_string(&mut content)?;
        println!("Default file content: {content}");
    }

    // Demonstrate custom refresh interval
    {
        const FILE_CONTENT: &str = "Data with 30s refresh";

        let cache = fcache::new()?.with_refresh_interval(Duration::from_secs(30));
        println!("Custom refresh interval: {:?}", cache.refresh_interval());

        let cache_file = cache.get("custom.txt", |mut file| {
            file.write_all(FILE_CONTENT.as_bytes())?;
            Ok(())
        })?;

        let mut content = String::new();
        cache_file.open()?.read_to_string(&mut content)?;
        println!("Custom file content: {content}");
    }

    // Demonstrate resetting to default
    {
        const FILE_CONTENT: &str = "Data with reset refresh";

        let cache = fcache::new()?
            .with_refresh_interval(Duration::from_secs(60))  // Set custom
            .with_default_refresh_interval(); // Reset to default
        println!("Reset to default: {:?}", cache.refresh_interval());

        let cache_file = cache.get("reset.txt", |mut file| {
            file.write_all(FILE_CONTENT.as_bytes())?;
            Ok(())
        })?;

        let mut content = String::new();
        cache_file.open()?.read_to_string(&mut content)?;
        println!("Reset file content: {content}");
    }

    println!("✓ Cache refresh intervals demonstration successful!");

    Ok(())
}
