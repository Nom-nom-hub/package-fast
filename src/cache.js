/**
 * Package Fast Caching System
 * Multi-level caching implementation for package metadata and binaries
 */

const fs = require('fs').promises;
const path = require('path');
const crypto = require('crypto');
const os = require('os');

class LRUCache {
  constructor(maxSize = 1000) {
    this.maxSize = maxSize;
    this.cache = new Map();
    this.accessOrder = new Set();
  }

  get(key) {
    if (this.cache.has(key)) {
      // Update access order
      this.accessOrder.delete(key);
      this.accessOrder.add(key);
      return this.cache.get(key);
    }
    return undefined;
  }

  set(key, value) {
    // Remove oldest entry if cache is full
    if (this.cache.size >= this.maxSize) {
      const oldestKey = this.accessOrder.values().next().value;
      this.cache.delete(oldestKey);
      this.accessOrder.delete(oldestKey);
    }

    this.cache.set(key, value);
    this.accessOrder.add(key);
  }

  has(key) {
    return this.cache.has(key);
  }

  delete(key) {
    this.accessOrder.delete(key);
    return this.cache.delete(key);
  }

  clear() {
    this.cache.clear();
    this.accessOrder.clear();
  }

  size() {
    return this.cache.size;
  }
}

class FileCache {
  constructor(cacheDir, options = {}) {
    this.cacheDir = cacheDir;
    this.ttl = options.ttl || 5 * 60 * 1000; // 5 minutes default
    this.ensureCacheDir();
  }

  async ensureCacheDir() {
    try {
      await fs.access(this.cacheDir);
    } catch (error) {
      await fs.mkdir(this.cacheDir, { recursive: true });
    }
  }

  async get(key) {
    const cachePath = path.join(this.cacheDir, this.sanitizeKey(key));
    
    try {
      const stat = await fs.stat(cachePath);
      const age = Date.now() - stat.mtime.getTime();
      
      if (age < this.ttl) {
        const data = await fs.readFile(cachePath, 'utf8');
        return JSON.parse(data);
      } else {
        // Expired, remove it
        await fs.unlink(cachePath);
      }
    } catch (error) {
      // File doesn't exist or other error
      return null;
    }
    
    return null;
  }

  async set(key, value) {
    const cachePath = path.join(this.cacheDir, this.sanitizeKey(key));
    const data = JSON.stringify(value);
    // Ensure the directory exists before writing
    // Optimize by checking if directory exists first to avoid unnecessary calls
    try {
      await fs.access(path.dirname(cachePath));
    } catch (error) {
      if (error.code === 'ENOENT') {
        await fs.mkdir(path.dirname(cachePath), { recursive: true });
      } else {
        throw error;
      }
    }
    await fs.writeFile(cachePath, data, 'utf8');
  }

  async has(key) {
    const result = await this.get(key);
    return result !== null;
  }

  async delete(key) {
    const cachePath = path.join(this.cacheDir, this.sanitizeKey(key));
    try {
      await fs.unlink(cachePath);
      return true;
    } catch (error) {
      return false;
    }
  }

  sanitizeKey(key) {
    // Create a safe filename from the key
    return crypto.createHash('md5').update(key).digest('hex') + '.json';
  }
}

class PackageCache {
  constructor(options = {}) {
    this.memoryCache = new LRUCache(options.memoryCacheSize || 1000);
    this.fileCache = new FileCache(
      options.fileCacheDir || path.join(os.homedir(), '.package-fast', 'cache'),
      { ttl: options.fileCacheTTL || 30 * 60 * 1000 } // 30 minutes default
    );
  }

  async get(key) {
    // Check memory cache first (fastest)
    let value = this.memoryCache.get(key);
    if (value !== undefined) {
      return value;
    }

    // Check file cache second
    value = await this.fileCache.get(key);
    if (value !== null) {
      // Promote to memory cache
      this.memoryCache.set(key, value);
      return value;
    }

    return null;
  }

  async set(key, value) {
    // Store in both caches
    this.memoryCache.set(key, value);
    await this.fileCache.set(key, value);
  }

  async has(key) {
    // Check if in either cache
    if (this.memoryCache.has(key)) {
      return true;
    }
    
    return await this.fileCache.has(key);
  }

  async delete(key) {
    // Remove from both caches
    this.memoryCache.delete(key);
    await this.fileCache.delete(key);
  }

  async clear() {
    // Clear both caches
    this.memoryCache.clear();
    // Note: We don't clear the file cache as it might be shared with other processes
  }
}

// Specialized caches for different types of data
class MetadataCache extends PackageCache {
  constructor(options = {}) {
    super({
      memoryCacheSize: options.memoryCacheSize || 500,
      fileCacheDir: options.fileCacheDir || path.join(os.homedir(), '.package-fast', 'metadata'),
      fileCacheTTL: options.ttl || 5 * 60 * 1000 // 5 minutes
    });
  }
}

class PackageBinaryCache extends PackageCache {
  constructor(options = {}) {
    super({
      memoryCacheSize: options.memoryCacheSize || 100, // Binary data is large
      fileCacheDir: options.fileCacheDir || path.join(os.homedir(), '.package-fast', 'packages'),
      fileCacheTTL: options.ttl || 24 * 60 * 60 * 1000 // 24 hours
    });
  }

  async getPackagePath(integrityHash) {
    const cachePath = path.join(this.fileCache.cacheDir, integrityHash);
    try {
      await fs.access(cachePath);
      return cachePath;
    } catch (error) {
      return null;
    }
  }

  async cachePackage(integrityHash, packagePath) {
    const cachePath = path.join(this.fileCache.cacheDir, integrityHash);
    try {
      await fs.access(cachePath);
      // Already cached
      return cachePath;
    } catch (error) {
      // Not cached, copy it
      await fs.mkdir(path.dirname(cachePath), { recursive: true });
      await fs.copyFile(packagePath, cachePath);
      return cachePath;
    }
  }
}

// Global cache instances
const metadataCache = new MetadataCache();
const packageCache = new PackageBinaryCache();

module.exports = {
  LRUCache,
  FileCache,
  PackageCache,
  MetadataCache,
  PackageBinaryCache,
  metadataCache,
  packageCache
};