# Runtime Protection

The runtime protection module provides security controls for executing package lifecycle scripts and other runtime operations.

## Overview

This module implements multiple layers of runtime protection including sandboxing, resource limits, and access controls to prevent malicious or accidental damage during script execution.

## Usage

### Basic Runtime Protection

```rust
use package_fast_security::runtime::{RuntimeProtection, RuntimeProtectionConfig};

// Create runtime protection with default configuration
let protection = RuntimeProtection::new();

// Check if file system access is allowed
protection.check_filesystem_access(Path::new("/path/to/file"))?;

// Check if network access is allowed
protection.check_network_access("example.com")?;

// Check if process execution is allowed
protection.check_process_execution("npm")?;
```

### Configuring Runtime Protection

```rust
use package_fast_security::runtime::{RuntimeProtection, RuntimeProtectionConfig};
use std::collections::HashSet;

let mut config = RuntimeProtectionConfig::default();
config.restrict_filesystem = true;
config.allowed_directories = [
    "./".to_string(),
    "./node_modules/".to_string(),
].iter().cloned().collect();

let protection = RuntimeProtection::with_config(config);
```

## Protection Features

### File System Access Control

- Restrict file system access to specific directories
- Prevent access to sensitive system files
- Control read/write/execute permissions

### Network Access Control

- Restrict network access to specific hosts
- Prevent unauthorized data exfiltration
- Control protocol and port access

### Process Execution Control

- Restrict execution of specific commands
- Prevent spawning of unauthorized processes
- Control process privileges

### Resource Limits

- Limit execution time to prevent hangs
- Limit memory usage to prevent resource exhaustion
- Limit file descriptor usage

## Sandbox Runtime Protection

For enhanced security, the sandbox module provides advanced sandboxing capabilities:

```rust
use package_fast_security::sandbox::SandboxRuntimeProtection;

let sandbox = SandboxRuntimeProtection::new();
let result = sandbox.execute_sandboxed(
    "npm", 
    &["install".to_string()], 
    Path::new("./")
).await?;
```

## Security Considerations

- Runtime protection should be enabled for all package scripts
- Configure restrictions based on the principle of least privilege
- Regularly review and update protection rules
- Monitor runtime protection violations for potential threats

## Performance

Runtime protection is designed to be lightweight:

- Minimal overhead on script execution
- Efficient access control checks
- Configurable protection levels for different environments
- Asynchronous monitoring to avoid blocking

## Integration

This module integrates with:
- Package lifecycle script execution
- Plugin system for custom runtime policies
- Audit trails for runtime protection events
- Security monitoring and alerting systems