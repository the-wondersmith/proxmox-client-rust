use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_storage_id;

/// A Proxmox VE storage configuration entry.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StorageConfig {
    /// Storage ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,

    /// Storage type (e.g., `dir`, `lvm`, `lvmthin`, `zfspool`, `nfs`, `cifs`,
    /// `cephfs`, `rbd`, `iscsi`, `iscsidirect`, `glusterfs`, `pbs`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,

    /// Allowed content types (comma-separated, e.g., `images,iso,backup,rootdir,vztmpl,snippets`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Comma-separated list of nodes where this storage is available (empty = all).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<String>,

    /// Whether the storage is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,

    /// Whether the storage is shared across nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub shared: Option<bool>,

    /// File-system path (for `dir`, and some other types).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Server address (for `nfs`, `cifs`, `iscsi`, `pbs`, `glusterfs`, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,

    /// NFS export path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export: Option<String>,

    /// ZFS/RBD/LVM pool name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,

    /// Volume group name (for `lvm`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vgname: Option<String>,

    /// Thin pool name (for `lvmthin`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinpool: Option<String>,

    /// Datastore name (for `pbs`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datastore: Option<String>,

    /// Username for authentication (for `pbs`, `cifs`, `iscsi`, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// CIFS share name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share: Option<String>,

    /// iSCSI portal address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal: Option<String>,

    /// iSCSI target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Maximum number of protected backups per guest (for `pbs`, backup-capable storage).
    #[serde(
        rename = "max-protected-backups",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_protected_backups: Option<i32>,

    /// Preallocation mode (for block-based storage).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preallocation: Option<String>,

    /// Additional storage-type-specific fields not explicitly modelled.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Summary of content types available on a storage.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StorageContentSummary {
    /// Content type (e.g., `images`, `iso`, `backup`, `rootdir`, `vztmpl`, `snippets`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Volume format (e.g., `raw`, `qcow2`, `vmdk`, `subvol`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// Volume identifier / path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volid: Option<String>,

    /// Volume size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,

    /// Used space in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<u64>,

    /// Associated VMID, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmid: Option<u32>,
}

/// Parameters for creating a new storage configuration.
#[derive(Clone, Default, Serialize)]
#[non_exhaustive]
pub struct StorageCreateParams {
    /// Storage ID (required).
    pub storage: String,

    /// Storage type (required, e.g., `dir`, `nfs`, `lvm`, `lvmthin`, `zfspool`, `rbd`, `pbs`).
    #[serde(rename = "type")]
    pub storage_type: String,

    /// Allowed content types (comma-separated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Comma-separated list of nodes where this storage is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<String>,

    /// Whether the storage is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,

    /// Whether the storage is shared across nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub shared: Option<bool>,

    /// File-system path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Server address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,

    /// NFS export path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export: Option<String>,

    /// ZFS/RBD/LVM pool name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,

    /// Volume group name (for `lvm`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vgname: Option<String>,

    /// Thin pool name (for `lvmthin`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinpool: Option<String>,

    /// Datastore name (for `pbs`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datastore: Option<String>,

    /// Username for authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Password for authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// CIFS share name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share: Option<String>,

    /// iSCSI portal address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal: Option<String>,

    /// iSCSI target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Preallocation mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preallocation: Option<String>,
}

impl StorageCreateParams {
    /// Creates new storage creation parameters with the given storage ID and type.
    pub fn new(storage: impl Into<String>, storage_type: impl Into<String>) -> Self {
        Self {
            storage: storage.into(),
            storage_type: storage_type.into(),
            ..Default::default()
        }
    }
}

impl fmt::Debug for StorageCreateParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StorageCreateParams")
            .field("storage", &self.storage)
            .field("type", &self.storage_type)
            .field("content", &self.content)
            .field("nodes", &self.nodes)
            .field("disable", &self.disable)
            .field("shared", &self.shared)
            .field("path", &self.path)
            .field("server", &self.server)
            .field("export", &self.export)
            .field("pool", &self.pool)
            .field("vgname", &self.vgname)
            .field("thinpool", &self.thinpool)
            .field("datastore", &self.datastore)
            .field("username", &self.username)
            .field("password", &self.password.as_ref().map(|_| "<redacted>"))
            .field("share", &self.share)
            .field("portal", &self.portal)
            .field("target", &self.target)
            .field("preallocation", &self.preallocation)
            .finish()
    }
}

/// Parameters for updating an existing storage configuration.
#[derive(Clone, Default, Serialize)]
#[non_exhaustive]
pub struct StorageUpdateParams {
    /// Allowed content types (comma-separated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Comma-separated list of nodes where this storage is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<String>,

    /// Whether the storage is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,

    /// Whether the storage is shared across nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub shared: Option<bool>,

    /// File-system path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Server address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,

    /// NFS export path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export: Option<String>,

    /// ZFS/RBD/LVM pool name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,

    /// Volume group name (for `lvm`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vgname: Option<String>,

    /// Thin pool name (for `lvmthin`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinpool: Option<String>,

    /// Datastore name (for `pbs`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datastore: Option<String>,

    /// Username for authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Password for authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// CIFS share name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share: Option<String>,

    /// iSCSI portal address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal: Option<String>,

    /// iSCSI target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Preallocation mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preallocation: Option<String>,

    /// Comma-separated list of properties to delete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<String>,
}

impl fmt::Debug for StorageUpdateParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StorageUpdateParams")
            .field("content", &self.content)
            .field("nodes", &self.nodes)
            .field("disable", &self.disable)
            .field("shared", &self.shared)
            .field("path", &self.path)
            .field("server", &self.server)
            .field("export", &self.export)
            .field("pool", &self.pool)
            .field("vgname", &self.vgname)
            .field("thinpool", &self.thinpool)
            .field("datastore", &self.datastore)
            .field("username", &self.username)
            .field("password", &self.password.as_ref().map(|_| "<redacted>"))
            .field("share", &self.share)
            .field("portal", &self.portal)
            .field("target", &self.target)
            .field("preallocation", &self.preallocation)
            .field("delete", &self.delete)
            .finish()
    }
}

impl ProxmoxClient {
    /// Lists all storage configurations.
    ///
    /// `GET /storage`
    pub async fn list_storage(&self) -> Result<Vec<StorageConfig>> {
        self.get_parsed("/storage", "storage").await
    }

    /// Creates a new storage configuration.
    ///
    /// `POST /storage`
    pub async fn create_storage(&self, params: &StorageCreateParams) -> Result<()> {
        validate_storage_id(&params.storage)?;
        let response = self.post("/storage")?.json(params).send().await?;
        Self::handle_error(response, "storage creation").await?;
        Ok(())
    }

    /// Gets a single storage configuration by storage ID.
    ///
    /// `GET /storage/{storage}`
    pub async fn get_storage(&self, storage: &str) -> Result<StorageConfig> {
        validate_storage_id(storage)?;
        self.get_parsed(
            &format!("/storage/{storage}"),
            &format!("storage {storage}"),
        )
        .await
    }

    /// Updates an existing storage configuration.
    ///
    /// `PUT /storage/{storage}`
    pub async fn update_storage(&self, storage: &str, params: &StorageUpdateParams) -> Result<()> {
        validate_storage_id(storage)?;
        let response = self
            .put(&format!("/storage/{storage}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("storage {storage}")).await?;
        Ok(())
    }

    /// Deletes a storage configuration.
    ///
    /// `DELETE /storage/{storage}`
    pub async fn delete_storage(&self, storage: &str) -> Result<()> {
        validate_storage_id(storage)?;
        self.delete_void(
            &format!("/storage/{storage}"),
            &format!("storage {storage}"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_config_serde_roundtrip() {
        let config = StorageConfig {
            storage: Some("local".to_string()),
            storage_type: Some("dir".to_string()),
            content: Some("images,iso,backup".to_string()),
            nodes: None,
            disable: Some(false),
            shared: Some(false),
            path: Some("/var/lib/vz".to_string()),
            server: None,
            export: None,
            pool: None,
            vgname: None,
            thinpool: None,
            datastore: None,
            username: None,
            share: None,
            portal: None,
            target: None,
            max_protected_backups: None,
            preallocation: None,
            extra: HashMap::new(),
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: StorageConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config, deserialized);
    }

    #[test]
    fn storage_config_skip_serializing_none() {
        let config = StorageConfig::default();
        let json = serde_json::to_value(&config).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default StorageConfig should serialize to {{}}"
        );
    }

    #[test]
    fn storage_config_type_field_rename() {
        let config = StorageConfig {
            storage_type: Some("nfs".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_value(&config).unwrap();
        assert!(
            json.get("type").is_some(),
            "type field must serialize as 'type'"
        );
    }

    #[test]
    fn storage_config_deserialize_dir() {
        let json = r#"{
            "storage": "local",
            "type": "dir",
            "content": "images,iso,backup,vztmpl,snippets",
            "path": "/var/lib/vz",
            "shared": 0,
            "disable": 0
        }"#;
        let config: StorageConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.storage.as_deref(), Some("local"));
        assert_eq!(config.storage_type.as_deref(), Some("dir"));
        assert_eq!(config.path.as_deref(), Some("/var/lib/vz"));
        assert_eq!(config.shared, Some(false));
    }

    #[test]
    fn storage_config_deserialize_nfs() {
        let json = r#"{
            "storage": "nfs-share",
            "type": "nfs",
            "content": "images,iso",
            "server": "192.168.1.100",
            "export": "/mnt/data",
            "shared": 1,
            "nodes": "pve1,pve2"
        }"#;
        let config: StorageConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.storage.as_deref(), Some("nfs-share"));
        assert_eq!(config.storage_type.as_deref(), Some("nfs"));
        assert_eq!(config.server.as_deref(), Some("192.168.1.100"));
        assert_eq!(config.export.as_deref(), Some("/mnt/data"));
        assert_eq!(config.shared, Some(true));
        assert_eq!(config.nodes.as_deref(), Some("pve1,pve2"));
    }

    #[test]
    fn storage_config_deserialize_with_extra_fields() {
        let json = r#"{
            "storage": "ceph-pool",
            "type": "rbd",
            "content": "images,rootdir",
            "pool": "rbd-pool",
            "monhost": "10.0.0.1,10.0.0.2,10.0.0.3",
            "krbd": 1
        }"#;
        let config: StorageConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.storage.as_deref(), Some("ceph-pool"));
        assert_eq!(config.storage_type.as_deref(), Some("rbd"));
        assert_eq!(config.pool.as_deref(), Some("rbd-pool"));
        assert_eq!(
            config.extra.get("monhost").and_then(|v| v.as_str()),
            Some("10.0.0.1,10.0.0.2,10.0.0.3")
        );
        assert_eq!(config.extra.get("krbd").and_then(|v| v.as_i64()), Some(1));
    }

    #[test]
    fn storage_config_max_protected_backups_rename() {
        let json = r#"{
            "storage": "pbs-store",
            "type": "pbs",
            "max-protected-backups": 5
        }"#;
        let config: StorageConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.max_protected_backups, Some(5));

        let serialized = serde_json::to_value(&config).unwrap();
        assert!(
            serialized.get("max-protected-backups").is_some(),
            "must serialize as max-protected-backups"
        );
        assert!(
            serialized.get("max_protected_backups").is_none(),
            "must not serialize as max_protected_backups"
        );
    }

    #[test]
    fn storage_config_unknown_fields_captured() {
        let json = r#"{
            "storage": "test",
            "type": "dir",
            "some_custom_field": "custom_value"
        }"#;
        let config: StorageConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.storage.as_deref(), Some("test"));
        assert_eq!(
            config
                .extra
                .get("some_custom_field")
                .and_then(|v| v.as_str()),
            Some("custom_value")
        );
    }

    #[test]
    fn storage_content_summary_serde_roundtrip() {
        let summary = StorageContentSummary {
            content: Some("images".to_string()),
            format: Some("qcow2".to_string()),
            volid: Some("local:100/vm-100-disk-0.qcow2".to_string()),
            size: Some(10_737_418_240),
            used: Some(2_147_483_648),
            vmid: Some(100),
        };

        let json = serde_json::to_string(&summary).unwrap();
        let deserialized: StorageContentSummary = serde_json::from_str(&json).unwrap();
        assert_eq!(summary, deserialized);
    }

    #[test]
    fn storage_content_summary_skip_serializing_none() {
        let summary = StorageContentSummary::default();
        let json = serde_json::to_value(&summary).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default StorageContentSummary should serialize to {{}}"
        );
    }

    #[test]
    fn storage_content_summary_deserialize_from_api() {
        let json = r#"{
            "content": "iso",
            "format": "iso",
            "volid": "local:iso/debian-12.iso",
            "size": 654311424
        }"#;
        let summary: StorageContentSummary = serde_json::from_str(json).unwrap();
        assert_eq!(summary.content.as_deref(), Some("iso"));
        assert_eq!(summary.format.as_deref(), Some("iso"));
        assert_eq!(summary.volid.as_deref(), Some("local:iso/debian-12.iso"));
        assert_eq!(summary.size, Some(654_311_424));
        assert_eq!(summary.vmid, None);
    }

    #[test]
    fn storage_create_params_serialization() {
        let mut params = StorageCreateParams::new("new-nfs", "nfs");
        params.content = Some("images,iso".to_string());
        params.server = Some("10.0.0.1".to_string());
        params.export = Some("/data".to_string());
        params.shared = Some(true);
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["storage"], "new-nfs");
        assert_eq!(json["type"], "nfs");
        assert_eq!(json["content"], "images,iso");
        assert_eq!(json["server"], "10.0.0.1");
        assert_eq!(json["export"], "/data");
        assert_eq!(json["shared"], 1);
        assert!(!json.as_object().unwrap().contains_key("path"));
        assert!(!json.as_object().unwrap().contains_key("pool"));
    }

    #[test]
    fn storage_create_params_type_rename() {
        let params = StorageCreateParams::new("test", "dir");
        let json = serde_json::to_value(&params).unwrap();
        assert!(
            json.get("type").is_some(),
            "type field must serialize as 'type'"
        );
    }

    #[test]
    fn storage_create_params_minimal() {
        let params = StorageCreateParams::new("minimal", "dir");
        let json = serde_json::to_value(&params).unwrap();
        let obj = json.as_object().unwrap();
        assert_eq!(
            obj.len(),
            2,
            "minimal create params should have only storage and type"
        );
        assert_eq!(json["storage"], "minimal");
        assert_eq!(json["type"], "dir");
    }

    #[test]
    fn storage_update_params_serialization() {
        let params = StorageUpdateParams {
            content: Some("images,iso,backup".to_string()),
            nodes: Some("pve1,pve2".to_string()),
            disable: Some(false),
            ..Default::default()
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["content"], "images,iso,backup");
        assert_eq!(json["nodes"], "pve1,pve2");
        assert_eq!(json["disable"], 0);
    }

    #[test]
    fn storage_update_params_skip_none() {
        let params = StorageUpdateParams::default();
        let json = serde_json::to_value(&params).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default StorageUpdateParams should serialize to {{}}"
        );
    }

    #[test]
    fn storage_update_params_with_delete() {
        let params = StorageUpdateParams {
            delete: Some("nodes,disable".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["delete"], "nodes,disable");
    }

    #[test]
    fn storage_create_params_debug_redacts_password() {
        let mut params = StorageCreateParams::new("nfs-store", "nfs");
        params.password = Some("nfs-secret-pass".to_string());
        let debug = format!("{params:?}");
        assert!(
            !debug.contains("nfs-secret-pass"),
            "password must be redacted in Debug output"
        );
        assert!(debug.contains("<redacted>"));
    }

    #[test]
    fn storage_update_params_debug_redacts_password() {
        let params = StorageUpdateParams {
            password: Some("update-secret-pass".to_string()),
            ..Default::default()
        };
        let debug = format!("{params:?}");
        assert!(
            !debug.contains("update-secret-pass"),
            "password must be redacted in Debug output"
        );
        assert!(debug.contains("<redacted>"));
    }
}
