use serde_json::Value;
use urlencoding::encode;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::firewall_types::{
    FirewallAlias, FirewallAliasParams, FirewallIpSet, FirewallIpSetCreateParams,
    FirewallIpSetEntry, FirewallIpSetEntryParams, FirewallRef, FirewallRule, FirewallRuleParams,
};
use crate::nodes::qemu::firewall::FirewallOptions;
use crate::validation::{validate_node_name, validate_resource_id, validate_vmid};

impl ProxmoxClient {
    // ── Container firewall rules ───────────────────────────────────

    /// Lists firewall rules for an LXC container.
    pub async fn list_container_firewall_rules(
        &self,
        node: &str,
        vmid: u32,
    ) -> Result<Vec<FirewallRule>> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/lxc/{vmid}/firewall/rules"),
            &format!("container {vmid} firewall rules"),
        )
        .await
    }

    /// Creates a firewall rule for an LXC container.
    pub async fn create_container_firewall_rule(
        &self,
        node: &str,
        vmid: u32,
        params: &FirewallRuleParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/lxc/{vmid}/firewall/rules"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("container {vmid} firewall rule")).await?;
        Ok(())
    }

    /// Returns a specific firewall rule by position for a container.
    pub async fn get_container_firewall_rule(
        &self,
        node: &str,
        vmid: u32,
        pos: u32,
    ) -> Result<FirewallRule> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/lxc/{vmid}/firewall/rules/{pos}"),
            &format!("container {vmid} firewall rule {pos}"),
        )
        .await
    }

    /// Updates a firewall rule by position for a container.
    pub async fn update_container_firewall_rule(
        &self,
        node: &str,
        vmid: u32,
        pos: u32,
        params: &FirewallRuleParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .put(&format!("/nodes/{node}/lxc/{vmid}/firewall/rules/{pos}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("container {vmid} firewall rule {pos}")).await?;
        Ok(())
    }

    /// Deletes a firewall rule by position for a container.
    pub async fn delete_container_firewall_rule(
        &self,
        node: &str,
        vmid: u32,
        pos: u32,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.delete_void(
            &format!("/nodes/{node}/lxc/{vmid}/firewall/rules/{pos}"),
            &format!("container {vmid} firewall rule {pos}"),
        )
        .await
    }

    // ── Container firewall options ─────────────────────────────────

    /// Returns firewall options for an LXC container.
    pub async fn get_container_firewall_options(
        &self,
        node: &str,
        vmid: u32,
    ) -> Result<FirewallOptions> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/lxc/{vmid}/firewall/options"),
            &format!("container {vmid} firewall options"),
        )
        .await
    }

    /// Sets firewall options for an LXC container.
    pub async fn set_container_firewall_options(
        &self,
        node: &str,
        vmid: u32,
        params: &FirewallOptions,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .put(&format!("/nodes/{node}/lxc/{vmid}/firewall/options"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("container {vmid} firewall options")).await?;
        Ok(())
    }

    // ── Container firewall log ─────────────────────────────────────

    /// Returns firewall log entries for an LXC container.
    pub async fn get_container_firewall_log(
        &self,
        node: &str,
        vmid: u32,
        start: Option<u64>,
        limit: Option<u64>,
    ) -> Result<Vec<Value>> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let start_str = start.map(|s| s.to_string());
        let limit_str = limit.map(|l| l.to_string());
        let path = Self::build_query_path(
            &format!("/nodes/{node}/lxc/{vmid}/firewall/log"),
            &[
                ("start", start_str.as_deref()),
                ("limit", limit_str.as_deref()),
            ],
        );
        self.get_parsed(&path, &format!("container {vmid} firewall log"))
            .await
    }

    /// Lists firewall references for an LXC container.
    pub async fn list_container_firewall_refs(
        &self,
        node: &str,
        vmid: u32,
    ) -> Result<Vec<FirewallRef>> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/lxc/{vmid}/firewall/refs"),
            &format!("container {vmid} firewall refs"),
        )
        .await
    }

    // ── Container firewall aliases ─────────────────────────────────

    /// Lists firewall aliases for an LXC container.
    pub async fn list_container_firewall_aliases(
        &self,
        node: &str,
        vmid: u32,
    ) -> Result<Vec<FirewallAlias>> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/lxc/{vmid}/firewall/aliases"),
            &format!("container {vmid} firewall aliases"),
        )
        .await
    }

    /// Creates a firewall alias for an LXC container.
    pub async fn create_container_firewall_alias(
        &self,
        node: &str,
        vmid: u32,
        params: &FirewallAliasParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/lxc/{vmid}/firewall/aliases"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("container {vmid} firewall alias")).await?;
        Ok(())
    }

    /// Returns a specific firewall alias for a container.
    pub async fn get_container_firewall_alias(
        &self,
        node: &str,
        vmid: u32,
        name: &str,
    ) -> Result<FirewallAlias> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        validate_resource_id(name)?;
        self.get_parsed(
            &format!("/nodes/{node}/lxc/{vmid}/firewall/aliases/{name}"),
            &format!("container {vmid} firewall alias {name}"),
        )
        .await
    }

    /// Updates a firewall alias for a container.
    pub async fn update_container_firewall_alias(
        &self,
        node: &str,
        vmid: u32,
        name: &str,
        params: &FirewallAliasParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        validate_resource_id(name)?;
        let response = self
            .put(&format!("/nodes/{node}/lxc/{vmid}/firewall/aliases/{name}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("container {vmid} firewall alias {name}")).await?;
        Ok(())
    }

    /// Deletes a firewall alias for a container.
    pub async fn delete_container_firewall_alias(
        &self,
        node: &str,
        vmid: u32,
        name: &str,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        validate_resource_id(name)?;
        self.delete_void(
            &format!("/nodes/{node}/lxc/{vmid}/firewall/aliases/{name}"),
            &format!("container {vmid} firewall alias {name}"),
        )
        .await
    }

    // ── Container firewall IP sets ─────────────────────────────────

    /// Lists firewall IP sets for an LXC container.
    pub async fn list_container_firewall_ipsets(
        &self,
        node: &str,
        vmid: u32,
    ) -> Result<Vec<FirewallIpSet>> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/lxc/{vmid}/firewall/ipset"),
            &format!("container {vmid} firewall ipsets"),
        )
        .await
    }

    /// Creates a firewall IP set for an LXC container.
    pub async fn create_container_firewall_ipset(
        &self,
        node: &str,
        vmid: u32,
        params: &FirewallIpSetCreateParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/lxc/{vmid}/firewall/ipset"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("container {vmid} firewall ipset")).await?;
        Ok(())
    }

    /// Lists entries in a container firewall IP set.
    pub async fn list_container_firewall_ipset_entries(
        &self,
        node: &str,
        vmid: u32,
        name: &str,
    ) -> Result<Vec<FirewallIpSetEntry>> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        validate_resource_id(name)?;
        self.get_parsed(
            &format!("/nodes/{node}/lxc/{vmid}/firewall/ipset/{name}"),
            &format!("container {vmid} firewall ipset {name}"),
        )
        .await
    }

    /// Adds an entry to a container firewall IP set.
    pub async fn add_container_firewall_ipset_entry(
        &self,
        node: &str,
        vmid: u32,
        name: &str,
        params: &FirewallIpSetEntryParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        validate_resource_id(name)?;
        let response = self
            .post(&format!("/nodes/{node}/lxc/{vmid}/firewall/ipset/{name}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(
            response,
            &format!("container {vmid} firewall ipset {name} entry"),
        )
        .await?;
        Ok(())
    }

    /// Returns a specific IP set entry for a container.
    pub async fn get_container_firewall_ipset_entry(
        &self,
        node: &str,
        vmid: u32,
        name: &str,
        cidr: &str,
    ) -> Result<FirewallIpSetEntry> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        validate_resource_id(name)?;
        self.get_parsed(
            &format!(
                "/nodes/{node}/lxc/{vmid}/firewall/ipset/{name}/{}",
                encode(cidr)
            ),
            &format!("container {vmid} firewall ipset {name} {cidr}"),
        )
        .await
    }

    /// Updates an IP set entry for a container.
    pub async fn update_container_firewall_ipset_entry(
        &self,
        node: &str,
        vmid: u32,
        name: &str,
        cidr: &str,
        params: &FirewallIpSetEntryParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        validate_resource_id(name)?;
        let response = self
            .put(&format!(
                "/nodes/{node}/lxc/{vmid}/firewall/ipset/{name}/{}",
                encode(cidr)
            ))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(
            response,
            &format!("container {vmid} firewall ipset {name} {cidr}"),
        )
        .await?;
        Ok(())
    }

    /// Deletes an IP set entry for a container.
    pub async fn delete_container_firewall_ipset_entry(
        &self,
        node: &str,
        vmid: u32,
        name: &str,
        cidr: &str,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        validate_resource_id(name)?;
        self.delete_void(
            &format!(
                "/nodes/{node}/lxc/{vmid}/firewall/ipset/{name}/{}",
                encode(cidr)
            ),
            &format!("container {vmid} firewall ipset {name} {cidr}"),
        )
        .await
    }

    /// Deletes a container firewall IP set.
    pub async fn delete_container_firewall_ipset(
        &self,
        node: &str,
        vmid: u32,
        name: &str,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        validate_resource_id(name)?;
        self.delete_void(
            &format!("/nodes/{node}/lxc/{vmid}/firewall/ipset/{name}"),
            &format!("container {vmid} firewall ipset {name}"),
        )
        .await
    }
}
