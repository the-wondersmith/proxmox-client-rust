use serde::{Deserialize, Serialize};
use serde_json::Value;

use urlencoding::encode;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_node_name;

/// An available APT update.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AptUpdate {
    /// Package name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,

    /// Title / description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Current installed version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_version: Option<String>,

    /// Available version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,

    /// Section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,

    /// Origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,

    /// Architecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
}

/// An APT repository.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AptRepository {
    /// Repository path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// File type.
    #[serde(rename = "FileType", skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,

    /// Whether enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Types (e.g., `deb`, `deb-src`).
    #[serde(rename = "Types", skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,

    /// URIs.
    #[serde(rename = "URIs", skip_serializing_if = "Option::is_none")]
    pub uris: Option<Vec<String>>,

    /// Suites.
    #[serde(rename = "Suites", skip_serializing_if = "Option::is_none")]
    pub suites: Option<Vec<String>>,

    /// Components.
    #[serde(rename = "Components", skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<String>>,
}

/// Package version information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AptPackageVersion {
    /// Package name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,

    /// Installed version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_version: Option<String>,

    /// Available version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Architecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,

    /// Section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,

    /// Priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,

    /// Manager type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,

    /// Running kernel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_kernel: Option<String>,
}

impl ProxmoxClient {
    /// Lists available APT updates on a node.
    pub async fn list_apt_updates(&self, node: &str) -> Result<Vec<AptUpdate>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/apt/update"),
            &format!("node {node} APT updates"),
        )
        .await
    }

    /// Triggers an APT update (package list refresh).
    pub async fn trigger_apt_update(&self, node: &str) -> Result<String> {
        validate_node_name(node)?;
        let response = self
            .post(&format!("/nodes/{node}/apt/update"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("node {node} APT update")).await
    }

    /// Returns the changelog for a package.
    pub async fn get_apt_changelog(&self, node: &str, package: &str) -> Result<String> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/apt/changelog?name={}", encode(package)),
            &format!("package {package} changelog"),
        )
        .await
    }

    /// Lists APT repositories.
    pub async fn list_apt_repositories(&self, node: &str) -> Result<Value> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/apt/repositories"),
            &format!("node {node} APT repositories"),
        )
        .await
    }

    /// Adds an APT repository.
    pub async fn add_apt_repository(&self, node: &str, handle: &str) -> Result<()> {
        validate_node_name(node)?;
        let params = serde_json::json!({ "handle": handle });
        let response = self
            .post(&format!("/nodes/{node}/apt/repositories"))?
            .json(&params)
            .send()
            .await?;
        Self::handle_error(response, &format!("node {node} APT repository")).await?;
        Ok(())
    }

    /// Changes APT repository settings (enable/disable).
    pub async fn change_apt_repository(
        &self,
        node: &str,
        index: u32,
        path: &str,
        enabled: bool,
    ) -> Result<()> {
        validate_node_name(node)?;
        let params = serde_json::json!({
            "index": index,
            "path": path,
            "enabled": enabled,
        });
        let response = self
            .put(&format!("/nodes/{node}/apt/repositories"))?
            .json(&params)
            .send()
            .await?;
        Self::handle_error(response, &format!("node {node} APT repository")).await?;
        Ok(())
    }

    /// Lists package versions on a node.
    pub async fn list_apt_versions(&self, node: &str) -> Result<Vec<AptPackageVersion>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/apt/versions"),
            &format!("node {node} APT versions"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apt_update_serde_roundtrip() {
        let json = r#"{
            "package": "pve-manager",
            "title": "Proxmox VE Manager",
            "old_version": "8.1.3",
            "version": "8.1.4",
            "priority": "optional",
            "section": "admin",
            "origin": "Proxmox"
        }"#;
        let update: AptUpdate = serde_json::from_str(json).unwrap();
        assert_eq!(update.package.as_deref(), Some("pve-manager"));
        assert_eq!(update.version.as_deref(), Some("8.1.4"));

        let serialized = serde_json::to_string(&update).unwrap();
        let deserialized: AptUpdate = serde_json::from_str(&serialized).unwrap();
        assert_eq!(update, deserialized);
    }

    #[test]
    fn apt_repository_serde_roundtrip() {
        let json = r#"{
            "path": "/etc/apt/sources.list",
            "FileType": "list",
            "enabled": true,
            "comment": "Proxmox VE",
            "Types": ["deb"],
            "URIs": ["http://download.proxmox.com/debian/pve"],
            "Suites": ["bookworm"],
            "Components": ["pve-no-subscription"]
        }"#;
        let repo: AptRepository = serde_json::from_str(json).unwrap();
        assert_eq!(repo.path.as_deref(), Some("/etc/apt/sources.list"));
        assert_eq!(repo.file_type.as_deref(), Some("list"));
        assert_eq!(repo.enabled, Some(true));
        assert_eq!(repo.types.as_deref(), Some(&["deb".to_string()][..]));
        assert_eq!(
            repo.uris.as_deref(),
            Some(&["http://download.proxmox.com/debian/pve".to_string()][..])
        );
        assert_eq!(repo.suites.as_deref(), Some(&["bookworm".to_string()][..]));
        assert_eq!(
            repo.components.as_deref(),
            Some(&["pve-no-subscription".to_string()][..])
        );

        let serialized = serde_json::to_string(&repo).unwrap();
        let deserialized: AptRepository = serde_json::from_str(&serialized).unwrap();
        assert_eq!(repo, deserialized);
    }

    #[test]
    fn apt_repository_pascal_case_renames() {
        let repo = AptRepository {
            path: None,
            file_type: Some("list".to_string()),
            enabled: None,
            comment: None,
            types: Some(vec!["deb".to_string()]),
            uris: Some(vec!["http://example.com".to_string()]),
            suites: Some(vec!["stable".to_string()]),
            components: Some(vec!["main".to_string()]),
        };
        let json = serde_json::to_value(&repo).unwrap();
        assert!(
            json.get("FileType").is_some(),
            "should serialize as FileType, not file_type"
        );
        assert!(
            json.get("Types").is_some(),
            "should serialize as Types, not types"
        );
        assert!(
            json.get("URIs").is_some(),
            "should serialize as URIs, not uris"
        );
        assert!(
            json.get("Suites").is_some(),
            "should serialize as Suites, not suites"
        );
        assert!(
            json.get("Components").is_some(),
            "should serialize as Components, not components"
        );
        assert!(json.get("file_type").is_none());
        assert!(json.get("types").is_none());
        assert!(json.get("uris").is_none());
    }
}
