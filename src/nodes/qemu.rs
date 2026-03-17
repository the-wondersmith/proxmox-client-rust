//! QEMU virtual machine management.

/// QEMU guest agent commands.
pub mod agent;
/// QEMU VM cloud-init support.
pub mod cloudinit;
/// QEMU VM configuration (get/set/update).
pub mod config;
/// QEMU VM firewall rules, aliases, IP sets, and options.
pub mod firewall;
/// QEMU VM migration.
pub mod migrate;
/// QEMU VM snapshots.
pub mod snapshot;

pub use agent::{
    AgentExecParams, AgentExecResponse, AgentExecStatus, AgentFileReadParams,
    AgentFileWriteParams, AgentSetPasswordParams,
};
pub use config::{VmConfig, VmConfigUpdateParams};
pub use firewall::{
    FirewallAlias, FirewallAliasParams, FirewallIpSet, FirewallIpSetCreateParams,
    FirewallIpSetEntry, FirewallIpSetEntryParams, FirewallOptions, FirewallRef, FirewallRule,
    FirewallRuleCreateParams, FirewallRuleParams,
};
pub use migrate::{VmMigrateParams, VmMigrationPreconditions, VmRemoteMigrateParams};
pub use snapshot::{Snapshot, SnapshotCreateParams};

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use urlencoding::encode;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::{validate_node_name, validate_vmid};

/// Summary of a QEMU VM as returned by `GET /nodes/{node}/qemu`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VmSummary {
    /// VM ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmid: Option<u32>,

    /// VM name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Current status (`running`, `stopped`, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Maximum memory in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxmem: Option<u64>,

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

    /// Current disk read in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<u64>,

    /// Current disk write in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diskwrite: Option<u64>,

    /// Current disk read in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diskread: Option<u64>,

    /// Network in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netin: Option<u64>,

    /// Network out bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netout: Option<u64>,

    /// Whether this is a template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<u32>,

    /// PID of the running QEMU process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<u32>,

    /// QEMU machine type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qmpstatus: Option<String>,

    /// Tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,

    /// Lock status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<String>,
}

/// Parameters for creating a QEMU VM.
#[derive(Debug, Clone, Default, Serialize)]
pub struct VmCreateParams {
    /// VM ID (100-999999999).
    pub vmid: u32,

    /// VM name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Memory in MB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<u64>,

    /// Number of CPU cores.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores: Option<u32>,

    /// Number of CPU sockets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sockets: Option<u32>,

    /// OS type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ostype: Option<String>,

    /// CDROM (ISO image).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdrom: Option<String>,

    /// Boot order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot: Option<String>,

    /// BIOS type (seabios, ovmf).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bios: Option<String>,

    /// Machine type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine: Option<String>,

    /// CPU type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,

    /// NUMA enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numa: Option<bool>,

    /// Start on boot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboot: Option<bool>,

    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,

    /// Storage for the VM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,

    /// QEMU agent enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,

    /// SCSI controller type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scsihw: Option<String>,

    /// EFI disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efidisk0: Option<String>,

    /// TPM state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpmstate0: Option<String>,

    /// Start the VM after creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<bool>,

    /// Pool to add the VM to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,

    /// Additional indexed parameters (net0, scsi0, ide0, etc.).
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl VmCreateParams {
    /// Creates a new `VmCreateParams` with the required fields.
    pub fn new(vmid: u32) -> Self {
        Self {
            vmid,
            ..Default::default()
        }
    }
}

/// Current status of a QEMU VM.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VmStatus {
    /// VM ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmid: Option<u32>,

    /// Current status (`running`, `stopped`, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// QMP status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qmpstatus: Option<String>,

    /// VM name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// CPU usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f64>,

    /// Number of CPUs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<u32>,

    /// Current memory usage in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem: Option<u64>,

    /// Maximum memory in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxmem: Option<u64>,

    /// Maximum disk in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxdisk: Option<u64>,

    /// Uptime in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime: Option<u64>,

    /// PID of the QEMU process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<u32>,

    /// Network in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netin: Option<u64>,

    /// Network out bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netout: Option<u64>,

    /// Disk read bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diskread: Option<u64>,

    /// Disk write bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diskwrite: Option<u64>,

    /// Whether a balloon device is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balloon: Option<u64>,

    /// HA state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ha: Option<Value>,

    /// Lock status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<String>,

    /// Tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

/// Parameters for cloning a VM.
#[derive(Debug, Clone, Default, Serialize)]
pub struct VmCloneParams {
    /// Target VMID for the clone.
    pub newid: u32,

    /// Name for the clone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Description for the clone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Target node (for cross-node clone).
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

    /// Format for the clone (raw, qcow2, vmdk).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// Bandwidth limit in KiB/s.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bwlimit: Option<u64>,
}

impl VmCloneParams {
    /// Creates a new `VmCloneParams` with the required fields.
    pub fn new(newid: u32) -> Self {
        Self {
            newid,
            ..Default::default()
        }
    }
}

/// Parameters for moving a disk.
#[derive(Debug, Clone, Default, Serialize)]
pub struct VmMoveDiskParams {
    /// Disk to move (e.g., `scsi0`).
    pub disk: String,

    /// Target storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,

    /// Target volume (for rename).
    #[serde(rename = "target-vmid", skip_serializing_if = "Option::is_none")]
    pub target_vmid: Option<u32>,

    /// Target disk slot.
    #[serde(rename = "target-disk", skip_serializing_if = "Option::is_none")]
    pub target_disk: Option<String>,

    /// Format for the new disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// Delete the original disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,

    /// Bandwidth limit in KiB/s.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bwlimit: Option<u64>,

    /// Prevent changes if current configuration differs (SHA1 digest).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

impl VmMoveDiskParams {
    /// Creates a new `VmMoveDiskParams` with the required fields.
    pub fn new(disk: impl Into<String>) -> Self {
        Self {
            disk: disk.into(),
            ..Default::default()
        }
    }
}

/// Parameters for sending a key event.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct SendKeyParams {
    /// Key to send (required).
    pub key: String,

    /// Skip lock check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skiplock: Option<bool>,
}

impl SendKeyParams {
    /// Creates a new `SendKeyParams` with the required fields.
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            ..Default::default()
        }
    }
}

/// Parameters for unlinking (detaching) disks.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct UnlinkParams {
    /// Comma-separated list of disk IDs to unlink (required).
    pub idlist: String,

    /// Force removal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl UnlinkParams {
    /// Creates a new `UnlinkParams` with the required fields.
    pub fn new(idlist: impl Into<String>) -> Self {
        Self {
            idlist: idlist.into(),
            ..Default::default()
        }
    }
}

/// Parameters for resizing a disk.
#[derive(Debug, Clone, Default, Serialize)]
pub struct VmResizeParams {
    /// Disk to resize (e.g., `scsi0`).
    pub disk: String,

    /// New size (e.g., `+10G` or `50G`).
    pub size: String,

    /// Prevent changes if current configuration differs (SHA1 digest).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,

    /// Skip lock check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skiplock: Option<bool>,
}

impl VmResizeParams {
    /// Creates a new `VmResizeParams` with the required fields.
    pub fn new(disk: impl Into<String>, size: impl Into<String>) -> Self {
        Self {
            disk: disk.into(),
            size: size.into(),
            ..Default::default()
        }
    }
}

/// VNC proxy connection data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VncProxyData {
    /// Ticket for the VNC connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,

    /// Port number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,

    /// Certificate for the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert: Option<String>,

    /// UPID for the proxy task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upid: Option<String>,

    /// User for the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// SPICE proxy connection data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SpiceProxyData {
    /// Proxy host.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// TLS port.
    #[serde(rename = "tls-port", skip_serializing_if = "Option::is_none")]
    pub tls_port: Option<String>,

    /// Password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// Proxy type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub proxy_type: Option<String>,

    /// Toggle for TLS.
    #[serde(rename = "toggle-fullscreen", skip_serializing_if = "Option::is_none")]
    pub toggle_fullscreen: Option<String>,
}

/// Parameters for VM power actions.
#[derive(Debug, Clone, Default, Serialize)]
pub struct VmPowerParams {
    /// Timeout in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,

    /// Force stop.
    #[serde(rename = "forceStop", skip_serializing_if = "Option::is_none")]
    pub force_stop: Option<bool>,

    /// Skip lock check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skiplock: Option<bool>,

    /// Do not deactivate storage volumes (for stop/shutdown).
    #[serde(rename = "keepActive", skip_serializing_if = "Option::is_none")]
    pub keep_active: Option<bool>,

    /// Overrule an active qmshutdown task (for stop).
    #[serde(rename = "overrule-shutdown", skip_serializing_if = "Option::is_none")]
    pub overrule_shutdown: Option<bool>,

    /// Machine type override.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine: Option<String>,

    /// Migration type.
    #[serde(rename = "migratedfrom", skip_serializing_if = "Option::is_none")]
    pub migrated_from: Option<String>,
}

/// Parameters for suspending a VM.
#[derive(Debug, Clone, Default, Serialize)]
pub struct VmSuspendParams {
    /// Skip lock check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skiplock: Option<bool>,

    /// Suspend to disk (hibernate).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub todisk: Option<bool>,

    /// Storage for the VM state (requires `todisk`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statestorage: Option<String>,
}

/// Parameters for deleting a VM.
#[derive(Debug, Clone, Default, Serialize)]
pub struct VmDeleteParams {
    /// Destroy unreferenced disks owned by the VM.
    #[serde(
        rename = "destroy-unreferenced-disks",
        skip_serializing_if = "Option::is_none"
    )]
    pub destroy_unreferenced_disks: Option<bool>,

    /// Remove from HA, replication, and backup jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purge: Option<bool>,

    /// Skip lock check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skiplock: Option<bool>,
}

/// Pending configuration changes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PendingChange {
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

impl ProxmoxClient {
    // ── QEMU VM lifecycle ───────────────────────────────────────────

    /// Lists all QEMU VMs on a node.
    pub async fn list_vms(&self, node: &str) -> Result<Vec<VmSummary>> {
        validate_node_name(node)?;
        self.get_parsed(&format!("/nodes/{node}/qemu"), &format!("node {node} VMs"))
            .await
    }

    /// Creates a new QEMU VM.
    pub async fn create_vm(&self, node: &str, params: &VmCreateParams) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(params.vmid)?;
        self.post_parsed(
            &format!("/nodes/{node}/qemu"),
            params,
            &format!("create VM {}", params.vmid),
        )
        .await
    }

    /// Deletes a QEMU VM.
    pub async fn delete_vm(
        &self,
        node: &str,
        vmid: u32,
        params: Option<&VmDeleteParams>,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let mut path = format!("/nodes/{node}/qemu/{vmid}");
        if let Some(p) = params {
            let qs = Self::serialize_as_query_string(p);
            if !qs.is_empty() {
                path.push('?');
                path.push_str(&qs);
            }
        }
        let response = self.delete(&path)?.send().await?;
        Self::parse_response(response, &format!("VM {vmid}")).await
    }

    /// Returns the current status of a QEMU VM.
    pub async fn get_vm_status(&self, node: &str, vmid: u32) -> Result<VmStatus> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/status/current"),
            &format!("VM {vmid} status"),
        )
        .await
    }

    /// Starts a QEMU VM.
    pub async fn start_vm(
        &self,
        node: &str,
        vmid: u32,
        params: Option<&VmPowerParams>,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let path = format!("/nodes/{node}/qemu/{vmid}/status/start");
        self.post_with_optional_body(&path, params, &format!("start VM {vmid}"))
            .await
    }

    /// Stops a QEMU VM immediately.
    pub async fn stop_vm(
        &self,
        node: &str,
        vmid: u32,
        params: Option<&VmPowerParams>,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let path = format!("/nodes/{node}/qemu/{vmid}/status/stop");
        self.post_with_optional_body(&path, params, &format!("stop VM {vmid}"))
            .await
    }

    /// Sends an ACPI shutdown to a QEMU VM.
    pub async fn shutdown_vm(
        &self,
        node: &str,
        vmid: u32,
        params: Option<&VmPowerParams>,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let path = format!("/nodes/{node}/qemu/{vmid}/status/shutdown");
        self.post_with_optional_body(&path, params, &format!("shutdown VM {vmid}"))
            .await
    }

    /// Reboots a QEMU VM.
    pub async fn reboot_vm(&self, node: &str, vmid: u32, timeout: Option<u64>) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let path = format!("/nodes/{node}/qemu/{vmid}/status/reboot");
        if let Some(t) = timeout {
            let params = serde_json::json!({ "timeout": t });
            self.post_parsed(&path, &params, &format!("reboot VM {vmid}"))
                .await
        } else {
            let response = self.post(&path)?.send().await?;
            Self::parse_response(response, &format!("reboot VM {vmid}")).await
        }
    }

    /// Resets a QEMU VM (hard reset).
    pub async fn reset_vm(&self, node: &str, vmid: u32, skiplock: Option<bool>) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let path = format!("/nodes/{node}/qemu/{vmid}/status/reset");
        if let Some(sl) = skiplock {
            let params = serde_json::json!({ "skiplock": sl });
            self.post_parsed(&path, &params, &format!("reset VM {vmid}"))
                .await
        } else {
            let response = self.post(&path)?.send().await?;
            Self::parse_response(response, &format!("reset VM {vmid}")).await
        }
    }

    /// Suspends a QEMU VM.
    pub async fn suspend_vm(
        &self,
        node: &str,
        vmid: u32,
        params: Option<&VmSuspendParams>,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let path = format!("/nodes/{node}/qemu/{vmid}/status/suspend");
        self.post_with_optional_body(&path, params, &format!("suspend VM {vmid}"))
            .await
    }

    /// Resumes a suspended QEMU VM.
    pub async fn resume_vm(&self, node: &str, vmid: u32) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/qemu/{vmid}/status/resume"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("resume VM {vmid}")).await
    }

    // ── QEMU VM operations ─────────────────────────────────────────

    /// Clones a QEMU VM.
    pub async fn clone_vm(&self, node: &str, vmid: u32, params: &VmCloneParams) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        validate_vmid(params.newid)?;
        self.post_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/clone"),
            params,
            &format!("clone VM {vmid}"),
        )
        .await
    }

    /// Converts a QEMU VM to a template.
    pub async fn convert_vm_to_template(&self, node: &str, vmid: u32) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/qemu/{vmid}/template"))?
            .send()
            .await?;
        Self::handle_error(response, &format!("VM {vmid} template")).await?;
        Ok(())
    }

    /// Moves a VM disk to different storage.
    pub async fn move_vm_disk(
        &self,
        node: &str,
        vmid: u32,
        params: &VmMoveDiskParams,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.post_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/move_disk"),
            params,
            &format!("VM {vmid} move disk"),
        )
        .await
    }

    /// Resizes a VM disk. Returns a task UPID.
    pub async fn resize_vm_disk(
        &self,
        node: &str,
        vmid: u32,
        params: &VmResizeParams,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .put(&format!("/nodes/{node}/qemu/{vmid}/resize"))?
            .json(params)
            .send()
            .await?;
        Self::parse_response(response, &format!("VM {vmid} resize")).await
    }

    /// Returns pending configuration changes for a VM.
    pub async fn get_vm_pending(&self, node: &str, vmid: u32) -> Result<Vec<PendingChange>> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/pending"),
            &format!("VM {vmid} pending"),
        )
        .await
    }

    /// Creates a VNC proxy for a VM.
    pub async fn create_vm_vnc_proxy(&self, node: &str, vmid: u32) -> Result<VncProxyData> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/qemu/{vmid}/vncproxy"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("VM {vmid} VNC proxy")).await
    }

    /// Creates a SPICE proxy for a VM.
    pub async fn create_vm_spice_proxy(&self, node: &str, vmid: u32) -> Result<SpiceProxyData> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/qemu/{vmid}/spiceproxy"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("VM {vmid} SPICE proxy")).await
    }

    /// Creates a terminal proxy for a VM.
    pub async fn create_vm_term_proxy(&self, node: &str, vmid: u32) -> Result<VncProxyData> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/qemu/{vmid}/termproxy"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("VM {vmid} terminal proxy")).await
    }

    /// Executes a QEMU monitor command.
    ///
    /// `POST /nodes/{node}/qemu/{vmid}/monitor`
    pub async fn execute_vm_monitor_command(
        &self,
        node: &str,
        vmid: u32,
        command: &str,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let params = serde_json::json!({ "command": command });
        self.post_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/monitor"),
            &params,
            &format!("VM {vmid} monitor command"),
        )
        .await
    }

    /// Sends a key event to a VM.
    ///
    /// `PUT /nodes/{node}/qemu/{vmid}/sendkey`
    pub async fn send_vm_key(&self, node: &str, vmid: u32, params: &SendKeyParams) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .put(&format!("/nodes/{node}/qemu/{vmid}/sendkey"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("VM {vmid} sendkey")).await?;
        Ok(())
    }

    /// Regenerates the cloud-init disk for a VM.
    ///
    /// `PUT /nodes/{node}/qemu/{vmid}/cloudinit`
    pub async fn regenerate_vm_cloudinit(&self, node: &str, vmid: u32) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .put(&format!("/nodes/{node}/qemu/{vmid}/cloudinit"))?
            .send()
            .await?;
        Self::handle_error(response, &format!("VM {vmid} cloudinit regenerate")).await?;
        Ok(())
    }

    /// Unlinks (detaches) disks from a VM.
    ///
    /// `PUT /nodes/{node}/qemu/{vmid}/unlink`
    pub async fn unlink_vm_disks(
        &self,
        node: &str,
        vmid: u32,
        params: &UnlinkParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .put(&format!("/nodes/{node}/qemu/{vmid}/unlink"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("VM {vmid} unlink disks")).await?;
        Ok(())
    }

    /// Checks if a specific feature is available for a VM.
    ///
    /// `GET /nodes/{node}/qemu/{vmid}/feature`
    pub async fn check_vm_feature(&self, node: &str, vmid: u32, feature: &str) -> Result<Value> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!(
                "/nodes/{node}/qemu/{vmid}/feature?feature={}",
                encode(feature)
            ),
            &format!("VM {vmid} feature {feature}"),
        )
        .await
    }

    /// Opens a websocket for VNC traffic.
    ///
    /// `GET /nodes/{node}/qemu/{vmid}/vncwebsocket`
    pub async fn get_vm_vnc_websocket(
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
                "/nodes/{node}/qemu/{vmid}/vncwebsocket?port={}&vncticket={}",
                encode(port),
                encode(vncticket)
            ),
            &format!("VM {vmid} VNC websocket"),
        )
        .await
    }

    /// Saves or stops the dbus-vmstate for a VM.
    ///
    /// `POST /nodes/{node}/qemu/{vmid}/dbus-vmstate`
    ///
    /// `action` must be `"start"` or `"stop"`.
    pub async fn save_vm_dbus_vmstate(&self, node: &str, vmid: u32, action: &str) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let params = serde_json::json!({ "action": action });
        let response = self
            .post(&format!("/nodes/{node}/qemu/{vmid}/dbus-vmstate"))?
            .json(&params)
            .send()
            .await?;
        Self::handle_error(response, &format!("VM {vmid} dbus-vmstate")).await?;
        Ok(())
    }

    /// Returns RRD data for a VM.
    pub async fn get_vm_rrddata(
        &self,
        node: &str,
        vmid: u32,
        timeframe: &str,
        cf: Option<&str>,
    ) -> Result<Vec<crate::nodes::RrdData>> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let mut path = format!(
            "/nodes/{node}/qemu/{vmid}/rrddata?timeframe={}",
            encode(timeframe)
        );
        if let Some(cf) = cf {
            path.push_str(&format!("&cf={}", encode(cf)));
        }
        self.get_parsed(&path, &format!("VM {vmid} rrddata")).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vm_summary_serde_roundtrip() {
        let json = r#"{
            "vmid": 100,
            "name": "test-vm",
            "status": "running",
            "maxmem": 2147483648,
            "maxdisk": 34359738368,
            "cpus": 4,
            "uptime": 86400,
            "cpu": 0.15,
            "mem": 1073741824,
            "netin": 1048576,
            "netout": 524288,
            "template": 0,
            "tags": "production;web"
        }"#;
        let vm: VmSummary = serde_json::from_str(json).unwrap();
        assert_eq!(vm.vmid, Some(100));
        assert_eq!(vm.name.as_deref(), Some("test-vm"));
        assert_eq!(vm.status.as_deref(), Some("running"));
        assert_eq!(vm.cpus, Some(4));

        let serialized = serde_json::to_string(&vm).unwrap();
        let deserialized: VmSummary = serde_json::from_str(&serialized).unwrap();
        assert_eq!(vm, deserialized);
    }

    #[test]
    fn vm_status_serde_roundtrip() {
        let json = r#"{
            "vmid": 100,
            "status": "running",
            "qmpstatus": "running",
            "name": "test-vm",
            "cpu": 0.25,
            "cpus": 4,
            "mem": 1073741824,
            "maxmem": 2147483648,
            "uptime": 3600,
            "pid": 12345
        }"#;
        let status: VmStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status.vmid, Some(100));
        assert_eq!(status.pid, Some(12345));

        let serialized = serde_json::to_string(&status).unwrap();
        let deserialized: VmStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(status, deserialized);
    }

    #[test]
    fn vm_create_params_serialize() {
        let mut extra = HashMap::new();
        extra.insert(
            "net0".to_string(),
            Value::String("virtio,bridge=vmbr0".to_string()),
        );
        extra.insert(
            "scsi0".to_string(),
            Value::String("local-lvm:32".to_string()),
        );

        let mut params = VmCreateParams::new(100);
        params.name = Some("test-vm".to_string());
        params.memory = Some(2048);
        params.cores = Some(4);
        params.extra = extra;
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["vmid"], 100);
        assert_eq!(json["name"], "test-vm");
        assert_eq!(json["net0"], "virtio,bridge=vmbr0");
        assert_eq!(json["scsi0"], "local-lvm:32");
    }

    #[test]
    fn vnc_proxy_data_serde_roundtrip() {
        let json = r#"{
            "ticket": "PVE:tkt:...",
            "port": "5900",
            "cert": "-----BEGIN CERTIFICATE-----",
            "upid": "UPID:pve1:...",
            "user": "root@pam"
        }"#;
        let data: VncProxyData = serde_json::from_str(json).unwrap();
        assert_eq!(data.port.as_deref(), Some("5900"));

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: VncProxyData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn vm_clone_params_skip_none() {
        let mut params = VmCloneParams::new(200);
        params.name = Some("clone-vm".to_string());
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["newid"], 200);
        assert!(json.get("target").is_none());
        assert!(json.get("storage").is_none());
    }
}
