use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::{validate_node_name, validate_vmid};

/// Parameters for migrating an LXC container.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ContainerMigrateParams {
    /// Target node name.
    pub target: String,

    /// Online migration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<bool>,

    /// Force migration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,

    /// Restart the container after migration (for offline migration).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<bool>,

    /// Target storage mapping.
    #[serde(rename = "target-storage", skip_serializing_if = "Option::is_none")]
    pub targetstorage: Option<String>,

    /// Override I/O bandwidth limit (in KiB/s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bwlimit: Option<f64>,

    /// Timeout in seconds for shutdown for restart migration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
}

impl ContainerMigrateParams {
    /// Creates a new `ContainerMigrateParams` with the required fields.
    pub fn new(target: impl Into<String>) -> Self {
        Self {
            target: target.into(),
            ..Default::default()
        }
    }
}

/// Preconditions check result for container migration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ContainerMigrationPreconditions {
    /// Whether migration is possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<bool>,

    /// List of allowed nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_nodes: Option<Vec<String>>,

    /// Local volumes that need migration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_disks: Option<Vec<Value>>,

    /// Local resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_resources: Option<Vec<Value>>,
}

impl ProxmoxClient {
    /// Migrates an LXC container to another node.
    pub async fn migrate_container(
        &self,
        node: &str,
        vmid: u32,
        params: &ContainerMigrateParams,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        validate_node_name(&params.target)?;
        self.post_parsed(
            &format!("/nodes/{node}/lxc/{vmid}/migrate"),
            params,
            &format!("container {vmid} migrate"),
        )
        .await
    }

    /// Checks migration preconditions for an LXC container.
    pub async fn check_container_migration(
        &self,
        node: &str,
        vmid: u32,
    ) -> Result<ContainerMigrationPreconditions> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/lxc/{vmid}/migrate"),
            &format!("container {vmid} migration check"),
        )
        .await
    }
}

/// Parameters for migrating an LXC container to a remote cluster (EXPERIMENTAL).
#[derive(Debug, Clone, Default, Serialize)]
pub struct ContainerRemoteMigrateParams {
    /// Remote target endpoint.
    #[serde(rename = "target-endpoint")]
    pub target_endpoint: String,

    /// Mapping from source to target storages.
    #[serde(rename = "target-storage")]
    pub target_storage: String,

    /// Mapping from source to target bridges.
    #[serde(rename = "target-bridge")]
    pub target_bridge: String,

    /// Use online/live migration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<bool>,

    /// Use restart migration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<bool>,

    /// Delete the original CT after successful migration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,

    /// Target VM ID on the remote cluster.
    #[serde(rename = "target-vmid", skip_serializing_if = "Option::is_none")]
    pub target_vmid: Option<u32>,

    /// Override I/O bandwidth limit (in KiB/s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bwlimit: Option<f64>,

    /// Timeout in seconds for shutdown for restart migration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
}

impl ContainerRemoteMigrateParams {
    /// Creates a new `ContainerRemoteMigrateParams` with the required fields.
    pub fn new(
        target_endpoint: impl Into<String>,
        target_storage: impl Into<String>,
        target_bridge: impl Into<String>,
    ) -> Self {
        Self {
            target_endpoint: target_endpoint.into(),
            target_storage: target_storage.into(),
            target_bridge: target_bridge.into(),
            ..Default::default()
        }
    }
}

impl ProxmoxClient {
    /// Migrates an LXC container to a remote cluster (EXPERIMENTAL).
    ///
    /// `POST /nodes/{node}/lxc/{vmid}/remote_migrate`
    pub async fn remote_migrate_container(
        &self,
        node: &str,
        vmid: u32,
        params: &ContainerRemoteMigrateParams,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.post_parsed(
            &format!("/nodes/{node}/lxc/{vmid}/remote_migrate"),
            params,
            &format!("container {vmid} remote migrate"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_migrate_params_serialize() {
        let mut params = ContainerMigrateParams::new("pve2");
        params.online = Some(true);
        params.restart = Some(false);
        params.targetstorage = Some("local-lvm".to_string());
        params.bwlimit = Some(10240.0);
        params.timeout = Some(300);
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["target"], "pve2");
        assert_eq!(json["online"], true);
        assert!(json.get("force").is_none());
        assert_eq!(json["target-storage"], "local-lvm");
        assert!(json.get("targetstorage").is_none());
        assert_eq!(json["bwlimit"], 10240.0);
        assert_eq!(json["timeout"], 300);
    }

    #[test]
    fn container_remote_migrate_params_serialize() {
        let params = ContainerRemoteMigrateParams::new(
            "apitoken=PVEAPIToken=user@pam!token=SECRET,host=10.0.0.1",
            "local-lvm",
            "vmbr0",
        );
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(
            json["target-endpoint"],
            "apitoken=PVEAPIToken=user@pam!token=SECRET,host=10.0.0.1"
        );
        assert_eq!(json["target-storage"], "local-lvm");
        assert_eq!(json["target-bridge"], "vmbr0");
        assert!(json.get("online").is_none());
        assert!(json.get("delete").is_none());
    }
}
