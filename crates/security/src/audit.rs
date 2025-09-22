//! Audit trail generation module
//! 
//! This module provides functions for generating comprehensive audit trails
//! of package operations for security and compliance purposes.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use tracing::info;

/// Types of audit events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditEventType {
    PackageInstall,
    PackageUninstall,
    PackageUpdate,
    IntegrityCheck,
    VulnerabilityScan,
    RuntimeProtection,
    ConfigurationChange,
}

/// Audit event record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: AuditEventType,
    pub package_name: Option<String>,
    pub package_version: Option<String>,
    pub user: Option<String>,
    pub ip_address: Option<String>,
    pub details: HashMap<String, String>,
    pub success: bool,
    pub error_message: Option<String>,
}

impl AuditEvent {
    /// Create a new audit event
    pub fn new(event_type: AuditEventType) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type,
            package_name: None,
            package_version: None,
            user: None,
            ip_address: None,
            details: HashMap::new(),
            success: true,
            error_message: None,
        }
    }

    /// Set the package name for this event
    pub fn with_package_name(mut self, name: String) -> Self {
        self.package_name = Some(name);
        self
    }

    /// Set the package version for this event
    pub fn with_package_version(mut self, version: String) -> Self {
        self.package_version = Some(version);
        self
    }

    /// Set the user for this event
    pub fn with_user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }

    /// Set the IP address for this event
    pub fn with_ip_address(mut self, ip: String) -> Self {
        self.ip_address = Some(ip);
        self
    }

    /// Add a detail to this event
    pub fn with_detail(mut self, key: String, value: String) -> Self {
        self.details.insert(key, value);
        self
    }

    /// Mark this event as failed with an error message
    pub fn with_error(mut self, error: String) -> Self {
        self.success = false;
        self.error_message = Some(error);
        self
    }
}

/// Audit trail manager
#[derive(Debug)]
pub struct AuditTrail {
    events: Vec<AuditEvent>,
    output_file: Option<String>,
}

impl AuditTrail {
    /// Create a new audit trail
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            output_file: None,
        }
    }

    /// Create a new audit trail with output file
    pub fn with_output_file(output_file: String) -> Self {
        Self {
            events: Vec::new(),
            output_file: Some(output_file),
        }
    }

    /// Add an event to the audit trail
    pub fn add_event(&mut self, event: AuditEvent) -> Result<()> {
        info!("Audit event: {:?}", event);
        self.events.push(event.clone());
        
        // If we have an output file, write the event to it
        if let Some(ref file_path) = self.output_file {
            self.write_event_to_file(&event, file_path)?;
        }
        
        Ok(())
    }

    /// Write an event to the output file
    fn write_event_to_file(&self, event: &AuditEvent, file_path: &str) -> Result<()> {
        let json = serde_json::to_string(event)?;
        
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;
            
        writeln!(file, "{}", json)?;
        
        Ok(())
    }

    /// Get all events in the audit trail
    pub fn events(&self) -> &[AuditEvent] {
        &self.events
    }

    /// Get events filtered by package name
    pub fn events_for_package(&self, package_name: &str) -> Vec<&AuditEvent> {
        self.events
            .iter()
            .filter(|event| {
                event.package_name
                    .as_ref()
                    .map_or(false, |name| name == package_name)
            })
            .collect()
    }

    /// Export the audit trail to a JSON file
    pub fn export_to_json<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = std::fs::File::create(path)?;
        serde_json::to_writer_pretty(file, &self.events)?;
        Ok(())
    }

    /// Export the audit trail to a CSV file
    pub fn export_to_csv<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut writer = csv::Writer::from_path(path)?;
        
        // Write headers
        writer.write_record(&[
            "ID",
            "Timestamp",
            "EventType",
            "PackageName",
            "PackageVersion",
            "User",
            "IPAddress",
            "Success",
            "ErrorMessage",
            "Details",
        ])?;
        
        // Write events
        for event in &self.events {
            let success_str = if event.success { "true" } else { "false" };
            let details_str = serde_json::to_string(&event.details).unwrap_or_default();
            
            writer.write_record(&[
                &event.id,
                &event.timestamp.to_rfc3339(),
                &format!("{:?}", event.event_type),
                event.package_name.as_deref().unwrap_or(""),
                event.package_version.as_deref().unwrap_or(""),
                event.user.as_deref().unwrap_or(""),
                event.ip_address.as_deref().unwrap_or(""),
                success_str,
                event.error_message.as_deref().unwrap_or(""),
                &details_str,
            ])?;
        }
        
        writer.flush()?;
        Ok(())
    }
}

impl Default for AuditTrail {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_audit_event_creation() {
        let event = AuditEvent::new(AuditEventType::PackageInstall)
            .with_package_name("test-package".to_string())
            .with_package_version("1.0.0".to_string())
            .with_user("test-user".to_string())
            .with_detail("source".to_string(), "npm".to_string());

        assert_eq!(event.event_type, AuditEventType::PackageInstall);
        assert_eq!(event.package_name, Some("test-package".to_string()));
        assert_eq!(event.package_version, Some("1.0.0".to_string()));
        assert_eq!(event.user, Some("test-user".to_string()));
        assert_eq!(event.details.get("source"), Some(&"npm".to_string()));
        assert!(event.success);
    }

    #[test]
    fn test_audit_trail_add_event() {
        let mut audit_trail = AuditTrail::new();
        let event = AuditEvent::new(AuditEventType::PackageInstall);

        assert!(audit_trail.add_event(event).is_ok());
        assert_eq!(audit_trail.events().len(), 1);
    }

    #[test]
    fn test_audit_trail_events_for_package() {
        let mut audit_trail = AuditTrail::new();
        
        let event1 = AuditEvent::new(AuditEventType::PackageInstall)
            .with_package_name("package-a".to_string());
        let event2 = AuditEvent::new(AuditEventType::PackageUninstall)
            .with_package_name("package-b".to_string());
        let event3 = AuditEvent::new(AuditEventType::PackageUpdate)
            .with_package_name("package-a".to_string());

        audit_trail.add_event(event1).unwrap();
        audit_trail.add_event(event2).unwrap();
        audit_trail.add_event(event3).unwrap();

        let package_a_events = audit_trail.events_for_package("package-a");
        assert_eq!(package_a_events.len(), 2);
    }

    #[test]
    fn test_audit_trail_export_json() {
        let mut audit_trail = AuditTrail::new();
        let event = AuditEvent::new(AuditEventType::PackageInstall)
            .with_package_name("test-package".to_string());

        audit_trail.add_event(event).unwrap();

        let temp_file = NamedTempFile::new().unwrap();
        assert!(audit_trail.export_to_json(temp_file.path()).is_ok());
    }

    #[test]
    fn test_audit_trail_export_csv() {
        let mut audit_trail = AuditTrail::new();
        let event = AuditEvent::new(AuditEventType::PackageInstall)
            .with_package_name("test-package".to_string());

        audit_trail.add_event(event).unwrap();

        let temp_file = NamedTempFile::new().unwrap();
        assert!(audit_trail.export_to_csv(temp_file.path()).is_ok());
    }
}