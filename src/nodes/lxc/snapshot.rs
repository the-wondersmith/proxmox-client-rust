use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::nodes::qemu::snapshot::{Snapshot, SnapshotCreateParams};
use crate::validation::{validate_node_name, validate_resource_id, validate_vmid};

/// Backward compatibility alias.
pub type ContainerSnapshot = Snapshot;
/// Backward compatibility alias.
pub type ContainerSnapshotCreateParams = SnapshotCreateParams;

impl ProxmoxClient {
    /// Lists all snapshots of an LXC container.
    pub async fn list_container_snapshots(&self, node: &str, vmid: u32) -> Result<Vec<Snapshot>> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/lxc/{vmid}/snapshot"),
            &format!("container {vmid} snapshots"),
        )
        .await
    }

    /// Creates a snapshot of an LXC container.
    pub async fn create_container_snapshot(
        &self,
        node: &str,
        vmid: u32,
        params: &SnapshotCreateParams,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.post_parsed(
            &format!("/nodes/{node}/lxc/{vmid}/snapshot"),
            params,
            &format!("container {vmid} snapshot"),
        )
        .await
    }

    /// Returns configuration of a container snapshot.
    pub async fn get_container_snapshot_config(
        &self,
        node: &str,
        vmid: u32,
        snapname: &str,
    ) -> Result<Snapshot> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        validate_resource_id(snapname)?;
        self.get_parsed(
            &format!("/nodes/{node}/lxc/{vmid}/snapshot/{snapname}/config"),
            &format!("container {vmid} snapshot {snapname}"),
        )
        .await
    }

    /// Updates the description of a container snapshot.
    pub async fn update_container_snapshot_description(
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
                "/nodes/{node}/lxc/{vmid}/snapshot/{snapname}/config"
            ))?
            .json(&params)
            .send()
            .await?;
        Self::handle_error(response, &format!("container {vmid} snapshot {snapname}")).await?;
        Ok(())
    }

    /// Deletes a container snapshot.
    pub async fn delete_container_snapshot(
        &self,
        node: &str,
        vmid: u32,
        snapname: &str,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        validate_resource_id(snapname)?;
        let response = self
            .delete(&format!("/nodes/{node}/lxc/{vmid}/snapshot/{snapname}"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("container {vmid} snapshot {snapname}")).await
    }

    /// Rolls back a container to a snapshot.
    pub async fn rollback_container_snapshot(
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
                "/nodes/{node}/lxc/{vmid}/snapshot/{snapname}/rollback"
            ))?
            .send()
            .await?;
        Self::parse_response(response, &format!("container {vmid} rollback {snapname}")).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_snapshot_serde_roundtrip() {
        let json = r#"{
            "name": "backup",
            "description": "Before update",
            "snaptime": 1700000000,
            "parent": "initial"
        }"#;
        let snap: Snapshot = serde_json::from_str(json).unwrap();
        assert_eq!(snap.name.as_deref(), Some("backup"));

        let serialized = serde_json::to_string(&snap).unwrap();
        let deserialized: Snapshot = serde_json::from_str(&serialized).unwrap();
        assert_eq!(snap, deserialized);
    }

    #[test]
    fn container_snapshot_create_params_serialize() {
        let params = SnapshotCreateParams {
            snapname: "test-snap".to_string(),
            description: Some("Test snapshot".to_string()),
            vmstate: None,
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["snapname"], "test-snap");
        assert_eq!(json["description"], "Test snapshot");
    }

    #[test]
    fn backward_compat_type_aliases() {
        // ContainerSnapshot is just a type alias for Snapshot.
        let snap = ContainerSnapshot {
            name: Some("test".to_string()),
            ..Default::default()
        };
        assert_eq!(snap.name.as_deref(), Some("test"));
    }
}
