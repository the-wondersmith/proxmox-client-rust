use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::{validate_node_name, validate_vmid};

/// Full configuration of an LXC container.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ContainerConfig {
    /// OS type (e.g., `ubuntu`, `debian`, `centos`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ostype: Option<String>,

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

    /// CPU limit (0 = unlimited).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpulimit: Option<f64>,

    /// CPU units (weight).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuunits: Option<u32>,

    /// Root filesystem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rootfs: Option<String>,

    /// Nameserver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nameserver: Option<String>,

    /// Search domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchdomain: Option<String>,

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

    /// Description / notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Features (e.g., `nesting=1`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<String>,

    /// Unprivileged container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprivileged: Option<bool>,

    /// Architecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,

    /// Console mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console: Option<bool>,

    /// Number of TTYs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<u32>,

    /// CMODE (console mode).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmode: Option<String>,

    /// Lock reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<String>,

    /// Template flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<bool>,

    /// Configuration digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,

    /// Hook script.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hookscript: Option<String>,

    /// All indexed parameters: net0-net31, mp0-mp255, etc.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Parameters for updating a container configuration.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ContainerConfigUpdateParams {
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

    /// CPU limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpulimit: Option<f64>,

    /// CPU units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuunits: Option<u32>,

    /// Nameserver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nameserver: Option<String>,

    /// Search domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchdomain: Option<String>,

    /// Start on boot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboot: Option<bool>,

    /// Startup order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startup: Option<String>,

    /// Protection flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection: Option<bool>,

    /// Tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,

    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<String>,

    /// OS type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ostype: Option<String>,

    /// Root filesystem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rootfs: Option<String>,

    /// Architecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,

    /// Configuration digest for CAS updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,

    /// Keys to delete (comma-separated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<String>,

    /// Revert pending changes for these keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revert: Option<String>,

    /// Additional indexed parameters (net0, mp0, etc.).
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl ProxmoxClient {
    /// Returns the full configuration of an LXC container.
    pub async fn get_container_config(&self, node: &str, vmid: u32) -> Result<ContainerConfig> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/lxc/{vmid}/config"),
            &format!("container {vmid} config"),
        )
        .await
    }

    /// Updates an LXC container configuration.
    pub async fn update_container_config(
        &self,
        node: &str,
        vmid: u32,
        params: &ContainerConfigUpdateParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .put(&format!("/nodes/{node}/lxc/{vmid}/config"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("container {vmid} config")).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_config_serde_roundtrip() {
        let json = r#"{
            "ostype": "ubuntu",
            "hostname": "test-ct",
            "memory": 1024,
            "swap": 512,
            "cores": 2,
            "rootfs": "local-lvm:vm-200-disk-0,size=8G",
            "onboot": true,
            "unprivileged": true,
            "features": "nesting=1",
            "net0": "name=eth0,bridge=vmbr0,ip=dhcp,type=veth",
            "mp0": "/mnt/data,mp=/data,size=10G",
            "digest": "abc123"
        }"#;
        let config: ContainerConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.hostname.as_deref(), Some("test-ct"));
        assert_eq!(config.memory, Some(1024));
        assert_eq!(config.unprivileged, Some(true));
        assert!(config.extra.contains_key("net0"));
        assert!(config.extra.contains_key("mp0"));

        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: ContainerConfig = serde_json::from_str(&serialized).unwrap();
        assert_eq!(config, deserialized);
    }

    #[test]
    fn container_config_update_params_serialize() {
        let mut extra = HashMap::new();
        extra.insert(
            "net0".to_string(),
            Value::String("name=eth0,bridge=vmbr0,ip=dhcp".to_string()),
        );
        let params = ContainerConfigUpdateParams {
            memory: Some(2048),
            cores: Some(4),
            extra,
            ..Default::default()
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["memory"], 2048);
        assert_eq!(json["cores"], 4);
        assert_eq!(json["net0"], "name=eth0,bridge=vmbr0,ip=dhcp");
        assert!(json.get("hostname").is_none());
    }
}
