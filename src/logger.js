/**
 * Package Fast Logging System
 * Structured logging with multiple levels and output formats
 */

const fs = require('fs').promises;
const path = require('path');
const os = require('os');

class Logger {
  constructor(options = {}) {
    this.level = options.level || 'info';
    this.output = options.output || 'console';
    this.logFile = options.logFile || path.join(os.homedir(), '.package-fast', 'logs', 'package-fast.log');
    this.levels = {
      error: 0,
      warn: 1,
      info: 2,
      debug: 3
    };
  }

  async log(level, message, metadata = {}) {
    // Check if this level should be logged
    if (this.levels[level] > this.levels[this.level]) {
      return;
    }

    const timestamp = new Date().toISOString();
    const logEntry = {
      timestamp,
      level,
      message,
      metadata,
      pid: process.pid
    };

    const formattedMessage = `[${timestamp}] ${level.toUpperCase()}: ${message}`;

    switch (this.output) {
      case 'console':
        if (level === 'error') {
          console.error(formattedMessage);
        } else if (level === 'warn') {
          console.warn(formattedMessage);
        } else {
          console.log(formattedMessage);
        }
        break;
        
      case 'file':
        await this.writeToFile(logEntry);
        break;
        
      case 'both':
        if (level === 'error') {
          console.error(formattedMessage);
        } else if (level === 'warn') {
          console.warn(formattedMessage);
        } else {
          console.log(formattedMessage);
        }
        await this.writeToFile(logEntry);
        break;
    }
  }

  async writeToFile(logEntry) {
    try {
      const logDir = path.dirname(this.logFile);
      await fs.mkdir(logDir, { recursive: true });
      
      const logLine = JSON.stringify(logEntry) + '\n';
      await fs.appendFile(this.logFile, logLine, 'utf8');
    } catch (error) {
      // If we can't write to file, fall back to console
      console.warn('Could not write to log file:', error.message);
    }
  }

  error(message, metadata = {}) {
    return this.log('error', message, metadata);
  }

  warn(message, metadata = {}) {
    return this.log('warn', message, metadata);
  }

  info(message, metadata = {}) {
    return this.log('info', message, metadata);
  }

  debug(message, metadata = {}) {
    return this.log('debug', message, metadata);
  }
}

// Global logger instance
const logger = new Logger();

module.exports = {
  Logger,
  logger
};