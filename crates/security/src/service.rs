//! Security service that integrates all security components
//! 
//! This module provides a unified interface for all security features
//! including integrity verification, vulnerability scanning, audit trails,
//! and runtime protection.

use anyhow::Result;
use std::path::Path;
use tracing::{info, warn};

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
#[cfg(windows)]
use std::os::windows::process::ExitStatusExt;

use crate::integrity::{verify_package_integrity, calculate_package_hash, IntegrityError};
use crate::vulnerability::{scan_for_vulnerabilities, VulnerabilityReport};
use crate::audit::{AuditTrail, AuditEvent, AuditEventType};
use crate::runtime::{RuntimeProtection, RuntimeProtectionError};
use crate::sandbox::SandboxRuntimeProtection;
use crate::performance::{PerformanceMonitor, MetricType};

/// Security service configuration
#[derive(Debug, Clone)]
pub struct SecurityServiceConfig {
    /// Whether to enable integrity verification
    pub verify_integrity: bool,
    /// Whether to enable vulnerability scanning
    pub scan_vulnerabilities: bool,
    /// Whether to generate audit trails
    pub generate_audit_trail: bool,
    /// Whether to enable runtime protection
    pub enable_runtime_protection: bool,
    /// Audit trail output file (optional)
    pub audit_trail_file: Option<String>,
}

impl Default for SecurityServiceConfig {
    fn default() -> Self {
        Self {
            verify_integrity: true,
            scan_vulnerabilities: true,
            generate_audit_trail: true,
            enable_runtime_protection: true,
            audit_trail_file: None,
        }
    }
}

/// Security service that provides a unified interface for all security features
#[derive(Debug)]
pub struct SecurityService {
    config: SecurityServiceConfig,
    audit_trail: AuditTrail,
    runtime_protection: RuntimeProtection,
    sandbox_protection: SandboxRuntimeProtection,
    performance_monitor: PerformanceMonitor,
}

impl SecurityService {
    /// Create a new security service with default configuration
    pub fn new() -> Self {
        let config = SecurityServiceConfig::default();
        let audit_trail = if let Some(ref file) = config.audit_trail_file {
            AuditTrail::with_output_file(file.clone())
        } else {
            AuditTrail::new()
        };
        
        Self {
            config,
            audit_trail,
            runtime_protection: RuntimeProtection::new(),
            sandbox_protection: SandboxRuntimeProtection::new(),
            performance_monitor: PerformanceMonitor::new(),
        }
    }

    /// Create a new security service with custom configuration
    pub fn with_config(config: SecurityServiceConfig) -> Self {
        let audit_trail = if let Some(ref file) = config.audit_trail_file {
            AuditTrail::with_output_file(file.clone())
        } else {
            AuditTrail::new()
        };
        
        Self {
            config,
            audit_trail,
            runtime_protection: RuntimeProtection::new(),
            sandbox_protection: SandboxRuntimeProtection::new(),
            performance_monitor: PerformanceMonitor::new(),
        }
    }

    /// Verify the integrity of a package file
    pub async fn verify_package_file_integrity(
        &mut self,
        package_name: &str,
        package_version: &str,
        file_path: &Path,
        expected_hash: &str,
    ) -> Result<(), IntegrityError> {
        info!("Verifying integrity of package {}@{}", package_name, package_version);
        
        let start = self.performance_monitor.start_timing();
        
        // Add audit event
        let event = AuditEvent::new(AuditEventType::IntegrityCheck)
            .with_package_name(package_name.to_string())
            .with_package_version(package_version.to_string());
        
        if let Err(e) = self.audit_trail.add_event(event) {
            warn!("Failed to add audit event: {}", e);
        }
        
        // Verify integrity
        let result = verify_package_integrity(file_path, expected_hash);
        
        self.performance_monitor.end_timing(start, MetricType::IntegrityVerification);
        
        result
    }

    /// Calculate the hash of a package file
    pub fn calculate_package_file_hash(&self, file_path: &Path) -> Result<String, IntegrityError> {
        calculate_package_hash(file_path)
    }

    /// Scan a package for vulnerabilities
    pub async fn scan_package_for_vulnerabilities(
        &mut self,
        package_name: &str,
        package_version: &str,
    ) -> Result<VulnerabilityReport, anyhow::Error> {
        info!("Scanning package {}@{} for vulnerabilities", package_name, package_version);
        
        let start = self.performance_monitor.start_timing();
        
        // Add audit event
        let event = AuditEvent::new(AuditEventType::VulnerabilityScan)
            .with_package_name(package_name.to_string())
            .with_package_version(package_version.to_string());
        
        if let Err(e) = self.audit_trail.add_event(event) {
            warn!("Failed to add audit event: {}", e);
        }
        
        // Scan for vulnerabilities
        let result = scan_for_vulnerabilities(package_name, package_version).await;
        
        self.performance_monitor.end_timing(start, MetricType::VulnerabilityScan);
        
        result
    }

    /// Execute a package script with runtime protection
    pub async fn execute_package_script<P: AsRef<Path>>(
        &mut self,
        package_name: &str,
        script_name: &str,
        script_path: P,
        working_dir: P,
    ) -> Result<std::process::Output, RuntimeProtectionError> {
        info!("Executing script '{}' for package {}", script_name, package_name);
        
        let start = self.performance_monitor.start_timing();
        
        // Add audit event
        let event = AuditEvent::new(AuditEventType::RuntimeProtection)
            .with_package_name(package_name.to_string())
            .with_detail("script_name".to_string(), script_name.to_string());
        
        if let Err(e) = self.audit_trail.add_event(event) {
            warn!("Failed to add audit event: {}", e);
        }
        
        // Execute with sandbox protection
        let result = self.sandbox_protection
            .execute_sandboxed(script_path.as_ref().to_str().unwrap_or(""), &[], working_dir)
            .await;
            
        self.performance_monitor.end_timing(start, MetricType::SandboxExecution);
            
        match result {
            Ok(sandbox_result) => {
                // Convert sandbox result to process output
                #[cfg(unix)]
                let status = std::process::ExitStatus::from_raw(0);
                #[cfg(windows)]
                let status = std::process::ExitStatus::from_raw(0);
                
                Ok(std::process::Output {
                    status,
                    stdout: sandbox_result.stdout,
                    stderr: sandbox_result.stderr,
                })
            }
            Err(e) => Err(RuntimeProtectionError::ExecutionBlocked {
                reason: e.to_string(),
            }),
        }
    }

    /// Get the audit trail
    pub fn audit_trail(&self) -> &AuditTrail {
        &self.audit_trail
    }

    /// Export the audit trail to a JSON file
    pub fn export_audit_trail_to_json<P: AsRef<Path>>(&self, path: P) -> Result<(), anyhow::Error> {
        self.audit_trail.export_to_json(path)
    }

    /// Export the audit trail to a CSV file
    pub fn export_audit_trail_to_csv<P: AsRef<Path>>(&self, path: P) -> Result<(), anyhow::Error> {
        self.audit_trail.export_to_csv(path)
    }

    /// Check if a file system access is allowed
    pub fn check_filesystem_access(&self, path: &Path) -> Result<(), RuntimeProtectionError> {
        self.runtime_protection.check_filesystem_access(path)
    }

    /// Check if network access to a host is allowed
    pub fn check_network_access(&self, host: &str) -> Result<(), RuntimeProtectionError> {
        self.runtime_protection.check_network_access(host)
    }

    /// Check if process execution is allowed
    pub fn check_process_execution(&self, command: &str) -> Result<(), RuntimeProtectionError> {
        self.runtime_protection.check_process_execution(command)
    }
}

impl Default for SecurityService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    #[test]
    fn test_security_service_creation() {
        let service = SecurityService::new();
        assert!(service.config.verify_integrity);
        assert!(service.config.scan_vulnerabilities);
        assert!(service.config.generate_audit_trail);
        assert!(service.config.enable_runtime_protection);
    }

    #[test]
    fn test_security_service_with_config() {
        let config = SecurityServiceConfig {
            verify_integrity: false,
            scan_vulnerabilities: false,
            generate_audit_trail: false,
            enable_runtime_protection: false,
            audit_trail_file: Some("test.log".to_string()),
        };
        
        let service = SecurityService::with_config(config);
        assert!(!service.config.verify_integrity);
        assert!(!service.config.scan_vulnerabilities);
        assert!(!service.config.generate_audit_trail);
        assert!(!service.config.enable_runtime_protection);
        assert_eq!(service.config.audit_trail_file, Some("test.log".to_string()));
    }

    #[tokio::test]
    async fn test_calculate_package_file_hash() {
        let service = SecurityService::new();
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "test content").unwrap();
        
        let hash = service.calculate_package_file_hash(file.path()).unwrap();
        assert_eq!(hash.len(), 128); // SHA-512 produces 128 hex characters
    }

    #[tokio::test]
    async fn test_audit_trail_functionality() {
        let mut service = SecurityService::new();
        
        // Add some events
        let event1 = AuditEvent::new(AuditEventType::PackageInstall)
            .with_package_name("test-package".to_string());
        let event2 = AuditEvent::new(AuditEventType::IntegrityCheck)
            .with_package_name("test-package".to_string());
            
        service.audit_trail.add_event(event1).unwrap();
        service.audit_trail.add_event(event2).unwrap();
        
        // Check that events were added
        assert_eq!(service.audit_trail.events().len(), 2);
    }

    #[tokio::test]
    async fn test_filesystem_access_check() {
        let service = SecurityService::new();
        let path = PathBuf::from("./package.json");
        assert!(service.check_filesystem_access(&path).is_ok());
    }
}