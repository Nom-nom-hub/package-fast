//! Runtime protection module
//! 
//! This module provides functions for protecting the runtime environment
//! when executing package lifecycle scripts.

use anyhow::Result;
use thiserror::Error;
use std::collections::HashSet;
use std::path::Path;
use tracing::info;

/// Error types for runtime protection
#[derive(Error, Debug)]
pub enum RuntimeProtectionError {
    #[error("Script execution blocked: {reason}")]
    ExecutionBlocked { reason: String },
    #[error("File system access violation: {path}")]
    FileSystemViolation { path: String },
    #[error("Network access violation: {host}")]
    NetworkViolation { host: String },
    #[error("Process execution violation: {command}")]
    ProcessViolation { command: String },
}

/// Runtime protection configuration
#[derive(Debug, Clone)]
pub struct RuntimeProtectionConfig {
    /// Whether to enable file system restrictions
    pub restrict_filesystem: bool,
    /// Allowed directories for file system access
    pub allowed_directories: HashSet<String>,
    /// Whether to enable network restrictions
    pub restrict_network: bool,
    /// Allowed hosts for network access
    pub allowed_hosts: HashSet<String>,
    /// Whether to enable process execution restrictions
    pub restrict_process_execution: bool,
    /// Allowed commands for process execution
    pub allowed_commands: HashSet<String>,
    /// Timeout for script execution (in seconds)
    pub execution_timeout: u64,
}

impl Default for RuntimeProtectionConfig {
    fn default() -> Self {
        Self {
            restrict_filesystem: true,
            allowed_directories: [
                "./".to_string(),
                "./node_modules/".to_string(),
                "./package.json".to_string(),
            ].iter().cloned().collect(),
            restrict_network: true,
            allowed_hosts: HashSet::new(),
            restrict_process_execution: true,
            allowed_commands: HashSet::new(),
            execution_timeout: 300, // 5 minutes
        }
    }
}

/// Runtime protection context
#[derive(Debug)]
pub struct RuntimeProtection {
    config: RuntimeProtectionConfig,
}

impl RuntimeProtection {
    /// Create a new runtime protection instance with default configuration
    pub fn new() -> Self {
        Self {
            config: RuntimeProtectionConfig::default(),
        }
    }

    /// Create a new runtime protection instance with custom configuration
    pub fn with_config(config: RuntimeProtectionConfig) -> Self {
        Self { config }
    }

    /// Check if a file system access is allowed
    pub fn check_filesystem_access(&self, path: &Path) -> Result<(), RuntimeProtectionError> {
        if !self.config.restrict_filesystem {
            return Ok(());
        }

        let path_str = path.to_string_lossy();
        
        // Check if the path is in an allowed directory
        for allowed_dir in &self.config.allowed_directories {
            if path_str.starts_with(allowed_dir) {
                return Ok(());
            }
        }
        
        Err(RuntimeProtectionError::FileSystemViolation {
            path: path_str.to_string(),
        })
    }

    /// Check if network access to a host is allowed
    pub fn check_network_access(&self, host: &str) -> Result<(), RuntimeProtectionError> {
        if !self.config.restrict_network {
            return Ok(());
        }

        if self.config.allowed_hosts.contains(host) {
            Ok(())
        } else {
            Err(RuntimeProtectionError::NetworkViolation {
                host: host.to_string(),
            })
        }
    }

    /// Check if process execution is allowed
    pub fn check_process_execution(&self, command: &str) -> Result<(), RuntimeProtectionError> {
        if !self.config.restrict_process_execution {
            return Ok(());
        }

        if self.config.allowed_commands.contains(command) {
            Ok(())
        } else {
            Err(RuntimeProtectionError::ProcessViolation {
                command: command.to_string(),
            })
        }
    }

    /// Execute a script with runtime protection
    pub async fn execute_script<P: AsRef<Path>>(
        &self,
        script_path: P,
        _working_dir: P,
    ) -> Result<std::process::Output, RuntimeProtectionError> {
        info!("Executing script with runtime protection: {:?}", script_path.as_ref());
        
        // In a real implementation, this would:
        // 1. Set up a sandboxed environment
        // 2. Apply the configured restrictions
        // 3. Execute the script with timeout
        // 4. Monitor for violations
        // 5. Return the result
        
        // For now, we'll just return a placeholder result
        // In a real implementation, we would actually execute the script here
        // with all the security protections in place
        
        // Simulate a successful execution
        #[cfg(unix)]
        let status = std::process::ExitStatus::from_raw(0);
        #[cfg(windows)]
        let status = {
            use std::os::windows::process::ExitStatusExt;
            std::process::ExitStatus::from_raw(0)
        };
        
        Ok(std::process::Output {
            status,
            stdout: vec![],
            stderr: vec![],
        })
    }
}

impl Default for RuntimeProtection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_runtime_protection_default_config() {
        let protection = RuntimeProtection::new();
        assert!(protection.config.restrict_filesystem);
        assert!(protection.config.restrict_network);
        assert!(protection.config.restrict_process_execution);
    }

    #[test]
    fn test_filesystem_access_allowed() {
        let protection = RuntimeProtection::new();
        let path = PathBuf::from("./package.json");
        assert!(protection.check_filesystem_access(&path).is_ok());
    }

    #[test]
    fn test_filesystem_access_blocked() {
        let protection = RuntimeProtection::new();
        let path = PathBuf::from("/etc/passwd");
        assert!(protection.check_filesystem_access(&path).is_err());
    }

    #[test]
    fn test_network_access_allowed() {
        let mut config = RuntimeProtectionConfig::default();
        config.allowed_hosts.insert("registry.npmjs.org".to_string());
        
        let protection = RuntimeProtection::with_config(config);
        assert!(protection.check_network_access("registry.npmjs.org").is_ok());
    }

    #[test]
    fn test_network_access_blocked() {
        let protection = RuntimeProtection::new();
        assert!(protection.check_network_access("malicious-site.com").is_err());
    }

    #[test]
    fn test_process_execution_allowed() {
        let mut config = RuntimeProtectionConfig::default();
        config.allowed_commands.insert("npm".to_string());
        
        let protection = RuntimeProtection::with_config(config);
        assert!(protection.check_process_execution("npm").is_ok());
    }

    #[test]
    fn test_process_execution_blocked() {
        let protection = RuntimeProtection::new();
        assert!(protection.check_process_execution("rm").is_err());
    }
}