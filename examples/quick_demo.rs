use minicache::MiniCache;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    println!("🚀 MiniCache Quick Performance Demo");
    println!("===================================\n");

    // Test 1: Basic Operations
    println!("📝 Test 1: Basic Operations");
    let cache = MiniCache::new(Duration::from_secs(60));

    let start = Instant::now();
    for i in 0..10_000 {
        cache.set(i, format!("value_{}", i), None).await;
    }
    let write_duration = start.elapsed();

    let start = Instant::now();
    for i in 0..10_000 {
        let _ = cache.get(&i).await;
    }
    let read_duration = start.elapsed();

    println!(
        "  ✅ 10K writes: {:?} ({:.0} ops/sec)",
        write_duration,
        10_000.0 / write_duration.as_secs_f64()
    );
    println!(
        "  ✅ 10K reads:  {:?} ({:.0} ops/sec)",
        read_duration,
        10_000.0 / read_duration.as_secs_f64()
    );
    println!("  📊 Cache size: {}\n", cache.len().await);

    // Test 2: TTL Operations
    println!("⏰ Test 2: TTL Operations");
    let ttl_cache = MiniCache::new(Duration::from_millis(100));

    let start = Instant::now();
    for i in 0..1_000 {
        ttl_cache
            .set(
                i,
                format!("ttl_value_{}", i),
                Some(Duration::from_millis(500)),
            )
            .await;
    }
    let ttl_write_duration = start.elapsed();

    println!(
        "  ✅ 1K TTL writes: {:?} ({:.0} ops/sec)",
        ttl_write_duration,
        1_000.0 / ttl_write_duration.as_secs_f64()
    );
    println!("  📊 TTL cache size: {}", ttl_cache.len().await);

    // Wait and check expiration
    tokio::time::sleep(Duration::from_millis(600)).await;
    println!("  ⏳ After expiration: {} entries\n", ttl_cache.len().await);

    // Test 3: Concurrent Operations
    println!("🔄 Test 3: Concurrent Operations");
    let concurrent_cache = Arc::new(MiniCache::new(Duration::from_secs(60)));

    let start = Instant::now();
    let mut handles = vec![];

    for task_id in 0..10 {
        let cache_clone = concurrent_cache.clone();
        let handle = tokio::spawn(async move {
            for i in 0..1_000 {
                cache_clone
                    .set(
                        format!("task_{}_{}", task_id, i),
                        format!("concurrent_value_{}_{}", task_id, i),
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

    let concurrent_duration = start.elapsed();
    println!(
        "  ✅ 10 tasks × 1K writes: {:?} ({:.0} ops/sec)",
        concurrent_duration,
        10_000.0 / concurrent_duration.as_secs_f64()
    );
    println!("  📊 Final cache size: {}\n", concurrent_cache.len().await);

    // Test 4: Memory Usage Estimation
    println!("💾 Test 4: Memory Usage Estimation");
    let memory_cache = MiniCache::new(Duration::from_secs(60));

    // Small test to estimate memory per entry
    for i in 0..1_000 {
        memory_cache
            .set(i, format!("memory_test_value_{}", i), None)
            .await;
    }

    let avg_key_size = std::mem::size_of::<i32>();
    let avg_value_size = "memory_test_value_000".len();
    let estimated_entry_size = avg_key_size + avg_value_size + 64; // +64 for overhead

    println!("  📏 Estimated entry size: ~{} bytes", estimated_entry_size);
    println!(
        "  📊 1K entries estimated memory: ~{} KB",
        (estimated_entry_size * 1_000) / 1024
    );
    println!(
        "  📈 10K entries estimated memory: ~{} MB",
        (estimated_entry_size * 10_000) / (1024 * 1024)
    );
    println!(
        "  📈 100K entries estimated memory: ~{} MB\n",
        (estimated_entry_size * 100_000) / (1024 * 1024)
    );

    // Test 5: Cache Operations
    println!("🔧 Test 5: Cache Operations");
    let ops_cache = MiniCache::new(Duration::from_secs(60));

    // Populate cache
    for i in 0..100 {
        ops_cache.set(i, format!("ops_value_{}", i), None).await;
    }

    let start = Instant::now();
    let keys = ops_cache.keys().await;
    let keys_duration = start.elapsed();

    println!(
        "  ✅ Get all keys: {:?} for {} keys",
        keys_duration,
        keys.len()
    );

    let start = Instant::now();
    let contains_true = ops_cache.contains(&50).await;
    let contains_false = ops_cache.contains(&200).await;
    let contains_duration = start.elapsed();

    println!(
        "  ✅ Contains check: {:?} (found: {}, missing: {})",
        contains_duration, contains_true, contains_false
    );

    let start = Instant::now();
    ops_cache.remove(&50).await;
    let remove_duration = start.elapsed();

    println!("  ✅ Remove operation: {:?}", remove_duration);
    println!("  📊 Cache size after remove: {}\n", ops_cache.len().await);

    // Summary
    println!("📋 Performance Summary");
    println!("====================");
    println!(
        "🏆 Best performance: Basic reads ({:.0} ops/sec)",
        10_000.0 / read_duration.as_secs_f64()
    );
    println!(
        "⚡ Write performance: {:.0} ops/sec",
        10_000.0 / write_duration.as_secs_f64()
    );
    println!(
        "🔄 Concurrent performance: {:.0} ops/sec",
        10_000.0 / concurrent_duration.as_secs_f64()
    );
    println!(
        "⏰ TTL overhead: {:.1}% slower",
        ((ttl_write_duration.as_secs_f64() / write_duration.as_secs_f64()) - 1.0) * 100.0
    );

    println!("\n🎯 To run comprehensive benchmarks:");
    println!("   ./run_benchmarks.sh");
    println!("\n📊 For detailed analysis:");
    println!("   cargo bench");
    println!("   cargo run --example memory_profiler");
}
