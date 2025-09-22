//! Package Fast Core - Performance-critical components for Package Fast

use anyhow::Result;
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, warn};

/// Package information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub dependencies: HashMap<String, String>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: HashMap<String, String>,
}

impl PackageInfo {
    /// Create a new package info instance
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
        }
    }
}

/// NPM Registry package metadata response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    #[serde(rename = "dist-tags")]
    pub dist_tags: HashMap<String, String>,
    pub versions: HashMap<String, PackageVersion>,
}

/// Package version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageVersion {
    pub name: String,
    pub version: String,
    pub dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<HashMap<String, String>>,
    pub dist: PackageDistribution,
}

/// Package distribution information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDistribution {
    pub tarball: String,
    pub shasum: String,
}

/// Installation options
#[derive(Debug, Clone)]
pub struct InstallOptions {
    pub dev_only: bool,
    pub prod_only: bool,
    pub force: bool,
}

impl Default for InstallOptions {
    fn default() -> Self {
        Self {
            dev_only: false,
            prod_only: false,
            force: false,
        }
    }
}

/// Package installation result
#[derive(Debug, Clone)]
pub struct InstallResult {
    pub installed_packages: Vec<PackageInfo>,
    pub duration: std::time::Duration,
    pub total_size: u64,
}

/// Fetch package metadata from npm registry
pub async fn fetch_package_metadata(name: &str) -> Result<PackageMetadata> {
    let url = format!("https://registry.npmjs.org/{}", name);
    info!("Fetching package metadata from {}", url);
    
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    
    if response.status().is_success() {
        let metadata: PackageMetadata = response.json().await?;
        Ok(metadata)
    } else {
        anyhow::bail!("Failed to fetch package metadata: HTTP {}", response.status());
    }
}

/// Get the latest version of a package
pub async fn get_latest_package_version(name: &str) -> Result<PackageVersion> {
    let metadata = fetch_package_metadata(name).await?;
    
    if let Some(latest_version) = metadata.dist_tags.get("latest") {
        if let Some(version_info) = metadata.versions.get(latest_version) {
            Ok(version_info.clone())
        } else {
            anyhow::bail!("Latest version not found in package metadata");
        }
    } else {
        anyhow::bail!("No latest tag found in package metadata");
    }
}

/// Install packages
pub async fn install_packages(packages: &[String], options: &InstallOptions) -> Result<InstallResult> {
    info!("Installing packages: {:?}", packages);
    
    let start_time = std::time::Instant::now();
    let mut installed_packages = Vec::new();
    
    for package_spec in packages {
        // Parse package name and version (if specified)
        let parts: Vec<&str> = package_spec.split('@').collect();
        let (name, version_req) = if parts.len() == 2 {
            (parts[0], Some(parts[1]))
        } else {
            (package_spec.as_str(), None)
        };
        
        info!("Processing package: {} {:?}", name, version_req);
        
        let version_info = if let Some(version) = version_req {
            let metadata = fetch_package_metadata(name).await?;
            if let Some(version_info) = metadata.versions.get(version) {
                version_info.clone()
            } else {
                warn!("Requested version {} not found for package {}, using latest", version, name);
                get_latest_package_version(name).await?
            }
        } else {
            get_latest_package_version(name).await?
        };
        
        let mut pkg_info = PackageInfo::new(&version_info.name, &version_info.version);
        
        if let Some(deps) = version_info.dependencies {
            pkg_info.dependencies = deps;
        }
        
        if let Some(dev_deps) = version_info.dev_dependencies {
            pkg_info.dev_dependencies = dev_deps;
        }
        
        installed_packages.push(pkg_info);
    }
    
    let duration = start_time.elapsed();
    
    Ok(InstallResult {
        installed_packages,
        duration,
        total_size: 0, // TODO: Calculate actual size
    })
}

/// Install all dependencies from package.json
pub async fn install_all_dependencies(options: &InstallOptions) -> Result<InstallResult> {
    info!("Installing all dependencies from package.json");
    
    // Placeholder implementation
    Ok(InstallResult {
        installed_packages: vec![],
        duration: std::time::Duration::from_secs(0),
        total_size: 0,
    })
}

/// Get package information
pub fn get_package_info(name: &str) -> PackageInfo {
    // Placeholder implementation
    PackageInfo::new(name, "1.0.0")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_info_creation() {
        let pkg = PackageInfo::new("test-package", "1.0.0");
        assert_eq!(pkg.name, "test-package");
        assert_eq!(pkg.version, "1.0.0");
    }

    #[test]
    fn test_get_package_info() {
        let pkg = get_package_info("test-package");
        assert_eq!(pkg.name, "test-package");
        assert_eq!(pkg.version, "1.0.0");
    }
    
    #[tokio::test]
    async fn test_install_packages() {
        let packages = vec!["package-fast-nonexistent-package-12345".to_string()];
        let options = InstallOptions::default();
        let result = install_packages(&packages, &options).await;
        // This should fail because the package doesn't exist
        assert!(result.is_err());
    }
}