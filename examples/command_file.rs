use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use anyhow::Result;
use fcache::prelude::*;
use signal_hook::consts::SIGINT;
use signal_hook::iterator::Signals;

fn main() -> Result<()> {
    // Set up a signal handler to gracefully exit on Ctrl+C
    let running = Arc::new(AtomicBool::new(true));
    let mut signals = Signals::new([SIGINT])?;
    {
        let running = Arc::clone(&running);
        thread::spawn(move || {
            if signals.forever().next().is_some() {
                running.store(false, Ordering::SeqCst);
            }
        });
    }

    // Print a header message
    println!("You should observe the uptime of the system being printed every second.");
    println!("The output changes every 5 seconds due to cache interval. Press Ctrl+C to stop.");

    // Wait for a few seconds so user can read the header
    thread::sleep(Duration::from_secs(5));

    // Print an empty line for better readability
    println!();

    // Create a new cache instance
    let cache = fcache::new()?;

    // Create a lazy file
    let file = {
        cache.get_lazy("uptime", move |mut file| {
            let uptime = Command::new("uptime").output()?;
            file.write_all(&uptime.stdout)?;
            Ok(())
        })?
    };
    // Check if the lazy file exists, as it should not be created until opened
    if file.path().exists() {
        anyhow::bail!("Lazy file should not exist");
    }

    while running.load(Ordering::SeqCst) {
        // Open the lazy file and read its content
        let mut content = String::new();
        file.open()?.read_to_string(&mut content)?;

        // Print the uptime content
        print!("Uptime: {content}");

        // Sleep for a second before the next iteration
        thread::sleep(Duration::from_secs(1));
    }

    println!("Program terminated gracefully.");

    Ok(())
}
