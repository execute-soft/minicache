/**
 * Advanced usage example of MiniCache N-API bindings
 * 
 * This example demonstrates advanced features:
 * - Batch operations
 * - Concurrent access patterns
 * - Performance testing
 * - TTL behavior
 */

const { JsCache, createCache, defaultCache } = require('../../index');

async function advancedExample() {
  console.log('🚀 MiniCache Advanced Example\n');
  
  // Create cache with fast cleanup for demonstration
  const cache = new JsCache({ cleanupIntervalMs: 1000 });
  
  // Batch operations example
  console.log('📦 Batch Operations Example');
  console.log('Setting multiple values at once...');
  
  await cache.setMany([
    { key: 'product:1', value: 'Laptop', ttlMs: 15000 },
    { key: 'product:2', value: 'Mouse', ttlMs: 20000 },
    { key: 'product:3', value: 'Keyboard' }, // No TTL
    { key: 'category:electronics', value: 'Active' },
    { key: 'inventory:count', value: '150' }
  ]);
  
  console.log(`✅ Set ${await cache.size()} items in batch`);
  
  // Batch retrieval
  const productKeys = ['product:1', 'product:2', 'product:3', 'product:404'];
  const products = await cache.getMany(productKeys);
  
  console.log('📤 Batch retrieval results:');
  products.forEach(({ key, value }) => {
    console.log(`  ${key} = ${value}`);
  });
  console.log('');
  
  // Performance test
  console.log('⚡ Performance Test');
  const iterations = 10000;
  
  console.log(`🏃‍♂️ Running ${iterations} set operations...`);
  const setStart = Date.now();
  
  for (let i = 0; i < iterations; i++) {
    await cache.set(`perf:${i}`, `value_${i}`);
  }
  
  const setTime = Date.now() - setStart;
  console.log(`✅ Set ${iterations} items in ${setTime}ms (${Math.round(iterations / setTime * 1000)} ops/sec)`);
  
  console.log(`🔍 Running ${iterations} get operations...`);
  const getStart = Date.now();
  let hitCount = 0;
  
  for (let i = 0; i < iterations; i++) {
    const value = await cache.get(`perf:${i}`);
    if (value !== null) hitCount++;
  }
  
  const getTime = Date.now() - getStart;
  console.log(`✅ Got ${hitCount}/${iterations} items in ${getTime}ms (${Math.round(iterations / getTime * 1000)} ops/sec)`);
  console.log('');
  
  // Concurrent access simulation
  console.log('🔀 Concurrent Access Simulation');
  
  const concurrentTasks = 100;
  const promises = [];
  
  // Simulate multiple concurrent operations
  for (let i = 0; i < concurrentTasks; i++) {
    promises.push(
      cache.set(`concurrent:${i}`, `worker_${i}_data`, 5000)
    );
  }
  
  console.log(`🚀 Starting ${concurrentTasks} concurrent set operations...`);
  const concurrentStart = Date.now();
  await Promise.all(promises);
  const concurrentTime = Date.now() - concurrentStart;
  
  console.log(`✅ Completed ${concurrentTasks} concurrent operations in ${concurrentTime}ms`);
  console.log(`📊 Final cache size: ${await cache.size()} entries`);
  console.log('');
  
  // TTL behavior demonstration
  console.log('⏰ TTL Behavior Demonstration');
  
  // Set items with different TTL values
  await cache.set('short:life', 'expires_soon', 2000);    // 2 seconds
  await cache.set('medium:life', 'expires_later', 5000);  // 5 seconds
  await cache.set('long:life', 'expires_much_later', 10000); // 10 seconds
  
  console.log('🔄 Monitoring TTL expiration...');
  
  for (let second = 1; second <= 6; second++) {
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    const short = await cache.has('short:life');
    const medium = await cache.has('medium:life');
    const long = await cache.has('long:life');
    
    console.log(`  After ${second}s: short=${short}, medium=${medium}, long=${long}`);
  }
  console.log('');
  
  // Demonstrate different cache creation methods
  console.log('🏗️  Alternative Cache Creation Methods');
  
  // Method 1: Using createCache function
  const cache2 = await createCache({ cleanupIntervalMs: 30000 });
  await cache2.set('method:create', 'created_with_function');
  console.log(`✅ createCache: ${await cache2.get('method:create')}`);
  
  // Method 2: Using defaultCache function
  const cache3 = await defaultCache();
  await cache3.set('method:default', 'created_with_default');
  console.log(`✅ defaultCache: ${await cache3.get('method:default')}`);
  console.log('');
  
  // Memory usage demonstration
  console.log('💾 Memory Usage Demonstration');
  const largeValue = 'x'.repeat(1000); // 1KB string
  const largeItemCount = 1000;
  
  console.log(`📊 Setting ${largeItemCount} items of ${largeValue.length} bytes each...`);
  const memStart = Date.now();
  
  for (let i = 0; i < largeItemCount; i++) {
    await cache.set(`large:${i}`, largeValue);
  }
  
  const memTime = Date.now() - memStart;
  console.log(`✅ Set ${largeItemCount} large items in ${memTime}ms`);
  console.log(`📊 Total cache size: ${await cache.size()} entries`);
  
  // Get all entries
  const allEntries = await cache.entries();
  const totalValueSize = allEntries.reduce((sum, entry) => sum + entry.value.length, 0);
  console.log(`📏 Total value size: ~${Math.round(totalValueSize / 1024)} KB`);
  console.log('');
  
  // Cleanup demonstration
  console.log('🧹 Final Cleanup');
  await cache.clear();
  await cache2.clear();
  await cache3.clear();
  
  console.log('✨ Advanced example completed!');
}

// Run the example
advancedExample().catch(console.error);