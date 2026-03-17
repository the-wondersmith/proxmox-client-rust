use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_resource_id;

/// Cluster Ceph metadata.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ClusterCephMetadata {
    /// Ceph version information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<serde_json::Value>,

    /// Node-specific metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<serde_json::Value>,

    /// Mon metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mon: Option<serde_json::Value>,

    /// Mgr metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgr: Option<serde_json::Value>,

    /// MDS metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mds: Option<serde_json::Value>,

    /// OSD metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osd: Option<serde_json::Value>,
}

/// Cluster-level Ceph status.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ClusterCephStatus {
    /// Health information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<serde_json::Value>,

    /// Mon status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monmap: Option<serde_json::Value>,

    /// OSD map summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osdmap: Option<serde_json::Value>,

    /// PG map summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pgmap: Option<serde_json::Value>,

    /// FSID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fsid: Option<String>,

    /// Quorum node names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quorum_names: Option<Vec<String>>,

    /// Election epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub election_epoch: Option<i64>,
}

/// Ceph global flags.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CephFlags {
    /// No-up flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noup: Option<i32>,

    /// No-down flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodown: Option<i32>,

    /// No-in flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noin: Option<i32>,

    /// No-out flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noout: Option<i32>,

    /// No-backfill flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nobackfill: Option<i32>,

    /// No-rebalance flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub norebalance: Option<i32>,

    /// No-recover flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub norecover: Option<i32>,

    /// No-deep-scrub flag.
    #[serde(rename = "nodeep-scrub", skip_serializing_if = "Option::is_none")]
    pub nodeep_scrub: Option<i32>,

    /// No-scrub flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noscrub: Option<i32>,

    /// Pause flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause: Option<i32>,

    /// No-thinning flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notieragent: Option<i32>,
}

impl ProxmoxClient {
    /// Gets cluster Ceph metadata.
    ///
    /// `GET /cluster/ceph/metadata`
    pub async fn get_cluster_ceph_metadata(&self) -> Result<ClusterCephMetadata> {
        self.get_parsed("/cluster/ceph/metadata", "cluster Ceph metadata")
            .await
    }

    /// Gets cluster Ceph status.
    ///
    /// `GET /cluster/ceph/status`
    pub async fn get_cluster_ceph_status(&self) -> Result<ClusterCephStatus> {
        self.get_parsed("/cluster/ceph/status", "cluster Ceph status")
            .await
    }

    /// Gets Ceph flags.
    ///
    /// `GET /cluster/ceph/flags`
    pub async fn get_ceph_flags(&self) -> Result<Vec<serde_json::Value>> {
        self.get_parsed("/cluster/ceph/flags", "Ceph flags").await
    }

    /// Sets Ceph flags.
    ///
    /// `PUT /cluster/ceph/flags`
    pub async fn set_ceph_flags(&self, params: &CephFlags) -> Result<()> {
        let response = self.put("/cluster/ceph/flags")?.json(params).send().await?;
        Self::handle_error(response, "Ceph flags").await?;
        Ok(())
    }

    /// Gets a specific Ceph flag.
    ///
    /// `GET /cluster/ceph/flags/{flag}`
    pub async fn get_ceph_flag(&self, flag: &str) -> Result<serde_json::Value> {
        validate_resource_id(flag)?;
        self.get_parsed(
            &format!("/cluster/ceph/flags/{flag}"),
            &format!("Ceph flag {flag}"),
        )
        .await
    }

    /// Sets a specific Ceph flag.
    ///
    /// `PUT /cluster/ceph/flags/{flag}`
    pub async fn set_ceph_flag(&self, flag: &str, params: &serde_json::Value) -> Result<()> {
        validate_resource_id(flag)?;
        let response = self
            .put(&format!("/cluster/ceph/flags/{flag}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("Ceph flag {flag}")).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cluster_ceph_metadata_serde_roundtrip() {
        let metadata = ClusterCephMetadata {
            version: Some(serde_json::json!({"major": 17, "minor": 2})),
            node: None,
            mon: None,
            mgr: None,
            mds: None,
            osd: None,
        };

        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: ClusterCephMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(metadata, deserialized);
    }

    #[test]
    fn cluster_ceph_metadata_skip_serializing_none() {
        let metadata = ClusterCephMetadata::default();
        let json = serde_json::to_value(&metadata).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default ClusterCephMetadata should serialize to {{}}"
        );
    }

    #[test]
    fn cluster_ceph_status_serde_roundtrip() {
        let status = ClusterCephStatus {
            health: Some(serde_json::json!({"status": "HEALTH_OK"})),
            monmap: None,
            osdmap: None,
            pgmap: None,
            fsid: Some("12345678-abcd-efgh-1234-567890abcdef".to_string()),
            quorum_names: Some(vec![
                "pve1".to_string(),
                "pve2".to_string(),
                "pve3".to_string(),
            ]),
            election_epoch: Some(42),
        };

        let json = serde_json::to_string(&status).unwrap();
        let deserialized: ClusterCephStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, deserialized);
    }

    #[test]
    fn ceph_flags_serde_roundtrip() {
        let flags = CephFlags {
            noup: Some(0),
            nodown: Some(0),
            noin: Some(0),
            noout: Some(1),
            nobackfill: Some(0),
            norebalance: Some(0),
            norecover: Some(0),
            nodeep_scrub: Some(0),
            noscrub: Some(0),
            pause: Some(0),
            notieragent: Some(0),
        };

        let json = serde_json::to_string(&flags).unwrap();
        let deserialized: CephFlags = serde_json::from_str(&json).unwrap();
        assert_eq!(flags, deserialized);
    }

    #[test]
    fn ceph_flags_skip_serializing_none() {
        let flags = CephFlags::default();
        let json = serde_json::to_value(&flags).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty(), "default CephFlags should serialize to {{}}");
    }

    #[test]
    fn ceph_flags_nodeep_scrub_rename() {
        let json = r#"{"nodeep-scrub": 1}"#;
        let flags: CephFlags = serde_json::from_str(json).unwrap();
        assert_eq!(flags.nodeep_scrub, Some(1));

        let serialized = serde_json::to_value(&flags).unwrap();
        assert!(serialized.get("nodeep-scrub").is_some());
        assert!(serialized.get("nodeep_scrub").is_none());
    }

    #[test]
    fn cluster_ceph_metadata_unknown_fields_ignored() {
        let json = r#"{"fsid": "test", "unknownField": true}"#;
        let status: ClusterCephStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status.fsid.as_deref(), Some("test"));
    }
}
