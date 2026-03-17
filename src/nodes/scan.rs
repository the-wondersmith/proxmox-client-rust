use serde::{Deserialize, Serialize};

use urlencoding::encode;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_node_name;

/// An NFS share found during scanning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NfsShare {
    /// Export path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
}

/// A CIFS share found during scanning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CifsShare {
    /// Share description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Share name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share: Option<String>,
}

/// An iSCSI target found during scanning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IscsiTarget {
    /// Target IQN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Portal address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal: Option<String>,
}

/// An LVM volume group found during scanning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LvmVolumeGroup {
    /// Volume group name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vg: Option<String>,

    /// Size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,

    /// Free space in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free: Option<u64>,
}

/// A ZFS dataset found during scanning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ZfsDataset {
    /// Pool name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,
}

/// An LVM thin pool found during scanning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LvmThinScanResult {
    /// Logical volume name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lv: Option<String>,
}

/// A PBS (Proxmox Backup Server) namespace found during scanning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PbsNamespace {
    /// Namespace name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ns: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl ProxmoxClient {
    /// Scans for NFS shares on a server.
    pub async fn scan_nfs(&self, node: &str, server: &str) -> Result<Vec<NfsShare>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/scan/nfs?server={}", encode(server)),
            &format!("NFS scan {server}"),
        )
        .await
    }

    /// Scans for CIFS shares on a server.
    ///
    /// # Security
    ///
    /// The Proxmox API requires credentials as query parameters for this GET endpoint.
    /// This means the password will appear in server access logs. Use a dedicated
    /// service account with minimal privileges and rotate credentials after use.
    pub async fn scan_cifs(
        &self,
        node: &str,
        server: &str,
        username: Option<&str>,
        password: Option<&str>,
        domain: Option<&str>,
    ) -> Result<Vec<CifsShare>> {
        validate_node_name(node)?;
        let mut path = format!("/nodes/{node}/scan/cifs?server={}", encode(server));
        if let Some(u) = username {
            path.push_str(&format!("&username={}", encode(u)));
        }
        if let Some(p) = password {
            path.push_str(&format!("&password={}", encode(p)));
        }
        if let Some(d) = domain {
            path.push_str(&format!("&domain={}", encode(d)));
        }
        self.get_parsed(&path, &format!("CIFS scan {server}")).await
    }

    /// Scans for iSCSI targets on a portal.
    pub async fn scan_iscsi(&self, node: &str, portal: &str) -> Result<Vec<IscsiTarget>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/scan/iscsi?portal={}", encode(portal)),
            &format!("iSCSI scan {portal}"),
        )
        .await
    }

    /// Scans for LVM volume groups.
    pub async fn scan_lvm(&self, node: &str) -> Result<Vec<LvmVolumeGroup>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/scan/lvm"),
            &format!("node {node} LVM scan"),
        )
        .await
    }

    /// Scans for ZFS pools/datasets.
    pub async fn scan_zfs(&self, node: &str) -> Result<Vec<ZfsDataset>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/scan/zfs"),
            &format!("node {node} ZFS scan"),
        )
        .await
    }

    /// Scans for LVM thin pools.
    pub async fn scan_lvmthin(&self, node: &str, vg: &str) -> Result<Vec<LvmThinScanResult>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/scan/lvmthin?vg={}", encode(vg)),
            &format!("node {node} LVM thin scan"),
        )
        .await
    }

    /// Scans for Proxmox Backup Server namespaces.
    ///
    /// # Security
    ///
    /// The Proxmox API requires credentials as query parameters for this GET endpoint.
    /// This means the password will appear in server access logs. Use a dedicated
    /// service account with minimal privileges and rotate credentials after use.
    pub async fn scan_pbs(
        &self,
        node: &str,
        server: &str,
        username: &str,
        password: &str,
        datastore: &str,
        fingerprint: Option<&str>,
    ) -> Result<Vec<PbsNamespace>> {
        validate_node_name(node)?;
        let mut path = format!(
            "/nodes/{node}/scan/pbs?server={}&username={}&password={}&store={}",
            encode(server),
            encode(username),
            encode(password),
            encode(datastore)
        );
        if let Some(f) = fingerprint {
            path.push_str(&format!("&fingerprint={}", encode(f)));
        }
        self.get_parsed(&path, &format!("PBS scan {server}")).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nfs_share_serde_roundtrip() {
        let json = r#"{"path": "/mnt/data", "options": "rw,no_root_squash"}"#;
        let share: NfsShare = serde_json::from_str(json).unwrap();
        assert_eq!(share.path.as_deref(), Some("/mnt/data"));

        let serialized = serde_json::to_string(&share).unwrap();
        let deserialized: NfsShare = serde_json::from_str(&serialized).unwrap();
        assert_eq!(share, deserialized);
    }

    #[test]
    fn cifs_share_serde_roundtrip() {
        let json = r#"{"description": "Shared folder", "share": "backups"}"#;
        let share: CifsShare = serde_json::from_str(json).unwrap();
        assert_eq!(share.share.as_deref(), Some("backups"));

        let serialized = serde_json::to_string(&share).unwrap();
        let deserialized: CifsShare = serde_json::from_str(&serialized).unwrap();
        assert_eq!(share, deserialized);
    }

    #[test]
    fn iscsi_target_serde_roundtrip() {
        let json = r#"{"target": "iqn.2023-01.com.example:storage", "portal": "10.0.0.1:3260"}"#;
        let target: IscsiTarget = serde_json::from_str(json).unwrap();
        assert_eq!(
            target.target.as_deref(),
            Some("iqn.2023-01.com.example:storage")
        );

        let serialized = serde_json::to_string(&target).unwrap();
        let deserialized: IscsiTarget = serde_json::from_str(&serialized).unwrap();
        assert_eq!(target, deserialized);
    }
}
