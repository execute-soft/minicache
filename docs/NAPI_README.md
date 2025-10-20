# MiniCache N-API Integration

This document explains how to use MiniCache with Node.js through N-API bindings for maximum performance.

## 🚀 Quick Start

### Installation

```bash
npm install minicache
```

### Basic Usage

```javascript
const { JsCache } = require('minicache');

async function example() {
  // Create cache with 60-second cleanup interval
  const cache = new JsCache({ cleanupIntervalMs: 60000 });
  
  // Set values with and without TTL
  await cache.set('user:123', 'John Doe');
  await cache.set('session:abc', 'temp_data', 300000); // 5 minutes TTL
  
  // Get values
  const user = await cache.get('user:123');
  console.log(user); // 'John Doe'
  
  // Check existence
  if (await cache.has('session:abc')) {
    console.log('Session is active');
  }
  
  // Get cache stats
  console.log(`Cache size: ${await cache.size()} entries`);
}

example();
```

## 🏗️ Building from Source

If you need to build the native addon from source:

### Prerequisites

1. **Rust** (latest stable)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **Node.js** (v16 or higher)

3. **Build tools** (platform-specific)
   - **Windows**: Visual Studio Build Tools or Visual Studio Community
   - **macOS**: Xcode Command Line Tools (`xcode-select --install`)
   - **Linux**: `build-essential` package

### Build Steps

```bash
# Clone the repository
git clone https://github.com/execute-soft/minicache
cd minicache

# Install Node.js dependencies
npm install

# Build the native addon (release mode)
npm run build

# Or build in debug mode for development
npm run build:debug

# Run tests
npm test
```

### Build Configuration

The build is configured in `package.json` under the `napi` section:

```json
{
  "napi": {
    "binaryName": "minicache",
    "cargoName": "minicache", 
    "cargoFlags": "--features napi",
    "targets": [
      "x86_64-apple-darwin",
      "aarch64-apple-darwin",
      "x86_64-pc-windows-msvc",
      "x86_64-unknown-linux-gnu",
      "aarch64-unknown-linux-gnu"
    ]
  }
}
```

## 📊 Performance

The N-API bindings provide near-native performance:

| Operation | Performance | Description |
|-----------|-------------|-------------|
| **Read** | ~13.7M ops/sec | Basic get operations |
| **Write** | ~9.6M ops/sec | Basic set operations |
| **Concurrent** | ~1.7M ops/sec | Multi-threaded access |
| **Memory** | ~162 bytes/entry | Per-entry overhead |

## 🔧 API Reference

### Constructor Options

```typescript
interface CacheOptions {
  cleanupIntervalMs?: number; // Default: 60000 (1 minute)
}
```

### Core Methods

```typescript
class JsCache {
  // Basic operations
  set(key: string, value: string, ttlMs?: number): Promise<void>
  get(key: string): Promise<string | null>
  remove(key: string): Promise<void>
  clear(): Promise<void>
  
  // Existence and size
  has(key: string): Promise<boolean>
  size(): Promise<number>
  isEmpty(): Promise<boolean>
  
  // Bulk operations
  keys(): Promise<string[]>
  entries(): Promise<CacheEntry[]>
  setMany(items: SetItem[]): Promise<void>
  getMany(keys: string[]): Promise<CacheEntry[]>
}
```

### Utility Functions

```typescript
// Alternative constructors
function createCache(options?: CacheOptions): Promise<JsCache>
function defaultCache(): Promise<JsCache>

// Information
function getInfo(): CacheInfo
```

## 🎯 TypeScript Support

Full TypeScript support is included:

```typescript
import { JsCache, CacheOptions, CacheEntry, SetItem } from 'minicache';

const options: CacheOptions = { cleanupIntervalMs: 30000 };
const cache = new JsCache(options);

// Type-safe operations
await cache.set('user:1', 'Alice', 300000);
const user: string | null = await cache.get('user:1');

// Batch operations
const items: SetItem[] = [
  { key: 'config:theme', value: 'dark' },
  { key: 'config:lang', value: 'en', ttlMs: 60000 }
];

await cache.setMany(items);
const entries: CacheEntry[] = await cache.entries();
```

## 🌍 Platform Support

Pre-built binaries are available for:

### Desktop Platforms
- **macOS**: Intel (x64) and Apple Silicon (ARM64)
- **Windows**: x64, x86, ARM64
- **Linux**: x64, ARM64, ARM (glibc and musl)
- **FreeBSD**: x64

### Mobile/Embedded
- **Android**: ARM64, ARM
- **Linux ARM**: Raspberry Pi and similar devices

## 🛠️ Development

### Project Structure

```
minicache/
├── src/
│   ├── lib.rs          # Main Rust library
│   ├── core.rs         # Core cache implementation
│   └── napi.rs         # N-API bindings
├── examples/nodejs/    # Node.js examples
├── package.json        # Node.js configuration
├── Cargo.toml          # Rust configuration
├── build.rs            # Build script
└── index.d.ts          # TypeScript definitions
```

### Feature Flags

The N-API bindings are gated behind the `napi` feature:

```toml
[features]
default = []
napi = ["dep:napi", "dep:napi-derive"]
```

To build only the Rust library without N-API:
```bash
cargo build --release
```

To build with N-API bindings:
```bash
cargo build --release --features napi
```

### Testing

```bash
# Run Rust tests
cargo test

# Run Node.js tests
npm test

# Run benchmarks
cargo bench
npm run bench
```

### Cross-compilation

For cross-compilation, use the `napi` CLI:

```bash
# Build for specific target
npm run build -- --target x86_64-unknown-linux-gnu

# Build for all targets
npm run artifacts
```

## 🔍 Troubleshooting

### Common Issues

1. **Build failures on Windows**
   - Ensure Visual Studio Build Tools are installed
   - Use Node.js built with the same MSVC version

2. **Missing symbols on Linux**
   - Install `build-essential` package
   - Ensure glibc version compatibility

3. **Performance issues**
   - Adjust cleanup interval based on TTL patterns
   - Use batch operations for multiple items
   - Monitor memory usage

### Debug Mode

Build in debug mode for additional logging:

```bash
npm run build:debug
```

### Environment Variables

- `RUST_LOG=debug` - Enable Rust debug logging
- `NODE_ENV=development` - Enable development mode

## 📖 Examples

See the `examples/nodejs/` directory for comprehensive examples:

- **basic.js** - Fundamental operations
- **advanced.js** - Performance testing and batch operations  
- **typescript-example.ts** - TypeScript integration

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Run `npm run format` and `npm run lint`
5. Submit a pull request

## 📄 License

MIT License - see LICENSE file for details.