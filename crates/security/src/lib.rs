//! Package Fast Security Module
//! 
//! This module provides security features for Package Fast including:
//! - Package integrity verification
//! - Vulnerability scanning
//! - Audit trail generation
//! - Runtime protection for package scripts

pub mod integrity;
pub mod vulnerability;
pub mod vuln_db;
pub mod audit;
pub mod runtime;
pub mod sandbox;
pub mod service;
pub mod performance;

// Re-export the main components for easier access
pub use integrity::{verify_package_integrity, IntegrityError};
pub use vulnerability::{scan_for_vulnerabilities, VulnerabilityReport};
pub use audit::{AuditTrail, AuditEvent};
pub use runtime::{RuntimeProtection, RuntimeProtectionError};
pub use sandbox::SandboxRuntimeProtection;
pub use service::SecurityService;
pub use performance::PerformanceMonitor;

/// Security module configuration
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    /// Whether to enable integrity verification
    pub verify_integrity: bool,
    /// Whether to enable vulnerability scanning
    pub scan_vulnerabilities: bool,
    /// Whether to generate audit trails
    pub generate_audit_trail: bool,
    /// Whether to enable runtime protection
    pub enable_runtime_protection: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            verify_integrity: true,
            scan_vulnerabilities: true,
            generate_audit_trail: true,
            enable_runtime_protection: true,
        }
    }
}

/// Main security context that holds configuration and state
#[derive(Debug)]
pub struct SecurityContext {
    config: SecurityConfig,
}

impl SecurityContext {
    /// Create a new security context with default configuration
    pub fn new() -> Self {
        Self {
            config: SecurityConfig::default(),
        }
    }

    /// Create a new security context with custom configuration
    pub fn with_config(config: SecurityConfig) -> Self {
        Self { config }
    }

    /// Get a reference to the current configuration
    pub fn config(&self) -> &SecurityConfig {
        &self.config
    }
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self::new()
    }
}