//! N-API bindings for minicache - JavaScript/Node.js interop layer
//!
//! This module provides JavaScript-compatible interfaces for the MiniCache functionality,
//! allowing Node.js applications to use the high-performance Rust cache directly.

use crate::core::MiniCache;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;
use std::time::Duration;

/// JavaScript-compatible cache class that wraps the Rust MiniCache
///
/// This provides a bridge between JavaScript and the high-performance Rust cache,
/// maintaining async compatibility and providing a familiar JavaScript API.
#[napi]
pub struct JsCache {
    /// Internal cache instance shared across all operations
    cache: MiniCache<String, String>,
}
/// Options for creating a new cache instance
#[napi(object)]
pub struct CacheOptions {
    /// Cleanup interval in milliseconds (default: 60000ms = 1 minute)
    pub cleanup_interval_ms: Option<u32>,
}

#[napi]
impl JsCache {
    /// Creates a new cache instance with the specified cleanup interval
    ///
    /// # Arguments
    /// * `options` - Configuration options for the cache
    ///
    /// # Examples
    /// ```javascript
    /// const { JsCache } = require('minicache');
    ///
    /// // Create with default 60-second cleanup
    /// const cache = new JsCache();
    ///
    /// // Create with custom 30-second cleanup
    /// const fastCache = new JsCache({ cleanupIntervalMs: 30000 });
    /// ```
    #[napi(constructor)]
    pub fn new(options: Option<CacheOptions>) -> Result<Self> {
        let cleanup_interval = options
            .and_then(|opts| opts.cleanup_interval_ms)
            .unwrap_or(60000); // Default 60 seconds

        let cache = MiniCache::new(Duration::from_millis(cleanup_interval as u64));

        Ok(JsCache {
            cache: MiniCache::new(Duration::from_millis(cleanup_interval as u64)),
        })
    }

    /// Sets a key-value pair in the cache with an optional TTL
    ///
    /// # Arguments
    /// * `key` - The cache key as a string
    /// * `value` - The value to store as a string  
    /// * `ttl_ms` - Optional TTL in milliseconds
    ///
    /// # Examples
    /// ```javascript
    /// // Set without expiration
    /// await cache.set('user:123', 'John Doe');
    ///
    /// // Set with 5-minute expiration
    /// await cache.set('session:abc', 'temp_data', 300000);
    /// ```
    #[napi]
    pub async fn set(&self, key: String, value: String, ttl_ms: Option<u32>) -> Result<()> {
        let ttl = ttl_ms.map(|ms| Duration::from_millis(ms as u64));
        self.cache.set(key, value, ttl).await;
        Ok(())
    }

    /// Retrieves a value from the cache by key
    ///
    /// # Arguments
    /// * `key` - The cache key to look up
    ///
    /// # Returns
    /// The cached value if it exists and hasn't expired, otherwise `null`
    ///
    /// # Examples
    /// ```javascript
    /// const value = await cache.get('user:123');
    /// if (value !== null) {
    ///     console.log('Found user:', value);
    /// }
    /// ```
    #[napi]
    pub async fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.cache.get(&key).await)
    }

    /// Removes a key from the cache manually
    ///
    /// # Arguments
    /// * `key` - The cache key to remove
    ///
    /// # Examples
    /// ```javascript
    /// await cache.remove('user:123');
    /// ```
    #[napi]
    pub async fn remove(&self, key: String) -> Result<()> {
        self.cache.remove(&key).await;
        Ok(())
    }

    /// Removes all entries from the cache
    ///
    /// # Examples
    /// ```javascript
    /// await cache.clear();
    /// console.log('Cache cleared');
    /// ```
    #[napi]
    pub async fn clear(&self) -> Result<()> {
        self.cache.clear().await;
        Ok(())
    }

    /// Checks if a key exists in the cache and has not expired
    ///
    /// # Arguments
    /// * `key` - The cache key to check
    ///
    /// # Returns
    /// `true` if the key exists and is not expired, `false` otherwise
    ///
    /// # Examples
    /// ```javascript
    /// if (await cache.has('user:123')) {
    ///     console.log('User is cached');
    /// }
    /// ```
    #[napi]
    pub async fn has(&self, key: String) -> Result<bool> {
        Ok(self.cache.contains(&key).await)
    }

    /// Returns the number of valid (non-expired) entries in the cache
    ///
    /// # Returns
    /// The number of valid entries currently in the cache
    ///
    /// # Examples
    /// ```javascript
    /// const size = await cache.size();
    /// console.log(`Cache has ${size} entries`);
    /// ```
    #[napi]
    pub async fn size(&self) -> Result<u32> {
        Ok(self.cache.len().await as u32)
    }

    /// Returns `true` if the cache contains no valid (non-expired) entries
    ///
    /// # Returns
    /// `true` if the cache is empty, `false` otherwise
    ///
    /// # Examples
    /// ```javascript
    /// if (await cache.isEmpty()) {
    ///     console.log('Cache is empty');
    /// }
    /// ```
    #[napi]
    pub async fn is_empty(&self) -> Result<bool> {
        Ok(self.cache.is_empty().await)
    }

    /// Returns an array of all valid (non-expired) keys in the cache
    ///
    /// # Returns
    /// An array containing all valid keys currently in the cache
    ///
    /// # Examples
    /// ```javascript
    /// const keys = await cache.keys();
    /// console.log('Cache keys:', keys);
    /// ```
    #[napi]
    pub async fn keys(&self) -> Result<Vec<String>> {
        Ok(self.cache.keys().await)
    }

    /// Returns key-value pairs for all valid (non-expired) entries
    ///
    /// # Returns
    /// An array of objects containing key-value pairs
    ///
    /// # Examples
    /// ```javascript
    /// const entries = await cache.entries();
    /// entries.forEach(({ key, value }) => {
    ///     console.log(`${key}: ${value}`);
    /// });
    /// ```
    #[napi]
    pub async fn entries(&self) -> Result<Vec<CacheEntry>> {
        let keys = self.cache.keys().await;
        let mut entries = Vec::new();

        for key in keys {
            if let Some(value) = self.cache.get(&key).await {
                entries.push(CacheEntry { key, value });
            }
        }

        Ok(entries)
    }

    /// Batch operation: sets multiple key-value pairs at once
    ///
    /// # Arguments
    /// * `items` - Array of items to set in the cache
    ///
    /// # Examples
    /// ```javascript
    /// await cache.setMany([
    ///     { key: 'user:1', value: 'Alice', ttlMs: 300000 },
    ///     { key: 'user:2', value: 'Bob' },
    /// ]);
    /// ```
    #[napi]
    pub async fn set_many(&self, items: Vec<SetItem>) -> Result<()> {
        for item in items {
            let ttl = item.ttl_ms.map(|ms| Duration::from_millis(ms as u64));
            self.cache.set(item.key, item.value, ttl).await;
        }
        Ok(())
    }

    /// Batch operation: gets multiple values by their keys
    ///
    /// # Arguments
    /// * `keys` - Array of keys to retrieve
    ///
    /// # Returns
    /// Array of key-value pairs for found entries
    ///
    /// # Examples
    /// ```javascript
    /// const results = await cache.getMany(['user:1', 'user:2', 'user:3']);
    /// results.forEach(({ key, value }) => {
    ///     console.log(`Found ${key}: ${value}`);
    /// });
    /// ```
    #[napi]
    pub async fn get_many(&self, keys: Vec<String>) -> Result<Vec<CacheEntry>> {
        let mut results = Vec::new();

        for key in keys {
            if let Some(value) = self.cache.get(&key).await {
                results.push(CacheEntry { key, value });
            }
        }

        Ok(results)
    }
}

/// Represents a cache entry with key and value
#[napi(object)]
pub struct CacheEntry {
    pub key: String,
    pub value: String,
}

/// Represents an item to be set in the cache
#[napi(object)]
pub struct SetItem {
    pub key: String,
    pub value: String,
    pub ttl_ms: Option<u32>,
}

/// Creates a new cache instance (alternative constructor function)
///
/// This provides a functional interface as an alternative to the class constructor.
///
/// # Arguments
/// * `options` - Configuration options for the cache
///
/// # Examples
/// ```javascript
/// const { createCache } = require('minicache');
///
/// const cache = await createCache({ cleanupIntervalMs: 30000 });
/// ```
#[napi]
pub async fn create_cache(options: Option<CacheOptions>) -> Result<JsCache> {
    JsCache::new(options)
}

/// Utility function to check if the native addon is working
///
/// # Returns
/// Version string and performance information
///
/// # Examples
/// ```javascript
/// const { getInfo } = require('minicache');
///
/// console.log(getInfo());
/// // Output: { version: "0.1.1", backend: "rust", performance: "native" }
/// ```
#[napi]
pub fn get_info() -> Result<CacheInfo> {
    Ok(CacheInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        backend: "rust".to_string(),
        performance: "native".to_string(),
        features: vec![
            "ttl".to_string(),
            "async".to_string(),
            "concurrent".to_string(),
            "auto-cleanup".to_string(),
        ],
    })
}

/// Information about the cache implementation
#[napi(object)]
pub struct CacheInfo {
    pub version: String,
    pub backend: String,
    pub performance: String,
    pub features: Vec<String>,
}

/// Convenience function for creating a cache with default settings
///
/// # Examples
/// ```javascript
/// const { defaultCache } = require('minicache');
///
/// const cache = await defaultCache();
/// await cache.set('key', 'value');
/// ```
#[napi]
pub async fn default_cache() -> Result<JsCache> {
    JsCache::new(None)
}
