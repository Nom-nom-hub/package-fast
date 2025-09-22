//! Performance monitoring for security operations
//! 
//! This module provides performance monitoring capabilities for security operations
//! including timing, resource usage tracking, and performance alerts.

use std::collections::HashMap;
use std::time::{Duration, Instant};
use tracing::{info, warn};

/// Performance metric types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MetricType {
    IntegrityVerification,
    VulnerabilityScan,
    AuditTrailGeneration,
    RuntimeProtection,
    SandboxExecution,
}

/// Performance metric data
#[derive(Debug, Clone)]
pub struct PerformanceMetric {
    /// Type of operation
    pub metric_type: MetricType,
    /// Duration of the operation
    pub duration: Duration,
    /// Memory usage in bytes (if available)
    pub memory_usage: Option<u64>,
    /// CPU usage percentage (if available)
    pub cpu_usage: Option<f64>,
    /// Timestamp of the measurement
    pub timestamp: Instant,
}

/// Performance monitoring configuration
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// Whether to enable performance monitoring
    pub enabled: bool,
    /// Thresholds for performance alerts
    pub thresholds: HashMap<MetricType, Duration>,
    /// Whether to log performance metrics
    pub log_metrics: bool,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        let mut thresholds = HashMap::new();
        thresholds.insert(MetricType::IntegrityVerification, Duration::from_millis(100));
        thresholds.insert(MetricType::VulnerabilityScan, Duration::from_secs(5));
        thresholds.insert(MetricType::AuditTrailGeneration, Duration::from_millis(10));
        thresholds.insert(MetricType::RuntimeProtection, Duration::from_millis(50));
        thresholds.insert(MetricType::SandboxExecution, Duration::from_secs(30));
        
        Self {
            enabled: true,
            thresholds,
            log_metrics: true,
        }
    }
}

/// Performance monitoring service
#[derive(Debug)]
pub struct PerformanceMonitor {
    config: PerformanceConfig,
    metrics: Vec<PerformanceMetric>,
}

impl PerformanceMonitor {
    /// Create a new performance monitor with default configuration
    pub fn new() -> Self {
        Self {
            config: PerformanceConfig::default(),
            metrics: Vec::new(),
        }
    }

    /// Create a new performance monitor with custom configuration
    pub fn with_config(config: PerformanceConfig) -> Self {
        Self {
            config,
            metrics: Vec::new(),
        }
    }

    /// Record a performance metric
    pub fn record_metric(&mut self, metric: PerformanceMetric) {
        if !self.config.enabled {
            return;
        }
        
        if self.config.log_metrics {
            info!("Performance metric: {:?} took {:?}", metric.metric_type, metric.duration);
        }
        
        // Check if the operation exceeded the threshold
        if let Some(threshold) = self.config.thresholds.get(&metric.metric_type) {
            if metric.duration > *threshold {
                warn!("Performance alert: {:?} took {:?}, which exceeds threshold of {:?}", 
                      metric.metric_type, metric.duration, threshold);
            }
        }
        
        self.metrics.push(metric);
    }

    /// Start timing an operation
    pub fn start_timing(&self) -> Instant {
        Instant::now()
    }

    /// End timing an operation and record the metric
    pub fn end_timing(&mut self, start: Instant, metric_type: MetricType) {
        let duration = start.elapsed();
        let metric = PerformanceMetric {
            metric_type,
            duration,
            memory_usage: None, // In a real implementation, we would get this from the system
            cpu_usage: None,    // In a real implementation, we would get this from the system
            timestamp: start,
        };
        self.record_metric(metric);
    }

    /// Get all recorded metrics
    pub fn metrics(&self) -> &[PerformanceMetric] {
        &self.metrics
    }

    /// Get metrics for a specific type
    pub fn metrics_for_type(&self, metric_type: &MetricType) -> Vec<&PerformanceMetric> {
        self.metrics
            .iter()
            .filter(|metric| &metric.metric_type == metric_type)
            .collect()
    }

    /// Calculate average duration for a specific metric type
    pub fn average_duration(&self, metric_type: &MetricType) -> Option<Duration> {
        let metrics: Vec<&PerformanceMetric> = self.metrics_for_type(metric_type);
        if metrics.is_empty() {
            return None;
        }
        
        let total_duration: Duration = metrics
            .iter()
            .map(|metric| metric.duration)
            .sum();
            
        Some(total_duration / metrics.len() as u32)
    }

    /// Clear all recorded metrics
    pub fn clear_metrics(&mut self) {
        self.metrics.clear();
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_performance_monitor_creation() {
        let monitor = PerformanceMonitor::new();
        assert!(monitor.config.enabled);
        assert!(monitor.config.log_metrics);
        assert_eq!(monitor.metrics().len(), 0);
    }

    #[test]
    fn test_performance_monitor_with_config() {
        let config = PerformanceConfig {
            enabled: false,
            thresholds: HashMap::new(),
            log_metrics: false,
        };
        
        let monitor = PerformanceMonitor::with_config(config);
        assert!(!monitor.config.enabled);
        assert!(!monitor.config.log_metrics);
    }

    #[test]
    fn test_record_metric() {
        let mut monitor = PerformanceMonitor::new();
        let metric = PerformanceMetric {
            metric_type: MetricType::IntegrityVerification,
            duration: Duration::from_millis(50),
            memory_usage: None,
            cpu_usage: None,
            timestamp: Instant::now(),
        };
        
        monitor.record_metric(metric);
        assert_eq!(monitor.metrics().len(), 1);
    }

    #[test]
    fn test_timing_operations() {
        let mut monitor = PerformanceMonitor::new();
        let start = monitor.start_timing();
        
        // Simulate some work
        thread::sleep(Duration::from_millis(10));
        
        monitor.end_timing(start, MetricType::IntegrityVerification);
        assert_eq!(monitor.metrics().len(), 1);
        
        let metric = &monitor.metrics()[0];
        assert_eq!(metric.metric_type, MetricType::IntegrityVerification);
        assert!(metric.duration >= Duration::from_millis(10));
    }

    #[test]
    fn test_metrics_for_type() {
        let mut monitor = PerformanceMonitor::new();
        
        // Add metrics of different types
        let metric1 = PerformanceMetric {
            metric_type: MetricType::IntegrityVerification,
            duration: Duration::from_millis(50),
            memory_usage: None,
            cpu_usage: None,
            timestamp: Instant::now(),
        };
        
        let metric2 = PerformanceMetric {
            metric_type: MetricType::VulnerabilityScan,
            duration: Duration::from_millis(100),
            memory_usage: None,
            cpu_usage: None,
            timestamp: Instant::now(),
        };
        
        let metric3 = PerformanceMetric {
            metric_type: MetricType::IntegrityVerification,
            duration: Duration::from_millis(75),
            memory_usage: None,
            cpu_usage: None,
            timestamp: Instant::now(),
        };
        
        monitor.record_metric(metric1);
        monitor.record_metric(metric2);
        monitor.record_metric(metric3);
        
        let integrity_metrics = monitor.metrics_for_type(&MetricType::IntegrityVerification);
        assert_eq!(integrity_metrics.len(), 2);
        
        let vulnerability_metrics = monitor.metrics_for_type(&MetricType::VulnerabilityScan);
        assert_eq!(vulnerability_metrics.len(), 1);
    }

    #[test]
    fn test_average_duration() {
        let mut monitor = PerformanceMonitor::new();
        
        // Add metrics
        let metric1 = PerformanceMetric {
            metric_type: MetricType::IntegrityVerification,
            duration: Duration::from_millis(50),
            memory_usage: None,
            cpu_usage: None,
            timestamp: Instant::now(),
        };
        
        let metric2 = PerformanceMetric {
            metric_type: MetricType::IntegrityVerification,
            duration: Duration::from_millis(100),
            memory_usage: None,
            cpu_usage: None,
            timestamp: Instant::now(),
        };
        
        monitor.record_metric(metric1);
        monitor.record_metric(metric2);
        
        let avg_duration = monitor.average_duration(&MetricType::IntegrityVerification);
        assert!(avg_duration.is_some());
        assert_eq!(avg_duration.unwrap(), Duration::from_millis(75));
    }

    #[test]
    fn test_clear_metrics() {
        let mut monitor = PerformanceMonitor::new();
        
        let metric = PerformanceMetric {
            metric_type: MetricType::IntegrityVerification,
            duration: Duration::from_millis(50),
            memory_usage: None,
            cpu_usage: None,
            timestamp: Instant::now(),
        };
        
        monitor.record_metric(metric);
        assert_eq!(monitor.metrics().len(), 1);
        
        monitor.clear_metrics();
        assert_eq!(monitor.metrics().len(), 0);
    }
}