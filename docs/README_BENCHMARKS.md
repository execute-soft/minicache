# 📊 MiniCache Benchmark Suite - Complete Guide

## 🎯 Overview

This comprehensive benchmark suite provides detailed performance and memory analysis for MiniCache, including:
- **Performance benchmarking** with statistical analysis
- **Memory usage profiling** across different scales
- **Concurrent access testing** under load
- **TTL (Time-To-Live) efficiency** measurement
- **Realistic usage scenarios** simulation

## 🚀 Quick Start Commands

```bash
# Complete benchmark suite (recommended)
./run_benchmarks.sh

# Quick performance demo
cargo run --release --example quick_demo

# Memory analysis only
cargo run --release --example memory_profiler

# Performance benchmarks only
cargo bench

# Build and run tests
cargo test && cargo build --release
```

## 📁 Files Structure

```
minicache/
├── benches/
│   └── minicache_benchmark.rs      # Criterion performance benchmarks
├── examples/
│   ├── memory_profiler.rs          # Detailed memory analysis
│   └── quick_demo.rs               # Quick performance demo
├── benchmark_results/              # Generated reports (after running)
├── run_benchmarks.sh              # Complete benchmark runner
├── BENCHMARKS.md                  # Benchmark documentation
└── PERFORMANCE_GUIDE.md           # Analysis and optimization guide
```

## 🔍 What Gets Measured

### 1. **Basic Operations**
- **Set operations**: Key-value insertion performance (100, 1K, 10K entries)
- **Get operations**: Value retrieval performance (100, 1K, 10K entries)
- **Scaling behavior**: How performance changes with cache size

### 2. **Concurrent Access**
- **Concurrent writes**: Multiple tasks writing simultaneously (10, 50, 100 tasks)
- **Read/write contention**: Mixed workload performance
- **Scalability**: How well performance scales with CPU cores

### 3. **TTL (Time-To-Live)**
- **TTL write overhead**: Performance impact of expiration metadata
- **Cleanup efficiency**: Automatic expired entry removal
- **Memory cleanup**: Memory reclamation after expiration

### 4. **Memory Usage**
- **Base overhead**: Memory used by empty cache
- **Per-entry cost**: Memory overhead per cached item
- **Growth patterns**: Memory scaling with cache size
- **Memory efficiency**: Actual vs. estimated memory usage

### 5. **Realistic Scenarios**
- **Web cache simulation**: Mixed read/write with TTL patterns
- **Session cache**: Frequent updates to same keys
- **Concurrent access patterns**: Real-world usage simulation

## 📈 Sample Results

```
🚀 Performance Highlights (MacBook Pro M1):
  ✅ Basic reads:  13.7M ops/sec
  ✅ Basic writes:  9.6M ops/sec  
  🔄 Concurrent:    1.7M ops/sec
  💾 Memory/entry: ~162 bytes
  ⏰ TTL cleanup:  Sub-millisecond
```

## 📊 Understanding Output

### Performance Metrics
- **ops/sec**: Operations per second (higher = better)
- **µs/operation**: Microseconds per operation (lower = better)
- **Confidence intervals**: Statistical reliability [low | mean | high]
- **Outliers**: Measurements outside normal range

### Memory Metrics
- **RSS**: Physical memory actually used
- **Virtual**: Total memory allocated
- **Growth rate**: Memory increase per cache entry
- **Overhead**: Fixed cost per entry

## 🛠 Customization

### Adding Custom Benchmarks

Edit `benches/minicache_benchmark.rs`:

```rust
fn bench_my_scenario(c: &mut Criterion) {
    let mut group = c.benchmark_group("my_scenario");
    
    group.bench_function("custom_test", |b| {
        b.iter(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                // Your benchmark code here
                let cache = MiniCache::new(Duration::from_secs(60));
                // ... test operations
            });
        });
    });
    
    group.finish();
}

// Add to criterion_group!
criterion_group!(benches, ..., bench_my_scenario);
```

### Modifying Test Parameters

```rust
// Change cache sizes tested
for size in [100, 1000, 10000, 100000].iter() { ... }

// Adjust concurrent task counts
for num_tasks in [10, 50, 100, 200].iter() { ... }

// Modify TTL durations
Some(Duration::from_millis(500)) // Instead of 100ms
```

## 📋 Generated Reports

After running `./run_benchmarks.sh`:

1. **`benchmark_results/benchmark_summary_*.md`**
   - Human-readable performance summary
   - System information
   - Key insights and recommendations

2. **`benchmark_results/benchmark_output_*.txt`**
   - Raw Criterion output
   - Detailed statistical analysis
   - Performance regression detection

3. **`benchmark_results/memory_profile_*.txt`**
   - Memory usage at different scales
   - Growth patterns and efficiency
   - Performance vs. memory trade-offs

4. **`target/criterion/report/index.html`**
   - Interactive HTML reports
   - Performance graphs and charts
   - Historical trend analysis

## 🚨 Troubleshooting

### Common Issues

**Build failures**:
```bash
rustup update
cargo clean && cargo build --release
```

**Inconsistent results**:
- Close other applications
- Run multiple times and average
- Check system temperature/throttling

**Memory profiling issues**:
- Reduce test sizes for low-memory systems
- Ensure sufficient disk space for reports

### Platform Notes

- **macOS**: May show high virtual memory (normal)
- **Linux**: More accurate memory reporting
- **Performance varies**: Battery vs. AC power significantly affects results

## 🎯 Performance Optimization

Based on benchmark results:

1. **High latency → Profile with flamegraph**:
   ```bash
   cargo install flamegraph
   cargo flamegraph --example quick_demo
   ```

2. **Memory usage → Consider data structures**:
   - Smaller key/value types
   - Custom serialization
   - Alternative hash maps

3. **Concurrency issues → Lock analysis**:
   - Monitor lock contention
   - Consider lock-free alternatives
   - Benchmark different RwLock implementations

## 📚 Next Steps

1. **Review HTML reports** for detailed visualizations
2. **Compare with baseline** performance expectations
3. **Profile specific bottlenecks** using the insights
4. **Optimize based on your use case** requirements

---

**🎉 Ready to benchmark?** Run `./run_benchmarks.sh` and dive into the performance analysis!