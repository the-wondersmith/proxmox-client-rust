use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::{validate_node_name, validate_vmid};

/// Full configuration of a QEMU VM.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VmConfig {
    /// VM name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Description / notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Number of CPU cores per socket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores: Option<u32>,

    /// Number of CPU sockets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sockets: Option<u32>,

    /// CPU type/model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,

    /// Memory in MB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<u64>,

    /// Balloon memory target in MB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balloon: Option<u64>,

    /// NUMA enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numa: Option<bool>,

    /// Boot order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot: Option<String>,

    /// CDROM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdrom: Option<String>,

    /// OS type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ostype: Option<String>,

    /// BIOS type (seabios or ovmf).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bios: Option<String>,

    /// Machine type (e.g., q35, i440fx).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine: Option<String>,

    /// SMBIOS type 1 settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smbios1: Option<String>,

    /// Start on boot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboot: Option<bool>,

    /// Startup/shutdown order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startup: Option<String>,

    /// Protection flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection: Option<bool>,

    /// Tags (semicolon-separated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,

    /// QEMU agent settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,

    /// Extra command-line arguments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<String>,

    /// Hotplug features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hotplug: Option<String>,

    /// VM generation ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmgenid: Option<String>,

    /// VM state storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmstatestorage: Option<String>,

    /// SCSI controller type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scsihw: Option<String>,

    /// EFI disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efidisk0: Option<String>,

    /// TPM state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpmstate0: Option<String>,

    /// Configuration digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,

    /// VGA type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vga: Option<String>,

    /// Tablet device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tablet: Option<bool>,

    /// KVM hardware virtualization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kvm: Option<bool>,

    /// CPU limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpulimit: Option<f64>,

    /// CPU units (weight).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuunits: Option<u32>,

    /// Freeze CPU at startup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freeze: Option<bool>,

    /// Lock reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<String>,

    /// Template flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<bool>,

    /// Watchdog device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchdog: Option<String>,

    /// Audio device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio0: Option<String>,

    /// RNG device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rng0: Option<String>,

    /// Cloud-init: IP config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipconfig0: Option<String>,

    /// Cloud-init: nameserver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nameserver: Option<String>,

    /// Cloud-init: search domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchdomain: Option<String>,

    /// Cloud-init: SSH public keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sshkeys: Option<String>,

    /// Cloud-init: user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciuser: Option<String>,

    /// Cloud-init: password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cipassword: Option<String>,

    /// Cloud-init: type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citype: Option<String>,

    /// Cloud-init: custom config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cicustom: Option<String>,

    /// Cloud-init: upgrade.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciupgrade: Option<bool>,

    /// All indexed parameters: net0-net31, scsi0-scsi30, ide0-ide3,
    /// sata0-sata5, virtio0-virtio15, serial0-serial3, usb0-usb14,
    /// hostpci0-hostpci15, ipconfig1-ipconfig15, numa0-numa7, etc.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Parameters for updating a VM configuration.
#[derive(Debug, Clone, Default, Serialize)]
pub struct VmConfigUpdateParams {
    /// VM name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Description / notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Number of CPU cores per socket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores: Option<u32>,

    /// Number of CPU sockets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sockets: Option<u32>,

    /// CPU type/model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,

    /// Memory in MB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<u64>,

    /// Balloon memory target in MB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balloon: Option<u64>,

    /// NUMA enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numa: Option<bool>,

    /// Boot order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot: Option<String>,

    /// OS type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ostype: Option<String>,

    /// BIOS type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bios: Option<String>,

    /// Machine type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine: Option<String>,

    /// Start on boot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboot: Option<bool>,

    /// QEMU agent settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,

    /// Tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,

    /// Protection flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection: Option<bool>,

    /// SCSI controller type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scsihw: Option<String>,

    /// VGA type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vga: Option<String>,

    /// CDROM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdrom: Option<String>,

    /// EFI disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efidisk0: Option<String>,

    /// Cloud-init: IP config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipconfig0: Option<String>,

    /// Cloud-init: nameserver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nameserver: Option<String>,

    /// Cloud-init: search domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchdomain: Option<String>,

    /// Cloud-init: SSH public keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sshkeys: Option<String>,

    /// Cloud-init: user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciuser: Option<String>,

    /// Cloud-init: password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cipassword: Option<String>,

    /// Configuration digest for CAS updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,

    /// Keys to delete (comma-separated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<String>,

    /// Revert pending changes for these keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revert: Option<String>,

    /// Additional indexed parameters (net0, scsi0, ide0, etc.).
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl ProxmoxClient {
    /// Returns the full configuration of a QEMU VM.
    pub async fn get_vm_config(&self, node: &str, vmid: u32) -> Result<VmConfig> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/config"),
            &format!("VM {vmid} config"),
        )
        .await
    }

    /// Sets VM configuration (async operation, returns UPID).
    pub async fn set_vm_config_async(
        &self,
        node: &str,
        vmid: u32,
        params: &VmConfigUpdateParams,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.post_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/config"),
            params,
            &format!("VM {vmid} config (async)"),
        )
        .await
    }

    /// Updates VM configuration (sync operation).
    pub async fn update_vm_config(
        &self,
        node: &str,
        vmid: u32,
        params: &VmConfigUpdateParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .put(&format!("/nodes/{node}/qemu/{vmid}/config"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("VM {vmid} config")).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vm_config_serde_roundtrip() {
        let json = r#"{
            "name": "test-vm",
            "cores": 4,
            "sockets": 1,
            "memory": 2048,
            "boot": "order=scsi0;ide2;net0",
            "ostype": "l26",
            "bios": "ovmf",
            "machine": "q35",
            "agent": "1",
            "onboot": true,
            "scsihw": "virtio-scsi-single",
            "net0": "virtio=AA:BB:CC:DD:EE:FF,bridge=vmbr0",
            "scsi0": "local-lvm:vm-100-disk-0,size=32G",
            "efidisk0": "local-lvm:vm-100-disk-1,efitype=4m",
            "digest": "abc123"
        }"#;
        let config: VmConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.name.as_deref(), Some("test-vm"));
        assert_eq!(config.cores, Some(4));
        assert_eq!(config.memory, Some(2048));
        assert_eq!(config.bios.as_deref(), Some("ovmf"));

        // Indexed params should be in extra
        assert_eq!(
            config.extra.get("net0").and_then(|v| v.as_str()),
            Some("virtio=AA:BB:CC:DD:EE:FF,bridge=vmbr0")
        );
        assert_eq!(
            config.extra.get("scsi0").and_then(|v| v.as_str()),
            Some("local-lvm:vm-100-disk-0,size=32G")
        );

        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: VmConfig = serde_json::from_str(&serialized).unwrap();
        assert_eq!(config, deserialized);
    }

    #[test]
    fn vm_config_update_params_serialize() {
        let mut extra = HashMap::new();
        extra.insert(
            "net0".to_string(),
            Value::String("virtio,bridge=vmbr0".to_string()),
        );
        let params = VmConfigUpdateParams {
            memory: Some(4096),
            cores: Some(8),
            extra,
            ..Default::default()
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["memory"], 4096);
        assert_eq!(json["cores"], 8);
        assert_eq!(json["net0"], "virtio,bridge=vmbr0");
        assert!(json.get("name").is_none());
    }

    #[test]
    fn vm_config_unknown_fields_captured() {
        let json = r#"{
            "name": "test",
            "hostpci0": "0000:01:00.0",
            "usb0": "host=1234:5678",
            "serial0": "socket",
            "numa0": "cpus=0-3,memory=1024"
        }"#;
        let config: VmConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.name.as_deref(), Some("test"));
        assert!(config.extra.contains_key("hostpci0"));
        assert!(config.extra.contains_key("usb0"));
        assert!(config.extra.contains_key("serial0"));
        assert!(config.extra.contains_key("numa0"));
    }
}
