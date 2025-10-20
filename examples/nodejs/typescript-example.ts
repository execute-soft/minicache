/**
 * TypeScript usage example of MiniCache N-API bindings
 * 
 * This example demonstrates TypeScript integration with proper typing.
 * To run this example: npx ts-node typescript-example.ts
 */

import { JsCache, CacheOptions, getInfo, createCache, CacheEntry, SetItem } from '../../index';

interface User {
    id: string;
    name: string;
    email: string;
    lastActive: number;
}

interface Session {
    userId: string;
    token: string;
    expiresAt: number;
}

class UserCacheService {
    private userCache: JsCache;
    private sessionCache: JsCache;

    constructor() {
        // Create separate caches for different data types
        const userCacheOptions: CacheOptions = { cleanupIntervalMs: 60000 }; // 1 minute
        const sessionCacheOptions: CacheOptions = { cleanupIntervalMs: 30000 }; // 30 seconds

        this.userCache = new JsCache(userCacheOptions);
        this.sessionCache = new JsCache(sessionCacheOptions);
    }

    async cacheUser(user: User, ttlMs?: number): Promise<void> {
        const userJson = JSON.stringify(user);
        await this.userCache.set(`user:${user.id}`, userJson, ttlMs);
    }

    async getUser(userId: string): Promise<User | null> {
        const userJson = await this.userCache.get(`user:${userId}`);
        if (userJson === null) return null;

        try {
            return JSON.parse(userJson) as User;
        } catch {
            // Remove corrupted data
            await this.userCache.remove(`user:${userId}`);
            return null;
        }
    }

    async cacheSession(session: Session): Promise<void> {
        const sessionJson = JSON.stringify(session);
        const ttl = session.expiresAt - Date.now();

        if (ttl > 0) {
            await this.sessionCache.set(`session:${session.token}`, sessionJson, ttl);
        }
    }

    async getSession(token: string): Promise<Session | null> {
        const sessionJson = await this.sessionCache.get(`session:${token}`);
        if (sessionJson === null) return null;

        try {
            const session = JSON.parse(sessionJson) as Session;

            // Double-check expiration
            if (session.expiresAt <= Date.now()) {
                await this.sessionCache.remove(`session:${token}`);
                return null;
            }

            return session;
        } catch {
            await this.sessionCache.remove(`session:${token}`);
            return null;
        }
    }

    async getUserStats(): Promise<{ userCount: number; sessionCount: number }> {
        const [userCount, sessionCount] = await Promise.all([
            this.userCache.size(),
            this.sessionCache.size()
        ]);

        return { userCount, sessionCount };
    }

    async clearAll(): Promise<void> {
        await Promise.all([
            this.userCache.clear(),
            this.sessionCache.clear()
        ]);
    }
}

async function typescriptExample(): Promise<void> {
    console.log('🚀 MiniCache TypeScript Example\n');

    // Display cache information with proper typing
    const info = getInfo();
    console.log('📊 Cache Info:');
    console.log(`  Version: ${info.version}`);
    console.log(`  Backend: ${info.backend}`);
    console.log(`  Performance: ${info.performance}`);
    console.log(`  Features: ${info.features.join(', ')}`);
    console.log('');

    // Create the service
    const service = new UserCacheService();

    // Sample data
    const users: User[] = [
        {
            id: '1',
            name: 'Alice Johnson',
            email: 'alice@example.com',
            lastActive: Date.now()
        },
        {
            id: '2',
            name: 'Bob Smith',
            email: 'bob@example.com',
            lastActive: Date.now() - 3600000 // 1 hour ago
        },
        {
            id: '3',
            name: 'Charlie Brown',
            email: 'charlie@example.com',
            lastActive: Date.now() - 86400000 // 1 day ago
        }
    ];

    const sessions: Session[] = [
        {
            userId: '1',
            token: 'token123',
            expiresAt: Date.now() + 1800000 // 30 minutes
        },
        {
            userId: '2',
            token: 'token456',
            expiresAt: Date.now() + 3600000 // 1 hour
        },
        {
            userId: '3',
            token: 'token789',
            expiresAt: Date.now() + 5000 // 5 seconds (for demo)
        }
    ];

    console.log('👥 Caching users...');
    for (const user of users) {
        // Cache active users for longer
        const ttl = user.lastActive > Date.now() - 3600000 ? 600000 : 300000; // 10min vs 5min
        await service.cacheUser(user, ttl);
    }

    console.log('🔑 Caching sessions...');
    for (const session of sessions) {
        await service.cacheSession(session);
    }

    // Display stats
    const stats = await service.getUserStats();
    console.log(`✅ Cached ${stats.userCount} users and ${stats.sessionCount} sessions\n`);

    // Test retrieval with proper typing
    console.log('🔍 Testing retrieval...');

    const user1 = await service.getUser('1');
    if (user1) {
        console.log(`  User 1: ${user1.name} (${user1.email})`);
    }

    const session1 = await service.getSession('token123');
    if (session1) {
        console.log(`  Session token123: User ${session1.userId}, expires ${new Date(session1.expiresAt)}`);
    }

    // Test non-existent data
    const nonExistentUser = await service.getUser('999');
    console.log(`  Non-existent user: ${nonExistentUser}`);
    console.log('');

    // Demonstrate batch operations with typing
    console.log('📦 Batch operations with TypeScript...');

    const cache = await createCache({ cleanupIntervalMs: 10000 });

    const batchItems: SetItem[] = [
        { key: 'config:theme', value: 'dark' },
        { key: 'config:lang', value: 'en', ttlMs: 30000 },
        { key: 'config:timezone', value: 'UTC' }
    ];

    await cache.setMany(batchItems);

    const configKeys = ['config:theme', 'config:lang', 'config:timezone', 'config:missing'];
    const configs: CacheEntry[] = await cache.getMany(configKeys);

    console.log('Configuration entries:');
    configs.forEach(({ key, value }) => {
        console.log(`  ${key} = ${value}`);
    });
    console.log('');

    // Wait for short-lived session to expire
    console.log('⏰ Waiting for short session to expire...');
    await new Promise(resolve => setTimeout(resolve, 6000));

    const expiredSession = await service.getSession('token789');
    console.log(`  Expired session check: ${expiredSession}`);

    const updatedStats = await service.getUserStats();
    console.log(`📊 Updated stats: ${updatedStats.userCount} users, ${updatedStats.sessionCount} sessions\n`);

    // Cleanup
    console.log('🧹 Cleaning up...');
    await service.clearAll();
    await cache.clear();

    console.log('✨ TypeScript example completed!');
}

// Run the example
typescriptExample().catch(console.error);