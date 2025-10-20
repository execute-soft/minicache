# 🚀 MiniCache

[![Crates.io](https://img.shields.io/crates/v/minicache.svg)](https://crates.io/crates/minicache)
[![Documentation](https://docs.rs/minicache/badge.svg)](https://docs.rs/minicache)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/execute-soft/minicache/workflows/CI/badge.svg)](https://github.com/execute-soft/minicache/actions)

A fast, lightweight, async-compatible in-memory cache for Rust with TTL (Time-To-Live) support and automatic cleanup. Perfect for async applications that need efficient caching without the complexity.

## ✨ Features

- **🔥 High Performance**: Millions of operations per second
- **⚡ Async/Await Ready**: Built for `tokio` and async applications  
- **⏰ TTL Support**: Automatic expiration with background cleanup
- **🔒 Thread-Safe**: Concurrent access with `Arc` + `RwLock`
- **💾 Memory Efficient**: Minimal overhead per cache entry
- **🛠 Easy to Use**: Simple API with comprehensive examples
- **📊 Battle Tested**: Extensive benchmarks and tests included

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
minicache = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## 🚀 Quick Start

```rust
use minicache::MiniCache;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Create cache with 60-second cleanup interval
    let cache = MiniCache::new(Duration::from_secs(60));

    // Set a value (no expiration)
    cache.set("user:123", "John Doe", None).await;

    // Set a value with TTL
    cache.set("session:abc", "temp_data", Some(Duration::from_secs(300))).await;

    // Get values
    if let Some(user) = cache.get(&"user:123").await {
        println!("User: {}", user);
    }

    // Check if key exists
    if cache.contains(&"session:abc").await {
        println!("Session is active");
    }

    // Remove a key
    cache.remove(&"user:123").await;

    // Get cache statistics
    println!("Cache size: {}", cache.len().await);
    println!("All keys: {:?}", cache.keys().await);
}
```

## 📚 Usage Examples

### Basic Operations

```rust
use minicache::MiniCache;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let cache = MiniCache::new(Duration::from_secs(60));

    // String keys and values
    cache.set("name", "Alice", None).await;
    assert_eq!(cache.get(&"name").await, Some("Alice"));

    // Numeric keys
    cache.set(42, "The Answer", None).await;
    assert_eq!(cache.get(&42).await, Some("The Answer"));

    // Custom types (must implement Clone)
    #[derive(Clone, PartialEq, Debug)]
    struct User { id: u32, name: String }
    
    let user = User { id: 1, name: "Bob".to_string() };
    cache.set("user:1", user.clone(), None).await;
    assert_eq!(cache.get(&"user:1").await, Some(user));
}
```

### TTL (Time-To-Live) Usage

```rust
use minicache::MiniCache;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let cache = MiniCache::new(Duration::from_millis(100));

    // Set with 200ms TTL
    cache.set("temp", "expires soon", Some(Duration::from_millis(200))).await;
    
    // Value exists immediately
    assert_eq!(cache.get(&"temp").await, Some("expires soon"));
    
    // Wait for expiration
    sleep(Duration::from_millis(250)).await;
    
    // Value has expired
    assert_eq!(cache.get(&"temp").await, None);
}
```

### Concurrent Access

```rust
use minicache::MiniCache;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let cache = Arc::new(MiniCache::new(Duration::from_secs(60)));
    let mut handles = vec![];

    // Spawn multiple tasks
    for i in 0..10 {
        let cache_clone = cache.clone();
        let handle = tokio::spawn(async move {
            // Each task writes 1000 entries
            for j in 0..1000 {
                let key = format!("task_{}_{}", i, j);
                let value = format!("value_{}_{}", i, j);
                cache_clone.set(key, value, None).await;
            }
        });
        handles.push(handle);
    }

    // Wait for all tasks
    for handle in handles {
        handle.await.unwrap();
    }

    println!("Total entries: {}", cache.len().await);
}
```

### Web Application Example

```rust
use minicache::MiniCache;
use std::sync::Arc;
use std::time::Duration;

// Shared cache instance
type SharedCache = Arc<MiniCache<String, String>>;

async fn get_user_profile(cache: SharedCache, user_id: &str) -> Option<String> {
    let cache_key = format!("user_profile:{}", user_id);
    
    // Try cache first
    if let Some(profile) = cache.get(&cache_key).await {
        return Some(profile);
    }
    
    // Simulate database lookup
    let profile = fetch_from_database(user_id).await;
    
    // Cache for 5 minutes
    cache.set(cache_key, profile.clone(), Some(Duration::from_secs(300))).await;
    
    Some(profile)
}

async fn fetch_from_database(user_id: &str) -> String {
    // Simulate slow database query
    tokio::time::sleep(Duration::from_millis(100)).await;
    format!("Profile data for user {}", user_id)
}

#[tokio::main]
async fn main() {
    let cache = Arc::new(MiniCache::new(Duration::from_secs(60)));
    
    // Multiple requests for same user - only first hits database
    for _ in 0..5 {
        let profile = get_user_profile(cache.clone(), "123").await;
        println!("Got profile: {:?}", profile);
    }
}
```

## 🔧 API Reference

### Core Methods

| Method | Description |
|--------|-------------|
| `new(cleanup_interval)` | Create new cache with cleanup interval |
| `set(key, value, ttl)` | Store key-value pair with optional TTL |
| `get(key)` | Retrieve value by key |
| `remove(key)` | Delete specific key |
| `contains(key)` | Check if key exists (and not expired) |
| `clear()` | Remove all entries |
| `len()` | Get number of valid entries |
| `keys()` | Get all valid keys |

### Generic Types

```rust
MiniCache<K, V>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
```

## ⚡ Performance

Based on benchmarks (MacBook Pro M1):

- **Basic Reads**: ~13.7M operations/second
- **Basic Writes**: ~9.6M operations/second  
- **Concurrent Access**: ~1.7M operations/second
- **Memory Overhead**: ~162 bytes per entry
- **TTL Cleanup**: Sub-millisecond automatic cleanup

Run benchmarks yourself:

```bash
cargo run --release --example quick_demo
```

## 🏆 Comparison

| Feature | MiniCache | HashMap | DashMap | moka |
|---------|-----------|---------|---------|------|
| Async/Await | ✅ | ❌ | ❌ | ✅ |
| TTL Support | ✅ | ❌ | ❌ | ✅ |
| Auto Cleanup | ✅ | ❌ | ❌ | ✅ |
| Zero Dependencies* | ✅ | ✅ | ❌ | ❌ |
| Memory Efficient | ✅ | ✅ | ❌ | ❌ |

*Except tokio for async runtime

## 🛠 Advanced Usage

### Custom Cleanup Intervals

```rust
// Fast cleanup for short-lived data
let fast_cache = MiniCache::new(Duration::from_millis(100));

// Slow cleanup for long-lived data
let slow_cache = MiniCache::new(Duration::from_secs(300));
```

### Error Handling

```rust
// MiniCache operations don't return Results - they're designed to never fail
// However, you might want to handle potential issues:

#[tokio::main]
async fn main() {
    let cache = MiniCache::new(Duration::from_secs(60));
    
    // These operations are guaranteed to succeed
    cache.set("key", "value", None).await;
    let value = cache.get(&"key").await; // Returns Option<V>
    
    // Handle missing values
    match cache.get(&"missing").await {
        Some(val) => println!("Found: {}", val),
        None => println!("Key not found or expired"),
    }
}
```

## 🔍 Monitoring and Debugging

```rust
use minicache::MiniCache;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let cache = MiniCache::new(Duration::from_secs(60));
    
    // Add some data
    cache.set("key1", "value1", Some(Duration::from_secs(10))).await;
    cache.set("key2", "value2", None).await;
    
    // Monitor cache state
    println!("Cache size: {}", cache.len().await);
    println!("All keys: {:?}", cache.keys().await);
    
    // Check specific keys
    for key in ["key1", "key2", "key3"] {
        if cache.contains(&key).await {
            println!("{}: exists", key);
        } else {
            println!("{}: missing or expired", key);
        }
    }
}
```

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

Run with output:

```bash
cargo test -- --nocapture
```

Test specific modules:

```bash
cargo test cache_operations
```

## 📊 Benchmarking

Quick performance demo:

```bash
cargo run --release --example quick_demo
```

Detailed benchmarks:

```bash
cargo bench
```

Memory profiling:

```bash
cargo run --release --example memory_profiler
```

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Run tests (`cargo test`)
4. Commit changes (`git commit -am 'Add amazing feature'`)
5. Push to branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [Documentation](https://docs.rs/minicache)
- [Crates.io](https://crates.io/crates/minicache)
- [Repository](https://github.com/yourusername/minicache)
- [Issues](https://github.com/yourusername/minicache/issues)

## 📈 Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history and breaking changes.

---

**Made with ❤️ for the Rust community**