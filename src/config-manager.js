/**
 * Package Fast Configuration Manager
 * Handles loading and managing configuration for the package manager
 */

const fs = require('fs').promises;
const path = require('path');
const os = require('os');

class ConfigManager {
  constructor(options = {}) {
    this.configPath = options.configPath || this.getDefaultConfigPath();
    this.config = this.getDefaultConfig();
  }

  getDefaultConfigPath() {
    // Check for config file in current directory, then in home directory
    const localConfig = path.join(process.cwd(), '.packagefastrc');
    const homeConfig = path.join(os.homedir(), '.packagefastrc');
    
    // For now, return the default home config path
    return homeConfig;
  }

  getDefaultConfig() {
    return {
      // Network settings
      registry: 'https://registry.npmjs.org',
      timeout: 30000,
      maxConnections: 10,
      
      // Cache settings
      cacheDir: path.join(os.homedir(), '.package-fast'),
      cacheTTL: 30 * 60 * 1000, // 30 minutes
      
      // Parallel processing settings
      concurrency: Math.min(16, require('os').cpus().length + 2),
      
      // Logging settings
      logLevel: 'info',
      
      // Security settings
      strictSSL: true,
      integrityCheck: true,
      
      // Performance settings
      useCache: true,
      useParallel: true,
    };
  }

  async load() {
    try {
      const configData = await fs.readFile(this.configPath, 'utf8');
      const loadedConfig = JSON.parse(configData);
      this.config = { ...this.config, ...loadedConfig };
      return this.config;
    } catch (error) {
      // If config file doesn't exist, use defaults
      if (error.code === 'ENOENT') {
        // Create default config file
        await this.save();
        return this.config;
      }
      throw error;
    }
  }

  async save() {
    try {
      await fs.mkdir(path.dirname(this.configPath), { recursive: true });
      await fs.writeFile(this.configPath, JSON.stringify(this.config, null, 2), 'utf8');
    } catch (error) {
      console.warn('Could not save configuration file:', error.message);
    }
  }

  get(key) {
    return this.config[key];
  }

  set(key, value) {
    this.config[key] = value;
  }

  getAll() {
    return { ...this.config };
  }
}

// Global config manager instance
const configManager = new ConfigManager();

module.exports = {
  ConfigManager,
  configManager
};