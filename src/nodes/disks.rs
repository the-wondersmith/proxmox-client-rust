use serde::{Deserialize, Serialize};
use serde_json::Value;

use urlencoding::encode;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_node_name;

/// A physical disk on a node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Disk {
    /// Device path (e.g., `/dev/sda`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devpath: Option<String>,

    /// Device name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Disk size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,

    /// Whether the disk is used by PVE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<String>,

    /// Disk model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Disk serial number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,

    /// Disk vendor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    /// Whether the disk has a GPT partition table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpt: Option<bool>,

    /// Disk transport type (e.g., `sata`, `nvme`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<String>,

    /// RPM (0 for SSD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpm: Option<Value>,

    /// Wearout level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wearout: Option<Value>,

    /// Health status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,

    /// By-id path.
    #[serde(rename = "by_id_link", skip_serializing_if = "Option::is_none")]
    pub by_id_link: Option<String>,
}

/// SMART data for a disk.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SmartData {
    /// Health status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,

    /// SMART attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Value>>,

    /// SMART text output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Device type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<String>,
}

/// LVM volume entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LvmVolume {
    /// LV name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Device path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,

    /// LV size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,

    /// Free space in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free: Option<u64>,

    /// LV count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lvcount: Option<u32>,

    /// Children/logical volumes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Value>>,
}

/// LVM thin pool entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LvmThinPool {
    /// Pool name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lv: Option<String>,

    /// Volume group name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vg: Option<String>,

    /// Pool size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lv_size: Option<u64>,

    /// Metadata size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_size: Option<u64>,

    /// Used fraction (0.0 - 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<f64>,

    /// Metadata usage fraction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_used: Option<f64>,
}

/// ZFS pool information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ZfsPool {
    /// Pool name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Pool size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,

    /// Free space in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free: Option<u64>,

    /// Allocated space in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc: Option<u64>,

    /// Pool health.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,

    /// Dedup ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedup: Option<f64>,

    /// Fragmentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frag: Option<f64>,

    /// Children (vdevs).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Value>>,

    /// ZFS action (e.g., resilvering).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// ZFS scan status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan: Option<String>,

    /// ZFS status message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// ZFS errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<String>,
}

/// Directory storage entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DiskDirectory {
    /// Path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,

    /// Filesystem type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,

    /// Options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,

    /// Unit file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unitfile: Option<String>,
}

/// Parameters for creating an LVM volume group.
#[derive(Debug, Clone, Default, Serialize)]
pub struct LvmCreateParams {
    /// Name for the volume group.
    pub name: String,

    /// Device path.
    pub device: String,

    /// Add as Proxmox storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_storage: Option<bool>,
}

impl LvmCreateParams {
    /// Creates a new `LvmCreateParams` with the required fields.
    pub fn new(name: impl Into<String>, device: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            device: device.into(),
            ..Default::default()
        }
    }
}

/// Parameters for creating an LVM thin pool.
#[derive(Debug, Clone, Default, Serialize)]
pub struct LvmThinCreateParams {
    /// Name for the thin pool.
    pub name: String,

    /// Device path.
    pub device: String,

    /// Add as Proxmox storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_storage: Option<bool>,
}

impl LvmThinCreateParams {
    /// Creates a new `LvmThinCreateParams` with the required fields.
    pub fn new(name: impl Into<String>, device: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            device: device.into(),
            ..Default::default()
        }
    }
}

/// Parameters for creating a ZFS pool.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ZfsCreateParams {
    /// Pool name.
    pub name: String,

    /// RAID level (e.g., `single`, `mirror`, `raidz`, `raidz2`, `raidz3`).
    pub raidlevel: String,

    /// Devices (space-separated).
    pub devices: String,

    /// Add as Proxmox storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_storage: Option<bool>,

    /// Compression algorithm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<String>,

    /// Ashift value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ashift: Option<u32>,
}

impl ZfsCreateParams {
    /// Creates a new `ZfsCreateParams` with the required fields.
    pub fn new(
        name: impl Into<String>,
        raidlevel: impl Into<String>,
        devices: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            raidlevel: raidlevel.into(),
            devices: devices.into(),
            ..Default::default()
        }
    }
}

/// Parameters for creating a directory.
#[derive(Debug, Clone, Default, Serialize)]
pub struct DirectoryCreateParams {
    /// Name for the directory storage.
    pub name: String,

    /// Device path.
    pub device: String,

    /// Filesystem type (e.g., `ext4`, `xfs`).
    pub filesystem: String,

    /// Add as Proxmox storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_storage: Option<bool>,
}

impl DirectoryCreateParams {
    /// Creates a new `DirectoryCreateParams` with the required fields.
    pub fn new(
        name: impl Into<String>,
        device: impl Into<String>,
        filesystem: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            device: device.into(),
            filesystem: filesystem.into(),
            ..Default::default()
        }
    }
}

/// Parameters for wiping a disk.
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub struct WipeDiskParams {
    /// Device path to wipe (required).
    pub disk: String,
}

impl WipeDiskParams {
    /// Creates a new `WipeDiskParams` with the required fields.
    pub fn new(disk: impl Into<String>) -> Self {
        Self { disk: disk.into() }
    }
}

impl ProxmoxClient {
    // ── Disk listing ───────────────────────────────────────────────

    /// Lists physical disks on a node.
    pub async fn list_disks(
        &self,
        node: &str,
        include_partitions: Option<bool>,
    ) -> Result<Vec<Disk>> {
        validate_node_name(node)?;
        let mut path = format!("/nodes/{node}/disks/list");
        if let Some(true) = include_partitions {
            path.push_str("?include-partitions=1");
        }
        self.get_parsed(&path, &format!("node {node} disks")).await
    }

    /// Returns SMART data for a disk.
    pub async fn get_disk_smart(&self, node: &str, disk: &str) -> Result<SmartData> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/disks/smart?disk={}", encode(disk)),
            &format!("disk {disk} SMART"),
        )
        .await
    }

    /// Initializes a GPT partition table on a disk.
    pub async fn init_gpt(&self, node: &str, disk: &str) -> Result<String> {
        validate_node_name(node)?;
        let params = serde_json::json!({ "disk": disk });
        self.post_parsed(
            &format!("/nodes/{node}/disks/initgpt"),
            &params,
            &format!("disk {disk} GPT init"),
        )
        .await
    }

    // ── LVM ────────────────────────────────────────────────────────

    /// Lists LVM volume groups.
    pub async fn list_lvm(&self, node: &str) -> Result<Vec<LvmVolume>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/disks/lvm"),
            &format!("node {node} LVM"),
        )
        .await
    }

    /// Creates an LVM volume group.
    pub async fn create_lvm(&self, node: &str, params: &LvmCreateParams) -> Result<String> {
        validate_node_name(node)?;
        self.post_parsed(
            &format!("/nodes/{node}/disks/lvm"),
            params,
            &format!("node {node} LVM create"),
        )
        .await
    }

    /// Deletes an LVM volume group.
    pub async fn delete_lvm(&self, node: &str, name: &str) -> Result<String> {
        validate_node_name(node)?;
        let response = self
            .delete(&format!("/nodes/{node}/disks/lvm/{name}"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("LVM {name}")).await
    }

    // ── LVM Thin ───────────────────────────────────────────────────

    /// Lists LVM thin pools.
    pub async fn list_lvm_thin(&self, node: &str) -> Result<Vec<LvmThinPool>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/disks/lvmthin"),
            &format!("node {node} LVM thin"),
        )
        .await
    }

    /// Creates an LVM thin pool.
    pub async fn create_lvm_thin(
        &self,
        node: &str,
        params: &LvmThinCreateParams,
    ) -> Result<String> {
        validate_node_name(node)?;
        self.post_parsed(
            &format!("/nodes/{node}/disks/lvmthin"),
            params,
            &format!("node {node} LVM thin create"),
        )
        .await
    }

    /// Deletes an LVM thin pool.
    pub async fn delete_lvm_thin(&self, node: &str, name: &str) -> Result<String> {
        validate_node_name(node)?;
        let response = self
            .delete(&format!("/nodes/{node}/disks/lvmthin/{name}"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("LVM thin {name}")).await
    }

    // ── ZFS ────────────────────────────────────────────────────────

    /// Lists ZFS pools.
    pub async fn list_zfs_pools(&self, node: &str) -> Result<Vec<ZfsPool>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/disks/zfs"),
            &format!("node {node} ZFS"),
        )
        .await
    }

    /// Creates a ZFS pool.
    pub async fn create_zfs_pool(&self, node: &str, params: &ZfsCreateParams) -> Result<String> {
        validate_node_name(node)?;
        self.post_parsed(
            &format!("/nodes/{node}/disks/zfs"),
            params,
            &format!("node {node} ZFS create"),
        )
        .await
    }

    /// Returns information about a ZFS pool.
    pub async fn get_zfs_pool(&self, node: &str, name: &str) -> Result<ZfsPool> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/disks/zfs/{name}"),
            &format!("ZFS pool {name}"),
        )
        .await
    }

    /// Deletes a ZFS pool.
    pub async fn delete_zfs_pool(&self, node: &str, name: &str) -> Result<String> {
        validate_node_name(node)?;
        let response = self
            .delete(&format!("/nodes/{node}/disks/zfs/{name}"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("ZFS pool {name}")).await
    }

    // ── Directory ──────────────────────────────────────────────────

    /// Lists directory storage entries.
    pub async fn list_disk_directories(&self, node: &str) -> Result<Vec<DiskDirectory>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/disks/directory"),
            &format!("node {node} directories"),
        )
        .await
    }

    /// Creates a directory storage.
    pub async fn create_disk_directory(
        &self,
        node: &str,
        params: &DirectoryCreateParams,
    ) -> Result<String> {
        validate_node_name(node)?;
        self.post_parsed(
            &format!("/nodes/{node}/disks/directory"),
            params,
            &format!("node {node} directory create"),
        )
        .await
    }

    /// Deletes a directory storage.
    pub async fn delete_disk_directory(&self, node: &str, name: &str) -> Result<String> {
        validate_node_name(node)?;
        let response = self
            .delete(&format!("/nodes/{node}/disks/directory/{name}"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("directory {name}")).await
    }

    // ── Wipe Disk ─────────────────────────────────────────────────

    /// Wipes a disk.
    ///
    /// `PUT /nodes/{node}/disks/wipedisk`
    pub async fn wipe_disk(&self, node: &str, params: &WipeDiskParams) -> Result<String> {
        validate_node_name(node)?;
        let response = self
            .put(&format!("/nodes/{node}/disks/wipedisk"))?
            .json(params)
            .send()
            .await?;
        Self::parse_response(response, &format!("node {node} wipe disk")).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disk_serde_roundtrip() {
        let json = r#"{
            "devpath": "/dev/sda",
            "name": "sda",
            "size": 1000204886016,
            "used": "LVM",
            "model": "Samsung SSD 870",
            "serial": "S1234567890",
            "type": "sata",
            "gpt": true,
            "health": "PASSED"
        }"#;
        let disk: Disk = serde_json::from_str(json).unwrap();
        assert_eq!(disk.devpath.as_deref(), Some("/dev/sda"));
        assert_eq!(disk.health.as_deref(), Some("PASSED"));

        let serialized = serde_json::to_string(&disk).unwrap();
        let deserialized: Disk = serde_json::from_str(&serialized).unwrap();
        assert_eq!(disk, deserialized);
    }

    #[test]
    fn zfs_pool_serde_roundtrip() {
        let json = r#"{
            "name": "rpool",
            "size": 1000000000000,
            "free": 800000000000,
            "alloc": 200000000000,
            "health": "ONLINE",
            "dedup": 1.0,
            "frag": 5.0
        }"#;
        let pool: ZfsPool = serde_json::from_str(json).unwrap();
        assert_eq!(pool.name.as_deref(), Some("rpool"));
        assert_eq!(pool.health.as_deref(), Some("ONLINE"));

        let serialized = serde_json::to_string(&pool).unwrap();
        let deserialized: ZfsPool = serde_json::from_str(&serialized).unwrap();
        assert_eq!(pool, deserialized);
    }
}
