use serde::{Deserialize, Serialize};

use urlencoding::encode;

use crate::client::ProxmoxClient;
use crate::error::Result;

/// A cluster-wide resource entry (VM, node, storage, pool).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ClusterResource {
    /// Resource ID (e.g., `node/pve1`, `qemu/100`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Resource type (`node`, `qemu`, `lxc`, `storage`, `pool`, `sdn`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,

    /// Node name the resource belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,

    /// Resource status (e.g., `running`, `stopped`, `online`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Resource name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// VM ID (for qemu/lxc resources).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmid: Option<i64>,

    /// Maximum memory in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxmem: Option<i64>,

    /// Current memory usage in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem: Option<i64>,

    /// Maximum disk size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxdisk: Option<i64>,

    /// Current disk usage in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<i64>,

    /// Maximum number of CPUs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxcpu: Option<f64>,

    /// Current CPU usage (fraction).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f64>,

    /// Uptime in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime: Option<i64>,

    /// Network input bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netin: Option<i64>,

    /// Network output bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netout: Option<i64>,

    /// Disk read bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diskread: Option<i64>,

    /// Disk write bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diskwrite: Option<i64>,

    /// Template flag (1 = template).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<i32>,

    /// HA state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hastate: Option<String>,

    /// Pool membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,

    /// Storage name (for storage resources).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,

    /// Storage content types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Lock status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<String>,

    /// Tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

/// Cluster status information.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ClusterStatus {
    /// Status entry type (`cluster` or `node`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub status_type: Option<String>,

    /// Cluster or node name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Node ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeid: Option<i64>,

    /// Whether the cluster is quorate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quorate: Option<i32>,

    /// Number of nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<i64>,

    /// Version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,

    /// Whether the node is online.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<i32>,

    /// Node level (for node entries).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,

    /// Local flag (1 = this is the local node).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local: Option<i32>,

    /// IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

/// An entry in the cluster log.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ClusterLogEntry {
    /// Log UID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    /// Timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,

    /// Timestamp as string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// Node name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,

    /// Process ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,

    /// User who initiated the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// Log severity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pri: Option<i64>,

    /// Log message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

impl ProxmoxClient {
    /// Lists all cluster resources (VMs, nodes, storage, pools).
    ///
    /// `GET /cluster/resources`
    pub async fn list_cluster_resources(&self) -> Result<Vec<ClusterResource>> {
        self.get_parsed("/cluster/resources", "cluster resources")
            .await
    }

    /// Lists cluster resources filtered by type.
    ///
    /// `GET /cluster/resources?type={resource_type}`
    pub async fn list_cluster_resources_by_type(
        &self,
        resource_type: &str,
    ) -> Result<Vec<ClusterResource>> {
        self.get_parsed(
            &format!("/cluster/resources?type={}", encode(resource_type)),
            "cluster resources",
        )
        .await
    }

    /// Gets the cluster status.
    ///
    /// `GET /cluster/status`
    pub async fn get_cluster_status(&self) -> Result<Vec<ClusterStatus>> {
        self.get_parsed("/cluster/status", "cluster status").await
    }

    /// Gets the next free VMID.
    ///
    /// `GET /cluster/nextid`
    pub async fn get_next_id(&self) -> Result<String> {
        self.get_parsed("/cluster/nextid", "next VMID").await
    }

    /// Gets the cluster log.
    ///
    /// `GET /cluster/log`
    pub async fn get_cluster_log(&self) -> Result<Vec<ClusterLogEntry>> {
        self.get_parsed("/cluster/log", "cluster log").await
    }

    /// Lists cluster-wide tasks.
    ///
    /// `GET /cluster/tasks`
    pub async fn list_cluster_tasks(&self) -> Result<Vec<serde_json::Value>> {
        self.get_parsed("/cluster/tasks", "cluster tasks").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cluster_resource_serde_roundtrip() {
        let resource = ClusterResource {
            id: Some("qemu/100".to_string()),
            resource_type: Some("qemu".to_string()),
            node: Some("pve1".to_string()),
            status: Some("running".to_string()),
            name: Some("test-vm".to_string()),
            vmid: Some(100),
            maxmem: Some(4294967296),
            mem: Some(1073741824),
            maxdisk: Some(34359738368),
            disk: Some(8589934592),
            maxcpu: Some(4.0),
            cpu: Some(0.15),
            uptime: Some(86400),
            netin: Some(1048576),
            netout: Some(524288),
            diskread: Some(2097152),
            diskwrite: Some(1048576),
            template: Some(0),
            hastate: None,
            pool: None,
            storage: None,
            content: None,
            lock: None,
            tags: Some("production".to_string()),
        };

        let json = serde_json::to_string(&resource).unwrap();
        let deserialized: ClusterResource = serde_json::from_str(&json).unwrap();
        assert_eq!(resource, deserialized);
    }

    #[test]
    fn cluster_resource_skip_serializing_none() {
        let resource = ClusterResource::default();
        let json = serde_json::to_value(&resource).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default ClusterResource should serialize to {{}}"
        );
    }

    #[test]
    fn cluster_resource_type_rename() {
        let json = r#"{"type": "qemu", "id": "qemu/100"}"#;
        let resource: ClusterResource = serde_json::from_str(json).unwrap();
        assert_eq!(resource.resource_type.as_deref(), Some("qemu"));

        let serialized = serde_json::to_value(&resource).unwrap();
        assert!(serialized.get("type").is_some());
        assert!(serialized.get("resource_type").is_none());
    }

    #[test]
    fn cluster_resource_deserialize_from_api() {
        let json = r#"{
            "id": "node/pve1",
            "type": "node",
            "node": "pve1",
            "status": "online",
            "maxmem": 16777216000,
            "mem": 8388608000,
            "maxdisk": 107374182400,
            "disk": 53687091200,
            "maxcpu": 8.0,
            "cpu": 0.25,
            "uptime": 172800
        }"#;
        let resource: ClusterResource = serde_json::from_str(json).unwrap();
        assert_eq!(resource.id.as_deref(), Some("node/pve1"));
        assert_eq!(resource.resource_type.as_deref(), Some("node"));
        assert_eq!(resource.uptime, Some(172800));
    }

    #[test]
    fn cluster_resource_unknown_fields_ignored() {
        let json = r#"{"id": "test", "unknownField": 42}"#;
        let resource: ClusterResource = serde_json::from_str(json).unwrap();
        assert_eq!(resource.id.as_deref(), Some("test"));
    }

    #[test]
    fn cluster_status_serde_roundtrip() {
        let status = ClusterStatus {
            status_type: Some("cluster".to_string()),
            name: Some("testcluster".to_string()),
            nodeid: None,
            quorate: Some(1),
            nodes: Some(3),
            version: Some(5),
            online: None,
            level: None,
            local: None,
            ip: None,
        };

        let json = serde_json::to_string(&status).unwrap();
        let deserialized: ClusterStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, deserialized);
    }

    #[test]
    fn cluster_status_type_rename() {
        let json = r#"{"type": "node", "name": "pve1"}"#;
        let status: ClusterStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status.status_type.as_deref(), Some("node"));
    }

    #[test]
    fn cluster_log_entry_serde_roundtrip() {
        let entry = ClusterLogEntry {
            uid: Some("1234".to_string()),
            time: Some(1700000000),
            tag: Some("qmstart".to_string()),
            node: Some("pve1".to_string()),
            pid: Some(12345),
            user: Some("root@pam".to_string()),
            pri: Some(6),
            msg: Some("VM 100 started".to_string()),
        };

        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: ClusterLogEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(entry, deserialized);
    }
}
