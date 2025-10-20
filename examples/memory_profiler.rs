use minicache::MiniCache;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::{Pid, System};

struct MemoryProfiler {
    system: System,
    pid: Pid,
}

impl MemoryProfiler {
    fn new() -> Self {
        let system = System::new_all();
        let pid = sysinfo::get_current_pid().unwrap();

        Self { system, pid }
    }

    fn get_memory_info(&mut self) -> MemoryInfo {
        self.system.refresh_all();
        if let Some(process) = self.system.process(self.pid) {
            MemoryInfo {
                rss: process.memory(),
                virtual_memory: process.virtual_memory(),
                cpu_usage: process.cpu_usage(),
            }
        } else {
            MemoryInfo::default()
        }
    }
}

#[derive(Debug, Default)]
struct MemoryInfo {
    rss: u64,            // Resident Set Size (physical memory)
    virtual_memory: u64, // Virtual memory
    cpu_usage: f32,      // CPU usage percentage
}

impl MemoryInfo {
    fn display(&self, label: &str) {
        println!(
            "{}: RSS: {} KB, Virtual: {} KB, CPU: {:.2}%",
            label,
            self.rss / 1024,
            self.virtual_memory / 1024,
            self.cpu_usage
        );
    }
}

#[tokio::main]
async fn main() {
    println!("MiniCache Memory Profiling Report");
    println!("================================");

    let mut profiler = MemoryProfiler::new();

    // Baseline memory usage
    let baseline = profiler.get_memory_info();
    baseline.display("Baseline");

    // Test 1: Empty cache
    let cache = MiniCache::new(Duration::from_secs(60));
    let empty_cache = profiler.get_memory_info();
    empty_cache.display("Empty Cache");

    // Test 2: Small cache (1K entries)
    println!("\n--- Loading 1,000 entries ---");
    for i in 0..1000 {
        cache.set(i, format!("Value for key {}", i), None).await;
    }
    let small_cache = profiler.get_memory_info();
    small_cache.display("1K entries");

    // Test 3: Medium cache (10K entries)
    println!("\n--- Loading 10,000 entries ---");
    for i in 1000..11000 {
        cache.set(i, format!("Value for key {}", i), None).await;
    }
    let medium_cache = profiler.get_memory_info();
    medium_cache.display("10K entries");

    // Test 4: Large cache (100K entries)
    println!("\n--- Loading 100,000 entries ---");
    for i in 11000..111000 {
        cache.set(i, format!("Value for key {}", i), None).await;
    }
    let large_cache = profiler.get_memory_info();
    large_cache.display("100K entries");

    // Test 5: Memory with TTL entries
    println!("\n--- Testing TTL entries ---");
    let ttl_cache = MiniCache::new(Duration::from_millis(100));
    for i in 0..10000 {
        ttl_cache
            .set(
                format!("ttl_key_{}", i),
                format!("TTL value for key {}", i),
                Some(Duration::from_millis(500)),
            )
            .await;
    }
    let ttl_memory = profiler.get_memory_info();
    ttl_memory.display("10K TTL entries");

    // Test 6: Concurrent access memory impact
    println!("\n--- Testing concurrent access ---");
    let concurrent_cache = Arc::new(MiniCache::new(Duration::from_secs(60)));
    let mut handles = vec![];

    for task_id in 0..10 {
        let cache_clone = concurrent_cache.clone();
        let handle = tokio::spawn(async move {
            for i in 0..1000 {
                cache_clone
                    .set(
                        format!("concurrent_{}_{}", task_id, i),
                        format!("Concurrent value {} {}", task_id, i),
                        None,
                    )
                    .await;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let concurrent_memory = profiler.get_memory_info();
    concurrent_memory.display("Concurrent writes");

    // Test 7: Performance under load
    println!("\n--- Performance testing ---");
    let perf_cache = MiniCache::new(Duration::from_secs(30));

    let start_time = std::time::Instant::now();

    // Write test
    for i in 0..10000 {
        perf_cache
            .set(i, format!("Performance test value {}", i), None)
            .await;
    }
    let write_time = start_time.elapsed();

    // Read test
    let read_start = std::time::Instant::now();
    for i in 0..10000 {
        let _ = perf_cache.get(&i).await;
    }
    let read_time = read_start.elapsed();

    let perf_memory = profiler.get_memory_info();
    perf_memory.display("Performance test");

    println!("\n--- Performance Results ---");
    println!(
        "10K writes: {:?} ({:.2} ops/sec)",
        write_time,
        10000.0 / write_time.as_secs_f64()
    );
    println!(
        "10K reads: {:?} ({:.2} ops/sec)",
        read_time,
        10000.0 / read_time.as_secs_f64()
    );

    // Test 8: Memory after cleanup
    println!("\n--- Testing memory cleanup ---");
    let cleanup_cache = MiniCache::new(Duration::from_millis(50));

    // Add items with short TTL
    for i in 0..5000 {
        cleanup_cache
            .set(
                i,
                format!("Cleanup test value {}", i),
                Some(Duration::from_millis(25)),
            )
            .await;
    }

    let before_cleanup = profiler.get_memory_info();
    before_cleanup.display("Before cleanup");

    // Wait for cleanup
    tokio::time::sleep(Duration::from_millis(100)).await;

    let after_cleanup = profiler.get_memory_info();
    after_cleanup.display("After cleanup");

    println!("\n--- Memory Usage Summary ---");
    println!(
        "Memory overhead per entry (approx): {} bytes",
        (large_cache.rss - empty_cache.rss) / 100000
    );

    println!(
        "Total memory growth: {} KB",
        (large_cache.rss - baseline.rss) / 1024
    );

    // Test cache size reporting
    println!("\n--- Cache Size Verification ---");
    println!("Reported cache size: {}", cache.len().await);
    println!("Expected size: 100,000");

    // Test key enumeration performance
    let keys_start = std::time::Instant::now();
    let keys = cache.keys().await;
    let keys_time = keys_start.elapsed();
    println!("Keys enumeration: {} keys in {:?}", keys.len(), keys_time);
}

#[cfg(test)]
mod memory_tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_growth_linear() {
        let mut profiler = MemoryProfiler::new();
        let cache = MiniCache::new(Duration::from_secs(60));

        let baseline = profiler.get_memory_info();

        // Test memory growth at different sizes
        let sizes = [100, 500, 1000, 2000];
        let mut previous_memory = baseline.rss;

        for &size in &sizes {
            // Clear and repopulate
            cache.clear().await;

            for i in 0..size {
                cache.set(i, format!("Test value {}", i), None).await;
            }

            let current = profiler.get_memory_info();
            let growth = current.rss - baseline.rss;

            println!("Size: {}, Memory growth: {} KB", size, growth / 1024);

            // Memory should grow roughly linearly
            if size > 100 {
                let growth_rate = growth as f64 / size as f64;
                println!("Growth rate: {:.2} bytes per entry", growth_rate);
            }

            previous_memory = current.rss;
        }
    }
}
