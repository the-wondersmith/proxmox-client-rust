use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_node_name;

/// SDN zone content entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SdnZoneContent {
    /// VNet name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vnet: Option<String>,

    /// Status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Status message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statusmsg: Option<String>,

    /// Additional data.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

impl ProxmoxClient {
    /// Lists SDN zone content on a node.
    pub async fn list_sdn_zone_content(
        &self,
        node: &str,
        zone: &str,
    ) -> Result<Vec<SdnZoneContent>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/sdn/zones/{zone}/content"),
            &format!("node {node} SDN zone {zone} content"),
        )
        .await
    }

    /// Gets routes for a fabric on a node.
    ///
    /// `GET /nodes/{node}/sdn/fabrics/{fabric}/routes`
    pub async fn get_node_sdn_fabric_routes(&self, node: &str, fabric: &str) -> Result<Vec<Value>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/sdn/fabrics/{fabric}/routes"),
            &format!("node {node} SDN fabric {fabric} routes"),
        )
        .await
    }

    /// Gets neighbors for a fabric on a node.
    ///
    /// `GET /nodes/{node}/sdn/fabrics/{fabric}/neighbors`
    pub async fn get_node_sdn_fabric_neighbors(
        &self,
        node: &str,
        fabric: &str,
    ) -> Result<Vec<Value>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/sdn/fabrics/{fabric}/neighbors"),
            &format!("node {node} SDN fabric {fabric} neighbors"),
        )
        .await
    }

    /// Gets interfaces for a fabric on a node.
    ///
    /// `GET /nodes/{node}/sdn/fabrics/{fabric}/interfaces`
    pub async fn get_node_sdn_fabric_interfaces(
        &self,
        node: &str,
        fabric: &str,
    ) -> Result<Vec<Value>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/sdn/fabrics/{fabric}/interfaces"),
            &format!("node {node} SDN fabric {fabric} interfaces"),
        )
        .await
    }

    /// Gets bridge info for a zone on a node.
    ///
    /// `GET /nodes/{node}/sdn/zones/{zone}/bridges`
    pub async fn get_node_sdn_zone_bridges(&self, node: &str, zone: &str) -> Result<Vec<Value>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/sdn/zones/{zone}/bridges"),
            &format!("node {node} SDN zone {zone} bridges"),
        )
        .await
    }

    /// Gets IP VRF info for a zone on a node.
    ///
    /// `GET /nodes/{node}/sdn/zones/{zone}/ip-vrf`
    pub async fn get_node_sdn_zone_ip_vrf(&self, node: &str, zone: &str) -> Result<Vec<Value>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/sdn/zones/{zone}/ip-vrf"),
            &format!("node {node} SDN zone {zone} IP VRF"),
        )
        .await
    }

    /// Returns the index of a specific SDN fabric on a node.
    ///
    /// `GET /nodes/{node}/sdn/fabrics/{fabric}`
    pub async fn get_node_sdn_fabric(&self, node: &str, fabric: &str) -> Result<Vec<Value>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/sdn/fabrics/{fabric}"),
            &format!("node {node} SDN fabric {fabric}"),
        )
        .await
    }

    /// Returns the index of a specific SDN VNet on a node.
    ///
    /// `GET /nodes/{node}/sdn/vnets/{vnet}`
    pub async fn get_node_sdn_vnet(&self, node: &str, vnet: &str) -> Result<Vec<Value>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/sdn/vnets/{vnet}"),
            &format!("node {node} SDN VNet {vnet}"),
        )
        .await
    }

    /// Returns the index of a specific SDN zone on a node.
    ///
    /// `GET /nodes/{node}/sdn/zones/{zone}`
    pub async fn get_node_sdn_zone(&self, node: &str, zone: &str) -> Result<Vec<Value>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/sdn/zones/{zone}"),
            &format!("node {node} SDN zone {zone}"),
        )
        .await
    }

    /// Gets MAC VRF info for a VNet on a node.
    ///
    /// `GET /nodes/{node}/sdn/vnets/{vnet}/mac-vrf`
    pub async fn get_node_sdn_vnet_mac_vrf(&self, node: &str, vnet: &str) -> Result<Vec<Value>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/sdn/vnets/{vnet}/mac-vrf"),
            &format!("node {node} SDN VNet {vnet} MAC VRF"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sdn_zone_content_serde_roundtrip() {
        let json = r#"{
            "vnet": "myvnet",
            "status": "available",
            "statusmsg": "OK"
        }"#;
        let content: SdnZoneContent = serde_json::from_str(json).unwrap();
        assert_eq!(content.vnet.as_deref(), Some("myvnet"));
        assert_eq!(content.status.as_deref(), Some("available"));

        let serialized = serde_json::to_string(&content).unwrap();
        let deserialized: SdnZoneContent = serde_json::from_str(&serialized).unwrap();
        assert_eq!(content, deserialized);
    }
}
