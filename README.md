# fcache

[![crates.io](https://img.shields.io/crates/v/fcache.svg?style=flat-square)](https://crates.io/crates/fcache)
[![Build](https://img.shields.io/github/actions/workflow/status/ventaquil/fcache/rust.yml?branch=master&style=flat-square&logo=github)](https://github.com/ventaquil/fcache/actions/workflows/rust.yml)
[![docs.rs](https://img.shields.io/docsrs/fcache?style=flat-square&logo=docsdotrs)](https://docs.rs/fcache/)
[![MSRV](https://img.shields.io/badge/MSRV-1.88.0-informational?style=flat-square)](https://github.com/ventaquil/fcache/blob/master/Cargo.toml)
[![LICENSE](https://img.shields.io/github/license/ventaquil/fcache?style=flat-square)](https://github.com/ventaquil/fcache/blob/master/LICENSE)

A Rust library for efficient file caching with a straightforward interface for creating, retrieving, and managing cached files efficiently.

## Setup

Add this to your `Cargo.toml`:

```toml
[dependencies]
fcache = "0.0.0"
```

Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:

```bash
cargo add fcache
```

## Usage

Use the library functions to create cache instances and manage files:

```rust
use fcache::prelude::*;

fn main() -> fcache::Result<()> {
    // Create a new cache instance
    let cache = fcache::new()?;

    // Create a file with callback
    let cache_file = cache.get("data.txt", |mut file| {
        file.write_all(b"Hello, World!")?;
        Ok(())
    })?;

    // Read from the file
    let mut content = String::new();
    cache_file.open()?.read_to_string(&mut content)?;
    println!("Content: {}", content);

    Ok(())
}
```

For more usage examples, refer to the documentation available at [docs.rs](https://docs.rs/fcache/).

## License

This crate is licensed under the MIT License.
