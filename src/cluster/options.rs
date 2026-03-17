use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;

/// Cluster-wide options.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ClusterOptions {
    /// Default keyboard layout for VNC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard: Option<String>,

    /// Default GUI language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Default console viewer (applet, vv, html5, xtermjs).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console: Option<String>,

    /// Email from address for cluster notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_from: Option<String>,

    /// Maximum number of workers for parallel operations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_workers: Option<i64>,

    /// Migration settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration: Option<String>,

    /// HA settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ha: Option<String>,

    /// HTTP proxy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_proxy: Option<String>,

    /// Bandwidth limit configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bwlimit: Option<String>,

    /// MAC address prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_prefix: Option<String>,

    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Fencing mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fencing: Option<String>,

    /// Migration network CIDR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_unsecure: Option<i32>,

    /// Registered user tag style.
    #[serde(rename = "tag-style", skip_serializing_if = "Option::is_none")]
    pub tag_style: Option<String>,

    /// Allowed tags.
    #[serde(rename = "registered-tags", skip_serializing_if = "Option::is_none")]
    pub registered_tags: Option<String>,

    /// User tag access.
    #[serde(rename = "user-tag-access", skip_serializing_if = "Option::is_none")]
    pub user_tag_access: Option<String>,

    /// CRS (Cluster Resource Scheduling) configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crs: Option<String>,

    /// Next VMID.
    #[serde(rename = "next-id", skip_serializing_if = "Option::is_none")]
    pub next_id: Option<String>,

    /// Notify settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<String>,
}

impl ProxmoxClient {
    /// Gets the cluster-wide options.
    ///
    /// `GET /cluster/options`
    pub async fn get_cluster_options(&self) -> Result<ClusterOptions> {
        self.get_parsed("/cluster/options", "cluster options").await
    }

    /// Sets cluster-wide options.
    ///
    /// `PUT /cluster/options`
    pub async fn set_cluster_options(&self, params: &ClusterOptions) -> Result<()> {
        let response = self.put("/cluster/options")?.json(params).send().await?;
        Self::handle_error(response, "cluster options").await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cluster_options_serde_roundtrip() {
        let options = ClusterOptions {
            keyboard: Some("en-us".to_string()),
            language: Some("en".to_string()),
            console: Some("html5".to_string()),
            email_from: Some("admin@example.com".to_string()),
            max_workers: Some(4),
            migration: None,
            ha: None,
            http_proxy: None,
            bwlimit: None,
            mac_prefix: Some("BC:24:11".to_string()),
            description: None,
            fencing: None,
            migration_unsecure: None,
            tag_style: None,
            registered_tags: None,
            user_tag_access: None,
            crs: None,
            next_id: None,
            notify: None,
        };

        let json = serde_json::to_string(&options).unwrap();
        let deserialized: ClusterOptions = serde_json::from_str(&json).unwrap();
        assert_eq!(options, deserialized);
    }

    #[test]
    fn cluster_options_skip_serializing_none() {
        let options = ClusterOptions::default();
        let json = serde_json::to_value(&options).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default ClusterOptions should serialize to {{}}"
        );
    }

    #[test]
    fn cluster_options_deserialize_from_api() {
        let json = r#"{
            "keyboard": "en-us",
            "language": "en",
            "console": "html5",
            "email_from": "admin@example.com",
            "max_workers": 4
        }"#;
        let options: ClusterOptions = serde_json::from_str(json).unwrap();
        assert_eq!(options.keyboard.as_deref(), Some("en-us"));
        assert_eq!(options.max_workers, Some(4));
    }

    #[test]
    fn cluster_options_unknown_fields_ignored() {
        let json = r#"{"keyboard": "en-us", "unknownField": true}"#;
        let options: ClusterOptions = serde_json::from_str(json).unwrap();
        assert_eq!(options.keyboard.as_deref(), Some("en-us"));
    }

    #[test]
    fn cluster_options_hyphenated_fields() {
        let json = r#"{"tag-style": "color-map", "registered-tags": "prod;dev", "next-id": "lower:100,upper:999999999"}"#;
        let options: ClusterOptions = serde_json::from_str(json).unwrap();
        assert_eq!(options.tag_style.as_deref(), Some("color-map"));
        assert_eq!(options.registered_tags.as_deref(), Some("prod;dev"));
        assert_eq!(
            options.next_id.as_deref(),
            Some("lower:100,upper:999999999")
        );

        let serialized = serde_json::to_value(&options).unwrap();
        assert!(serialized.get("tag-style").is_some());
        assert!(serialized.get("tag_style").is_none());
    }
}
