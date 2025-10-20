//! Core implementation of MiniCache - an async-compatible in-memory cache with TTL support.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::interval;

/// A fast, thread-safe, async-compatible in-memory cache with TTL support and automatic cleanup.
///
/// `MiniCache` provides a simple key-value store that can automatically expire entries after
/// a specified time-to-live (TTL). It uses `Arc<RwLock<HashMap>>` internally for thread-safe
/// concurrent access and spawns a background task for cleaning up expired entries.
///
/// # Type Parameters
///
/// * `K` - Key type. Must implement `Hash + Eq + Clone + Send + Sync + 'static`
/// * `V` - Value type. Must implement `Clone + Send + Sync + 'static`
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use minicache::MiniCache;
/// use std::time::Duration;
///
/// #[tokio::main]
/// async fn main() {
///     let cache = MiniCache::new(Duration::from_secs(60));
///     
///     // Store a value
///     cache.set("key1", "value1", None).await;
///     
///     // Retrieve a value
///     if let Some(value) = cache.get(&"key1").await {
///         println!("Found: {}", value);
///     }
/// }
/// ```
///
/// ## With TTL
///
/// ```rust
/// use minicache::MiniCache;
/// use std::time::Duration;
/// use tokio::time::sleep;
///
/// #[tokio::main]
/// async fn main() {
///     let cache = MiniCache::new(Duration::from_secs(1));
///     
///     // Store with 100ms TTL
///     cache.set("temp", "expires", Some(Duration::from_millis(100))).await;
///     
///     // Value exists
///     assert_eq!(cache.get(&"temp").await, Some("expires"));
///     
///     // Wait for expiration
///     sleep(Duration::from_millis(150)).await;
///     
///     // Value has expired
///     assert_eq!(cache.get(&"temp").await, None);
/// }
/// ```
#[derive(Clone)]
pub struct MiniCache<K, V> {
    inner: Arc<RwLock<HashMap<K, (V, Option<Instant>)>>>,
}

impl<K, V> MiniCache<K, V>
where
    K: std::hash::Hash + Eq + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    /// Creates a new `MiniCache` with the specified cleanup interval.
    ///
    /// The cleanup interval determines how often the background task runs to remove
    /// expired entries. A shorter interval means more frequent cleanup but higher
    /// CPU usage. A longer interval means less CPU usage but potentially more
    /// memory usage from expired entries.
    ///
    /// # Arguments
    ///
    /// * `cleanup_interval` - How often to run the background cleanup task
    ///
    /// # Examples
    ///
    /// ```rust
    /// use minicache::MiniCache;
    /// use std::time::Duration;
    ///
    /// // Fast cleanup every 100ms
    /// let fast_cache = MiniCache::<String, String>::new(Duration::from_millis(100));
    ///
    /// // Slow cleanup every 5 minutes
    /// let slow_cache = MiniCache::<String, String>::new(Duration::from_secs(300));
    /// ```
    pub fn new(cleanup_interval: Duration) -> Self {
        let cache = MiniCache {
            inner: Arc::new(RwLock::new(HashMap::new())),
        };
        cache.spawn_cleaner(cleanup_interval);
        cache
    }

    /// Spawns a background task that periodically removes expired entries.
    ///
    /// This method is called automatically by `new()` and doesn't need to be
    /// called manually.
    fn spawn_cleaner(&self, interval_duration: Duration) {
        let map = self.inner.clone();
        tokio::spawn(async move {
            let mut ticker = interval(interval_duration);
            loop {
                ticker.tick().await;
                let now = Instant::now();
                let mut write_guard = map.write().await;
                write_guard.retain(|_, (_, expire_at)| expire_at.map_or(true, |t| now < t));
            }
        });
    }

    /// Stores a key-value pair in the cache with an optional TTL.
    ///
    /// If a TTL is specified, the entry will automatically expire after that duration.
    /// If the key already exists, it will be overwritten with the new value and TTL.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to store
    /// * `value` - The value to associate with the key
    /// * `ttl` - Optional time-to-live duration. If `None`, the entry never expires
    ///
    /// # Examples
    ///
    /// ```rust
    /// use minicache::MiniCache;
    /// use std::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let cache = MiniCache::new(Duration::from_secs(60));
    ///     
    ///     // Store without expiration
    ///     cache.set("permanent", "value", None).await;
    ///     
    ///     // Store with 5-second expiration
    ///     cache.set("temporary", "value", Some(Duration::from_secs(5))).await;
    /// }
    /// ```
    pub async fn set(&self, key: K, value: V, ttl: Option<Duration>) {
        let expire_at = ttl.map(|d| Instant::now() + d);
        self.inner.write().await.insert(key, (value, expire_at));
    }

    /// Retrieves a value from the cache by key.
    ///
    /// If the key exists and hasn't expired, returns `Some(value)`.
    /// If the key doesn't exist or has expired, returns `None`.
    /// Expired entries are automatically removed when accessed.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// `Some(value)` if the key exists and is not expired, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use minicache::MiniCache;
    /// use std::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let cache = MiniCache::new(Duration::from_secs(60));
    ///     
    ///     cache.set("key1", "value1", None).await;
    ///     
    ///     match cache.get(&"key1").await {
    ///         Some(value) => println!("Found: {}", value),
    ///         None => println!("Key not found or expired"),
    ///     }
    /// }
    /// ```
    pub async fn get(&self, key: &K) -> Option<V> {
        let mut map = self.inner.write().await;
        if let Some((v, expire_at)) = map.get(key) {
            if expire_at.map_or(true, |t| Instant::now() < t) {
                return Some(v.clone());
            } else {
                map.remove(key);
            }
        }
        None
    }

    /// Removes a key from the cache manually.
    ///
    /// This immediately removes the key-value pair from the cache, regardless
    /// of any TTL that may have been set.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to remove
    ///
    /// # Examples
    ///
    /// ```rust
    /// use minicache::MiniCache;
    /// use std::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let cache = MiniCache::new(Duration::from_secs(60));
    ///     
    ///     cache.set("key1", "value1", None).await;
    ///     assert_eq!(cache.get(&"key1").await, Some("value1"));
    ///     
    ///     cache.remove(&"key1").await;
    ///     assert_eq!(cache.get(&"key1").await, None);
    /// }
    /// ```
    pub async fn remove(&self, key: &K) {
        self.inner.write().await.remove(key);
    }

    /// Removes all entries from the cache.
    ///
    /// This operation clears the entire cache, removing all key-value pairs
    /// regardless of their expiration status.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use minicache::MiniCache;
    /// use std::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let cache = MiniCache::new(Duration::from_secs(60));
    ///     
    ///     cache.set("key1", "value1", None).await;
    ///     cache.set("key2", "value2", None).await;
    ///     assert_eq!(cache.len().await, 2);
    ///     
    ///     cache.clear().await;
    ///     assert_eq!(cache.len().await, 0);
    /// }
    /// ```
    pub async fn clear(&self) {
        self.inner.write().await.clear();
    }

    /// Checks if a key exists in the cache and has not expired.
    ///
    /// This is equivalent to calling `get(key).is_some()` but doesn't return
    /// the actual value, which can be more efficient for existence checks.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check
    ///
    /// # Returns
    ///
    /// `true` if the key exists and is not expired, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use minicache::MiniCache;
    /// use std::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let cache = MiniCache::new(Duration::from_secs(60));
    ///     
    ///     cache.set("key1", "value1", None).await;
    ///     
    ///     if cache.contains(&"key1").await {
    ///         println!("Key exists!");
    ///     }
    /// }
    /// ```
    pub async fn contains(&self, key: &K) -> bool {
        self.get(key).await.is_some()
    }

    /// Returns the number of valid (non-expired) entries in the cache.
    ///
    /// This method iterates through all entries and counts only those that
    /// haven't expired. Note that this operation requires checking expiration
    /// times, so it has O(n) complexity.
    ///
    /// # Returns
    ///
    /// The number of valid entries currently in the cache
    ///
    /// # Examples
    ///
    /// ```rust
    /// use minicache::MiniCache;
    /// use std::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let cache = MiniCache::new(Duration::from_secs(60));
    ///     
    ///     cache.set("key1", "value1", None).await;
    ///     cache.set("key2", "value2", None).await;
    ///     
    ///     println!("Cache has {} entries", cache.len().await);
    /// }
    /// ```
    pub async fn len(&self) -> usize {
        let map = self.inner.read().await;
        map.iter()
            .filter(|(_, (_, expire_at))| expire_at.map_or(true, |t| Instant::now() < t))
            .count()
    }

    /// Returns a vector of all valid (non-expired) keys in the cache.
    ///
    /// This method iterates through all entries and collects keys for those
    /// that haven't expired. The order of keys in the returned vector is
    /// not guaranteed.
    ///
    /// # Returns
    ///
    /// A vector containing all valid keys currently in the cache
    ///
    /// # Examples
    ///
    /// ```rust
    /// use minicache::MiniCache;
    /// use std::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let cache = MiniCache::new(Duration::from_secs(60));
    ///     
    ///     cache.set("key1", "value1", None).await;
    ///     cache.set("key2", "value2", None).await;
    ///     
    ///     let keys = cache.keys().await;
    ///     println!("Cache keys: {:?}", keys);
    /// }
    /// ```
    pub async fn keys(&self) -> Vec<K> {
        let map = self.inner.read().await;
        map.iter()
            .filter_map(|(k, (_, expire_at))| {
                if expire_at.map_or(true, |t| Instant::now() < t) {
                    Some(k.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_basic_set_and_get() {
        let cache = MiniCache::new(Duration::from_secs(1));

        cache.set("key1", "value1", None).await;
        let result = cache.get(&"key1").await;

        assert_eq!(result, Some("value1"));
    }

    #[tokio::test]
    async fn test_get_nonexistent_key() {
        let cache: MiniCache<&str, &str> = MiniCache::new(Duration::from_secs(1));
        let result = cache.get(&"nonexistent").await;

        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_ttl_expiration() {
        let cache = MiniCache::new(Duration::from_millis(100));

        // Set with 50ms TTL
        cache
            .set("key1", "value1", Some(Duration::from_millis(50)))
            .await;

        // Should exist immediately
        assert_eq!(cache.get(&"key1").await, Some("value1"));

        // Wait for expiration
        sleep(Duration::from_millis(100)).await;

        // Should be expired
        assert_eq!(cache.get(&"key1").await, None);
    }

    #[tokio::test]
    async fn test_no_ttl_persistence() {
        let cache = MiniCache::new(Duration::from_millis(100));

        cache.set("key1", "value1", None).await;

        // Wait longer than cleanup interval
        sleep(Duration::from_millis(200)).await;

        // Should still exist (no TTL)
        assert_eq!(cache.get(&"key1").await, Some("value1"));
    }

    #[tokio::test]
    async fn test_overwrite_value() {
        let cache = MiniCache::new(Duration::from_secs(1));

        cache.set("key1", "value1", None).await;
        cache.set("key1", "value2", None).await;

        assert_eq!(cache.get(&"key1").await, Some("value2"));
    }

    #[tokio::test]
    async fn test_remove() {
        let cache = MiniCache::new(Duration::from_secs(1));

        cache.set("key1", "value1", None).await;
        assert_eq!(cache.get(&"key1").await, Some("value1"));

        cache.remove(&"key1").await;
        assert_eq!(cache.get(&"key1").await, None);
    }

    #[tokio::test]
    async fn test_clear() {
        let cache = MiniCache::new(Duration::from_secs(1));

        cache.set("key1", "value1", None).await;
        cache.set("key2", "value2", None).await;

        assert_eq!(cache.len().await, 2);

        cache.clear().await;

        assert_eq!(cache.len().await, 0);
        assert_eq!(cache.get(&"key1").await, None);
        assert_eq!(cache.get(&"key2").await, None);
    }

    #[tokio::test]
    async fn test_contains() {
        let cache = MiniCache::new(Duration::from_secs(1));

        assert!(!cache.contains(&"key1").await);

        cache.set("key1", "value1", None).await;
        assert!(cache.contains(&"key1").await);

        cache.remove(&"key1").await;
        assert!(!cache.contains(&"key1").await);
    }

    #[tokio::test]
    async fn test_contains_with_expired_key() {
        let cache = MiniCache::new(Duration::from_millis(100));

        cache
            .set("key1", "value1", Some(Duration::from_millis(50)))
            .await;
        assert!(cache.contains(&"key1").await);

        sleep(Duration::from_millis(100)).await;
        assert!(!cache.contains(&"key1").await);
    }

    #[tokio::test]
    async fn test_len() {
        let cache = MiniCache::new(Duration::from_secs(1));

        assert_eq!(cache.len().await, 0);

        cache.set("key1", "value1", None).await;
        assert_eq!(cache.len().await, 1);

        cache.set("key2", "value2", None).await;
        assert_eq!(cache.len().await, 2);

        cache.remove(&"key1").await;
        assert_eq!(cache.len().await, 1);
    }

    #[tokio::test]
    async fn test_len_with_expired_items() {
        let cache = MiniCache::new(Duration::from_millis(100));

        cache
            .set("key1", "value1", Some(Duration::from_millis(50)))
            .await;
        cache.set("key2", "value2", None).await;

        assert_eq!(cache.len().await, 2);

        sleep(Duration::from_millis(100)).await;

        // Only key2 should remain (key1 expired)
        assert_eq!(cache.len().await, 1);
    }

    #[tokio::test]
    async fn test_keys() {
        let cache = MiniCache::new(Duration::from_secs(1));

        cache.set("key1", "value1", None).await;
        cache.set("key2", "value2", None).await;
        cache.set("key3", "value3", None).await;

        let mut keys = cache.keys().await;
        keys.sort();

        assert_eq!(keys, vec!["key1", "key2", "key3"]);
    }

    #[tokio::test]
    async fn test_keys_with_expired_items() {
        let cache = MiniCache::new(Duration::from_millis(100));

        cache
            .set("key1", "value1", Some(Duration::from_millis(50)))
            .await;
        cache.set("key2", "value2", None).await;
        cache.set("key3", "value3", None).await;

        sleep(Duration::from_millis(100)).await;

        let mut keys = cache.keys().await;
        keys.sort();

        // Only key2 and key3 should remain (key1 expired)
        assert_eq!(keys, vec!["key2", "key3"]);
    }

    #[tokio::test]
    async fn test_automatic_cleanup() {
        let cache = MiniCache::new(Duration::from_millis(50));

        cache
            .set("key1", "value1", Some(Duration::from_millis(25)))
            .await;
        cache.set("key2", "value2", None).await;

        // Wait for cleanup to run
        sleep(Duration::from_millis(100)).await;

        // Expired item should be cleaned up
        assert_eq!(cache.get(&"key1").await, None);
        assert_eq!(cache.get(&"key2").await, Some("value2"));
    }

    #[tokio::test]
    async fn test_different_key_types() {
        let cache: MiniCache<i32, String> = MiniCache::new(Duration::from_secs(1));

        cache.set(1, "value1".to_string(), None).await;
        cache.set(2, "value2".to_string(), None).await;

        assert_eq!(cache.get(&1).await, Some("value1".to_string()));
        assert_eq!(cache.get(&2).await, Some("value2".to_string()));
        assert_eq!(cache.get(&3).await, None);
    }

    #[tokio::test]
    async fn test_concurrent_access() {
        let cache = Arc::new(MiniCache::new(Duration::from_secs(1)));
        let mut handles = vec![];

        // Spawn multiple tasks that write to cache
        for i in 0..10 {
            let cache_clone = cache.clone();
            let handle = tokio::spawn(async move {
                cache_clone.set(i, format!("value{}", i), None).await;
            });
            handles.push(handle);
        }

        // Wait for all writes to complete
        for handle in handles {
            handle.await.unwrap();
        }

        // Verify all values are present
        for i in 0..10 {
            assert_eq!(cache.get(&i).await, Some(format!("value{}", i)));
        }
    }

    #[tokio::test]
    async fn test_cache_clone() {
        let cache1 = MiniCache::new(Duration::from_secs(1));
        let cache2 = cache1.clone();

        cache1.set("key1", "value1", None).await;

        // Both caches should see the same data (shared inner state)
        assert_eq!(cache2.get(&"key1").await, Some("value1"));

        cache2.set("key2", "value2", None).await;
        assert_eq!(cache1.get(&"key2").await, Some("value2"));
    }
}
