# Security Service

The security service provides a unified interface for all security features in Package Fast.

## Overview

The security service integrates all security components into a single, easy-to-use interface. It provides methods for package integrity verification, vulnerability scanning, audit trail generation, runtime protection, and performance monitoring.

## Usage

### Basic Usage

```rust
use package_fast_security::SecurityService;

// Create a security service
let mut security_service = SecurityService::new();

// Verify package integrity
security_service.verify_package_file_integrity(
    "package-name",
    "1.0.0",
    Path::new("/path/to/package.tgz"),
    "expected-sha512-hash"
).await?;

// Scan for vulnerabilities
let report = security_service.scan_package_for_vulnerabilities(
    "package-name",
    "1.0.0"
).await?;

// Execute a package script with runtime protection
security_service.execute_package_script(
    "package-name",
    "postinstall",
    Path::new("/path/to/script.sh"),
    Path::new("/working/directory")
).await?;
```

### Configuring the Security Service

```rust
use package_fast_security::{SecurityService, SecurityServiceConfig};

let config = SecurityServiceConfig {
    verify_integrity: true,
    scan_vulnerabilities: true,
    generate_audit_trail: true,
    enable_runtime_protection: true,
    audit_trail_file: Some("security-audit.log".to_string()),
};

let mut security_service = SecurityService::with_config(config);
```

### Accessing Audit Trails

```rust
use package_fast_security::SecurityService;

let security_service = SecurityService::new();

// Get the audit trail
let audit_trail = security_service.audit_trail();

// Export audit trail
security_service.export_audit_trail_to_json("audit-trail.json")?;
security_service.export_audit_trail_to_csv("audit-trail.csv")?;
```

### Performance Monitoring

```rust
use package_fast_security::SecurityService;

let security_service = SecurityService::new();

// Access performance metrics
let metrics = security_service.performance_monitor().metrics();
println!("Recorded {} performance metrics", metrics.len());
```

## Security Service Configuration

The `SecurityServiceConfig` allows customization of security features:

- `verify_integrity`: Enable/disable package integrity verification
- `scan_vulnerabilities`: Enable/disable vulnerability scanning
- `generate_audit_trail`: Enable/disable audit trail generation
- `enable_runtime_protection`: Enable/disable runtime protection
- `audit_trail_file`: File path for audit trail output

## Integration with Other Components

The security service is designed to integrate seamlessly with other Package Fast components:

### Package Resolution Engine
- Verify integrity of resolved packages
- Scan dependencies for vulnerabilities

### Fetch and Cache Manager
- Verify integrity of downloaded packages
- Scan cached packages for vulnerabilities

### Installation Executor
- Enforce runtime protection during script execution
- Generate audit trails for installation operations

### Plugin System
- Allow custom security policies
- Extend security features through plugins

## Error Handling

The security service provides comprehensive error handling:

- Specific error types for different failure scenarios
- Detailed error messages for troubleshooting
- Graceful degradation when security features fail

## Performance

The security service is optimized for performance:

- Asynchronous operations to avoid blocking
- Efficient resource usage
- Configurable security levels for different environments
- Performance monitoring for all operations

## Testing

The security service includes comprehensive tests:

- Unit tests for each security feature
- Integration tests for end-to-end workflows
- Performance tests for security operations
- Security tests for protection mechanisms

## Extending the Security Service

To extend the security service:

1. Add new methods to the `SecurityService` implementation
2. Update the performance monitoring to track new operations
3. Add new audit event types for new operations
4. Ensure proper error handling and logging
5. Add comprehensive tests for new functionality