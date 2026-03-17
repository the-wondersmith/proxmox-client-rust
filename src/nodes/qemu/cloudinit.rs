use serde_json::Value;

use urlencoding::encode;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::{validate_node_name, validate_vmid};

impl ProxmoxClient {
    /// Dumps the cloud-init configuration for a QEMU VM.
    ///
    /// The `config_type` parameter can be `user`, `network`, or `meta`.
    pub async fn dump_vm_cloudinit(
        &self,
        node: &str,
        vmid: u32,
        config_type: &str,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!(
                "/nodes/{node}/qemu/{vmid}/cloudinit/dump?type={}",
                encode(config_type)
            ),
            &format!("VM {vmid} cloudinit dump"),
        )
        .await
    }

    /// Returns the pending cloud-init configuration for a QEMU VM.
    ///
    /// `GET /nodes/{node}/qemu/{vmid}/cloudinit`
    pub async fn get_vm_cloudinit_pending(&self, node: &str, vmid: u32) -> Result<Vec<Value>> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/cloudinit"),
            &format!("VM {vmid} cloudinit pending"),
        )
        .await
    }
}
