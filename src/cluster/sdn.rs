use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::nodes::qemu::firewall::{FirewallOptions, FirewallRule, FirewallRuleCreateParams};
use crate::validation::validate_resource_id;

/// An SDN VNet.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SdnVnet {
    /// VNet ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vnet: Option<String>,

    /// Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,

    /// Alias/display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    /// VLAN tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<i64>,

    /// Whether the VNet is VLAN-aware.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub vlanaware: Option<bool>,

    /// Resource type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub vnet_type: Option<String>,

    /// Status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

/// Parameters for creating an SDN VNet.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct SdnVnetCreateParams {
    /// VNet ID (required).
    pub vnet: String,

    /// Zone ID (required).
    pub zone: String,

    /// Alias/display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    /// VLAN tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<i64>,

    /// Whether the VNet is VLAN-aware.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub vlanaware: Option<bool>,
}

impl SdnVnetCreateParams {
    /// Creates new parameters with required fields.
    pub fn new(vnet: impl Into<String>, zone: impl Into<String>) -> Self {
        Self {
            vnet: vnet.into(),
            zone: zone.into(),
            ..Default::default()
        }
    }
}

/// An SDN subnet.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SdnSubnet {
    /// Subnet ID (CIDR notation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,

    /// Resource type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub subnet_type: Option<String>,

    /// Gateway address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,

    /// Whether SNAT is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub snat: Option<bool>,

    /// DNS zone prefix.
    #[serde(rename = "dnszoneprefix", skip_serializing_if = "Option::is_none")]
    pub dns_zone_prefix: Option<String>,

    /// DHCP range.
    #[serde(rename = "dhcp-range", skip_serializing_if = "Option::is_none")]
    pub dhcp_range: Option<String>,

    /// DHCP DNS server.
    #[serde(rename = "dhcp-dns-server", skip_serializing_if = "Option::is_none")]
    pub dhcp_dns_server: Option<String>,

    /// VNet reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vnet: Option<String>,

    /// Digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

/// Parameters for creating an SDN subnet.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct SdnSubnetCreateParams {
    /// Subnet CIDR (required).
    pub subnet: String,

    /// Subnet type (required).
    #[serde(rename = "type")]
    pub subnet_type: String,

    /// Gateway address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,

    /// Whether SNAT is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub snat: Option<bool>,

    /// DNS zone prefix.
    #[serde(rename = "dnszoneprefix", skip_serializing_if = "Option::is_none")]
    pub dns_zone_prefix: Option<String>,

    /// DHCP range.
    #[serde(rename = "dhcp-range", skip_serializing_if = "Option::is_none")]
    pub dhcp_range: Option<String>,
}

impl SdnSubnetCreateParams {
    /// Creates new parameters with required fields.
    pub fn new(subnet: impl Into<String>, subnet_type: impl Into<String>) -> Self {
        Self {
            subnet: subnet.into(),
            subnet_type: subnet_type.into(),
            ..Default::default()
        }
    }
}

/// An SDN zone.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SdnZone {
    /// Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,

    /// Zone type (e.g., `simple`, `vlan`, `qinq`, `vxlan`, `evpn`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub zone_type: Option<String>,

    /// Bridge name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge: Option<String>,

    /// MTU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i64>,

    /// Nodes this zone is available on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<String>,

    /// IPAM reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipam: Option<String>,

    /// DNS reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<String>,

    /// Reverse DNS reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversedns: Option<String>,

    /// DNS zone name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dnszone: Option<String>,

    /// Controller reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<String>,

    /// VXLAN port.
    #[serde(rename = "vxlan-port", skip_serializing_if = "Option::is_none")]
    pub vxlan_port: Option<i64>,

    /// Peers list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peers: Option<String>,

    /// Tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<i64>,

    /// VLAN protocol.
    #[serde(rename = "vlan-protocol", skip_serializing_if = "Option::is_none")]
    pub vlan_protocol: Option<String>,

    /// Pending changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<serde_json::Value>,

    /// Status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

/// Parameters for creating an SDN zone.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct SdnZoneCreateParams {
    /// Zone ID (required).
    pub zone: String,

    /// Zone type (required, e.g., `simple`, `vlan`, `qinq`, `vxlan`, `evpn`).
    #[serde(rename = "type")]
    pub zone_type: String,

    /// Bridge name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge: Option<String>,

    /// MTU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i64>,

    /// Nodes this zone is available on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<String>,

    /// IPAM reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipam: Option<String>,

    /// DNS reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<String>,

    /// Reverse DNS reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversedns: Option<String>,

    /// DNS zone name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dnszone: Option<String>,

    /// Peers list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peers: Option<String>,

    /// Tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<i64>,
}

impl SdnZoneCreateParams {
    /// Creates new parameters with required fields.
    pub fn new(zone: impl Into<String>, zone_type: impl Into<String>) -> Self {
        Self {
            zone: zone.into(),
            zone_type: zone_type.into(),
            ..Default::default()
        }
    }
}

/// An SDN controller.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SdnController {
    /// Controller ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<String>,

    /// Controller type (e.g., `evpn`, `bgp`, `isis`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub controller_type: Option<String>,

    /// ASN (Autonomous System Number).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i64>,

    /// Peers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peers: Option<String>,

    /// Node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,

    /// Whether BGP multipath AS path relax is enabled.
    #[serde(
        default,
        rename = "bgp-multipath-as-path-relax",
        skip_serializing_if = "Option::is_none",
        with = "crate::serde_helpers::option_bool_as_int"
    )]
    pub bgp_multipath_as_path_relax: Option<bool>,

    /// Whether EBGP is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub ebgp: Option<bool>,

    /// EBGP multihop.
    #[serde(rename = "ebgp-multihop", skip_serializing_if = "Option::is_none")]
    pub ebgp_multihop: Option<i64>,

    /// Loopback address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loopback: Option<String>,

    /// Pending changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<serde_json::Value>,

    /// Status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

/// Parameters for creating an SDN controller.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct SdnControllerCreateParams {
    /// Controller ID (required).
    pub controller: String,

    /// Controller type (required).
    #[serde(rename = "type")]
    pub controller_type: String,

    /// ASN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i64>,

    /// Peers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peers: Option<String>,

    /// Node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,

    /// Whether EBGP is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub ebgp: Option<bool>,
}

impl SdnControllerCreateParams {
    /// Creates new parameters with required fields.
    pub fn new(controller: impl Into<String>, controller_type: impl Into<String>) -> Self {
        Self {
            controller: controller.into(),
            controller_type: controller_type.into(),
            ..Default::default()
        }
    }
}

/// An SDN IPAM configuration.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SdnIpam {
    /// IPAM ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipam: Option<String>,

    /// IPAM type (e.g., `pve`, `phpipam`, `netbox`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub ipam_type: Option<String>,

    /// URL for external IPAM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Token for authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    /// Section (for phpIPAM).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<i64>,

    /// Digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,

    /// Status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Parameters for creating an SDN IPAM.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct SdnIpamCreateParams {
    /// IPAM ID (required).
    pub ipam: String,

    /// IPAM type (required, e.g., `pve`, `phpipam`, `netbox`).
    #[serde(rename = "type")]
    pub ipam_type: String,

    /// URL for external IPAM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Token for authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    /// Section (for phpIPAM).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<i64>,
}

impl SdnIpamCreateParams {
    /// Creates new parameters with required fields.
    pub fn new(ipam: impl Into<String>, ipam_type: impl Into<String>) -> Self {
        Self {
            ipam: ipam.into(),
            ipam_type: ipam_type.into(),
            ..Default::default()
        }
    }
}

/// An SDN IP mapping (for VNet IPs).
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct SdnIpMapping {
    /// IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// MAC address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,

    /// Associated VMID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmid: Option<u32>,

    /// Zone name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

/// An SDN DNS plugin.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SdnDns {
    /// DNS plugin ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<String>,

    /// Plugin type (e.g., `powerdns`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub dns_type: Option<String>,

    /// DNS server URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// Reverse DNS zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversemaskv6: Option<i64>,

    /// TTL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,

    /// Digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,

    /// Status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Parameters for creating an SDN DNS plugin.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct SdnDnsCreateParams {
    /// DNS plugin ID (required).
    pub dns: String,

    /// Plugin type (required, e.g., `powerdns`).
    #[serde(rename = "type")]
    pub dns_type: String,

    /// DNS server URL (required).
    pub url: String,

    /// API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// Reverse mask for IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversemaskv6: Option<i64>,

    /// TTL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
}

impl SdnDnsCreateParams {
    /// Creates new parameters with required fields.
    pub fn new(
        dns: impl Into<String>,
        dns_type: impl Into<String>,
        url: impl Into<String>,
    ) -> Self {
        Self {
            dns: dns.into(),
            dns_type: dns_type.into(),
            url: url.into(),
            ..Default::default()
        }
    }
}

impl ProxmoxClient {
    // --- SDN Index ---

    /// Gets the SDN configuration index.
    ///
    /// `GET /cluster/sdn`
    pub async fn get_sdn_index(&self) -> Result<Vec<serde_json::Value>> {
        self.get_parsed("/cluster/sdn", "SDN index").await
    }

    /// Applies the pending SDN configuration.
    ///
    /// `PUT /cluster/sdn`
    pub async fn apply_sdn_config(&self) -> Result<()> {
        let response = self.put("/cluster/sdn")?.send().await?;
        Self::handle_error(response, "SDN config apply").await?;
        Ok(())
    }

    // --- SDN VNets ---

    /// Lists SDN VNets.
    ///
    /// `GET /cluster/sdn/vnets`
    pub async fn list_sdn_vnets(&self) -> Result<Vec<SdnVnet>> {
        self.get_parsed("/cluster/sdn/vnets", "SDN VNets").await
    }

    /// Creates an SDN VNet.
    ///
    /// `POST /cluster/sdn/vnets`
    pub async fn create_sdn_vnet(&self, params: &SdnVnetCreateParams) -> Result<()> {
        let response = self.post("/cluster/sdn/vnets")?.json(params).send().await?;
        Self::handle_error(response, "SDN VNet creation").await?;
        Ok(())
    }

    /// Gets a specific SDN VNet.
    ///
    /// `GET /cluster/sdn/vnets/{vnet}`
    pub async fn get_sdn_vnet(&self, vnet: &str) -> Result<SdnVnet> {
        validate_resource_id(vnet)?;
        self.get_parsed(
            &format!("/cluster/sdn/vnets/{vnet}"),
            &format!("SDN VNet {vnet}"),
        )
        .await
    }

    /// Updates an SDN VNet.
    ///
    /// `PUT /cluster/sdn/vnets/{vnet}`
    pub async fn update_sdn_vnet(&self, vnet: &str, params: &serde_json::Value) -> Result<()> {
        validate_resource_id(vnet)?;
        let response = self
            .put(&format!("/cluster/sdn/vnets/{vnet}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("SDN VNet {vnet}")).await?;
        Ok(())
    }

    /// Deletes an SDN VNet.
    ///
    /// `DELETE /cluster/sdn/vnets/{vnet}`
    pub async fn delete_sdn_vnet(&self, vnet: &str) -> Result<()> {
        validate_resource_id(vnet)?;
        self.delete_void(
            &format!("/cluster/sdn/vnets/{vnet}"),
            &format!("SDN VNet {vnet}"),
        )
        .await
    }

    // --- SDN Subnets ---

    /// Lists subnets for an SDN VNet.
    ///
    /// `GET /cluster/sdn/vnets/{vnet}/subnets`
    pub async fn list_sdn_subnets(&self, vnet: &str) -> Result<Vec<SdnSubnet>> {
        validate_resource_id(vnet)?;
        self.get_parsed(
            &format!("/cluster/sdn/vnets/{vnet}/subnets"),
            &format!("SDN VNet {vnet} subnets"),
        )
        .await
    }

    /// Creates a subnet for an SDN VNet.
    ///
    /// `POST /cluster/sdn/vnets/{vnet}/subnets`
    pub async fn create_sdn_subnet(
        &self,
        vnet: &str,
        params: &SdnSubnetCreateParams,
    ) -> Result<()> {
        validate_resource_id(vnet)?;
        let response = self
            .post(&format!("/cluster/sdn/vnets/{vnet}/subnets"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("SDN VNet {vnet} subnet creation")).await?;
        Ok(())
    }

    /// Gets a specific subnet for an SDN VNet.
    ///
    /// `GET /cluster/sdn/vnets/{vnet}/subnets/{subnet}`
    pub async fn get_sdn_subnet(&self, vnet: &str, subnet: &str) -> Result<SdnSubnet> {
        validate_resource_id(vnet)?;
        self.get_parsed(
            &format!("/cluster/sdn/vnets/{vnet}/subnets/{subnet}"),
            &format!("SDN VNet {vnet} subnet {subnet}"),
        )
        .await
    }

    /// Updates a subnet for an SDN VNet.
    ///
    /// `PUT /cluster/sdn/vnets/{vnet}/subnets/{subnet}`
    pub async fn update_sdn_subnet(
        &self,
        vnet: &str,
        subnet: &str,
        params: &serde_json::Value,
    ) -> Result<()> {
        validate_resource_id(vnet)?;
        let response = self
            .put(&format!("/cluster/sdn/vnets/{vnet}/subnets/{subnet}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("SDN VNet {vnet} subnet {subnet}")).await?;
        Ok(())
    }

    /// Deletes a subnet from an SDN VNet.
    ///
    /// `DELETE /cluster/sdn/vnets/{vnet}/subnets/{subnet}`
    pub async fn delete_sdn_subnet(&self, vnet: &str, subnet: &str) -> Result<()> {
        validate_resource_id(vnet)?;
        self.delete_void(
            &format!("/cluster/sdn/vnets/{vnet}/subnets/{subnet}"),
            &format!("SDN VNet {vnet} subnet {subnet}"),
        )
        .await
    }

    // --- SDN Zones ---

    /// Lists SDN zones.
    ///
    /// `GET /cluster/sdn/zones`
    pub async fn list_sdn_zones(&self) -> Result<Vec<SdnZone>> {
        self.get_parsed("/cluster/sdn/zones", "SDN zones").await
    }

    /// Creates an SDN zone.
    ///
    /// `POST /cluster/sdn/zones`
    pub async fn create_sdn_zone(&self, params: &SdnZoneCreateParams) -> Result<()> {
        let response = self.post("/cluster/sdn/zones")?.json(params).send().await?;
        Self::handle_error(response, "SDN zone creation").await?;
        Ok(())
    }

    /// Gets a specific SDN zone.
    ///
    /// `GET /cluster/sdn/zones/{zone}`
    pub async fn get_sdn_zone(&self, zone: &str) -> Result<SdnZone> {
        validate_resource_id(zone)?;
        self.get_parsed(
            &format!("/cluster/sdn/zones/{zone}"),
            &format!("SDN zone {zone}"),
        )
        .await
    }

    /// Updates an SDN zone.
    ///
    /// `PUT /cluster/sdn/zones/{zone}`
    pub async fn update_sdn_zone(&self, zone: &str, params: &serde_json::Value) -> Result<()> {
        validate_resource_id(zone)?;
        let response = self
            .put(&format!("/cluster/sdn/zones/{zone}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("SDN zone {zone}")).await?;
        Ok(())
    }

    /// Deletes an SDN zone.
    ///
    /// `DELETE /cluster/sdn/zones/{zone}`
    pub async fn delete_sdn_zone(&self, zone: &str) -> Result<()> {
        validate_resource_id(zone)?;
        self.delete_void(
            &format!("/cluster/sdn/zones/{zone}"),
            &format!("SDN zone {zone}"),
        )
        .await
    }

    // --- SDN Controllers ---

    /// Lists SDN controllers.
    ///
    /// `GET /cluster/sdn/controllers`
    pub async fn list_sdn_controllers(&self) -> Result<Vec<SdnController>> {
        self.get_parsed("/cluster/sdn/controllers", "SDN controllers")
            .await
    }

    /// Creates an SDN controller.
    ///
    /// `POST /cluster/sdn/controllers`
    pub async fn create_sdn_controller(&self, params: &SdnControllerCreateParams) -> Result<()> {
        let response = self
            .post("/cluster/sdn/controllers")?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, "SDN controller creation").await?;
        Ok(())
    }

    /// Gets a specific SDN controller.
    ///
    /// `GET /cluster/sdn/controllers/{controller}`
    pub async fn get_sdn_controller(&self, controller: &str) -> Result<SdnController> {
        validate_resource_id(controller)?;
        self.get_parsed(
            &format!("/cluster/sdn/controllers/{controller}"),
            &format!("SDN controller {controller}"),
        )
        .await
    }

    /// Updates an SDN controller.
    ///
    /// `PUT /cluster/sdn/controllers/{controller}`
    pub async fn update_sdn_controller(
        &self,
        controller: &str,
        params: &serde_json::Value,
    ) -> Result<()> {
        validate_resource_id(controller)?;
        let response = self
            .put(&format!("/cluster/sdn/controllers/{controller}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("SDN controller {controller}")).await?;
        Ok(())
    }

    /// Deletes an SDN controller.
    ///
    /// `DELETE /cluster/sdn/controllers/{controller}`
    pub async fn delete_sdn_controller(&self, controller: &str) -> Result<()> {
        validate_resource_id(controller)?;
        self.delete_void(
            &format!("/cluster/sdn/controllers/{controller}"),
            &format!("SDN controller {controller}"),
        )
        .await
    }

    // --- SDN IPAMs ---

    /// Lists SDN IPAMs.
    ///
    /// `GET /cluster/sdn/ipams`
    pub async fn list_sdn_ipams(&self) -> Result<Vec<SdnIpam>> {
        self.get_parsed("/cluster/sdn/ipams", "SDN IPAMs").await
    }

    /// Creates an SDN IPAM.
    ///
    /// `POST /cluster/sdn/ipams`
    pub async fn create_sdn_ipam(&self, params: &SdnIpamCreateParams) -> Result<()> {
        let response = self.post("/cluster/sdn/ipams")?.json(params).send().await?;
        Self::handle_error(response, "SDN IPAM creation").await?;
        Ok(())
    }

    /// Gets a specific SDN IPAM.
    ///
    /// `GET /cluster/sdn/ipams/{ipam}`
    pub async fn get_sdn_ipam(&self, ipam: &str) -> Result<SdnIpam> {
        validate_resource_id(ipam)?;
        self.get_parsed(
            &format!("/cluster/sdn/ipams/{ipam}"),
            &format!("SDN IPAM {ipam}"),
        )
        .await
    }

    /// Updates an SDN IPAM.
    ///
    /// `PUT /cluster/sdn/ipams/{ipam}`
    pub async fn update_sdn_ipam(&self, ipam: &str, params: &serde_json::Value) -> Result<()> {
        validate_resource_id(ipam)?;
        let response = self
            .put(&format!("/cluster/sdn/ipams/{ipam}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("SDN IPAM {ipam}")).await?;
        Ok(())
    }

    /// Deletes an SDN IPAM.
    ///
    /// `DELETE /cluster/sdn/ipams/{ipam}`
    pub async fn delete_sdn_ipam(&self, ipam: &str) -> Result<()> {
        validate_resource_id(ipam)?;
        self.delete_void(
            &format!("/cluster/sdn/ipams/{ipam}"),
            &format!("SDN IPAM {ipam}"),
        )
        .await
    }

    // --- SDN DNS ---

    /// Lists SDN DNS plugins.
    ///
    /// `GET /cluster/sdn/dns`
    pub async fn list_sdn_dns_plugins(&self) -> Result<Vec<SdnDns>> {
        self.get_parsed("/cluster/sdn/dns", "SDN DNS plugins").await
    }

    /// Creates an SDN DNS plugin.
    ///
    /// `POST /cluster/sdn/dns`
    pub async fn create_sdn_dns_plugin(&self, params: &SdnDnsCreateParams) -> Result<()> {
        let response = self.post("/cluster/sdn/dns")?.json(params).send().await?;
        Self::handle_error(response, "SDN DNS plugin creation").await?;
        Ok(())
    }

    /// Gets a specific SDN DNS plugin.
    ///
    /// `GET /cluster/sdn/dns/{dns}`
    pub async fn get_sdn_dns_plugin(&self, dns: &str) -> Result<SdnDns> {
        validate_resource_id(dns)?;
        self.get_parsed(
            &format!("/cluster/sdn/dns/{dns}"),
            &format!("SDN DNS plugin {dns}"),
        )
        .await
    }

    /// Updates an SDN DNS plugin.
    ///
    /// `PUT /cluster/sdn/dns/{dns}`
    pub async fn update_sdn_dns_plugin(&self, dns: &str, params: &serde_json::Value) -> Result<()> {
        validate_resource_id(dns)?;
        let response = self
            .put(&format!("/cluster/sdn/dns/{dns}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("SDN DNS plugin {dns}")).await?;
        Ok(())
    }

    /// Deletes an SDN DNS plugin.
    ///
    /// `DELETE /cluster/sdn/dns/{dns}`
    pub async fn delete_sdn_dns_plugin(&self, dns: &str) -> Result<()> {
        validate_resource_id(dns)?;
        self.delete_void(
            &format!("/cluster/sdn/dns/{dns}"),
            &format!("SDN DNS plugin {dns}"),
        )
        .await
    }

    // --- SDN Lock ---

    /// Acquires the SDN configuration lock.
    ///
    /// `POST /cluster/sdn/lock`
    pub async fn acquire_sdn_lock(&self) -> Result<()> {
        let response = self.post("/cluster/sdn/lock")?.send().await?;
        Self::handle_error(response, "SDN lock acquire").await?;
        Ok(())
    }

    /// Releases the SDN configuration lock.
    ///
    /// `DELETE /cluster/sdn/lock`
    pub async fn release_sdn_lock(&self) -> Result<()> {
        self.delete_void("/cluster/sdn/lock", "SDN lock release")
            .await
    }

    /// Rolls back the pending SDN configuration.
    ///
    /// `POST /cluster/sdn/rollback`
    pub async fn rollback_sdn_config(&self) -> Result<()> {
        let response = self.post("/cluster/sdn/rollback")?.send().await?;
        Self::handle_error(response, "SDN config rollback").await?;
        Ok(())
    }

    // --- SDN IPAM Status ---

    /// Gets the status of an SDN IPAM.
    ///
    /// `GET /cluster/sdn/ipams/{ipam}/status`
    pub async fn get_sdn_ipam_status(&self, ipam: &str) -> Result<Vec<serde_json::Value>> {
        validate_resource_id(ipam)?;
        self.get_parsed(
            &format!("/cluster/sdn/ipams/{ipam}/status"),
            &format!("SDN IPAM {ipam} status"),
        )
        .await
    }

    // --- SDN VNet IPs ---

    /// Creates an IP mapping for an SDN VNet.
    ///
    /// `POST /cluster/sdn/vnets/{vnet}/ips`
    pub async fn create_sdn_vnet_ip(&self, vnet: &str, params: &SdnIpMapping) -> Result<()> {
        validate_resource_id(vnet)?;
        let response = self
            .post(&format!("/cluster/sdn/vnets/{vnet}/ips"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("SDN VNet {vnet} IP creation")).await?;
        Ok(())
    }

    /// Updates an IP mapping for an SDN VNet.
    ///
    /// `PUT /cluster/sdn/vnets/{vnet}/ips`
    pub async fn update_sdn_vnet_ip(&self, vnet: &str, params: &SdnIpMapping) -> Result<()> {
        validate_resource_id(vnet)?;
        let response = self
            .put(&format!("/cluster/sdn/vnets/{vnet}/ips"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("SDN VNet {vnet} IP update")).await?;
        Ok(())
    }

    /// Deletes an IP mapping from an SDN VNet.
    ///
    /// `DELETE /cluster/sdn/vnets/{vnet}/ips`
    pub async fn delete_sdn_vnet_ip(&self, vnet: &str, params: &SdnIpMapping) -> Result<()> {
        validate_resource_id(vnet)?;
        let response = self
            .delete(&format!("/cluster/sdn/vnets/{vnet}/ips"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("SDN VNet {vnet} IP deletion")).await?;
        Ok(())
    }

    // --- SDN VNet Firewall ---

    /// Returns firewall options for an SDN VNet.
    ///
    /// `GET /cluster/sdn/vnets/{vnet}/firewall/options`
    pub async fn get_sdn_vnet_firewall_options(&self, vnet: &str) -> Result<FirewallOptions> {
        validate_resource_id(vnet)?;
        self.get_parsed(
            &format!("/cluster/sdn/vnets/{vnet}/firewall/options"),
            &format!("SDN VNet {vnet} firewall options"),
        )
        .await
    }

    /// Sets firewall options for an SDN VNet.
    ///
    /// `PUT /cluster/sdn/vnets/{vnet}/firewall/options`
    pub async fn set_sdn_vnet_firewall_options(
        &self,
        vnet: &str,
        params: &FirewallOptions,
    ) -> Result<()> {
        validate_resource_id(vnet)?;
        let response = self
            .put(&format!("/cluster/sdn/vnets/{vnet}/firewall/options"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("SDN VNet {vnet} firewall options")).await?;
        Ok(())
    }

    /// Lists firewall rules for an SDN VNet.
    ///
    /// `GET /cluster/sdn/vnets/{vnet}/firewall/rules`
    pub async fn list_sdn_vnet_firewall_rules(&self, vnet: &str) -> Result<Vec<FirewallRule>> {
        validate_resource_id(vnet)?;
        self.get_parsed(
            &format!("/cluster/sdn/vnets/{vnet}/firewall/rules"),
            &format!("SDN VNet {vnet} firewall rules"),
        )
        .await
    }

    /// Creates a firewall rule for an SDN VNet.
    ///
    /// `POST /cluster/sdn/vnets/{vnet}/firewall/rules`
    pub async fn create_sdn_vnet_firewall_rule(
        &self,
        vnet: &str,
        params: &FirewallRuleCreateParams,
    ) -> Result<()> {
        validate_resource_id(vnet)?;
        let response = self
            .post(&format!("/cluster/sdn/vnets/{vnet}/firewall/rules"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("SDN VNet {vnet} firewall rule")).await?;
        Ok(())
    }

    /// Returns a specific firewall rule by position for an SDN VNet.
    ///
    /// `GET /cluster/sdn/vnets/{vnet}/firewall/rules/{pos}`
    pub async fn get_sdn_vnet_firewall_rule(&self, vnet: &str, pos: u32) -> Result<FirewallRule> {
        validate_resource_id(vnet)?;
        self.get_parsed(
            &format!("/cluster/sdn/vnets/{vnet}/firewall/rules/{pos}"),
            &format!("SDN VNet {vnet} firewall rule {pos}"),
        )
        .await
    }

    /// Updates a firewall rule by position for an SDN VNet.
    ///
    /// `PUT /cluster/sdn/vnets/{vnet}/firewall/rules/{pos}`
    pub async fn update_sdn_vnet_firewall_rule(
        &self,
        vnet: &str,
        pos: u32,
        params: &FirewallRuleCreateParams,
    ) -> Result<()> {
        validate_resource_id(vnet)?;
        let response = self
            .put(&format!("/cluster/sdn/vnets/{vnet}/firewall/rules/{pos}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("SDN VNet {vnet} firewall rule {pos}")).await?;
        Ok(())
    }

    /// Deletes a firewall rule by position for an SDN VNet.
    ///
    /// `DELETE /cluster/sdn/vnets/{vnet}/firewall/rules/{pos}`
    pub async fn delete_sdn_vnet_firewall_rule(&self, vnet: &str, pos: u32) -> Result<()> {
        validate_resource_id(vnet)?;
        self.delete_void(
            &format!("/cluster/sdn/vnets/{vnet}/firewall/rules/{pos}"),
            &format!("SDN VNet {vnet} firewall rule {pos}"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sdn_vnet_serde_roundtrip() {
        let vnet = SdnVnet {
            vnet: Some("myvnet".to_string()),
            zone: Some("myzone".to_string()),
            alias: Some("My VNet".to_string()),
            tag: Some(100),
            vlanaware: Some(false),
            vnet_type: Some("vnet".to_string()),
            status: None,
            digest: None,
        };

        let json = serde_json::to_string(&vnet).unwrap();
        let deserialized: SdnVnet = serde_json::from_str(&json).unwrap();
        assert_eq!(vnet, deserialized);
    }

    #[test]
    fn sdn_vnet_skip_serializing_none() {
        let vnet = SdnVnet::default();
        let json = serde_json::to_value(&vnet).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty(), "default SdnVnet should serialize to {{}}");
    }

    #[test]
    fn sdn_vnet_create_params_serialization() {
        let params = SdnVnetCreateParams {
            vnet: "myvnet".to_string(),
            zone: "myzone".to_string(),
            alias: Some("Test VNet".to_string()),
            tag: Some(100),
            vlanaware: None,
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["vnet"], "myvnet");
        assert_eq!(json["zone"], "myzone");
        assert_eq!(json["tag"], 100);
        assert!(!json.as_object().unwrap().contains_key("vlanaware"));
    }

    #[test]
    fn sdn_zone_serde_roundtrip() {
        let zone = SdnZone {
            zone: Some("myzone".to_string()),
            zone_type: Some("vxlan".to_string()),
            bridge: Some("vmbr0".to_string()),
            mtu: Some(1450),
            nodes: Some("pve1,pve2".to_string()),
            ipam: Some("pve".to_string()),
            dns: None,
            reversedns: None,
            dnszone: None,
            controller: None,
            vxlan_port: Some(4789),
            peers: Some("10.0.0.1,10.0.0.2".to_string()),
            tag: None,
            vlan_protocol: None,
            pending: None,
            status: None,
            digest: None,
        };

        let json = serde_json::to_string(&zone).unwrap();
        let deserialized: SdnZone = serde_json::from_str(&json).unwrap();
        assert_eq!(zone, deserialized);
    }

    #[test]
    fn sdn_zone_type_rename() {
        let json = r#"{"type": "vlan", "zone": "myzone"}"#;
        let zone: SdnZone = serde_json::from_str(json).unwrap();
        assert_eq!(zone.zone_type.as_deref(), Some("vlan"));

        let serialized = serde_json::to_value(&zone).unwrap();
        assert!(serialized.get("type").is_some());
        assert!(serialized.get("zone_type").is_none());
    }

    #[test]
    fn sdn_zone_create_params_serialization() {
        let mut params = SdnZoneCreateParams::new("myzone", "simple");
        params.bridge = Some("vmbr0".to_string());
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["zone"], "myzone");
        assert_eq!(json["type"], "simple");
        assert_eq!(json["bridge"], "vmbr0");
        assert!(!json.as_object().unwrap().contains_key("mtu"));
    }

    #[test]
    fn sdn_controller_serde_roundtrip() {
        let controller = SdnController {
            controller: Some("evpn1".to_string()),
            controller_type: Some("evpn".to_string()),
            asn: Some(65000),
            peers: Some("10.0.0.1".to_string()),
            node: None,
            bgp_multipath_as_path_relax: None,
            ebgp: None,
            ebgp_multihop: None,
            loopback: None,
            pending: None,
            status: None,
            digest: None,
        };

        let json = serde_json::to_string(&controller).unwrap();
        let deserialized: SdnController = serde_json::from_str(&json).unwrap();
        assert_eq!(controller, deserialized);
    }

    #[test]
    fn sdn_controller_type_rename() {
        let json = r#"{"type": "bgp", "controller": "bgp1"}"#;
        let controller: SdnController = serde_json::from_str(json).unwrap();
        assert_eq!(controller.controller_type.as_deref(), Some("bgp"));
    }

    #[test]
    fn sdn_ipam_serde_roundtrip() {
        let ipam = SdnIpam {
            ipam: Some("pve".to_string()),
            ipam_type: Some("pve".to_string()),
            url: None,
            token: None,
            section: None,
            digest: None,
            status: None,
        };

        let json = serde_json::to_string(&ipam).unwrap();
        let deserialized: SdnIpam = serde_json::from_str(&json).unwrap();
        assert_eq!(ipam, deserialized);
    }

    #[test]
    fn sdn_ipam_type_rename() {
        let json = r#"{"type": "netbox", "ipam": "nb1"}"#;
        let ipam: SdnIpam = serde_json::from_str(json).unwrap();
        assert_eq!(ipam.ipam_type.as_deref(), Some("netbox"));
    }

    #[test]
    fn sdn_dns_serde_roundtrip() {
        let dns = SdnDns {
            dns: Some("powerdns1".to_string()),
            dns_type: Some("powerdns".to_string()),
            url: Some("http://ns.example.com:8081/api/v1/servers/localhost".to_string()),
            key: Some("myapikey".to_string()),
            reversemaskv6: Some(64),
            ttl: Some(3600),
            digest: None,
            status: None,
        };

        let json = serde_json::to_string(&dns).unwrap();
        let deserialized: SdnDns = serde_json::from_str(&json).unwrap();
        assert_eq!(dns, deserialized);
    }

    #[test]
    fn sdn_dns_type_rename() {
        let json = r#"{"type": "powerdns", "dns": "pdns1"}"#;
        let dns: SdnDns = serde_json::from_str(json).unwrap();
        assert_eq!(dns.dns_type.as_deref(), Some("powerdns"));
    }

    #[test]
    fn sdn_dns_create_params_serialization() {
        let mut params = SdnDnsCreateParams::new("pdns1", "powerdns", "http://ns.example.com:8081");
        params.key = Some("mykey".to_string());
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["dns"], "pdns1");
        assert_eq!(json["type"], "powerdns");
        assert_eq!(json["url"], "http://ns.example.com:8081");
        assert!(!json.as_object().unwrap().contains_key("ttl"));
    }

    #[test]
    fn sdn_subnet_serde_roundtrip() {
        let subnet = SdnSubnet {
            subnet: Some("10.0.0.0/24".to_string()),
            subnet_type: Some("subnet".to_string()),
            gateway: Some("10.0.0.1".to_string()),
            snat: Some(true),
            dns_zone_prefix: None,
            dhcp_range: None,
            dhcp_dns_server: None,
            vnet: Some("myvnet".to_string()),
            digest: None,
        };

        let json = serde_json::to_string(&subnet).unwrap();
        let deserialized: SdnSubnet = serde_json::from_str(&json).unwrap();
        assert_eq!(subnet, deserialized);
    }

    #[test]
    fn sdn_vnet_unknown_fields_ignored() {
        let json = r#"{"vnet": "test", "unknownField": true}"#;
        let vnet: SdnVnet = serde_json::from_str(json).unwrap();
        assert_eq!(vnet.vnet.as_deref(), Some("test"));
    }

    #[test]
    fn sdn_zone_hyphenated_fields() {
        let json = r#"{"zone": "test", "vxlan-port": 4789, "vlan-protocol": "802.1q"}"#;
        let zone: SdnZone = serde_json::from_str(json).unwrap();
        assert_eq!(zone.vxlan_port, Some(4789));
        assert_eq!(zone.vlan_protocol.as_deref(), Some("802.1q"));
    }
}
