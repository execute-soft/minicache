//! N-API bindings for minicache - JavaScript/Node.js interop layer

use crate::core::MiniCache;
use napi_derive::napi;
use std::time::Duration;

/// JavaScript-compatible cache class
#[napi]
pub struct JsCache {
    cache: MiniCache<String, String>,
}

#[napi]
impl JsCache {
    #[napi(constructor)]
    pub fn new() -> Self {
        let cache = MiniCache::new(Duration::from_secs(60));
        JsCache { cache }
    }

    #[napi]
    pub async fn set(&self, key: String, value: String, ttl_ms: Option<u32>) -> napi::Result<()> {
        let ttl = ttl_ms.map(|ms| Duration::from_millis(ms as u64));
        self.cache.set(key, value, ttl).await;
        Ok(())
    }

    #[napi]
    pub async fn get(&self, key: String) -> napi::Result<Option<String>> {
        Ok(self.cache.get(&key).await)
    }

    #[napi]
    pub async fn remove(&self, key: String) -> napi::Result<()> {
        self.cache.remove(&key).await;
        Ok(())
    }

    #[napi]
    pub async fn clear(&self) -> napi::Result<()> {
        self.cache.clear().await;
        Ok(())
    }

    #[napi]
    pub async fn size(&self) -> napi::Result<u32> {
        Ok(self.cache.len().await as u32)
    }
}

#[napi]
pub fn get_info() -> String {
    format!("minicache v{} - Rust backend", env!("CARGO_PKG_VERSION"))
}
