# Examples

This directory contains practical examples demonstrating the fcache library features. Each example is a complete, runnable program showcasing different caching scenarios.

## Running Examples

To run any example, use the following command from the project root:

```bash
cargo run --example <example_name>
```

For example:

```bash
cargo run --example temp_cache
```

## Available Examples

### 1. `temp_cache.rs`

**Basic temporary cache usage**

Introduction to fcache:

- Creating a temporary cache with `fcache::new()`
- Creating files with immediate initialization
- Writing and reading file content
- Basic error handling

### 2. `dir_cache.rs`

**Directory-based cache**

Persistent caching:

- Using `fcache::with_dir()` for specific cache location
- Creating files in persistent directories
- Understanding temporary vs persistent caches

### 3. `cache_refresh.rs`

**Cache-level refresh intervals**

Cache configuration:

- Setting custom refresh intervals with `with_refresh_interval()`
- Resetting to defaults with `with_default_refresh_interval()`
- How cache settings affect all files

### 4. `file_refresh.rs`

**File-level refresh intervals**

Fine-grained refresh control:

- Setting individual file refresh intervals
- Overriding cache defaults for specific files
- Resetting file intervals back to cache defaults
- Managing multiple files with different refresh patterns

### 5. `lazy_file.rs`

**Lazy file creation**

Deferred initialization:

- Creating files with `cache.get_lazy()`
- Understanding when callback functions execute
- Verifying lazy behavior with existence checks
- Optimizing for expensive operations

### 6. `force_refresh.rs`

**Manual refresh operations**

Cache invalidation:

- Using `force_refresh()` to bypass cache timing
- Working with longer refresh intervals
- Tracking callback execution with counters
- Understanding when to force updates

### 7. `file_locking.rs`

**File locking mechanisms**

Synchronization and consistency:

- Locking files with `lock()` and `unlock()`
- Understanding lock behavior and limitations
- Error handling for double lock/unlock attempts
- Preventing refresh during critical operations

### 8. `command_file.rs`

**External command integration**

Complex real-world usage:

- Executing external commands in callbacks
- Real-time content updates with automatic refresh
- Signal handling for graceful shutdown
- Background thread management
- Continuous operation patterns
