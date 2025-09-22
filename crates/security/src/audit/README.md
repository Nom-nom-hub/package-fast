# Audit Trail Generation

The audit module provides comprehensive logging of security-relevant operations for compliance and forensic purposes.

## Overview

This module generates detailed audit trails of all security operations, including package installations, integrity checks, vulnerability scans, and runtime protection events. Audit trails can be exported in multiple formats for analysis and compliance reporting.

## Usage

### Creating and Using Audit Trails

```rust
use package_fast_security::audit::{AuditTrail, AuditEvent, AuditEventType};

// Create a new audit trail
let mut audit_trail = AuditTrail::new();

// Create audit events
let event = AuditEvent::new(AuditEventType::PackageInstall)
    .with_package_name("package-name".to_string())
    .with_package_version("1.0.0".to_string())
    .with_user("admin".to_string());

// Add events to the audit trail
audit_trail.add_event(event)?;

// Export audit trail
audit_trail.export_to_json("audit-trail.json")?;
audit_trail.export_to_csv("audit-trail.csv")?;
```

### Filtering Audit Events

```rust
use package_fast_security::audit::AuditTrail;

let audit_trail = AuditTrail::new();
// ... add events ...

// Get events for a specific package
let package_events = audit_trail.events_for_package("package-name");
println!("Found {} events for package", package_events.len());
```

## Audit Event Types

The module supports various audit event types:

- `PackageInstall`: Package installation events
- `PackageUninstall`: Package removal events
- `PackageUpdate`: Package update events
- `IntegrityCheck`: Package integrity verification events
- `VulnerabilityScan`: Vulnerability scanning events
- `RuntimeProtection`: Runtime protection events
- `ConfigurationChange`: Security configuration changes

## Audit Event Structure

Each `AuditEvent` contains:

- `id`: Unique identifier for the event
- `timestamp`: When the event occurred
- `event_type`: Type of security event
- `package_name`: Name of the affected package (if applicable)
- `package_version`: Version of the affected package (if applicable)
- `user`: User responsible for the event (if applicable)
- `ip_address`: IP address associated with the event (if applicable)
- `details`: Additional event-specific details
- `success`: Whether the operation was successful
- `error_message`: Error message if the operation failed

## Export Formats

Audit trails can be exported in multiple formats:

1. **JSON**: Detailed structured format for programmatic analysis
2. **CSV**: Tabular format for spreadsheet analysis

## Security Considerations

- Audit trails should be stored securely to prevent tampering
- Consider encrypting audit trails for sensitive environments
- Regularly backup audit trails for compliance purposes
- Monitor audit trails for suspicious activities

## Performance

Audit trail generation is designed to be lightweight:

- Asynchronous file I/O to avoid blocking operations
- Streaming export to handle large audit trails
- Configurable buffering for performance optimization
- Minimal overhead on security operations

## Integration

This module integrates with:
- All security operations for comprehensive logging
- Compliance reporting systems
- Security information and event management (SIEM) systems
- Forensic analysis tools