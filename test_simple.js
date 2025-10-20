const { JsCache, getInfo } = require('./index');

async function simpleTest() {
  try {
    console.log('Testing N-API module...');
    
    // Test info function
    console.log('Info:', getInfo());
    
    // Test cache creation
    const cache = new JsCache();
    console.log('Cache created successfully');
    
    // Test basic operations
    await cache.set('test', 'value');
    const result = await cache.get('test');
    console.log('Set and get test:', result);
    
    const size = await cache.size();
    console.log('Cache size:', size);
    
    console.log('✅ All tests passed!');
  } catch (error) {
    console.error('❌ Test failed:', error);
  }
}

simpleTest();