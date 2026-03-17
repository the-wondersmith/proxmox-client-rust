use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::{validate_node_name, validate_vmid};

/// Parameters for migrating a QEMU VM.
#[derive(Debug, Clone, Default, Serialize)]
pub struct VmMigrateParams {
    /// Target node name.
    pub target: String,

    /// Online migration (live migration).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<bool>,

    /// Force migration even with local resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,

    /// Use SSH tunnel for migration.
    #[serde(rename = "with-local-disks", skip_serializing_if = "Option::is_none")]
    pub with_local_disks: Option<bool>,

    /// Target storage mapping (e.g., `local-lvm:remote-lvm`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targetstorage: Option<String>,

    /// Migration network CIDR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_network: Option<String>,

    /// Migration type (`secure`, `insecure`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_type: Option<String>,

    /// Migrate conntrack table entries.
    #[serde(
        rename = "with-conntrack-state",
        skip_serializing_if = "Option::is_none"
    )]
    pub with_conntrack_state: Option<bool>,

    /// Override I/O bandwidth limit (in KiB/s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bwlimit: Option<u64>,
}

impl VmMigrateParams {
    /// Creates a new `VmMigrateParams` with the required fields.
    pub fn new(target: impl Into<String>) -> Self {
        Self {
            target: target.into(),
            ..Default::default()
        }
    }
}

/// Preconditions check result for VM migration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VmMigrationPreconditions {
    /// Whether migration is possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<bool>,

    /// List of allowed nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_nodes: Option<Vec<String>>,

    /// Local disks that would need to be migrated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_disks: Option<Vec<Value>>,

    /// Local resources that prevent migration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_resources: Option<Vec<Value>>,

    /// Not allowed nodes with reasons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_allowed_nodes: Option<Value>,
}

impl ProxmoxClient {
    /// Migrates a QEMU VM to another node.
    pub async fn migrate_vm(
        &self,
        node: &str,
        vmid: u32,
        params: &VmMigrateParams,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        validate_node_name(&params.target)?;
        self.post_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/migrate"),
            params,
            &format!("VM {vmid} migrate"),
        )
        .await
    }

    /// Checks migration preconditions for a QEMU VM.
    pub async fn check_vm_migration(
        &self,
        node: &str,
        vmid: u32,
    ) -> Result<VmMigrationPreconditions> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/migrate"),
            &format!("VM {vmid} migration check"),
        )
        .await
    }
}

/// Parameters for migrating a QEMU VM to a remote cluster (EXPERIMENTAL).
#[derive(Debug, Clone, Default, Serialize)]
pub struct VmRemoteMigrateParams {
    /// Remote target endpoint.
    #[serde(rename = "target-endpoint")]
    pub target_endpoint: String,

    /// Mapping from source to target storages.
    #[serde(rename = "target-storage")]
    pub target_storage: String,

    /// Mapping from source to target bridges.
    #[serde(rename = "target-bridge")]
    pub target_bridge: String,

    /// Use online/live migration if VM is running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<bool>,

    /// Delete the original VM after successful migration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,

    /// Target VM ID on the remote cluster.
    #[serde(rename = "target-vmid", skip_serializing_if = "Option::is_none")]
    pub target_vmid: Option<u32>,

    /// Override I/O bandwidth limit (in KiB/s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bwlimit: Option<u64>,
}

impl VmRemoteMigrateParams {
    /// Creates a new `VmRemoteMigrateParams` with the required fields.
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
    /// Migrates a QEMU VM to a remote cluster (EXPERIMENTAL).
    ///
    /// `POST /nodes/{node}/qemu/{vmid}/remote_migrate`
    pub async fn remote_migrate_vm(
        &self,
        node: &str,
        vmid: u32,
        params: &VmRemoteMigrateParams,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.post_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/remote_migrate"),
            params,
            &format!("VM {vmid} remote migrate"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vm_migrate_params_serialize() {
        let mut params = VmMigrateParams::new("pve2");
        params.online = Some(true);
        params.with_local_disks = Some(true);
        params.bwlimit = Some(10240);
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["target"], "pve2");
        assert_eq!(json["online"], true);
        assert_eq!(json["with-local-disks"], true);
        assert_eq!(json["bwlimit"], 10240);
    }

    #[test]
    fn vm_remote_migrate_params_serialize() {
        let params = VmRemoteMigrateParams::new(
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
        assert!(json.get("bwlimit").is_none());
    }

    #[test]
    fn vm_migration_preconditions_serde_roundtrip() {
        let json = r#"{
            "running": true,
            "allowed_nodes": ["pve2", "pve3"],
            "local_disks": [],
            "local_resources": []
        }"#;
        let preconds: VmMigrationPreconditions = serde_json::from_str(json).unwrap();
        assert_eq!(preconds.running, Some(true));
        assert_eq!(
            preconds.allowed_nodes.as_deref(),
            Some(vec!["pve2".to_string(), "pve3".to_string()].as_slice())
        );

        let serialized = serde_json::to_string(&preconds).unwrap();
        let deserialized: VmMigrationPreconditions = serde_json::from_str(&serialized).unwrap();
        assert_eq!(preconds, deserialized);
    }
}
