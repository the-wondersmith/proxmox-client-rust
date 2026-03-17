use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_node_name;

/// Cluster configuration information.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ClusterConfig {
    /// Cluster name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,

    /// Cluster version/config version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,

    /// Number of nodes in the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<i64>,

    /// Number of quorate votes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quorate: Option<i64>,

    /// Additional configuration data.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Parameters for creating a new cluster.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct ClusterCreateParams {
    /// The cluster name (required).
    pub clustername: String,

    /// Node ID for the local node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeid: Option<i64>,

    /// Number of votes for this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub votes: Option<i64>,

    /// Corosync link address(es).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link0: Option<String>,

    /// Second Corosync link address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link1: Option<String>,
}

impl ClusterCreateParams {
    /// Creates a new `ClusterCreateParams` with the required fields.
    pub fn new(clustername: impl Into<String>) -> Self {
        Self {
            clustername: clustername.into(),
            ..Default::default()
        }
    }
}

/// Information needed to join an existing cluster.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ClusterJoinInfo {
    /// Configuration file data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configdigest: Option<String>,

    /// Preferred node for joining.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_node: Option<String>,

    /// Totem configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totem: Option<serde_json::Value>,

    /// Node list with addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodelist: Option<Vec<serde_json::Value>>,
}

/// Parameters for joining a cluster.
#[derive(Clone, Default, Serialize)]
#[non_exhaustive]
pub struct ClusterJoinParams {
    /// Hostname or IP of an existing cluster node (required).
    pub hostname: String,

    /// Superuser password of the existing cluster node (required).
    pub password: String,

    /// SSL fingerprint of the existing cluster node (required).
    pub fingerprint: String,

    /// Node ID for the joining node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeid: Option<i64>,

    /// Number of votes for this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub votes: Option<i64>,

    /// Force join even if issues are detected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<i32>,

    /// Corosync link address(es).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link0: Option<String>,

    /// Second Corosync link address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link1: Option<String>,
}

impl ClusterJoinParams {
    /// Creates a new `ClusterJoinParams` with the required fields.
    pub fn new(
        hostname: impl Into<String>,
        password: impl Into<String>,
        fingerprint: impl Into<String>,
    ) -> Self {
        Self {
            hostname: hostname.into(),
            password: password.into(),
            fingerprint: fingerprint.into(),
            ..Default::default()
        }
    }
}

impl std::fmt::Debug for ClusterJoinParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClusterJoinParams")
            .field("hostname", &self.hostname)
            .field("password", &"<redacted>")
            .field("fingerprint", &self.fingerprint)
            .field("nodeid", &self.nodeid)
            .field("votes", &self.votes)
            .field("force", &self.force)
            .field("link0", &self.link0)
            .field("link1", &self.link1)
            .finish()
    }
}

/// A node in the cluster configuration.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ClusterNode {
    /// Node name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Node ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeid: Option<i64>,

    /// Number of votes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub votes: Option<i64>,

    /// Whether the node is quorate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quorate: Option<i32>,

    /// Node ring addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ring0_addr: Option<String>,

    /// Second ring address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ring1_addr: Option<String>,
}

/// Corosync totem configuration.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TotemConfig {
    /// Cluster name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,

    /// Config version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_version: Option<i64>,

    /// Crypto cipher.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secauth: Option<String>,

    /// Interface configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface: Option<serde_json::Value>,

    /// IP version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_version: Option<String>,

    /// Link mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_mode: Option<String>,
}

impl ProxmoxClient {
    /// Gets the cluster configuration.
    ///
    /// `GET /cluster/config`
    pub async fn get_cluster_config(&self) -> Result<Vec<ClusterConfig>> {
        self.get_parsed("/cluster/config", "cluster config").await
    }

    /// Creates a new cluster.
    ///
    /// `POST /cluster/config`
    pub async fn create_cluster(&self, params: &ClusterCreateParams) -> Result<String> {
        self.post_parsed("/cluster/config", params, "cluster creation")
            .await
    }

    /// Gets the cluster join information.
    ///
    /// `GET /cluster/config/join`
    pub async fn get_cluster_join_info(&self) -> Result<ClusterJoinInfo> {
        self.get_parsed("/cluster/config/join", "cluster join info")
            .await
    }

    /// Joins an existing cluster.
    ///
    /// `POST /cluster/config/join`
    pub async fn join_cluster(&self, params: &ClusterJoinParams) -> Result<String> {
        self.post_parsed("/cluster/config/join", params, "cluster join")
            .await
    }

    /// Lists nodes in the cluster configuration.
    ///
    /// `GET /cluster/config/nodes`
    pub async fn list_cluster_config_nodes(&self) -> Result<Vec<ClusterNode>> {
        self.get_parsed("/cluster/config/nodes", "cluster config nodes")
            .await
    }

    /// Adds a node to the cluster configuration.
    ///
    /// `POST /cluster/config/nodes/{node}`
    pub async fn add_cluster_config_node(
        &self,
        node: &str,
        params: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        validate_node_name(node)?;
        self.post_parsed(
            &format!("/cluster/config/nodes/{node}"),
            params,
            &format!("cluster config node {node}"),
        )
        .await
    }

    /// Removes a node from the cluster configuration.
    ///
    /// `DELETE /cluster/config/nodes/{node}`
    pub async fn remove_cluster_config_node(&self, node: &str) -> Result<()> {
        validate_node_name(node)?;
        self.delete_void(
            &format!("/cluster/config/nodes/{node}"),
            &format!("cluster config node {node}"),
        )
        .await
    }

    /// Gets the Corosync totem configuration.
    ///
    /// `GET /cluster/config/totem`
    pub async fn get_cluster_totem_config(&self) -> Result<TotemConfig> {
        self.get_parsed("/cluster/config/totem", "cluster totem config")
            .await
    }

    /// Gets the QDevice status.
    ///
    /// `GET /cluster/config/qdevice`
    pub async fn get_cluster_qdevice_status(&self) -> Result<serde_json::Value> {
        self.get_parsed("/cluster/config/qdevice", "cluster qdevice")
            .await
    }

    /// Gets the cluster API version.
    ///
    /// `GET /cluster/config/apiversion`
    pub async fn get_cluster_api_version(&self) -> Result<serde_json::Value> {
        self.get_parsed("/cluster/config/apiversion", "cluster API version")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cluster_config_serde_roundtrip() {
        let config = ClusterConfig {
            cluster_name: Some("testcluster".to_string()),
            version: Some(3),
            nodes: Some(3),
            quorate: Some(1),
            extra: std::collections::HashMap::new(),
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: ClusterConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config, deserialized);
    }

    #[test]
    fn cluster_config_skip_serializing_none() {
        let config = ClusterConfig::default();
        let json = serde_json::to_value(&config).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default ClusterConfig should serialize to {{}}"
        );
    }

    #[test]
    fn cluster_config_unknown_fields_ignored() {
        let json = r#"{"cluster_name": "test", "unknownField": true}"#;
        let config: ClusterConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.cluster_name.as_deref(), Some("test"));
    }

    #[test]
    fn cluster_create_params_serialization() {
        let params = ClusterCreateParams {
            clustername: "mycluster".to_string(),
            nodeid: Some(1),
            votes: Some(1),
            link0: Some("10.0.0.1".to_string()),
            link1: None,
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["clustername"], "mycluster");
        assert_eq!(json["nodeid"], 1);
        assert_eq!(json["votes"], 1);
        assert_eq!(json["link0"], "10.0.0.1");
        assert!(!json.as_object().unwrap().contains_key("link1"));
    }

    #[test]
    fn cluster_join_info_serde_roundtrip() {
        let info = ClusterJoinInfo {
            configdigest: Some("abc123".to_string()),
            preferred_node: Some("pve1".to_string()),
            totem: None,
            nodelist: None,
        };

        let json = serde_json::to_string(&info).unwrap();
        let deserialized: ClusterJoinInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(info, deserialized);
    }

    #[test]
    fn cluster_join_params_serialization() {
        let mut params = ClusterJoinParams::new("pve1", "secret", "AA:BB:CC:DD");
        params.nodeid = Some(2);
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["hostname"], "pve1");
        assert_eq!(json["password"], "secret");
        assert_eq!(json["fingerprint"], "AA:BB:CC:DD");
        assert_eq!(json["nodeid"], 2);
        assert!(!json.as_object().unwrap().contains_key("force"));
    }

    #[test]
    fn cluster_node_serde_roundtrip() {
        let node = ClusterNode {
            name: Some("pve1".to_string()),
            nodeid: Some(1),
            votes: Some(1),
            quorate: Some(1),
            ring0_addr: Some("10.0.0.1".to_string()),
            ring1_addr: None,
        };

        let json = serde_json::to_string(&node).unwrap();
        let deserialized: ClusterNode = serde_json::from_str(&json).unwrap();
        assert_eq!(node, deserialized);
    }

    #[test]
    fn totem_config_serde_roundtrip() {
        let totem = TotemConfig {
            cluster_name: Some("mycluster".to_string()),
            config_version: Some(2),
            secauth: Some("on".to_string()),
            interface: None,
            ip_version: Some("ipv4".to_string()),
            link_mode: Some("passive".to_string()),
        };

        let json = serde_json::to_string(&totem).unwrap();
        let deserialized: TotemConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(totem, deserialized);
    }

    #[test]
    fn cluster_join_params_debug_redacts_password() {
        let params = ClusterJoinParams::new("pve1", "super-secret", "AA:BB:CC");
        let debug = format!("{params:?}");
        assert!(
            !debug.contains("super-secret"),
            "password must be redacted in Debug output"
        );
        assert!(debug.contains("<redacted>"));
    }
}
