/**
 * Package Fast HTTP Client
 * Optimized HTTP client with connection pooling and HTTP/2 support
 */

const http = require('http');
const https = require('https');
const { URL } = require('url');

// Try to import http2, but fallback gracefully if not available
let http2;
try {
  http2 = require('http2');
} catch (error) {
  // http2 not available, use http/https only
  http2 = null;
}

class HttpClient {
  constructor(options = {}) {
    this.maxConnectionsPerHost = options.maxConnectionsPerHost || 10;
    this.connectionPool = new Map();
    this.timeout = options.timeout || 30000; // 30 seconds
    this.userAgent = options.userAgent || 'PackageFast/1.0';
    this.useHttp2 = options.useHttp2 !== false; // Enable HTTP/2 by default
    this.enableBatching = options.enableBatching !== false; // Enable batching by default
    this.batchManager = this.enableBatching ? new BatchedRequestManager(this) : null;
  }

  async request(url, options = {}) {
    // Use batching if enabled
    if (this.batchManager) {
      return this.batchManager.request(url, options);
    }
    
    const parsedUrl = new URL(url);
    const hostKey = `${parsedUrl.protocol}//${parsedUrl.host}`;
    
    // Get or create connection pool for host
    if (!this.connectionPool.has(hostKey)) {
      this.connectionPool.set(hostKey, {
        connections: [],
        pending: []
      });
    }
    
    const pool = this.connectionPool.get(hostKey);
    
    // Try to reuse existing connection
    let poolConnection = this.getAvailableConnection(pool, parsedUrl);
    
    if (!poolConnection) {
      // Create new connection if under limit
      if (pool.connections.length < this.maxConnectionsPerHost) {
        const connection = await this.createConnection(parsedUrl);
        poolConnection = {
          connection,
          inUse: true,
          lastUsed: Date.now()
        };
        pool.connections.push(poolConnection);
      } else {
        // Wait for available connection
        const connection = await this.waitForAvailableConnection(pool, parsedUrl);
        poolConnection = {
          connection,
          inUse: true,
          lastUsed: Date.now()
        };
      }
    } else {
      // Mark connection as in use
      poolConnection.inUse = true;
      poolConnection.lastUsed = Date.now();
    }
    
    try {
      // Make request using connection
      const result = await this.makeRequest(poolConnection.connection, parsedUrl, options);
      return result;
    } finally {
      // Mark connection as available
      if (poolConnection) {
        poolConnection.inUse = false;
      }
    }
  }

  getAvailableConnection(pool, parsedUrl) {
    // Find available connection (not in use and not expired)
    const now = Date.now();
    const connection = pool.connections.find(conn => 
      !conn.inUse && (now - conn.lastUsed) < 30000 // 30 second reuse window
    );
    
    return connection;
  }

  async waitForAvailableConnection(pool, parsedUrl) {
    return new Promise((resolve, reject) => {
      const timeout = setTimeout(() => {
        reject(new Error('Timeout waiting for available connection'));
      }, this.timeout);
      
      pool.pending.push({ resolve, reject, timeout, parsedUrl });
    });
  }

  async createConnection(parsedUrl) {
    // For now, we'll use standard HTTP/HTTPS agents with keep-alive
    // A full implementation would add HTTP/2 support
    
    const isHttps = parsedUrl.protocol === 'https:';
    
    // Try HTTP/2 if available and it's an HTTPS connection
    if (http2 && isHttps && this.useHttp2 !== false) {
      try {
        const client = http2.connect(parsedUrl.origin);
        return { client, isHttps, isHttp2: true };
      } catch (error) {
        // Fallback to HTTPS if HTTP/2 fails
        console.warn('HTTP/2 connection failed, falling back to HTTPS:', error.message);
      }
    }
    
    const agent = isHttps ? 
      new https.Agent({
        keepAlive: true,
        maxSockets: this.maxConnectionsPerHost,
        timeout: this.timeout
      }) :
      new http.Agent({
        keepAlive: true,
        maxSockets: this.maxConnectionsPerHost,
        timeout: this.timeout
      });
    
    return { agent, isHttps, isHttp2: false };
  }

  async makeRequest(connection, parsedUrl, options) {
    // Handle HTTP/2 requests
    if (connection.isHttp2) {
      return new Promise((resolve, reject) => {
        const headers = {
          ':method': options.method || 'GET',
          ':path': parsedUrl.pathname + parsedUrl.search,
          'user-agent': this.userAgent,
          'accept': 'application/json',
          ...options.headers
        };
        
        const req = connection.client.request(headers);
        
        let data = '';
        
        req.on('response', (headers, flags) => {
          // Headers received
        });
        
        req.on('data', (chunk) => {
          data += chunk;
        });
        
        req.on('end', () => {
          try {
            const result = {
              statusCode: 200, // HTTP/2 doesn't have status codes in the same way
              headers: {}, // Simplified for this example
              body: data,
              json: data.trim().startsWith('{') || data.trim().startsWith('[') ? 
                JSON.parse(data) : 
                data
            };
            resolve(result);
          } catch (error) {
            reject(error);
          }
        });
        
        req.on('error', reject);
        
        req.setTimeout(this.timeout, () => {
          req.close();
          reject(new Error('Request timeout'));
        });
        
        if (options.body) {
          req.write(options.body);
        }
        
        req.end();
      });
    }
    
    // Handle HTTP/HTTPS requests
    return new Promise((resolve, reject) => {
      const isHttps = connection.isHttps !== undefined ? connection.isHttps : parsedUrl.protocol === 'https:';
      const requestOptions = {
        hostname: parsedUrl.hostname,
        port: parsedUrl.port || (isHttps ? 443 : 80),
        path: parsedUrl.pathname + parsedUrl.search,
        method: options.method || 'GET',
        headers: {
          'User-Agent': this.userAgent,
          'Accept': 'application/json',
          ...options.headers
        },
        agent: connection.agent
      };
      
      const req = (isHttps ? https : http).request(requestOptions, (res) => {
        let data = '';
        
        res.on('data', (chunk) => {
          data += chunk;
        });
        
        res.on('end', () => {
          try {
            const result = {
              statusCode: res.statusCode,
              headers: res.headers,
              body: data,
              json: res.headers['content-type'] && res.headers['content-type'].includes('application/json') ? 
                JSON.parse(data) : 
                data
            };
            resolve(result);
          } catch (error) {
            reject(error);
          }
        });
      });
      
      req.on('error', reject);
      req.on('timeout', () => {
        req.destroy();
        reject(new Error('Request timeout'));
      });
      
      req.setTimeout(this.timeout);
      
      if (options.body) {
        req.write(options.body);
      }
      
      req.end();
    });
  }

  async get(url, options = {}) {
    return this.request(url, { ...options, method: 'GET' });
  }

  async post(url, options = {}) {
    return this.request(url, { ...options, method: 'POST' });
  }

  async put(url, options = {}) {
    return this.request(url, { ...options, method: 'PUT' });
  }

  async delete(url, options = {}) {
    return this.request(url, { ...options, method: 'DELETE' });
  }

  close() {
    // Close all connections
    for (const [hostKey, pool] of this.connectionPool) {
      for (const { connection } of pool.connections) {
        if (connection.agent && connection.agent.destroy) {
          connection.agent.destroy();
        }
      }
    }
    this.connectionPool.clear();
  }
}

// Batched request manager for reducing the number of HTTP requests
class BatchedRequestManager {
  constructor(httpClient, options = {}) {
    this.httpClient = httpClient;
    this.batchInterval = options.batchInterval || 10; // ms
    this.maxBatchSize = options.maxBatchSize || 50;
    this.pendingRequests = [];
    this.requestCache = new Map(); // For deduplication
  }

  async request(url, options = {}) {
    // Check if identical request is already pending
    const cacheKey = `${url}-${JSON.stringify(options)}`;
    if (this.requestCache.has(cacheKey)) {
      return this.requestCache.get(cacheKey);
    }

    // Create new request promise
    const requestPromise = new Promise((resolve, reject) => {
      this.pendingRequests.push({
        url,
        options,
        resolve,
        reject,
        cacheKey
      });
    });

    // Cache promise to deduplicate
    this.requestCache.set(cacheKey, requestPromise);

    // Schedule batch execution
    if (this.pendingRequests.length === 1) {
      setTimeout(() => this.executeBatch(), this.batchInterval);
    } else if (this.pendingRequests.length >= this.maxBatchSize) {
      this.executeBatch();
    }

    return requestPromise;
  }

  async executeBatch() {
    if (this.pendingRequests.length === 0) return;

    const batch = this.pendingRequests.splice(0, this.maxBatchSize);

    try {
      // Try to use registry's bulk API if available
      // For now, we'll execute requests in parallel
      const promises = batch.map(req => 
        this.httpClient.request(req.url, req.options)
          .then(result => ({ success: true, result, request: req }))
          .catch(error => ({ success: false, error, request: req }))
      );

      const results = await Promise.all(promises);

      // Resolve individual promises
      for (const result of results) {
        if (result.success) {
          result.request.resolve(result.result);
        } else {
          result.request.reject(result.error);
        }
        this.requestCache.delete(result.request.cacheKey);
      }
    } catch (error) {
      // Fallback: reject all pending requests
      for (const req of batch) {
        req.reject(error);
        this.requestCache.delete(req.cacheKey);
      }
    }
  }
}

// Global HTTP client instance
const httpClient = new HttpClient();

module.exports = {
  HttpClient,
  BatchedRequestManager,
  httpClient
};