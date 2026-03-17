use serde::{Deserialize, Serialize};

use urlencoding::encode;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_node_name;

/// A network interface on a node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NetworkInterface {
    /// Interface name (e.g., `vmbr0`, `eth0`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iface: Option<String>,

    /// Interface type (e.g., `bridge`, `bond`, `eth`, `vlan`, `OVSBridge`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub iface_type: Option<String>,

    /// Whether the interface is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// Whether the interface is autostart.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autostart: Option<bool>,

    /// Method (e.g., `static`, `dhcp`, `manual`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,

    /// IPv6 method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method6: Option<String>,

    /// IPv4 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// IPv4 netmask.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netmask: Option<String>,

    /// IPv4 CIDR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,

    /// IPv4 gateway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,

    /// IPv6 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address6: Option<String>,

    /// IPv6 netmask.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netmask6: Option<String>,

    /// IPv6 CIDR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr6: Option<String>,

    /// IPv6 gateway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway6: Option<String>,

    /// Bridge ports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_ports: Option<String>,

    /// Bridge STP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_stp: Option<String>,

    /// Bridge fd.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_fd: Option<String>,

    /// Bridge VLAN aware.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_vlan_aware: Option<bool>,

    /// Bond slaves.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slaves: Option<String>,

    /// Bond mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bond_mode: Option<String>,

    /// Bond primary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bond_primary: Option<String>,

    /// Bond XMIT hash policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bond_xmit_hash_policy: Option<String>,

    /// VLAN ID.
    #[serde(rename = "vlan-id", skip_serializing_if = "Option::is_none")]
    pub vlan_id: Option<u32>,

    /// VLAN raw device.
    #[serde(rename = "vlan-raw-device", skip_serializing_if = "Option::is_none")]
    pub vlan_raw_device: Option<String>,

    /// MTU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<u32>,

    /// Comments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,

    /// Whether this interface exists (vs pending creation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exists: Option<bool>,

    /// Families (inet, inet6).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub families: Option<Vec<String>>,

    /// Priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u32>,
}

/// Parameters for creating a network interface.
#[derive(Debug, Clone, Default, Serialize)]
pub struct NetworkInterfaceCreateParams {
    /// Interface name.
    pub iface: String,

    /// Interface type.
    #[serde(rename = "type")]
    pub iface_type: String,

    /// IPv4 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// IPv4 netmask.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netmask: Option<String>,

    /// IPv4 CIDR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,

    /// IPv4 gateway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,

    /// IPv6 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address6: Option<String>,

    /// IPv6 gateway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway6: Option<String>,

    /// Autostart.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autostart: Option<bool>,

    /// Bridge ports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_ports: Option<String>,

    /// Bridge VLAN aware.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_vlan_aware: Option<bool>,

    /// Bond slaves.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slaves: Option<String>,

    /// Bond mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bond_mode: Option<String>,

    /// MTU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<u32>,

    /// Comments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

impl NetworkInterfaceCreateParams {
    /// Creates a new `NetworkInterfaceCreateParams` with the required fields.
    pub fn new(iface: impl Into<String>, iface_type: impl Into<String>) -> Self {
        Self {
            iface: iface.into(),
            iface_type: iface_type.into(),
            ..Default::default()
        }
    }
}

/// Parameters for updating a network interface.
#[derive(Debug, Clone, Default, Serialize)]
pub struct NetworkInterfaceUpdateParams {
    /// Interface type.
    #[serde(rename = "type")]
    pub iface_type: String,

    /// IPv4 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// IPv4 netmask.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netmask: Option<String>,

    /// IPv4 CIDR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,

    /// IPv4 gateway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,

    /// IPv6 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address6: Option<String>,

    /// IPv6 gateway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway6: Option<String>,

    /// Autostart.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autostart: Option<bool>,

    /// Bridge ports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_ports: Option<String>,

    /// Bridge VLAN aware.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_vlan_aware: Option<bool>,

    /// Bond slaves.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slaves: Option<String>,

    /// Bond mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bond_mode: Option<String>,

    /// MTU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<u32>,

    /// Comments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,

    /// Keys to delete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<String>,
}

impl ProxmoxClient {
    /// Lists network interfaces on a node.
    pub async fn list_network_interfaces(
        &self,
        node: &str,
        iface_type: Option<&str>,
    ) -> Result<Vec<NetworkInterface>> {
        validate_node_name(node)?;
        let mut path = format!("/nodes/{node}/network");
        if let Some(t) = iface_type {
            path.push_str(&format!("?type={}", encode(t)));
        }
        self.get_parsed(&path, &format!("node {node} network"))
            .await
    }

    /// Creates a network interface on a node.
    pub async fn create_network_interface(
        &self,
        node: &str,
        params: &NetworkInterfaceCreateParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        let response = self
            .post(&format!("/nodes/{node}/network"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("node {node} network interface")).await?;
        Ok(())
    }

    /// Returns a specific network interface.
    pub async fn get_network_interface(&self, node: &str, iface: &str) -> Result<NetworkInterface> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/network/{iface}"),
            &format!("network interface {iface}"),
        )
        .await
    }

    /// Updates a network interface.
    pub async fn update_network_interface(
        &self,
        node: &str,
        iface: &str,
        params: &NetworkInterfaceUpdateParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        let response = self
            .put(&format!("/nodes/{node}/network/{iface}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("network interface {iface}")).await?;
        Ok(())
    }

    /// Deletes a network interface.
    pub async fn delete_network_interface(&self, node: &str, iface: &str) -> Result<()> {
        validate_node_name(node)?;
        self.delete_void(
            &format!("/nodes/{node}/network/{iface}"),
            &format!("network interface {iface}"),
        )
        .await
    }

    /// Reverts pending network changes.
    pub async fn revert_network_changes(&self, node: &str) -> Result<()> {
        validate_node_name(node)?;
        let response = self
            .delete(&format!("/nodes/{node}/network"))?
            .send()
            .await?;
        Self::handle_error(response, &format!("node {node} network revert")).await?;
        Ok(())
    }

    /// Applies (reloads) network configuration.
    pub async fn apply_network_changes(&self, node: &str) -> Result<String> {
        validate_node_name(node)?;
        let response = self.put(&format!("/nodes/{node}/network"))?.send().await?;
        Self::parse_response(response, &format!("node {node} network apply")).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn network_interface_serde_roundtrip() {
        let json = r#"{
            "iface": "vmbr0",
            "type": "bridge",
            "active": true,
            "autostart": true,
            "method": "static",
            "address": "10.0.0.1",
            "netmask": "255.255.255.0",
            "cidr": "10.0.0.1/24",
            "gateway": "10.0.0.254",
            "bridge_ports": "eno1",
            "bridge_stp": "off",
            "bridge_fd": "0"
        }"#;
        let iface: NetworkInterface = serde_json::from_str(json).unwrap();
        assert_eq!(iface.iface.as_deref(), Some("vmbr0"));
        assert_eq!(iface.iface_type.as_deref(), Some("bridge"));
        assert_eq!(iface.address.as_deref(), Some("10.0.0.1"));

        let serialized = serde_json::to_string(&iface).unwrap();
        let deserialized: NetworkInterface = serde_json::from_str(&serialized).unwrap();
        assert_eq!(iface, deserialized);
    }
}
