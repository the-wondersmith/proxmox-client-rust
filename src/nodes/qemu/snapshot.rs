use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::{validate_node_name, validate_resource_id, validate_vmid};

/// A VM snapshot.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Snapshot {
    /// Snapshot name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Snapshot creation time (epoch).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snaptime: Option<u64>,

    /// Whether VM state (RAM) is included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmstate: Option<u32>,

    /// Parent snapshot name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
}

/// Parameters for creating a snapshot.
#[derive(Debug, Clone, Default, Serialize)]
pub struct SnapshotCreateParams {
    /// Snapshot name.
    pub snapname: String,

    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Include VM state (RAM).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmstate: Option<bool>,
}

impl SnapshotCreateParams {
    /// Creates a new `SnapshotCreateParams` with the required fields.
    pub fn new(snapname: impl Into<String>) -> Self {
        Self {
            snapname: snapname.into(),
            ..Default::default()
        }
    }
}

impl ProxmoxClient {
    /// Lists all snapshots of a QEMU VM.
    pub async fn list_vm_snapshots(&self, node: &str, vmid: u32) -> Result<Vec<Snapshot>> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/snapshot"),
            &format!("VM {vmid} snapshots"),
        )
        .await
    }

    /// Creates a snapshot of a QEMU VM.
    pub async fn create_vm_snapshot(
        &self,
        node: &str,
        vmid: u32,
        params: &SnapshotCreateParams,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.post_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/snapshot"),
            params,
            &format!("VM {vmid} snapshot"),
        )
        .await
    }

    /// Returns configuration of a VM snapshot.
    pub async fn get_vm_snapshot_config(
        &self,
        node: &str,
        vmid: u32,
        snapname: &str,
    ) -> Result<Snapshot> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        validate_resource_id(snapname)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/snapshot/{snapname}/config"),
            &format!("VM {vmid} snapshot {snapname}"),
        )
        .await
    }

    /// Updates the description of a VM snapshot.
    pub async fn update_vm_snapshot_description(
        &self,
        node: &str,
        vmid: u32,
        snapname: &str,
        description: &str,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        validate_resource_id(snapname)?;
        let params = serde_json::json!({ "description": description });
        let response = self
            .put(&format!(
                "/nodes/{node}/qemu/{vmid}/snapshot/{snapname}/config"
            ))?
            .json(&params)
            .send()
            .await?;
        Self::handle_error(response, &format!("VM {vmid} snapshot {snapname}")).await?;
        Ok(())
    }

    /// Deletes a VM snapshot.
    pub async fn delete_vm_snapshot(
        &self,
        node: &str,
        vmid: u32,
        snapname: &str,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        validate_resource_id(snapname)?;
        let response = self
            .delete(&format!("/nodes/{node}/qemu/{vmid}/snapshot/{snapname}"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("VM {vmid} snapshot {snapname}")).await
    }

    /// Rolls back a VM to a snapshot.
    pub async fn rollback_vm_snapshot(
        &self,
        node: &str,
        vmid: u32,
        snapname: &str,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        validate_resource_id(snapname)?;
        let response = self
            .post(&format!(
                "/nodes/{node}/qemu/{vmid}/snapshot/{snapname}/rollback"
            ))?
            .send()
            .await?;
        Self::parse_response(response, &format!("VM {vmid} rollback {snapname}")).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_serde_roundtrip() {
        let json = r#"{
            "name": "before-upgrade",
            "description": "Snapshot before system upgrade",
            "snaptime": 1700000000,
            "vmstate": 1,
            "parent": "initial"
        }"#;
        let snap: Snapshot = serde_json::from_str(json).unwrap();
        assert_eq!(snap.name.as_deref(), Some("before-upgrade"));
        assert_eq!(snap.vmstate, Some(1));

        let serialized = serde_json::to_string(&snap).unwrap();
        let deserialized: Snapshot = serde_json::from_str(&serialized).unwrap();
        assert_eq!(snap, deserialized);
    }

    #[test]
    fn snapshot_create_params_serialize() {
        let params = SnapshotCreateParams {
            snapname: "test-snap".to_string(),
            description: Some("Test snapshot".to_string()),
            vmstate: Some(true),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["snapname"], "test-snap");
        assert_eq!(json["vmstate"], true);
    }
}
