use serde::{Deserialize, Serialize};
use serde_json::Value;

use urlencoding::encode;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_node_name;

/// Parameters for creating a vzdump backup.
#[derive(Debug, Clone, Default, Serialize)]
pub struct VzdumpParams {
    /// VM/CT IDs to back up (comma-separated, or `all`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmid: Option<String>,

    /// Storage to use for the backup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,

    /// Backup mode: `snapshot`, `suspend`, `stop`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// Compression: `zstd`, `lzo`, `gzip`, `0` (none).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<String>,

    /// Notification mode.
    #[serde(rename = "notification-mode", skip_serializing_if = "Option::is_none")]
    pub notification_mode: Option<String>,

    /// Mail to address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailto: Option<String>,

    /// Mail notification: `always`, `failure`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailnotification: Option<String>,

    /// Max backup files to keep.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxfiles: Option<u32>,

    /// Prune backups schedule.
    #[serde(rename = "prune-backups", skip_serializing_if = "Option::is_none")]
    pub prune_backups: Option<String>,

    /// Notes template.
    #[serde(rename = "notes-template", skip_serializing_if = "Option::is_none")]
    pub notes_template: Option<String>,

    /// Protected backup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,

    /// Remove old backups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<bool>,

    /// Performance settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance: Option<String>,

    /// Bandwidth limit (KB/s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bwlimit: Option<u64>,

    /// I/O priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ionice: Option<u32>,

    /// Lock wait time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lockwait: Option<u32>,

    /// Stop wait time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopwait: Option<u32>,

    /// Temporary directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmpdir: Option<String>,

    /// Dump directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dumpdir: Option<String>,

    /// Exclude path patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<String>,

    /// Exclude VMs.
    #[serde(rename = "exclude-path", skip_serializing_if = "Option::is_none")]
    pub exclude_path: Option<String>,

    /// Pool to back up.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,

    /// All VMs flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,

    /// Stdout output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout: Option<bool>,

    /// Fleecing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleecing: Option<String>,
}

/// Vzdump default settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VzdumpDefaults {
    /// Default storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,

    /// Default mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// Default compression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<String>,

    /// Default mailto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailto: Option<String>,

    /// Default mail notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailnotification: Option<String>,

    /// Default max files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxfiles: Option<u32>,

    /// Default prune backups.
    #[serde(rename = "prune-backups", skip_serializing_if = "Option::is_none")]
    pub prune_backups: Option<String>,

    /// Default bandwidth limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bwlimit: Option<u64>,

    /// Default ionice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ionice: Option<u32>,

    /// Additional defaults.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

impl ProxmoxClient {
    /// Creates a vzdump backup job.
    pub async fn create_vzdump(&self, node: &str, params: &VzdumpParams) -> Result<String> {
        validate_node_name(node)?;
        self.post_parsed(
            &format!("/nodes/{node}/vzdump"),
            params,
            &format!("node {node} vzdump"),
        )
        .await
    }

    /// Returns vzdump default settings.
    pub async fn get_vzdump_defaults(&self, node: &str) -> Result<VzdumpDefaults> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/vzdump/defaults"),
            &format!("node {node} vzdump defaults"),
        )
        .await
    }

    /// Extracts configuration from a vzdump backup.
    pub async fn extract_vzdump_config(&self, node: &str, volume: &str) -> Result<String> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!(
                "/nodes/{node}/vzdump/extractconfig?volume={}",
                encode(volume)
            ),
            "vzdump extract config",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vzdump_params_serialize() {
        let params = VzdumpParams {
            vmid: Some("100,200".to_string()),
            storage: Some("local".to_string()),
            mode: Some("snapshot".to_string()),
            compress: Some("zstd".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["vmid"], "100,200");
        assert_eq!(json["storage"], "local");
        assert_eq!(json["mode"], "snapshot");
        assert!(json.get("mailto").is_none());
    }

    #[test]
    fn vzdump_defaults_serde_roundtrip() {
        let json = r#"{
            "storage": "local",
            "mode": "snapshot",
            "compress": "zstd",
            "maxfiles": 3,
            "bwlimit": 0,
            "ionice": 7
        }"#;
        let defaults: VzdumpDefaults = serde_json::from_str(json).unwrap();
        assert_eq!(defaults.storage.as_deref(), Some("local"));
        assert_eq!(defaults.mode.as_deref(), Some("snapshot"));
        assert_eq!(defaults.maxfiles, Some(3));

        let serialized = serde_json::to_string(&defaults).unwrap();
        let deserialized: VzdumpDefaults = serde_json::from_str(&serialized).unwrap();
        assert_eq!(defaults, deserialized);
    }

    #[test]
    fn vzdump_params_hyphenated_fields() {
        let params = VzdumpParams {
            notification_mode: Some("auto".to_string()),
            prune_backups: Some("keep-last=3".to_string()),
            notes_template: Some("{{guestname}}".to_string()),
            exclude_path: Some("/tmp".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["notification-mode"], "auto");
        assert_eq!(json["prune-backups"], "keep-last=3");
        assert_eq!(json["notes-template"], "{{guestname}}");
        assert_eq!(json["exclude-path"], "/tmp");
    }
}
