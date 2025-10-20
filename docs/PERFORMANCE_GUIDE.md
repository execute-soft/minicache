# MiniCache Performance Analysis Guide

This guide provides comprehensive instructions for benchmarking and analyzing the performance and memory usage of MiniCache.

## 🚀 Quick Start

### Run Complete Benchmark Suite
```bash
# Run all benchmarks and generate reports
./run_benchmarks.sh
```

### Run Individual Components
```bash
# Memory analysis only
cargo run --release --example memory_profiler

# Performance benchmarks only  
cargo bench

# Quick performance test
cargo run --release --example quick_test
```

## 📊 Understanding the Results

### 1. Memory Profiling Results

The memory profiler provides detailed analysis of memory usage patterns:

```
--- Memory Usage Summary ---
Memory overhead per entry (approx): 162 bytes
Total memory growth: 16048 KB
```

**Key Metrics:**
- **RSS (Resident Set Size)**: Physical memory actually used
- **Virtual Memory**: Total memory allocated by the process
- **Memory per entry**: Overhead for each cache entry (~160-200 bytes typical)
- **Growth rate**: How memory scales with cache size

### 2. Criterion Benchmark Results

Criterion provides statistical analysis of performance:

```
basic_operations/set_operation/100
    time:   [243.67 µs 246.83 µs 250.35 µs]
```

**Interpreting the numbers:**
- **Lower bound | Mean | Upper bound**: Statistical confidence interval
- **Time per iteration**: Total time divided by number of operations
- **Outliers**: Measurements outside normal range

### 3. Performance Baselines

**Expected performance on modern hardware:**

| Operation | Scale | Expected Performance |
|-----------|-------|---------------------|
| Sequential writes | 10K entries | 10M+ ops/sec |
| Sequential reads | 10K entries | 20M+ ops/sec |  
| Concurrent writes | 100 tasks | Scales with cores |
| TTL operations | 1K entries | Similar to basic ops |
| Memory per entry | Any size | 160-200 bytes |

## 🔍 Analyzing Performance Issues

### 1. Slow Write Performance
**Symptoms**: Set operations taking >1µs per operation
**Possible causes**: 
- Lock contention (check concurrent benchmarks)
- Memory allocation overhead
- Async runtime overhead

**Investigation**:
```bash
# Compare different cache sizes
cargo bench -- set_operation

# Check memory allocation patterns
cargo run --example memory_profiler
```

### 2. High Memory Usage
**Symptoms**: Memory per entry >300 bytes
**Possible causes**:
- String overhead in keys/values
- HashMap overhead
- TTL metadata overhead

**Investigation**: Look at memory growth patterns across different cache sizes.

### 3. Concurrent Performance Issues
**Symptoms**: Performance doesn't scale with CPU cores
**Possible causes**:
- RwLock contention
- False sharing
- Async task overhead

## 📈 Benchmark Scenarios Explained

### Basic Operations
- **Purpose**: Baseline performance measurement
- **Tests**: Sequential reads/writes at 100, 1K, 10K scales
- **Metrics**: Operations per second, latency distribution

### Concurrent Operations
- **Purpose**: Multi-threaded performance testing
- **Tests**: 10, 50, 100 concurrent tasks
- **Metrics**: Throughput under contention, scalability

### TTL Operations
- **Purpose**: Time-to-live functionality overhead
- **Tests**: Setting entries with expiration, cleanup efficiency
- **Metrics**: Performance impact of TTL metadata

### Realistic Scenarios
- **Web cache simulation**: Mixed read/write with typical web patterns
- **Session cache simulation**: Frequent updates to same keys

## 🛠 Customizing Benchmarks

### Adding New Benchmarks

1. **Edit** `benches/minicache_benchmark.rs`
2. **Add new function**:
```rust
fn bench_custom_scenario(c: &mut Criterion) {
    let mut group = c.benchmark_group("custom_scenario");
    
    group.bench_function("my_test", |b| {
        b.iter(|| {
            // Your benchmark code here
        });
    });
    
    group.finish();
}
```

3. **Update criterion_group!**:
```rust
criterion_group!(
    benches, 
    bench_basic_operations,
    bench_custom_scenario  // Add your function
);
```

### Modifying Test Parameters

**Cache sizes**: Change the arrays in benchmark functions
```rust
for size in [100, 1000, 10000, 100000].iter() { // Add 100K
```

**Concurrent tasks**: Modify concurrent test parameters
```rust
for num_tasks in [10, 50, 100, 200].iter() { // Add 200 tasks
```

**TTL durations**: Adjust expiration times
```rust
Some(Duration::from_millis(500)) // Change from 100ms to 500ms
```

## 📁 Output Files Guide

After running benchmarks, you'll find these files:

### `benchmark_results/`
- `benchmark_output_*.txt`: Raw Criterion output with timing data
- `memory_profile_*.txt`: Detailed memory analysis
- `benchmark_summary_*.md`: Human-readable summary report

### `target/criterion/`
- `report/index.html`: Interactive HTML reports with graphs
- Individual benchmark directories with detailed statistics

## 🚨 Troubleshooting

### Common Issues

**Build failures**:
```bash
# Update Rust toolchain
rustup update
cargo clean
cargo build --release
```

**Permission errors on macOS**:
```bash
chmod +x run_benchmarks.sh
```

**Memory profiling issues**:
- Reduce test sizes if running out of memory
- Check system resource availability
- Ensure no other heavy processes are running

**Inconsistent results**:
- Run benchmarks multiple times
- Ensure system is under low load
- Check for thermal throttling on laptops

### Platform-Specific Notes

**macOS**:
- Uses `sysinfo` crate for memory monitoring
- May need Xcode command line tools installed
- Performance can vary significantly on battery vs. plugged in

**Linux**:
- More accurate memory reporting
- Better CPU isolation options available
- Consider using `taskset` for CPU pinning

## 🎯 Performance Optimization Tips

Based on benchmark results, consider these optimizations:

1. **High latency**: Profile with `perf` or `flamegraph`
2. **Memory usage**: Consider alternative key/value representations
3. **Contention**: Experiment with different RwLock implementations
4. **TTL overhead**: Optimize cleanup algorithms

## 📚 Further Reading

- [Criterion.rs User Guide](https://bheisler.github.io/criterion.rs/book/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Async Performance Best Practices](https://ryhl.io/blog/async-what-is-blocking/)