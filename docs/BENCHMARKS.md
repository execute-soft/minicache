# MiniCache Benchmarks

This directory contains comprehensive benchmarks for measuring MiniCache performance and memory usage.

## Quick Start

Run all benchmarks with a single command:
```bash
./run_benchmarks.sh
```

## Individual Components

### 1. Criterion Benchmarks (`benches/minicache_benchmark.rs`)
- **Basic Operations**: Set and get performance at different scales
- **Concurrent Operations**: Multi-threaded performance testing
- **TTL Operations**: Time-to-live functionality performance
- **Realistic Scenarios**: Web cache and session cache simulations

Run with:
```bash
cargo bench
```

### 2. Memory Profiler (`examples/memory_profiler.rs`)
- Memory usage analysis at different cache sizes
- Memory growth patterns
- Performance metrics (operations per second)
- Cleanup efficiency testing

Run with:
```bash
cargo run --release --example memory_profiler
```

## Benchmark Categories

### Basic Operations
- **set_operation**: Time to insert key-value pairs (100, 1K, 10K entries)
- **get_operation**: Time to retrieve values (100, 1K, 10K entries)

### Concurrent Operations  
- **concurrent_writes**: Performance under concurrent write load (10, 50, 100 tasks)
- **concurrent_reads**: Performance under concurrent read load

### TTL Operations
- **set_with_ttl**: Performance of setting items with expiration
- **cleanup_efficiency**: Automatic cleanup performance

### Realistic Scenarios
- **web_cache_simulation**: Mixed read/write pattern with TTL
- **session_cache_simulation**: Frequent updates to same keys

## Memory Analysis

The memory profiler tests:
- Empty cache overhead
- Memory per entry (1K, 10K, 100K entries)
- Memory with TTL entries
- Concurrent access memory impact
- Memory cleanup efficiency

## Output Files

After running benchmarks, you'll find:
- `benchmark_results/benchmark_output_*.txt` - Raw benchmark output
- `benchmark_results/memory_profile_*.txt` - Memory analysis results
- `benchmark_results/benchmark_summary_*.md` - Human-readable summary
- `target/criterion/` - HTML reports with graphs and charts

## Reading Results

### Criterion Metrics
- **time**: Average execution time per iteration (lower is better)
- **thrpt**: Throughput in operations per second (higher is better)
- **R²**: Coefficient of determination for measurement reliability

### Memory Metrics
- **RSS**: Resident Set Size (actual physical memory used)
- **Virtual**: Virtual memory allocated
- **Growth rate**: Memory usage per cache entry

## System Requirements

- Rust 1.70+
- Tokio runtime
- ~1GB RAM for full benchmark suite
- ~5-10 minutes execution time

## Customizing Benchmarks

Edit `benches/minicache_benchmark.rs` to:
- Change cache sizes tested
- Modify concurrent task counts  
- Add new benchmark scenarios
- Adjust TTL durations

Edit `examples/memory_profiler.rs` to:
- Test different data sizes
- Change profiling intervals
- Add custom memory tests

## Performance Baselines

Expected performance on modern hardware:
- **Basic operations**: 1M+ ops/sec
- **Concurrent writes**: Scales with CPU cores
- **Memory overhead**: ~64-128 bytes per entry
- **TTL cleanup**: Sub-millisecond for 1K entries

## Troubleshooting

### Common Issues
1. **Permission denied**: Run `chmod +x run_benchmarks.sh`
2. **Build failures**: Ensure Rust toolchain is up to date
3. **Memory errors**: Reduce test sizes in benchmark code
4. **HTML reports missing**: Check `target/criterion/report/index.html`

### Platform-Specific Notes
- **macOS**: Uses `sysctl` for system info
- **Linux**: Uses `/proc` filesystem and `lscpu`  
- **Windows**: Limited system information collection

## Contributing

When adding new benchmarks:
1. Add test to appropriate category
2. Include both performance and memory testing
3. Update this README with new test descriptions
4. Ensure benchmarks are deterministic and repeatable