/**
 * Package Fast Performance Monitoring
 * Tools for monitoring and profiling performance
 */

class PerformanceMonitor {
  constructor(options = {}) {
    this.isEnabled = options.enabled !== false;
    this.metrics = new Map();
    this.timers = new Map();
    this.thresholds = options.thresholds || {
      resolutionTime: 1000, // ms
      installTime: 5000, // ms
      networkRequest: 5000 // ms
    };
  }

  /**
   * Start timing an operation
   * @param {string} operationName - Name of the operation
   * @param {object} context - Optional context data
   */
  startOperation(operationName, context = {}) {
    if (!this.isEnabled) return;
    
    const startTime = process.hrtime.bigint();
    const operationId = `${operationName}-${Date.now()}-${Math.random()}`;
    
    this.timers.set(operationId, {
      name: operationName,
      startTime,
      context
    });
    
    return operationId;
  }

  /**
   * End timing an operation
   * @param {string} operationId - ID returned from startOperation
   */
  endOperation(operationId) {
    if (!this.isEnabled || !this.timers.has(operationId)) return;
    
    const timer = this.timers.get(operationId);
    const endTime = process.hrtime.bigint();
    const duration = Number(endTime - timer.startTime) / 1000000; // Convert to milliseconds
    
    this.timers.delete(operationId);
    
    // Store metrics
    if (!this.metrics.has(timer.name)) {
      this.metrics.set(timer.name, []);
    }
    
    const metric = {
      duration,
      timestamp: new Date().toISOString(),
      context: timer.context
    };
    
    this.metrics.get(timer.name).push(metric);
    
    // Check if this operation exceeded thresholds
    if (this.thresholds[timer.name] && duration > this.thresholds[timer.name]) {
      console.warn(`Performance warning: ${timer.name} took ${duration.toFixed(2)}ms (threshold: ${this.thresholds[timer.name]}ms)`);
    }
    
    return metric;
  }

  /**
   * Record a metric without timing
   * @param {string} metricName - Name of the metric
   * @param {number} value - Metric value
   * @param {object} context - Optional context data
   */
  recordMetric(metricName, value, context = {}) {
    if (!this.isEnabled) return;
    
    if (!this.metrics.has(metricName)) {
      this.metrics.set(metricName, []);
    }
    
    const metric = {
      value,
      timestamp: new Date().toISOString(),
      context
    };
    
    this.metrics.get(metricName).push(metric);
    
    return metric;
  }

  /**
   * Get aggregated metrics for an operation
   * @param {string} operationName - Name of the operation
   * @returns {object} Aggregated metrics
   */
  getAggregatedMetrics(operationName) {
    if (!this.metrics.has(operationName)) {
      return null;
    }
    
    const metrics = this.metrics.get(operationName);
    const durations = metrics.map(m => 'duration' in m ? m.duration : m.value);
    
    return {
      count: metrics.length,
      total: durations.reduce((sum, d) => sum + d, 0),
      average: durations.reduce((sum, d) => sum + d, 0) / durations.length,
      min: Math.min(...durations),
      max: Math.max(...durations),
      median: this.calculateMedian(durations)
    };
  }

  /**
   * Calculate median of an array
   * @param {number[]} values - Array of values
   * @returns {number} Median value
   */
  calculateMedian(values) {
    if (values.length === 0) return 0;
    
    const sorted = [...values].sort((a, b) => a - b);
    const middle = Math.floor(sorted.length / 2);
    
    if (sorted.length % 2 === 0) {
      return (sorted[middle - 1] + sorted[middle]) / 2;
    } else {
      return sorted[middle];
    }
  }

  /**
   * Get all metrics
   * @returns {Map} All collected metrics
   */
  getAllMetrics() {
    const result = {};
    for (const [name, metrics] of this.metrics) {
      result[name] = this.getAggregatedMetrics(name);
    }
    return result;
  }

  /**
   * Reset all metrics
   */
  reset() {
    this.metrics.clear();
    this.timers.clear();
  }

  /**
   * Export metrics to JSON
   * @returns {string} JSON string of all metrics
   */
  exportToJson() {
    const exportData = {
      timestamp: new Date().toISOString(),
      metrics: {}
    };
    
    for (const [name, metrics] of this.metrics) {
      exportData.metrics[name] = metrics;
    }
    
    return JSON.stringify(exportData, null, 2);
  }

  /**
   * Export metrics to CSV format
   * @returns {string} CSV string of all metrics
   */
  exportToCsv() {
    let csv = 'operation,timestamp,duration,value,context\n';
    
    for (const [name, metrics] of this.metrics) {
      for (const metric of metrics) {
        const duration = 'duration' in metric ? metric.duration : '';
        const value = 'value' in metric ? metric.value : '';
        const context = JSON.stringify(metric.context).replace(/"/g, '""');
        
        csv += `"${name}",${metric.timestamp},${duration},${value},"${context}"\n`;
      }
    }
    
    return csv;
  }
}

// Decorator for automatically timing function execution
function timed(target, propertyName, descriptor) {
  const method = descriptor.value;
  
  descriptor.value = function(...args) {
    const monitor = global.performanceMonitor || new PerformanceMonitor();
    const operationId = monitor.startOperation(propertyName);
    
    const result = method.apply(this, args);
    
    if (result && typeof result.then === 'function') {
      // Handle async functions
      return result.then(value => {
        monitor.endOperation(operationId);
        return value;
      }).catch(error => {
        monitor.endOperation(operationId);
        throw error;
      });
    } else {
      // Handle sync functions
      monitor.endOperation(operationId);
      return result;
    }
  };
  
  return descriptor;
}

// Global instance
const performanceMonitor = new PerformanceMonitor();

// Export for use in other modules
module.exports = {
  PerformanceMonitor,
  performanceMonitor,
  timed
};