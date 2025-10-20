/**
 * Basic usage example of MiniCache N-API bindings
 * 
 * This example demonstrates the fundamental cache operations:
 * - Creating a cache instance
 * - Setting values with and without TTL
 * - Getting values
 * - Checking cache size and existence
 */

const { JsCache, getInfo } = require('../../index');

async function basicExample() {
  console.log('🚀 MiniCache Basic Example\n');
  
  // Display cache information
  const info = getInfo();
  console.log('📊 Cache Info:', info);
  console.log('');
  
  // Create cache with 5-second cleanup interval
  const cache = new JsCache({ cleanupIntervalMs: 5000 });
  
  console.log('📝 Setting values...');
  
  // Set permanent values (no TTL)
  await cache.set('user:1', 'Alice Smith');
  await cache.set('user:2', 'Bob Johnson');
  await cache.set('config:theme', 'dark');
  
  // Set temporary values with TTL
  await cache.set('session:abc123', 'user1_session', 10000); // 10 seconds
  await cache.set('temp:data', 'temporary_value', 3000);     // 3 seconds
  
  console.log('✅ Values set successfully\n');
  
  // Check cache size
  console.log(`📊 Cache size: ${await cache.size()} entries`);
  console.log(`📊 Cache empty: ${await cache.isEmpty()}`);
  console.log('');
  
  // Get values
  console.log('🔍 Getting values...');
  const user1 = await cache.get('user:1');
  const session = await cache.get('session:abc123');
  const nonExistent = await cache.get('does:not:exist');
  
  console.log(`  user:1 = ${user1}`);
  console.log(`  session:abc123 = ${session}`);
  console.log(`  does:not:exist = ${nonExistent}`);
  console.log('');
  
  // Check existence
  console.log('🔎 Checking existence...');
  console.log(`  user:1 exists: ${await cache.has('user:1')}`);
  console.log(`  temp:data exists: ${await cache.has('temp:data')}`);
  console.log(`  fake:key exists: ${await cache.has('fake:key')}`);
  console.log('');
  
  // List all keys
  console.log('🗂️  Current keys:', await cache.keys());
  console.log('');
  
  // Wait for some values to expire
  console.log('⏰ Waiting 4 seconds for temp:data to expire...');
  await new Promise(resolve => setTimeout(resolve, 4000));
  
  console.log(`🔍 temp:data after expiration: ${await cache.get('temp:data')}`);
  console.log(`📊 Cache size after expiration: ${await cache.size()} entries`);
  console.log('🗂️  Keys after expiration:', await cache.keys());
  console.log('');
  
  // Manual removal
  console.log('🗑️  Removing user:2...');
  await cache.remove('user:2');
  console.log(`📊 Cache size after removal: ${await cache.size()} entries`);
  console.log('🗂️  Keys after removal:', await cache.keys());
  console.log('');
  
  // Clear all
  console.log('🧹 Clearing cache...');
  await cache.clear();
  console.log(`📊 Cache size after clear: ${await cache.size()} entries`);
  console.log(`📊 Cache empty: ${await cache.isEmpty()}`);
  
  console.log('\n✨ Basic example completed!');
}

// Run the example
basicExample().catch(console.error);