/**
 * MiniCache - A fast, lightweight, async-compatible in-memory cache with TTL support
 *
 * This is a high-performance Rust-based cache with JavaScript bindings.
 * Perfect for Node.js applications that need efficient caching with automatic expiration.
 */

export interface CacheOptions {
  /**
   * Cleanup interval in milliseconds (default: 60000ms = 1 minute)
   * Determines how often expired entries are automatically removed
   */
  cleanupIntervalMs?: number
}

export interface CacheEntry {
  /** Cache key */
  key: string
  /** Cache value */
  value: string
}

export interface SetItem {
  /** Cache key */
  key: string
  /** Cache value */
  value: string
  /** Optional TTL in milliseconds */
  ttlMs?: number
}

export interface CacheInfo {
  /** Version of the minicache library */
  version: string
  /** Backend implementation (always "rust") */
  backend: string
  /** Performance profile (always "native") */
  performance: string
  /** Available features */
  features: string[]
}

/**
 * High-performance cache implementation with TTL support and automatic cleanup
 *
 * @example
 * ```typescript
 * import { JsCache } from 'minicache';
 *
 * // Create cache with 60-second cleanup interval
 * const cache = new JsCache();
 *
 * // Set values with and without TTL
 * await cache.set('user:123', 'John Doe');
 * await cache.set('session:abc', 'temp_data', 300000); // 5 minutes
 *
 * // Get values
 * const user = await cache.get('user:123');
 * console.log(user); // 'John Doe'
 *
 * // Check existence
 * if (await cache.has('session:abc')) {
 *   console.log('Session is active');
 * }
 * ```
 */
export declare class JsCache {
  /**
   * Creates a new cache instance
   * @param options Configuration options for the cache
   */
  constructor(options?: CacheOptions)

  /**
   * Sets a key-value pair in the cache with an optional TTL
   * @param key The cache key as a string
   * @param value The value to store as a string
   * @param ttlMs Optional TTL in milliseconds
   * @example
   * ```typescript
   * // Set without expiration
   * await cache.set('user:123', 'John Doe');
   *
   * // Set with 5-minute expiration
   * await cache.set('session:abc', 'temp_data', 300000);
   * ```
   */
  set(key: string, value: string, ttlMs?: number): Promise<void>

  /**
   * Retrieves a value from the cache by key
   * @param key The cache key to look up
   * @returns The cached value if it exists and hasn't expired, otherwise null
   * @example
   * ```typescript
   * const value = await cache.get('user:123');
   * if (value !== null) {
   *   console.log('Found user:', value);
   * }
   * ```
   */
  get(key: string): Promise<string | null>

  /**
   * Removes a key from the cache manually
   * @param key The cache key to remove
   * @example
   * ```typescript
   * await cache.remove('user:123');
   * ```
   */
  remove(key: string): Promise<void>

  /**
   * Removes all entries from the cache
   * @example
   * ```typescript
   * await cache.clear();
   * console.log('Cache cleared');
   * ```
   */
  clear(): Promise<void>

  /**
   * Checks if a key exists in the cache and has not expired
   * @param key The cache key to check
   * @returns true if the key exists and is not expired, false otherwise
   * @example
   * ```typescript
   * if (await cache.has('user:123')) {
   *   console.log('User is cached');
   * }
   * ```
   */
  has(key: string): Promise<boolean>

  /**
   * Returns the number of valid (non-expired) entries in the cache
   * @returns The number of valid entries currently in the cache
   * @example
   * ```typescript
   * const size = await cache.size();
   * console.log(`Cache has ${size} entries`);
   * ```
   */
  size(): Promise<number>

  /**
   * Returns true if the cache contains no valid (non-expired) entries
   * @returns true if the cache is empty, false otherwise
   * @example
   * ```typescript
   * if (await cache.isEmpty()) {
   *   console.log('Cache is empty');
   * }
   * ```
   */
  isEmpty(): Promise<boolean>

  /**
   * Returns an array of all valid (non-expired) keys in the cache
   * @returns An array containing all valid keys currently in the cache
   * @example
   * ```typescript
   * const keys = await cache.keys();
   * console.log('Cache keys:', keys);
   * ```
   */
  keys(): Promise<string[]>

  /**
   * Returns key-value pairs for all valid (non-expired) entries
   * @returns An array of objects containing key-value pairs
   * @example
   * ```typescript
   * const entries = await cache.entries();
   * entries.forEach(({ key, value }) => {
   *   console.log(`${key}: ${value}`);
   * });
   * ```
   */
  entries(): Promise<CacheEntry[]>

  /**
   * Batch operation: sets multiple key-value pairs at once
   * @param items Array of items to set in the cache
   * @example
   * ```typescript
   * await cache.setMany([
   *   { key: 'user:1', value: 'Alice', ttlMs: 300000 },
   *   { key: 'user:2', value: 'Bob' },
   * ]);
   * ```
   */
  setMany(items: SetItem[]): Promise<void>

  /**
   * Batch operation: gets multiple values by their keys
   * @param keys Array of keys to retrieve
   * @returns Array of key-value pairs for found entries
   * @example
   * ```typescript
   * const results = await cache.getMany(['user:1', 'user:2', 'user:3']);
   * results.forEach(({ key, value }) => {
   *   console.log(`Found ${key}: ${value}`);
   * });
   * ```
   */
  getMany(keys: string[]): Promise<CacheEntry[]>
}

/**
 * Creates a new cache instance (alternative constructor function)
 * @param options Configuration options for the cache
 * @returns A new JsCache instance
 * @example
 * ```typescript
 * import { createCache } from 'minicache';
 *
 * const cache = await createCache({ cleanupIntervalMs: 30000 });
 * ```
 */
export declare function createCache(options?: CacheOptions): Promise<JsCache>

/**
 * Utility function to check if the native addon is working
 * @returns Version string and performance information
 * @example
 * ```typescript
 * import { getInfo } from 'minicache';
 *
 * console.log(getInfo());
 * // Output: { version: "0.1.1", backend: "rust", performance: "native" }
 * ```
 */
export declare function getInfo(): CacheInfo

/**
 * Convenience function for creating a cache with default settings
 * @returns A new JsCache instance with default configuration
 * @example
 * ```typescript
 * import { defaultCache } from 'minicache';
 *
 * const cache = await defaultCache();
 * await cache.set('key', 'value');
 * ```
 */
export declare function defaultCache(): Promise<JsCache>

// Export the main class as default for convenience
export default JsCache
