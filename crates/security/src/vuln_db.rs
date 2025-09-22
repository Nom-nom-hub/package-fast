//! Vulnerability database clients
//! 
//! This module provides clients for integrating with various vulnerability databases
//! such as NVD, OSV, and GitHub Advisory Database.

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::info;

/// NVD (National Vulnerability Database) CVE entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvdCve {
    pub id: String,
    pub source_identifier: Option<String>,
    pub published: String,
    pub last_modified: String,
    pub vuln_status: Option<String>,
    pub descriptions: Vec<NvdDescription>,
    pub metrics: Option<NvdMetrics>,
    pub weaknesses: Option<Vec<NvdWeakness>>,
    pub configurations: Option<Vec<NvdConfiguration>>,
    pub references: Vec<NvdReference>,
}

/// NVD CVE description
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvdDescription {
    pub lang: String,
    pub value: String,
}

/// NVD CVE metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvdMetrics {
    #[serde(rename = "cvssMetricV31")]
    pub cvss_metric_v31: Option<Vec<CvssMetricV31>>,
    #[serde(rename = "cvssMetricV2")]
    pub cvss_metric_v2: Option<Vec<CvssMetricV2>>,
}

/// CVSS v3.1 metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CvssMetricV31 {
    pub source: String,
    pub r#type: String,
    pub cvss_data: CvssDataV31,
    pub base_severity: String,
    pub exploitability_score: f64,
    pub impact_score: f64,
}

/// CVSS v3.1 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CvssDataV31 {
    pub version: String,
    pub vector_string: String,
    pub attack_vector: Option<String>,
    pub attack_complexity: Option<String>,
    pub privileges_required: Option<String>,
    pub user_interaction: Option<String>,
    pub scope: Option<String>,
    pub confidentiality_impact: Option<String>,
    pub integrity_impact: Option<String>,
    pub availability_impact: Option<String>,
    pub base_score: f64,
    pub base_severity: String,
}

/// CVSS v2 metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CvssMetricV2 {
    pub source: String,
    pub r#type: String,
    pub cvss_data: CvssDataV2,
    pub base_severity: String,
    pub exploitability_score: f64,
    pub impact_score: f64,
    pub ac_insuf_info: bool,
    pub obtain_all_privilege: bool,
    pub obtain_user_privilege: bool,
    pub obtain_other_privilege: bool,
    pub user_interaction_required: Option<bool>,
}

/// CVSS v2 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CvssDataV2 {
    pub version: String,
    pub vector_string: String,
    pub access_vector: Option<String>,
    pub access_complexity: Option<String>,
    pub authentication: Option<String>,
    pub confidentiality_impact: Option<String>,
    pub integrity_impact: Option<String>,
    pub availability_impact: Option<String>,
    pub base_score: f64,
}

/// NVD weakness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvdWeakness {
    pub source: String,
    pub r#type: String,
    pub description: Vec<NvdDescription>,
}

/// NVD configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvdConfiguration {
    pub nodes: Vec<NvdNode>,
}

/// NVD node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvdNode {
    pub operator: String,
    pub negate: Option<bool>,
    pub cpe_match: Option<Vec<CpeMatch>>,
    pub children: Option<Vec<NvdNode>>,
}

/// CPE match
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpeMatch {
    pub vulnerable: bool,
    pub criteria: String,
    pub version_start_including: Option<String>,
    pub version_start_excluding: Option<String>,
    pub version_end_including: Option<String>,
    pub version_end_excluding: Option<String>,
}

/// NVD reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvdReference {
    pub url: String,
    pub source: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// NVD API response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvdResponse {
    pub results_per_page: u32,
    pub start_index: u32,
    pub total_results: u32,
    pub format: String,
    pub version: String,
    pub timestamp: String,
    pub vulnerabilities: Vec<NvdVulnerability>,
}

/// NVD vulnerability wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvdVulnerability {
    pub cve: NvdCve,
}

/// GitHub Security Advisory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubAdvisory {
    pub database_id: u32,
    pub id: String,
    pub ghsa_id: String,
    pub node_id: String,
    pub url: String,
    pub html_url: String,
    pub identifiers: Vec<Identifier>,
    pub summary: String,
    pub description: String,
    pub severity: String,
    pub author: Author,
    pub publisher: Publisher,
    pub references: Vec<Reference>,
    pub published_at: String,
    pub updated_at: String,
    pub withdrawn_at: Option<String>,
    pub vulnerabilities: Vec<GithubVulnerability>,
    pub cvss: Option<Cvss>,
    pub cwes: Vec<Cwe>,
    pub credits: Vec<Credit>,
}

/// Identifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identifier {
    pub r#type: String,
    pub value: String,
}

/// Author
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub login: String,
    pub id: u32,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub r#type: String,
    pub site_admin: bool,
}

/// Publisher
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Publisher {
    pub login: String,
    pub id: u32,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub r#type: String,
    pub site_admin: bool,
}

/// Reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    pub url: String,
}

/// GitHub Vulnerability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubVulnerability {
    pub package: Package,
    pub severity: String,
    pub vulnerable_version_range: String,
    pub first_patched_version: Option<FirstPatchedVersion>,
}

/// Package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub ecosystem: String,
    pub name: String,
}

/// First Patched Version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirstPatchedVersion {
    pub identifier: String,
}

/// CVSS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cvss {
    pub vector_string: String,
    pub score: f64,
}

/// CWE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cwe {
    pub cwe_id: String,
    pub name: String,
}

/// Credit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credit {
    pub login: String,
    pub r#type: String,
}

/// OSV (Open Source Vulnerabilities) entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsvEntry {
    pub id: String,
    pub summary: Option<String>,
    pub details: Option<String>,
    pub modified: String,
    pub published: Option<String>,
    pub withdrawn: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub related: Option<Vec<String>>,
    pub affected: Option<Vec<OsvAffected>>,
    pub references: Option<Vec<OsvReference>>,
    pub database_specific: Option<serde_json::Value>,
}

/// OSV affected package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsvAffected {
    pub package: OsvPackage,
    pub ranges: Option<Vec<OsvRange>>,
    pub versions: Option<Vec<String>>,
    pub ecosystem_specific: Option<serde_json::Value>,
    pub database_specific: Option<serde_json::Value>,
}

/// OSV package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsvPackage {
    pub name: String,
    pub ecosystem: String,
    pub purl: Option<String>,
}

/// OSV range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsvRange {
    pub r#type: String,
    pub repo: Option<String>,
    pub events: Vec<OsvEvent>,
    pub database_specific: Option<serde_json::Value>,
}

/// OSV event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsvEvent {
    pub introduced: Option<String>,
    pub fixed: Option<String>,
    pub limit: Option<String>,
    pub last_affected: Option<String>,
}

/// OSV reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsvReference {
    pub r#type: String,
    pub url: String,
}

/// Vulnerability database client
#[derive(Debug)]
pub struct VulnerabilityDatabaseClient {
    client: Client,
    nvd_api_key: Option<String>,
    github_token: Option<String>,
}

impl VulnerabilityDatabaseClient {
    /// Create a new vulnerability database client
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("package-fast-security/0.1.0")
            .build()
            .expect("Failed to create HTTP client");
            
        Self {
            client,
            nvd_api_key: None,
            github_token: None,
        }
    }

    /// Create a new vulnerability database client with API keys
    pub fn with_api_keys(nvd_api_key: Option<String>, github_token: Option<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("package-fast-security/0.1.0")
            .build()
            .expect("Failed to create HTTP client");
            
        Self {
            client,
            nvd_api_key,
            github_token,
        }
    }

    /// Query NVD for vulnerabilities affecting a specific package
    pub async fn query_nvd(&self, package_name: &str, version: Option<&str>) -> Result<Vec<NvdVulnerability>> {
        info!("Querying NVD for package: {} version: {:?}", package_name, version);
        
        let mut url = format!("https://services.nvd.nist.gov/rest/json/cves/2.0?keywordSearch={}", package_name);
        
        if let Some(_version) = version {
            url.push_str("&keywordExactMatch");
        }
        
        if let Some(api_key) = &self.nvd_api_key {
            url.push_str(&format!("&apiKey={}", api_key));
        }
        
        let response = self.client.get(&url).send().await?;
        
        if response.status().is_success() {
            let nvd_response: NvdResponse = response.json().await?;
            Ok(nvd_response.vulnerabilities)
        } else {
            anyhow::bail!("Failed to query NVD: HTTP {}", response.status());
        }
    }

    /// Query GitHub Advisory Database for vulnerabilities affecting a specific package
    pub async fn query_github_advisories(&self, package_name: &str, ecosystem: &str) -> Result<Vec<GithubAdvisory>> {
        info!("Querying GitHub Advisory Database for package: {} ecosystem: {}", package_name, ecosystem);
        
        // This is a simplified implementation. In practice, you would use the GitHub GraphQL API
        // or the REST API with proper authentication and pagination.
        Ok(vec![])
    }

    /// Query OSV for vulnerabilities affecting a specific package
    pub async fn query_osv(&self, package_name: &str, ecosystem: &str) -> Result<Vec<OsvEntry>> {
        info!("Querying OSV for package: {} ecosystem: {}", package_name, ecosystem);
        
        // This is a simplified implementation. In practice, you would use the OSV API
        // with proper query parameters.
        Ok(vec![])
    }
}

impl Default for VulnerabilityDatabaseClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = VulnerabilityDatabaseClient::new();
        assert!(client.nvd_api_key.is_none());
        assert!(client.github_token.is_none());
    }

    #[test]
    fn test_client_with_api_keys() {
        let client = VulnerabilityDatabaseClient::with_api_keys(
            Some("nvd-key".to_string()),
            Some("github-token".to_string()),
        );
        assert_eq!(client.nvd_api_key, Some("nvd-key".to_string()));
        assert_eq!(client.github_token, Some("github-token".to_string()));
    }
}