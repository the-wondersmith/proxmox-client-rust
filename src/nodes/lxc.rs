//! LXC container management.

/// LXC container configuration (get/update).
pub mod config;
/// LXC container firewall rules, aliases, IP sets, and options.
pub mod firewall;
/// LXC container migration.
pub mod migrate;
/// LXC container snapshots.
pub mod snapshot;
/// LXC container status and power actions.
pub mod status;

pub use config::{ContainerConfig, ContainerConfigUpdateParams};
pub use firewall as ct_firewall;
pub use migrate::{
    ContainerMigrateParams, ContainerMigrationPreconditions, ContainerRemoteMigrateParams,
};
pub use snapshot::{ContainerSnapshot, ContainerSnapshotCreateParams};
pub use status::{
    ContainerShutdownParams, ContainerStartParams, ContainerStatus, ContainerStopParams,
};

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use urlencoding::encode;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::nodes::qemu::{SpiceProxyData, VncProxyData};
use crate::nodes::status::RrdData;
use crate::validation::{validate_node_name, validate_vmid};

/// Summary of an LXC container as returned by `GET /nodes/{node}/lxc`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ContainerSummary {
    /// Container ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmid: Option<u32>,

    /// Container name / hostname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Current status (`running`, `stopped`, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Maximum memory in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxmem: Option<u64>,

    /// Maximum swap in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxswap: Option<u64>,

    /// Maximum disk size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxdisk: Option<u64>,

    /// Number of CPUs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<u32>,

    /// Uptime in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime: Option<u64>,

    /// Current CPU usage (0.0 - 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f64>,

    /// Current memory usage in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem: Option<u64>,

    /// Current swap usage in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap: Option<u64>,

    /// Current disk usage in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<u64>,

    /// Network in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netin: Option<u64>,

    /// Network out bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netout: Option<u64>,

    /// Whether this is a template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<u32>,

    /// PID of the running container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<u32>,

    /// Tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,

    /// Lock status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<String>,

    /// Container type (e.g., `lxc`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub ct_type: Option<String>,
}

/// Parameters for creating an LXC container.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ContainerCreateParams {
    /// Container ID (100-999999999).
    pub vmid: u32,

    /// OS template (e.g., `local:vztmpl/ubuntu-22.04-standard_22.04-1_amd64.tar.zst`).
    pub ostemplate: String,

    /// Hostname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    /// Memory in MB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<u64>,

    /// Swap in MB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap: Option<u64>,

    /// Number of CPU cores.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores: Option<u32>,

    /// Root filesystem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rootfs: Option<String>,

    /// Storage for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,

    /// Root password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// SSH public keys.
    #[serde(rename = "ssh-public-keys", skip_serializing_if = "Option::is_none")]
    pub ssh_public_keys: Option<String>,

    /// Unprivileged container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprivileged: Option<bool>,

    /// Start on boot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboot: Option<bool>,

    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,

    /// Nameserver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nameserver: Option<String>,

    /// Search domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchdomain: Option<String>,

    /// Start the container after creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<bool>,

    /// Pool to add the container to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,

    /// Features (e.g., `nesting=1`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<String>,

    /// OS type (e.g., `ubuntu`, `debian`, `centos`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ostype: Option<String>,

    /// CPU limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpulimit: Option<f64>,

    /// CPU units (weight).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuunits: Option<u32>,

    /// Additional indexed parameters (net0, mp0, etc.).
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl ContainerCreateParams {
    /// Creates a new `ContainerCreateParams` with the required fields.
    pub fn new(vmid: u32, ostemplate: impl Into<String>) -> Self {
        Self {
            vmid,
            ostemplate: ostemplate.into(),
            ..Default::default()
        }
    }
}

/// Parameters for cloning a container.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ContainerCloneParams {
    /// Target VMID for the clone.
    pub newid: u32,

    /// Hostname for the clone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    /// Description for the clone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Target node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Target storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,

    /// Full clone (not linked).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full: Option<bool>,

    /// Pool to add the clone to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,

    /// Snapshot name to clone from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapname: Option<String>,

    /// Bandwidth limit in KiB/s.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bwlimit: Option<u64>,
}

impl ContainerCloneParams {
    /// Creates a new `ContainerCloneParams` with the required fields.
    pub fn new(newid: u32) -> Self {
        Self {
            newid,
            ..Default::default()
        }
    }
}

/// Parameters for moving a volume.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ContainerMoveVolumeParams {
    /// Volume to move (e.g., `rootfs`, `mp0`).
    pub volume: String,

    /// Target storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,

    /// Target VMID (for moving to another container).
    #[serde(rename = "target-vmid", skip_serializing_if = "Option::is_none")]
    pub target_vmid: Option<u32>,

    /// Target volume.
    #[serde(rename = "target-volume", skip_serializing_if = "Option::is_none")]
    pub target_volume: Option<String>,

    /// Delete the original volume.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,

    /// Bandwidth limit in KiB/s.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bwlimit: Option<u64>,

    /// Prevent changes if current configuration differs (SHA1 digest).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

impl ContainerMoveVolumeParams {
    /// Creates a new `ContainerMoveVolumeParams` with the required fields.
    pub fn new(volume: impl Into<String>) -> Self {
        Self {
            volume: volume.into(),
            ..Default::default()
        }
    }
}

/// Parameters for resizing a container disk.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ContainerResizeParams {
    /// Disk to resize (e.g., `rootfs`, `mp0`).
    pub disk: String,

    /// New size (e.g., `+10G` or `50G`).
    pub size: String,
}

impl ContainerResizeParams {
    /// Creates a new `ContainerResizeParams` with the required fields.
    pub fn new(disk: impl Into<String>, size: impl Into<String>) -> Self {
        Self {
            disk: disk.into(),
            size: size.into(),
        }
    }
}

/// A container network interface.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ContainerInterface {
    /// Interface name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Hardware (MAC) address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwaddr: Option<String>,

    /// IPv4 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet: Option<String>,

    /// IPv6 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet6: Option<String>,
}

/// Pending configuration changes for a container.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ContainerPendingChange {
    /// Configuration key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// Current value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,

    /// Pending value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<Value>,

    /// Delete flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<u32>,
}

/// Parameters for deleting a container.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ContainerDeleteParams {
    /// Destroy unreferenced disks owned by the container.
    #[serde(
        rename = "destroy-unreferenced-disks",
        skip_serializing_if = "Option::is_none"
    )]
    pub destroy_unreferenced_disks: Option<bool>,

    /// Force destroy even if running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,

    /// Remove from HA, replication, and backup jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purge: Option<bool>,
}

impl ProxmoxClient {
    // ── LXC container lifecycle ────────────────────────────────────

    /// Lists all LXC containers on a node.
    pub async fn list_containers(&self, node: &str) -> Result<Vec<ContainerSummary>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/lxc"),
            &format!("node {node} containers"),
        )
        .await
    }

    /// Creates a new LXC container.
    pub async fn create_container(
        &self,
        node: &str,
        params: &ContainerCreateParams,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(params.vmid)?;
        self.post_parsed(
            &format!("/nodes/{node}/lxc"),
            params,
            &format!("create container {}", params.vmid),
        )
        .await
    }

    /// Deletes an LXC container.
    pub async fn delete_container(
        &self,
        node: &str,
        vmid: u32,
        params: Option<&ContainerDeleteParams>,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let mut path = format!("/nodes/{node}/lxc/{vmid}");
        if let Some(p) = params {
            let qs = Self::serialize_as_query_string(p);
            if !qs.is_empty() {
                path.push('?');
                path.push_str(&qs);
            }
        }
        let response = self.delete(&path)?.send().await?;
        Self::parse_response(response, &format!("container {vmid}")).await
    }

    // ── LXC container operations ───────────────────────────────────

    /// Clones an LXC container.
    pub async fn clone_container(
        &self,
        node: &str,
        vmid: u32,
        params: &ContainerCloneParams,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        validate_vmid(params.newid)?;
        self.post_parsed(
            &format!("/nodes/{node}/lxc/{vmid}/clone"),
            params,
            &format!("clone container {vmid}"),
        )
        .await
    }

    /// Converts an LXC container to a template.
    pub async fn convert_container_to_template(&self, node: &str, vmid: u32) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/lxc/{vmid}/template"))?
            .send()
            .await?;
        Self::handle_error(response, &format!("container {vmid} template")).await?;
        Ok(())
    }

    /// Moves a container volume to different storage.
    pub async fn move_container_volume(
        &self,
        node: &str,
        vmid: u32,
        params: &ContainerMoveVolumeParams,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.post_parsed(
            &format!("/nodes/{node}/lxc/{vmid}/move_volume"),
            params,
            &format!("container {vmid} move volume"),
        )
        .await
    }

    /// Resizes a container disk.
    pub async fn resize_container_disk(
        &self,
        node: &str,
        vmid: u32,
        params: &ContainerResizeParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .put(&format!("/nodes/{node}/lxc/{vmid}/resize"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("container {vmid} resize")).await?;
        Ok(())
    }

    /// Creates a VNC proxy for a container.
    pub async fn create_container_vnc_proxy(&self, node: &str, vmid: u32) -> Result<VncProxyData> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/lxc/{vmid}/vncproxy"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("container {vmid} VNC proxy")).await
    }

    /// Creates a SPICE proxy for a container.
    pub async fn create_container_spice_proxy(
        &self,
        node: &str,
        vmid: u32,
    ) -> Result<SpiceProxyData> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/lxc/{vmid}/spiceproxy"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("container {vmid} SPICE proxy")).await
    }

    /// Creates a terminal proxy for a container.
    pub async fn create_container_term_proxy(&self, node: &str, vmid: u32) -> Result<VncProxyData> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/lxc/{vmid}/termproxy"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("container {vmid} terminal proxy")).await
    }

    /// Returns RRD data for a container.
    pub async fn get_container_rrddata(
        &self,
        node: &str,
        vmid: u32,
        timeframe: &str,
        cf: Option<&str>,
    ) -> Result<Vec<RrdData>> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let mut path = format!(
            "/nodes/{node}/lxc/{vmid}/rrddata?timeframe={}",
            encode(timeframe)
        );
        if let Some(cf) = cf {
            path.push_str(&format!("&cf={}", encode(cf)));
        }
        self.get_parsed(&path, &format!("container {vmid} rrddata"))
            .await
    }

    /// Checks if a container feature is supported.
    ///
    /// `GET /nodes/{node}/lxc/{vmid}/feature`
    pub async fn check_container_feature(
        &self,
        node: &str,
        vmid: u32,
        feature: &str,
    ) -> Result<Value> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!(
                "/nodes/{node}/lxc/{vmid}/feature?feature={}",
                encode(feature)
            ),
            &format!("container {vmid} feature {feature}"),
        )
        .await
    }

    /// Gets the network interfaces of a running container.
    ///
    /// `GET /nodes/{node}/lxc/{vmid}/interfaces`
    pub async fn get_container_interfaces(
        &self,
        node: &str,
        vmid: u32,
    ) -> Result<Vec<ContainerInterface>> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/lxc/{vmid}/interfaces"),
            &format!("container {vmid} interfaces"),
        )
        .await
    }

    /// Opens a websocket for VNC traffic for a container.
    ///
    /// `GET /nodes/{node}/lxc/{vmid}/vncwebsocket`
    pub async fn get_container_vnc_websocket(
        &self,
        node: &str,
        vmid: u32,
        port: &str,
        vncticket: &str,
    ) -> Result<Value> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!(
                "/nodes/{node}/lxc/{vmid}/vncwebsocket?port={}&vncticket={}",
                encode(port),
                encode(vncticket)
            ),
            &format!("container {vmid} VNC websocket"),
        )
        .await
    }

    /// Returns pending configuration changes for a container.
    pub async fn get_container_pending(
        &self,
        node: &str,
        vmid: u32,
    ) -> Result<Vec<ContainerPendingChange>> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/lxc/{vmid}/pending"),
            &format!("container {vmid} pending"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_summary_serde_roundtrip() {
        let json = r#"{
            "vmid": 200,
            "name": "test-ct",
            "status": "running",
            "maxmem": 1073741824,
            "maxswap": 536870912,
            "maxdisk": 8589934592,
            "cpus": 2,
            "uptime": 3600,
            "cpu": 0.05,
            "mem": 268435456,
            "swap": 0,
            "netin": 1024,
            "netout": 512,
            "template": 0,
            "type": "lxc"
        }"#;
        let ct: ContainerSummary = serde_json::from_str(json).unwrap();
        assert_eq!(ct.vmid, Some(200));
        assert_eq!(ct.name.as_deref(), Some("test-ct"));
        assert_eq!(ct.ct_type.as_deref(), Some("lxc"));

        let serialized = serde_json::to_string(&ct).unwrap();
        let deserialized: ContainerSummary = serde_json::from_str(&serialized).unwrap();
        assert_eq!(ct, deserialized);
    }

    #[test]
    fn container_create_params_serialize() {
        let mut params =
            ContainerCreateParams::new(200, "local:vztmpl/ubuntu-22.04-standard.tar.zst");
        params.hostname = Some("test-ct".to_string());
        params.memory = Some(1024);
        params.cores = Some(2);
        params.rootfs = Some("local-lvm:8".to_string());
        params.unprivileged = Some(true);
        params.extra.insert(
            "net0".to_string(),
            Value::String("name=eth0,bridge=vmbr0,ip=dhcp".to_string()),
        );
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["vmid"], 200);
        assert_eq!(json["hostname"], "test-ct");
        assert_eq!(json["net0"], "name=eth0,bridge=vmbr0,ip=dhcp");
    }
}
