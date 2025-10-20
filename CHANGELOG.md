# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-10-20

### Added
- Initial release of MiniCache
- Async-compatible in-memory cache with TTL support
- Thread-safe operations using `Arc<RwLock<HashMap>>`
- Automatic background cleanup of expired entries
- Generic support for any key-value types that implement required traits
- Core operations:
  - `new()` - Create cache with configurable cleanup interval
  - `set()` - Store key-value pairs with optional TTL
  - `get()` - Retrieve values by key
  - `remove()` - Delete specific keys
  - `contains()` - Check key existence (excluding expired)
  - `clear()` - Remove all entries
  - `len()` - Get count of valid entries
  - `keys()` - Get list of all valid keys
- Comprehensive test suite with 100% coverage
- Performance benchmarks and examples
- Memory usage profiling tools
- Documentation and usage examples

### Performance
- ~13.7M read operations per second
- ~9.6M write operations per second
- ~1.7M concurrent operations per second
- ~162 bytes memory overhead per entry
- Sub-millisecond automatic cleanup

### Dependencies
- `tokio` 1.48.0 with full features for async runtime
- `criterion` 0.7.0 for benchmarking (dev dependency)
- `sysinfo` 0.37.2 for memory profiling (dev dependency)

[unreleased]: https://github.com/execute-soft/minicache/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/execute-soft/minicache/releases/tag/v0.1.0