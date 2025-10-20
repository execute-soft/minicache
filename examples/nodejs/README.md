# Node.js Integration for MiniCache

This directory contains Node.js examples demonstrating how to use MiniCache's N-API bindings for maximum performance in JavaScript/TypeScript applications.

## Installation

```bash
# Install dependencies (if running examples)
npm install

# Build the native addon
npm run build

# Or build in debug mode
npm run build:debug
```

## Quick Start

### JavaScript (ESM)
```javascript
import { JsCache } from 'minicache';

const cache = new JsCache({ cleanupIntervalMs: 60000 });

// Set values
await cache.set('user:123', 'John Doe');
await cache.set('session:abc', 'temp_data', 300000); // 5 minutes TTL

// Get values
const user = await cache.get('user:123');
console.log(user); // 'John Doe'

// Check existence
if (await cache.has('session:abc')) {
  console.log('Session is active');
}
```

### JavaScript (CommonJS)
```javascript
const { JsCache } = require('minicache');

async function example() {
  const cache = new JsCache();
  await cache.set('key', 'value');
  const result = await cache.get('key');
  console.log(result);
}

example();
```

### TypeScript
```typescript
import { JsCache, CacheOptions } from 'minicache';

const options: CacheOptions = { cleanupIntervalMs: 30000 };
const cache = new JsCache(options);

await cache.set('typed:key', 'typed:value', 60000);
const value: string | null = await cache.get('typed:key');
```

## Examples

1. **`basic.js`** - Fundamental cache operations and TTL behavior
2. **`advanced.js`** - Batch operations, performance testing, and concurrent access
3. **`typescript-example.ts`** - TypeScript integration with proper typing

### Running Examples

```bash
# Basic example
node examples/nodejs/basic.js

# Advanced example  
node examples/nodejs/advanced.js

# TypeScript example (requires ts-node)
npx ts-node examples/nodejs/typescript-example.ts
```

## API Reference

### Class: JsCache

#### Constructor
- `new JsCache(options?)` - Create a new cache instance
  - `options.cleanupIntervalMs` - Cleanup interval in milliseconds (default: 60000)

#### Methods
- `set(key, value, ttlMs?)` - Set a key-value pair with optional TTL
- `get(key)` - Get a value by key (returns `null` if not found/expired)
- `remove(key)` - Remove a key from the cache
- `clear()` - Remove all entries from the cache
- `has(key)` - Check if a key exists and is not expired
- `size()` - Get the number of valid entries
- `isEmpty()` - Check if the cache is empty
- `keys()` - Get all valid keys
- `entries()` - Get all key-value pairs
- `setMany(items)` - Batch set operation
- `getMany(keys)` - Batch get operation

#### Utility Functions
- `createCache(options?)` - Alternative constructor function
- `defaultCache()` - Create cache with default settings
- `getInfo()` - Get version and performance information

## Performance

The N-API bindings provide near-native performance:

- **Read Operations**: ~13.7M ops/second
- **Write Operations**: ~9.6M ops/second  
- **Concurrent Access**: ~1.7M ops/second
- **Memory Overhead**: ~162 bytes per entry

## Platform Support

Pre-built binaries are available for:
- macOS (x64, ARM64)
- Windows (x64, x86, ARM64)
- Linux (x64, ARM64, ARM, musl variants)
- FreeBSD (x64)
- Android (ARM64, ARM)

## Building from Source

```bash
# Install Rust and Node.js dependencies
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
npm install

# Build the native addon
npm run build

# Run tests
npm test
```

## Error Handling

```javascript
try {
  const cache = new JsCache();
  await cache.set('key', 'value');
  const result = await cache.get('key');
} catch (error) {
  console.error('Cache operation failed:', error);
}
```

## Best Practices

1. **Cleanup Interval**: Choose based on your TTL patterns
   - Short TTLs (seconds): 1-5 second cleanup
   - Medium TTLs (minutes): 10-60 second cleanup  
   - Long TTLs (hours): 1-5 minute cleanup

2. **Batch Operations**: Use `setMany`/`getMany` for better performance

3. **Memory Management**: Monitor cache size and clear periodically if needed

4. **Error Handling**: Always handle potential errors in async operations

5. **TypeScript**: Use provided type definitions for better development experience