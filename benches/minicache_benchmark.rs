use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use minicache::MiniCache;
use std::hint::black_box;
use std::sync::Arc;
use std::time::Duration;

// Benchmark basic operations
fn bench_basic_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic_operations");

    // Test different cache sizes
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("set_operation", size), size, |b, &size| {
            b.iter(|| {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    let cache = MiniCache::new(Duration::from_secs(60));
                    for i in 0..size {
                        cache
                            .set(black_box(i), black_box(format!("value_{}", i)), None)
                            .await;
                    }
                });
            });
        });

        group.bench_with_input(BenchmarkId::new("get_operation", size), size, |b, &size| {
            b.iter(|| {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    let cache = MiniCache::new(Duration::from_secs(60));
                    // Pre-populate cache
                    for i in 0..size {
                        cache.set(i, format!("value_{}", i), None).await;
                    }

                    // Benchmark get operations
                    for i in 0..size {
                        let _ = cache.get(black_box(&i)).await;
                    }
                });
            });
        });
    }

    group.finish();
}

// Benchmark concurrent operations
fn bench_concurrent_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_operations");

    for num_tasks in [10, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::new("concurrent_writes", num_tasks),
            num_tasks,
            |b, &num_tasks| {
                b.iter(|| {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(async {
                        let cache = Arc::new(MiniCache::new(Duration::from_secs(60)));
                        let mut handles = vec![];

                        for i in 0..num_tasks {
                            let cache_clone = cache.clone();
                            let handle = tokio::spawn(async move {
                                for j in 0..100 {
                                    cache_clone
                                        .set(
                                            black_box(i * 100 + j),
                                            black_box(format!("value_{}_{}", i, j)),
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
                    });
                });
            },
        );
    }

    group.finish();
}

// Benchmark TTL and expiration
fn bench_ttl_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("ttl_operations");

    group.bench_function("set_with_ttl", |b| {
        b.iter(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let cache = MiniCache::new(Duration::from_millis(100));
                for i in 0..1000 {
                    cache
                        .set(
                            black_box(i),
                            black_box(format!("value_{}", i)),
                            Some(Duration::from_millis(100)),
                        )
                        .await;
                }
            });
        });
    });

    group.finish();
}

// Benchmark realistic scenarios
fn bench_realistic_scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("realistic_scenarios");

    // Web cache scenario: mix of reads and writes with TTL
    group.bench_function("web_cache_simulation", |b| {
        b.iter(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let cache = Arc::new(MiniCache::new(Duration::from_secs(1)));

                // Simulate web cache usage pattern
                let mut handles = vec![];

                // Writer task (background data updates)
                let cache_writer = cache.clone();
                let writer_handle = tokio::spawn(async move {
                    for i in 0..100 {
                        cache_writer
                            .set(
                                black_box(format!("user:{}", i % 20)),
                                black_box(format!("User data for user {}", i % 20)),
                                Some(Duration::from_millis(500)),
                            )
                            .await;
                        tokio::time::sleep(Duration::from_millis(1)).await;
                    }
                });

                // Multiple reader tasks (simulating concurrent requests)
                for reader_id in 0..5 {
                    let cache_reader = cache.clone();
                    let reader_handle = tokio::spawn(async move {
                        for i in 0..50 {
                            let key = format!("user:{}", (reader_id * 50 + i) % 20);
                            let _ = cache_reader.get(black_box(&key)).await;
                            tokio::time::sleep(Duration::from_millis(2)).await;
                        }
                    });
                    handles.push(reader_handle);
                }

                handles.push(writer_handle);

                for handle in handles {
                    handle.await.unwrap();
                }
            });
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_basic_operations,
    bench_concurrent_operations,
    bench_ttl_operations,
    bench_realistic_scenarios
);

criterion_main!(benches);
