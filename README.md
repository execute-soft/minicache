# 🚀 MiniCaches - High-Performance Node.js Cache

[![npm version](https://badge.fury.io/js/minicaches.svg)](https://badge.fury.io/js/minicaches)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A **blazing-fast**, lightweight, async-compatible in-memory cache for Node.js with TTL support and automatic cleanup. **Powered by Rust** for maximum performance.

## ✨ Features

- **🔥 Native Performance**: Rust-powered for maximum speed
- **⚡ Async/Await Ready**: Built for modern Node.js applications  
- **⏰ TTL Support**: Automatic expiration with background cleanup
- **🔒 Thread-Safe**: Concurrent access support
- **💾 Memory Efficient**: Minimal overhead per cache entry
- **🛠 Easy to Use**: Simple, intuitive JavaScript API
- **📊 High Throughput**: Millions of operations per second
- **🌍 Cross-Platform**: Works on Windows, macOS, and Linux

## 📦 Installation

```bash
npm install minicaches
# or
yarn add minicaches
```

## 🚀 Quick Start

### JavaScript (CommonJS)

```javascript
const { JsCache } = require('minicaches');

async function example() {
  // Create cache
  const cache = new JsCache();
  
  // Set values with optional TTL
  await cache.set('user:123', 'John Doe');
  await cache.set('session:abc', 'temp_data', 300000); // 5 minutes TTL
  
  // Get values
  const user = await cache.get('user:123');
  console.log(user); // 'John Doe'
  
  // Check if key exists
  const hasSession = await cache.has('session:abc');
  console.log(hasSession); // true
  
  // Get cache size
  const size = await cache.size();
  console.log(`Cache has ${size} entries`);
}

example();
```

### TypeScript

```typescript
import { JsCache } from 'minicaches';

const cache = new JsCache();

// Type-safe operations
await cache.set('user:1', 'Alice', 300000);
const user: string | null = await cache.get('user:1');

if (user) {
  console.log(`Found user: ${user}`);
}
```

## 📖 API Reference

### Constructor

```typescript
const cache = new JsCache();
```

### Methods

#### `set(key: string, value: string, ttlMs?: number): Promise<void>`
Set a key-value pair with optional TTL in milliseconds.

```javascript
await cache.set('key', 'value');           // No expiration
await cache.set('key', 'value', 60000);    // Expires in 1 minute
```

#### `get(key: string): Promise<string | null>`
Get a value by key. Returns `null` if not found or expired.

```javascript
const value = await cache.get('key');
```

#### `remove(key: string): Promise<void>`
Remove a key from the cache.

```javascript
await cache.remove('key');
```

#### `clear(): Promise<void>`
Remove all entries from the cache.

```javascript
await cache.clear();
```

#### `size(): Promise<number>`
Get the number of entries in the cache.

```javascript
const count = await cache.size();
```

#### `getInfo(): string`
Get information about the cache implementation.

```javascript
const { getInfo } = require('minicaches');
console.log(getInfo()); // Version and backend info
```

## 🚀 Performance

MiniCaches delivers exceptional performance thanks to its Rust backend:

- **Read Operations**: ~13.7M operations/second
- **Write Operations**: ~9.6M operations/second  
- **Concurrent Access**: ~1.7M operations/second
- **Memory Overhead**: ~162 bytes per entry

## 🛠 Use Cases

Perfect for:
- **API Response Caching**: Cache expensive API calls
- **Session Storage**: Fast session data retrieval
- **Computed Results**: Cache heavy computations
- **Rate Limiting**: Track request counts with TTL
- **Configuration Cache**: Store frequently accessed config
- **Temporary Data**: Auto-expiring temporary storage

## 🌍 Platform Support

Pre-built binaries available for:
- **macOS**: Intel (x64) and Apple Silicon (ARM64)
- **Windows**: x64, x86, ARM64
- **Linux**: x64, ARM64, ARM (glibc and musl)
- **FreeBSD**: x64

## 🔧 Examples

### Session Management

```javascript
const { JsCache } = require('minicaches');
const cache = new JsCache();

// Store session with 30-minute expiration
await cache.set('session:' + userId, sessionData, 30 * 60 * 1000);

// Check if session is valid
const session = await cache.get('session:' + userId);
if (session) {
  // Session is valid
  console.log('User is authenticated');
} else {
  // Session expired or doesn't exist
  console.log('Please log in');
}
```

### API Response Caching

```javascript
async function getCachedApiData(endpoint) {
  const cacheKey = `api:${endpoint}`;
  
  // Try cache first
  let data = await cache.get(cacheKey);
  if (data) {
    return JSON.parse(data);
  }
  
  // Fetch from API
  const response = await fetch(endpoint);
  data = await response.json();
  
  // Cache for 5 minutes
  await cache.set(cacheKey, JSON.stringify(data), 5 * 60 * 1000);
  
  return data;
}
```

### Rate Limiting

```javascript
async function checkRateLimit(userId, maxRequests = 100) {
  const key = `rate_limit:${userId}`;
  const current = await cache.get(key);
  
  if (current === null) {
    // First request in this window
    await cache.set(key, '1', 60 * 1000); // 1 minute window
    return true;
  }
  
  const count = parseInt(current);
  if (count >= maxRequests) {
    return false; // Rate limit exceeded
  }
  
  // Increment counter
  await cache.set(key, (count + 1).toString(), 60 * 1000);
  return true;
}
```

## 🤝 Contributing

Contributions welcome! Please check the [GitHub repository](https://github.com/execute-soft/minicache).

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🔗 Links

- [GitHub Repository](https://github.com/execute-soft/minicache)
- [npm Package](https://www.npmjs.com/package/minicaches)
- [Documentation](https://github.com/execute-soft/minicache#readme)

---

**Built with ❤️ using Rust and N-API**