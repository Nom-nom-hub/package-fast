//! Package integrity verification module
//! 
//! This module provides functions for verifying the integrity of packages
//! using cryptographic hashes and digital signatures.

use sha2::{Sha512, Digest};
use anyhow::Result;
use thiserror::Error;
use std::fs;
use std::path::Path;

/// Error types for integrity verification
#[derive(Error, Debug)]
pub enum IntegrityError {
    #[error("Hash mismatch: expected {expected}, got {actual}")]
    HashMismatch { expected: String, actual: String },
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid hash format")]
    InvalidHashFormat,
}

/// Verify the integrity of a package file using SHA-512
/// 
/// # Arguments
/// * `file_path` - Path to the package file to verify
/// * `expected_hash` - Expected SHA-512 hash as a hex string
/// 
/// # Returns
/// * `Ok(())` if the file's hash matches the expected hash
/// * `Err(IntegrityError)` if verification fails
pub fn verify_package_integrity(file_path: &Path, expected_hash: &str) -> Result<(), IntegrityError> {
    // Read the file content
    let content = fs::read(file_path)?;
    
    // Calculate SHA-512 hash of the content
    let mut hasher = Sha512::new();
    hasher.update(&content);
    let calculated_hash = format!("{:x}", hasher.finalize());
    
    // Compare with expected hash
    if calculated_hash == expected_hash {
        Ok(())
    } else {
        Err(IntegrityError::HashMismatch {
            expected: expected_hash.to_string(),
            actual: calculated_hash,
        })
    }
}

/// Calculate the SHA-512 hash of a package file
/// 
/// # Arguments
/// * `file_path` - Path to the package file
/// 
/// # Returns
/// * `Ok(String)` with the SHA-512 hash as a hex string
/// * `Err(IntegrityError)` if reading the file fails
pub fn calculate_package_hash(file_path: &Path) -> Result<String, IntegrityError> {
    // Read the file content
    let content = fs::read(file_path)?;
    
    // Calculate SHA-512 hash of the content
    let mut hasher = Sha512::new();
    hasher.update(&content);
    let hash = format!("{:x}", hasher.finalize());
    
    Ok(hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_calculate_package_hash() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "test content").unwrap();
        
        let hash = calculate_package_hash(file.path()).unwrap();
        assert_eq!(hash.len(), 128); // SHA-512 produces 128 hex characters
    }

    #[test]
    fn test_verify_package_integrity_success() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "test content").unwrap();
        
        let hash = calculate_package_hash(file.path()).unwrap();
        assert!(verify_package_integrity(file.path(), &hash).is_ok());
    }

    #[test]
    fn test_verify_package_integrity_failure() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "test content").unwrap();
        
        let result = verify_package_integrity(file.path(), "invalid_hash");
        assert!(result.is_err());
        
        match result {
            Err(IntegrityError::HashMismatch { .. }) => {},
            _ => panic!("Expected HashMismatch error"),
        }
    }
}