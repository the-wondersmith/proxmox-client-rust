use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_node_name;

/// Node-level replication job status.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NodeReplicationStatus {
    /// Replication job ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Guest type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub guest_type: Option<String>,

    /// Source node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Target node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Guest VMID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmid: Option<u32>,

    /// Last successful sync timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync: Option<i64>,

    /// Last attempt timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_try: Option<i64>,

    /// Next scheduled sync timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_sync: Option<i64>,

    /// Duration of last sync in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,

    /// Number of consecutive failures.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_count: Option<i64>,

    /// Last error message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// Whether the job is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub enabled: Option<bool>,

    /// Schedule expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl ProxmoxClient {
    /// Lists replication jobs on a node.
    ///
    /// `GET /nodes/{node}/replication`
    pub async fn list_node_replication(&self, node: &str) -> Result<Vec<NodeReplicationStatus>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/replication"),
            &format!("node {node} replication"),
        )
        .await
    }

    /// Returns the index of a specific replication job on a node.
    ///
    /// `GET /nodes/{node}/replication/{id}`
    pub async fn get_node_replication_job(&self, node: &str, id: &str) -> Result<Vec<Value>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/replication/{id}"),
            &format!("node {node} replication {id}"),
        )
        .await
    }

    /// Gets the status of a specific replication job on a node.
    ///
    /// `GET /nodes/{node}/replication/{id}/status`
    pub async fn get_node_replication_status(
        &self,
        node: &str,
        id: &str,
    ) -> Result<NodeReplicationStatus> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/replication/{id}/status"),
            &format!("node {node} replication {id} status"),
        )
        .await
    }

    /// Gets the log of a specific replication job on a node.
    ///
    /// `GET /nodes/{node}/replication/{id}/log`
    pub async fn get_node_replication_log(&self, node: &str, id: &str) -> Result<Vec<Value>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/replication/{id}/log"),
            &format!("node {node} replication {id} log"),
        )
        .await
    }

    /// Schedules a replication job to run now.
    ///
    /// `POST /nodes/{node}/replication/{id}/schedule_now`
    pub async fn schedule_replication_now(&self, node: &str, id: &str) -> Result<String> {
        validate_node_name(node)?;
        let response = self
            .post(&format!("/nodes/{node}/replication/{id}/schedule_now"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("replication {id} schedule now")).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_replication_status_serde_roundtrip() {
        let status = NodeReplicationStatus {
            id: Some("100-0".to_string()),
            guest_type: Some("qemu".to_string()),
            source: Some("pve1".to_string()),
            target: Some("pve2".to_string()),
            vmid: Some(100),
            last_sync: Some(1700000000),
            duration: Some(5.2),
            fail_count: Some(0),
            ..Default::default()
        };

        let json = serde_json::to_string(&status).unwrap();
        let deserialized: NodeReplicationStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, deserialized);
    }

    #[test]
    fn node_replication_status_skip_serializing_none() {
        let status = NodeReplicationStatus::default();
        let json = serde_json::to_value(&status).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default NodeReplicationStatus should serialize to {{}}"
        );
    }

    #[test]
    fn node_replication_status_type_rename() {
        let json = r#"{"id": "100-0", "type": "qemu"}"#;
        let status: NodeReplicationStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status.guest_type.as_deref(), Some("qemu"));

        let serialized = serde_json::to_value(&status).unwrap();
        assert!(serialized.get("type").is_some());
        assert!(serialized.get("guest_type").is_none());
    }

    #[test]
    fn node_replication_status_unknown_fields_ignored() {
        let json = r#"{"id": "100-0", "unknownField": true}"#;
        let status: NodeReplicationStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status.id.as_deref(), Some("100-0"));
    }
}
