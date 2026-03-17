use serde::{Deserialize, Serialize};
use serde_json::Value;

use urlencoding::encode;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::nodes::status::RrdData;
use crate::validation::{validate_node_name, validate_storage_id};

/// A storage entry on a node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NodeStorage {
    /// Storage ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,

    /// Storage type (e.g., `dir`, `lvm`, `zfspool`, `nfs`, `cifs`, `rbd`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,

    /// Content types (e.g., `images,rootdir,vztmpl,backup,iso,snippets`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Whether the storage is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<u32>,

    /// Whether the storage is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<u32>,

    /// Whether the storage is shared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<u32>,

    /// Total space in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,

    /// Used space in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<u64>,

    /// Available space in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail: Option<u64>,

    /// Usage fraction (0.0 - 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_fraction: Option<f64>,
}

/// Storage status information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NodeStorageStatus {
    /// Storage ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,

    /// Storage type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,

    /// Content types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Total space in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,

    /// Used space in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<u64>,

    /// Available space in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail: Option<u64>,

    /// Whether the storage is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// Whether the storage is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Whether the storage is shared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
}

/// A storage content entry (volume, ISO, template, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StorageContent {
    /// Volume ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volid: Option<String>,

    /// Content type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Format (e.g., `raw`, `qcow2`, `vmdk`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// Size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,

    /// Used size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<u64>,

    /// Associated VMID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmid: Option<u32>,

    /// Parent volume (for linked clones).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    /// Creation time (epoch).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctime: Option<u64>,

    /// Notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    /// Whether this volume is protected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,

    /// Whether verification is outdated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<Value>,
}

/// Volume information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StorageVolume {
    /// Volume path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Volume format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// Volume size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,

    /// Used size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<u64>,
}

/// A file restore entry.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FileRestoreEntry {
    /// File path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filepath: Option<String>,

    /// Display text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Whether this is a leaf node (file) vs directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf: Option<i32>,

    /// File size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,

    /// Modification time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtime: Option<i64>,

    /// File type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
}

/// Parameters for OCI registry pull.
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub struct OciRegistryPullParams {
    /// OCI image name (required).
    pub image: String,

    /// Image tag (required).
    pub tag: String,
}

impl OciRegistryPullParams {
    /// Creates a new `OciRegistryPullParams` with the required fields.
    pub fn new(image: impl Into<String>, tag: impl Into<String>) -> Self {
        Self {
            image: image.into(),
            tag: tag.into(),
        }
    }
}

/// Parameters for downloading from a URL.
#[derive(Debug, Clone, Default, Serialize)]
pub struct DownloadUrlParams {
    /// Content type.
    pub content: String,

    /// Filename.
    pub filename: String,

    /// URL to download from.
    pub url: String,

    /// Checksum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,

    /// Checksum algorithm.
    #[serde(rename = "checksum-algorithm", skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,

    /// Verify certificates.
    #[serde(
        rename = "verify-certificates",
        skip_serializing_if = "Option::is_none"
    )]
    pub verify_certificates: Option<bool>,
}

impl DownloadUrlParams {
    /// Creates a new `DownloadUrlParams` with the required fields.
    pub fn new(
        content: impl Into<String>,
        filename: impl Into<String>,
        url: impl Into<String>,
    ) -> Self {
        Self {
            content: content.into(),
            filename: filename.into(),
            url: url.into(),
            ..Default::default()
        }
    }
}

impl ProxmoxClient {
    /// Lists storages available on a node.
    pub async fn list_node_storages(
        &self,
        node: &str,
        content: Option<&str>,
    ) -> Result<Vec<NodeStorage>> {
        validate_node_name(node)?;
        let mut path = format!("/nodes/{node}/storage");
        if let Some(c) = content {
            path.push_str(&format!("?content={}", encode(c)));
        }
        self.get_parsed(&path, &format!("node {node} storages"))
            .await
    }

    /// Returns the directory index for a specific storage on a node.
    ///
    /// `GET /nodes/{node}/storage/{storage}`
    pub async fn get_node_storage_info(&self, node: &str, storage: &str) -> Result<Vec<Value>> {
        validate_node_name(node)?;
        validate_storage_id(storage)?;
        self.get_parsed(
            &format!("/nodes/{node}/storage/{storage}"),
            &format!("storage {storage} info"),
        )
        .await
    }

    /// Returns the status of a specific storage on a node.
    pub async fn get_node_storage_status(
        &self,
        node: &str,
        storage: &str,
    ) -> Result<NodeStorageStatus> {
        validate_node_name(node)?;
        validate_storage_id(storage)?;
        self.get_parsed(
            &format!("/nodes/{node}/storage/{storage}/status"),
            &format!("storage {storage}"),
        )
        .await
    }

    /// Lists content of a storage on a node.
    pub async fn list_storage_content(
        &self,
        node: &str,
        storage: &str,
        content: Option<&str>,
        vmid: Option<u32>,
    ) -> Result<Vec<StorageContent>> {
        validate_node_name(node)?;
        validate_storage_id(storage)?;
        let vmid_s = vmid.map(|v| v.to_string());
        let path = Self::build_query_path(
            &format!("/nodes/{node}/storage/{storage}/content"),
            &[("content", content), ("vmid", vmid_s.as_deref())],
        );
        self.get_parsed(&path, &format!("storage {storage} content"))
            .await
    }

    /// Creates/allocates a volume on a storage.
    pub async fn create_storage_volume(
        &self,
        node: &str,
        storage: &str,
        filename: &str,
        size: &str,
        vmid: u32,
        format: Option<&str>,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_storage_id(storage)?;
        let mut params = serde_json::json!({
            "filename": filename,
            "size": size,
            "vmid": vmid,
        });
        if let Some(f) = format {
            params["format"] = Value::String(f.to_owned());
        }
        self.post_parsed(
            &format!("/nodes/{node}/storage/{storage}/content"),
            &params,
            &format!("storage {storage} volume"),
        )
        .await
    }

    /// Returns info about a specific volume.
    pub async fn get_storage_volume_info(
        &self,
        node: &str,
        storage: &str,
        volume: &str,
    ) -> Result<StorageVolume> {
        validate_node_name(node)?;
        validate_storage_id(storage)?;
        self.get_parsed(
            &format!("/nodes/{node}/storage/{storage}/content/{volume}"),
            &format!("volume {volume}"),
        )
        .await
    }

    /// Deletes a volume from storage.
    pub async fn delete_storage_volume(
        &self,
        node: &str,
        storage: &str,
        volume: &str,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_storage_id(storage)?;
        self.delete_void(
            &format!("/nodes/{node}/storage/{storage}/content/{volume}"),
            &format!("volume {volume}"),
        )
        .await
    }

    /// Updates attributes (notes, protected flag) of a storage volume.
    ///
    /// `PUT /nodes/{node}/storage/{storage}/content/{volume}`
    pub async fn update_storage_volume_attributes(
        &self,
        node: &str,
        storage: &str,
        volume: &str,
        params: &Value,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_storage_id(storage)?;
        let response = self
            .put(&format!("/nodes/{node}/storage/{storage}/content/{volume}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("volume {volume} attributes")).await?;
        Ok(())
    }

    /// Copies a volume (e.g., for backup restore).
    pub async fn copy_storage_volume(
        &self,
        node: &str,
        storage: &str,
        volume: &str,
        target: &str,
        target_node: Option<&str>,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_storage_id(storage)?;
        let mut params = serde_json::json!({ "target": target });
        if let Some(tn) = target_node {
            params["target_node"] = Value::String(tn.to_owned());
        }
        self.post_parsed(
            &format!("/nodes/{node}/storage/{storage}/content/{volume}"),
            &params,
            &format!("copy volume {volume}"),
        )
        .await
    }

    /// Returns RRD data for a storage.
    pub async fn get_storage_rrddata(
        &self,
        node: &str,
        storage: &str,
        timeframe: &str,
        cf: Option<&str>,
    ) -> Result<Vec<RrdData>> {
        validate_node_name(node)?;
        validate_storage_id(storage)?;
        let mut path = format!(
            "/nodes/{node}/storage/{storage}/rrddata?timeframe={}",
            encode(timeframe)
        );
        if let Some(cf) = cf {
            path.push_str(&format!("&cf={}", encode(cf)));
        }
        self.get_parsed(&path, &format!("storage {storage} rrddata"))
            .await
    }

    /// Downloads a file from a URL to storage.
    pub async fn download_url_to_storage(
        &self,
        node: &str,
        storage: &str,
        params: &DownloadUrlParams,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_storage_id(storage)?;
        self.post_parsed(
            &format!("/nodes/{node}/storage/{storage}/download-url"),
            params,
            &format!("storage {storage} download"),
        )
        .await
    }

    /// Gets prune information for backups.
    pub async fn get_prune_backups_info(&self, node: &str, storage: &str) -> Result<Vec<Value>> {
        validate_node_name(node)?;
        validate_storage_id(storage)?;
        self.get_parsed(
            &format!("/nodes/{node}/storage/{storage}/prunebackups"),
            &format!("storage {storage} prune info"),
        )
        .await
    }

    /// Lists files available for restore from a backup volume.
    ///
    /// `GET /nodes/{node}/storage/{storage}/file-restore/list`
    pub async fn list_file_restore(
        &self,
        node: &str,
        storage: &str,
        volume: &str,
        filepath: Option<&str>,
    ) -> Result<Vec<FileRestoreEntry>> {
        validate_node_name(node)?;
        validate_storage_id(storage)?;
        let mut path = format!(
            "/nodes/{node}/storage/{storage}/file-restore/list?volume={}",
            encode(volume)
        );
        if let Some(fp) = filepath {
            path.push_str(&format!("&filepath={}", encode(fp)));
        }
        self.get_parsed(&path, &format!("storage {storage} file restore"))
            .await
    }

    /// Downloads a file or directory (as zip/tar.zst archive) from a PBS backup.
    ///
    /// `GET /nodes/{node}/storage/{storage}/file-restore/download`
    pub async fn download_file_restore(
        &self,
        node: &str,
        storage: &str,
        volume: &str,
        filepath: &str,
        tar: Option<bool>,
    ) -> Result<Vec<u8>> {
        validate_node_name(node)?;
        validate_storage_id(storage)?;
        let mut path = format!(
            "/nodes/{node}/storage/{storage}/file-restore/download?volume={}&filepath={}",
            encode(volume),
            encode(filepath)
        );
        if let Some(true) = tar {
            path.push_str("&tar=1");
        }
        self.get_bytes(&path, &format!("storage {storage} file restore download"))
            .await
    }

    /// Gets import metadata for a volume.
    ///
    /// `GET /nodes/{node}/storage/{storage}/import-metadata`
    pub async fn get_import_metadata(
        &self,
        node: &str,
        storage: &str,
        volume: &str,
    ) -> Result<Value> {
        validate_node_name(node)?;
        validate_storage_id(storage)?;
        self.get_parsed(
            &format!(
                "/nodes/{node}/storage/{storage}/import-metadata?volume={}",
                encode(volume)
            ),
            &format!("storage {storage} import metadata"),
        )
        .await
    }

    /// Uploads a file to storage.
    ///
    /// `POST /nodes/{node}/storage/{storage}/upload`
    pub async fn upload_to_storage(
        &self,
        node: &str,
        storage: &str,
        content: &str,
        filename: &str,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_storage_id(storage)?;
        let params = serde_json::json!({
            "content": content,
            "filename": filename,
        });
        self.post_parsed(
            &format!("/nodes/{node}/storage/{storage}/upload"),
            &params,
            &format!("storage {storage} upload"),
        )
        .await
    }

    /// Pulls an OCI image to storage.
    ///
    /// `POST /nodes/{node}/storage/{storage}/oci-registry-pull`
    pub async fn oci_registry_pull(
        &self,
        node: &str,
        storage: &str,
        params: &OciRegistryPullParams,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_storage_id(storage)?;
        self.post_parsed(
            &format!("/nodes/{node}/storage/{storage}/oci-registry-pull"),
            params,
            &format!("storage {storage} OCI pull"),
        )
        .await
    }

    /// Prunes backups on a storage.
    pub async fn prune_backups(&self, node: &str, storage: &str, params: &Value) -> Result<String> {
        validate_node_name(node)?;
        validate_storage_id(storage)?;
        let response = self
            .delete(&format!("/nodes/{node}/storage/{storage}/prunebackups"))?
            .json(params)
            .send()
            .await?;
        Self::parse_response(response, &format!("storage {storage} prune")).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_storage_serde_roundtrip() {
        let json = r#"{
            "storage": "local-lvm",
            "type": "lvmthin",
            "content": "images,rootdir",
            "active": 1,
            "enabled": 1,
            "shared": 0,
            "total": 107374182400,
            "used": 32212254720,
            "avail": 75161927680,
            "used_fraction": 0.3
        }"#;
        let storage: NodeStorage = serde_json::from_str(json).unwrap();
        assert_eq!(storage.storage.as_deref(), Some("local-lvm"));
        assert_eq!(storage.storage_type.as_deref(), Some("lvmthin"));

        let serialized = serde_json::to_string(&storage).unwrap();
        let deserialized: NodeStorage = serde_json::from_str(&serialized).unwrap();
        assert_eq!(storage, deserialized);
    }

    #[test]
    fn storage_content_serde_roundtrip() {
        let json = r#"{
            "volid": "local-lvm:vm-100-disk-0",
            "content": "images",
            "format": "raw",
            "size": 34359738368,
            "vmid": 100
        }"#;
        let content: StorageContent = serde_json::from_str(json).unwrap();
        assert_eq!(content.volid.as_deref(), Some("local-lvm:vm-100-disk-0"));
        assert_eq!(content.vmid, Some(100));

        let serialized = serde_json::to_string(&content).unwrap();
        let deserialized: StorageContent = serde_json::from_str(&serialized).unwrap();
        assert_eq!(content, deserialized);
    }
}
