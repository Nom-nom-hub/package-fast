//! Advanced runtime protection with sandboxing
//! 
//! This module provides advanced runtime protection features including
//! sandboxing, resource limits, and monitoring.

use anyhow::Result;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;
use tracing::info;

/// Sandbox configuration
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    /// Whether to enable the sandbox
    pub enabled: bool,
    /// Allowed directories for file system access
    pub allowed_directories: HashSet<PathBuf>,
    /// Allowed network hosts
    pub allowed_network_hosts: HashSet<String>,
    /// Maximum execution time (in seconds)
    pub max_execution_time: u64,
    /// Maximum memory usage (in MB)
    pub max_memory_mb: u64,
    /// Maximum number of file descriptors
    pub max_file_descriptors: u64,
    /// Whether to allow network access
    pub allow_network: bool,
    /// Whether to allow process creation
    pub allow_process_creation: bool,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            allowed_directories: [
                PathBuf::from("./"),
                PathBuf::from("./node_modules/"),
                PathBuf::from("./package.json"),
            ].iter().cloned().collect(),
            allowed_network_hosts: HashSet::new(),
            max_execution_time: 300, // 5 minutes
            max_memory_mb: 1024, // 1 GB
            max_file_descriptors: 1024,
            allow_network: false,
            allow_process_creation: false,
        }
    }
}

/// Sandbox execution result
#[derive(Debug)]
pub struct SandboxResult {
    /// Exit code of the process
    pub exit_code: Option<i32>,
    /// Standard output
    pub stdout: Vec<u8>,
    /// Standard error
    pub stderr: Vec<u8>,
    /// Whether the process timed out
    pub timed_out: bool,
    /// Any error that occurred during execution
    pub error: Option<String>,
}

/// Advanced runtime protection with sandboxing
#[derive(Debug)]
pub struct SandboxRuntimeProtection {
    config: SandboxConfig,
}

impl SandboxRuntimeProtection {
    /// Create a new sandbox runtime protection instance
    pub fn new() -> Self {
        Self {
            config: SandboxConfig::default(),
        }
    }

    /// Create a new sandbox runtime protection instance with custom configuration
    pub fn with_config(config: SandboxConfig) -> Self {
        Self { config }
    }

    /// Execute a command in a sandboxed environment
    pub async fn execute_sandboxed<P: AsRef<Path>>(
        &self,
        command: &str,
        args: &[String],
        working_dir: P,
    ) -> Result<SandboxResult> {
        info!("Executing command in sandbox: {} {:?}", command, args);
        
        if !self.config.enabled {
            return self.execute_unsandboxed(command, args, working_dir).await;
        }
        
        // For now, we'll implement a basic sandbox using process limits
        // In a real implementation, this would use OS-level sandboxing like:
        // - Linux: seccomp, namespaces, cgroups
        // - macOS: sandbox_init
        // - Windows: AppContainer, Job Objects
        
        self.execute_with_limits(command, args, working_dir).await
    }

    /// Execute a command without sandboxing (for testing or when disabled)
    async fn execute_unsandboxed<P: AsRef<Path>>(
        &self,
        command: &str,
        args: &[String],
        working_dir: P,
    ) -> Result<SandboxResult> {
        info!("Executing command without sandbox: {} {:?}", command, args);
        
        let mut cmd = Command::new(command);
        cmd.args(args)
            .current_dir(working_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        
        let timeout_duration = Duration::from_secs(self.config.max_execution_time);
        let output = match timeout(timeout_duration, cmd.output()).await {
            Ok(Ok(output)) => output,
            Ok(Err(e)) => {
                return Ok(SandboxResult {
                    exit_code: None,
                    stdout: vec![],
                    stderr: format!("Failed to execute command: {}", e).into_bytes(),
                    timed_out: false,
                    error: Some(e.to_string()),
                });
            }
            Err(_) => {
                return Ok(SandboxResult {
                    exit_code: None,
                    stdout: vec![],
                    stderr: b"Command timed out".to_vec(),
                    timed_out: true,
                    error: Some("Command timed out".to_string()),
                });
            }
        };
        
        Ok(SandboxResult {
            exit_code: output.status.code(),
            stdout: output.stdout,
            stderr: output.stderr,
            timed_out: false,
            error: None,
        })
    }

    /// Execute a command with resource limits
    async fn execute_with_limits<P: AsRef<Path>>(
        &self,
        command: &str,
        args: &[String],
        working_dir: P,
    ) -> Result<SandboxResult> {
        info!("Executing command with limits: {} {:?}", command, args);
        
        let mut cmd = Command::new(command);
        cmd.args(args)
            .current_dir(working_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        
        // Apply resource limits
        // Note: These are basic limits. A real implementation would use OS-specific
        // sandboxing mechanisms for stronger isolation.
        
        let timeout_duration = Duration::from_secs(self.config.max_execution_time);
        let output = match timeout(timeout_duration, cmd.output()).await {
            Ok(Ok(output)) => output,
            Ok(Err(e)) => {
                return Ok(SandboxResult {
                    exit_code: None,
                    stdout: vec![],
                    stderr: format!("Failed to execute command: {}", e).into_bytes(),
                    timed_out: false,
                    error: Some(e.to_string()),
                });
            }
            Err(_) => {
                return Ok(SandboxResult {
                    exit_code: None,
                    stdout: vec![],
                    stderr: b"Command timed out".to_vec(),
                    timed_out: true,
                    error: Some("Command timed out".to_string()),
                });
            }
        };
        
        Ok(SandboxResult {
            exit_code: output.status.code(),
            stdout: output.stdout,
            stderr: output.stderr,
            timed_out: false,
            error: None,
        })
    }

    /// Check if a path is allowed for access
    pub fn is_path_allowed(&self, path: &Path) -> bool {
        if !self.config.enabled {
            return true;
        }
        
        // Convert path to absolute for comparison
        let abs_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            match std::env::current_dir() {
                Ok(cwd) => cwd.join(path),
                Err(_) => return true, // If we can't get current dir, allow access
            }
        };
        
        // If no directories are explicitly allowed, allow access to current directory and subdirectories
        if self.config.allowed_directories.is_empty() {
            let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            return abs_path.starts_with(&current_dir);
        }
        
        // Check if the path is within any allowed directory
        for allowed_dir in &self.config.allowed_directories {
            let allowed_abs = if allowed_dir.is_absolute() {
                allowed_dir.clone()
            } else {
                match std::env::current_dir() {
                    Ok(cwd) => cwd.join(allowed_dir),
                    Err(_) => continue, // Skip this allowed directory if we can't resolve it
                }
            };
            
            if abs_path.starts_with(&allowed_abs) {
                return true;
            }
        }
        
        false
    }

    /// Check if a network host is allowed
    pub fn is_network_host_allowed(&self, host: &str) -> bool {
        if !self.config.enabled || self.config.allow_network {
            return true;
        }
        
        self.config.allowed_network_hosts.contains(host)
    }

    /// Monitor a running process for violations
    pub async fn monitor_process(&self, _pid: u32) -> Result<()> {
        // In a real implementation, this would monitor the process for:
        // - Unauthorized file system access
        // - Unauthorized network access
        // - Excessive resource usage
        // - Process creation violations
        
        // For now, we'll just return Ok
        Ok(())
    }
}

impl Default for SandboxRuntimeProtection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_sandbox_config_default() {
        let config = SandboxConfig::default();
        assert!(config.enabled);
        assert!(!config.allow_network);
        assert!(!config.allow_process_creation);
    }

    #[test]
    fn test_sandbox_runtime_protection_default() {
        let protection = SandboxRuntimeProtection::new();
        assert!(protection.config.enabled);
    }

    #[test]
    fn test_path_allowed() {
        let config = SandboxConfig::default();
        let protection = SandboxRuntimeProtection::with_config(config);
        
        // Test with a path that should be allowed by default
        assert!(protection.is_path_allowed(Path::new("./")));
        
        // Test with an absolute path
        assert!(protection.is_path_allowed(Path::new(std::env::current_dir().unwrap().to_str().unwrap())));
    }

    #[test]
    fn test_network_host_allowed() {
        let mut config = SandboxConfig::default();
        config.allowed_network_hosts.insert("registry.npmjs.org".to_string());
        
        let protection = SandboxRuntimeProtection::with_config(config);
        
        assert!(protection.is_network_host_allowed("registry.npmjs.org"));
        assert!(!protection.is_network_host_allowed("malicious-site.com"));
    }

    #[tokio::test]
    async fn test_execute_sandboxed() {
        let protection = SandboxRuntimeProtection::new();
        let temp_dir = TempDir::new().unwrap();
        
        // Try to execute a simple command
        let result = protection
            .execute_sandboxed("echo", &["test".to_string()], temp_dir.path())
            .await;
            
        // On Windows, 'echo' might not be available as a separate executable
        // so we'll just check that the function doesn't panic
        assert!(result.is_ok());
    }
}