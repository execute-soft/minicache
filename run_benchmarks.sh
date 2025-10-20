#!/bin/bash

echo "MiniCache Benchmark Suite"
echo "========================"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Cargo.toml not found. Please run this script from the project root."
    exit 1
fi

# Create results directory
RESULTS_DIR="benchmark_results"
mkdir -p "$RESULTS_DIR"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

print_status "Building project in release mode..."
if cargo build --release; then
    print_success "Build completed successfully"
else
    print_error "Build failed"
    exit 1
fi

print_status "Running unit tests to ensure correctness..."
if cargo test; then
    print_success "All tests passed"
else
    print_error "Tests failed"
    exit 1
fi

print_status "Running memory profiler..."
if cargo run --release --example memory_profiler > "$RESULTS_DIR/memory_profile_$TIMESTAMP.txt" 2>&1; then
    print_success "Memory profiling completed"
    echo "Results saved to: $RESULTS_DIR/memory_profile_$TIMESTAMP.txt"
else
    print_warning "Memory profiling failed or incomplete"
fi

print_status "Running Criterion benchmarks..."
if cargo bench -- --output-format html > "$RESULTS_DIR/benchmark_output_$TIMESTAMP.txt" 2>&1; then
    print_success "Benchmarks completed"
    echo "Results saved to: $RESULTS_DIR/benchmark_output_$TIMESTAMP.txt"
    echo "HTML report should be available in: target/criterion/"
else
    print_warning "Benchmarks completed with warnings or failed"
fi

print_status "Generating summary report..."

# Create a summary report
SUMMARY_FILE="$RESULTS_DIR/benchmark_summary_$TIMESTAMP.md"

cat > "$SUMMARY_FILE" << EOF
# MiniCache Benchmark Report

**Generated on:** $(date)
**System:** $(uname -a)
**Rust Version:** $(rustc --version)

## Test Environment
- **OS:** $(uname -s) $(uname -r)
- **Architecture:** $(uname -m)
- **CPU Cores:** $(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo "Unknown")
- **Memory:** $(free -h 2>/dev/null | grep '^Mem:' | awk '{print $2}' || echo "Unknown")

## Benchmark Categories

### 1. Basic Operations
- **set_operation**: Measures the time to insert key-value pairs
- **get_operation**: Measures the time to retrieve values by key

### 2. Concurrent Operations  
- **concurrent_writes**: Tests performance under concurrent write load
- **concurrent_reads**: Tests performance under concurrent read load

### 3. TTL Operations
- **set_with_ttl**: Tests performance of setting items with time-to-live
- **expiration_handling**: Tests cleanup and expiration performance

### 4. Realistic Scenarios
- **web_cache_simulation**: Simulates typical web caching patterns
- **session_cache_simulation**: Simulates session storage patterns

## Files Generated
- Benchmark output: \`benchmark_output_$TIMESTAMP.txt\`
- Memory profile: \`memory_profile_$TIMESTAMP.txt\`
- HTML reports: \`target/criterion/\`

## How to Interpret Results

### Criterion Output
- **time**: Average execution time per iteration
- **thrpt**: Throughput (operations per second)
- **Lower is better** for time measurements
- **Higher is better** for throughput measurements

### Memory Profile
- **RSS**: Resident Set Size (physical memory usage)
- **Virtual**: Virtual memory usage
- **Growth rate**: Memory usage per cache entry

## Next Steps
1. Review the detailed HTML reports in \`target/criterion/\`
2. Check memory usage patterns in the memory profile
3. Compare results with different cache sizes and workloads
4. Optimize based on identified bottlenecks

EOF

print_success "Summary report generated: $SUMMARY_FILE"

print_status "Collecting system information..."
echo "" >> "$SUMMARY_FILE"
echo "## System Information" >> "$SUMMARY_FILE"
echo "\`\`\`" >> "$SUMMARY_FILE"
echo "Date: $(date)" >> "$SUMMARY_FILE"
echo "System: $(uname -a)" >> "$SUMMARY_FILE"
echo "Rust: $(rustc --version)" >> "$SUMMARY_FILE"
echo "Cargo: $(cargo --version)" >> "$SUMMARY_FILE"
if command -v lscpu &> /dev/null; then
    echo "" >> "$SUMMARY_FILE"
    echo "CPU Information:" >> "$SUMMARY_FILE"
    lscpu | head -20 >> "$SUMMARY_FILE"
fi
echo "\`\`\`" >> "$SUMMARY_FILE"

print_status "Running quick performance test..."

# Quick performance test
cat > /tmp/quick_test.rs << 'EOF'
use std::time::Instant;
use minicache::MiniCache;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("\n=== Quick Performance Test ===");
    
    let cache = MiniCache::new(Duration::from_secs(60));
    
    // Test 1: Sequential writes
    let start = Instant::now();
    for i in 0..10000 {
        cache.set(i, format!("value_{}", i), None).await;
    }
    let write_time = start.elapsed();
    println!("10K sequential writes: {:?} ({:.0} ops/sec)", 
             write_time, 10000.0 / write_time.as_secs_f64());
    
    // Test 2: Sequential reads
    let start = Instant::now();
    for i in 0..10000 {
        let _ = cache.get(&i).await;
    }
    let read_time = start.elapsed();
    println!("10K sequential reads: {:?} ({:.0} ops/sec)", 
             read_time, 10000.0 / read_time.as_secs_f64());
    
    // Test 3: Cache size
    println!("Cache size: {}", cache.len().await);
    
    // Test 4: Memory estimate
    let entry_size = std::mem::size_of::<i32>() + "value_0000".len();
    println!("Estimated memory per entry: ~{} bytes", entry_size);
    println!("Estimated total memory for 10K entries: ~{} KB", 
             (entry_size * 10000) / 1024);
}
EOF

if cargo run --release --bin minicache 2>/dev/null || echo "use minicache::*; $(cat /tmp/quick_test.rs)" | cargo run --release 2>/dev/null; then
    print_success "Quick performance test completed"
else
    # Run it as a script since binary doesn't exist
    echo 'use minicache::MiniCache;
use std::time::{Instant, Duration};

#[tokio::main]
async fn main() {
    println!("\n=== Quick Performance Test ===");
    
    let cache = MiniCache::new(Duration::from_secs(60));
    
    // Test 1: Sequential writes
    let start = Instant::now();
    for i in 0..10000 {
        cache.set(i, format!("value_{}", i), None).await;
    }
    let write_time = start.elapsed();
    println!("10K sequential writes: {:?} ({:.0} ops/sec)", 
             write_time, 10000.0 / write_time.as_secs_f64());
    
    // Test 2: Sequential reads
    let start = Instant::now();
    for i in 0..10000 {
        let _ = cache.get(&i).await;
    }
    let read_time = start.elapsed();
    println!("10K sequential reads: {:?} ({:.0} ops/sec)", 
             read_time, 10000.0 / read_time.as_secs_f64());
    
    println!("Cache size: {}", cache.len().await);
}' > examples/quick_test.rs
    
    if cargo run --release --example quick_test; then
        print_success "Quick performance test completed"
    else
        print_warning "Quick performance test failed"
    fi
fi

# Cleanup
rm -f /tmp/quick_test.rs examples/quick_test.rs 2>/dev/null

echo ""
print_success "Benchmark suite completed!"
echo ""
echo "📊 Results Summary:"
echo "  - Benchmark output: $RESULTS_DIR/benchmark_output_$TIMESTAMP.txt"
echo "  - Memory profile: $RESULTS_DIR/memory_profile_$TIMESTAMP.txt"  
echo "  - Summary report: $SUMMARY_FILE"
echo "  - HTML reports: target/criterion/"
echo ""
echo "🔍 Next steps:"
echo "  1. Open target/criterion/report/index.html for detailed visualizations"
echo "  2. Review the memory profile for memory usage patterns"
echo "  3. Check the summary report for key insights"
echo ""