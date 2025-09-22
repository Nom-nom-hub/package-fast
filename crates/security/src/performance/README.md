# Performance Monitoring

The performance monitoring module provides detailed performance metrics and monitoring for security operations.

## Overview

This module tracks the performance of security operations including timing, resource usage, and threshold alerts to ensure security features don't negatively impact system performance.

## Usage

### Basic Performance Monitoring

```rust
use package_fast_security::performance::{PerformanceMonitor, MetricType};

// Create a performance monitor
let mut monitor = PerformanceMonitor::new();

// Time an operation
let start = monitor.start_timing();
// ... perform security operation ...
monitor.end_timing(start, MetricType::IntegrityVerification);
```

### Recording Metrics

```rust
use package_fast_security::performance::{PerformanceMonitor, PerformanceMetric, MetricType};
use std::time::{Duration, Instant};

let mut monitor = PerformanceMonitor::new();

let metric = PerformanceMetric {
    metric_type: MetricType::VulnerabilityScan,
    duration: Duration::from_millis(150),
    memory_usage: Some(50_000_000), // 50 MB
    cpu_usage: Some(25.5), // 25.5%
    timestamp: Instant::now(),
};

monitor.record_metric(metric);
```

### Analyzing Performance Data

```rust
use package_fast_security::performance::{PerformanceMonitor, MetricType};

let monitor = PerformanceMonitor::new();
// ... record metrics ...

// Get average duration for a specific operation
if let Some(avg_duration) = monitor.average_duration(&MetricType::IntegrityVerification) {
    println!("Average integrity verification time: {:?}", avg_duration);
}

// Get all metrics for a specific type
let metrics = monitor.metrics_for_type(&MetricType::VulnerabilityScan);
println!("Recorded {} vulnerability scan metrics", metrics.len());
```

## Metric Types

The module tracks performance for various security operations:

- `IntegrityVerification`: Package integrity verification operations
- `VulnerabilityScan`: Vulnerability scanning operations
- `AuditTrailGeneration`: Audit trail generation operations
- `RuntimeProtection`: Runtime protection operations
- `SandboxExecution`: Sandbox execution operations

## Performance Thresholds

The module can alert when operations exceed configured thresholds:

```rust
use package_fast_security::performance::{PerformanceConfig, MetricType};
use std::collections::HashMap;
use std::time::Duration;

let mut thresholds = HashMap::new();
thresholds.insert(MetricType::IntegrityVerification, Duration::from_millis(100));
thresholds.insert(MetricType::VulnerabilityScan, Duration::from_secs(5));

let config = PerformanceConfig {
    enabled: true,
    thresholds,
    log_metrics: true,
};
```

## Security Considerations

- Performance monitoring should not impact security operations
- Store performance data securely to prevent information leakage
- Monitor performance for potential denial-of-service attacks
- Use performance data to optimize security operations

## Integration

This module integrates with:
- Security service for automatic performance tracking
- Monitoring and alerting systems
- Performance dashboards
- Capacity planning tools