use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::client::ProxmoxClient;
use crate::error::Result;
use urlencoding::encode;

use crate::validation::{validate_node_name, validate_resource_id, validate_vmid};

// Re-export unified types so downstream code importing from this module continues to work.
pub use crate::firewall_types::{
    FirewallAlias, FirewallAliasParams, FirewallIpSet, FirewallIpSetCreateParams,
    FirewallIpSetEntry, FirewallIpSetEntryParams, FirewallRef, FirewallRule, FirewallRuleParams,
};

/// Backward compatibility alias.
pub type FirewallRuleCreateParams = FirewallRuleParams;

/// Firewall options (specific to VM / container level).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FirewallOptions {
    /// Enable/disable firewall.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,

    /// Enable DHCP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp: Option<bool>,

    /// Enable NDP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ndp: Option<bool>,

    /// Enable Router Advertisement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radv: Option<bool>,

    /// Enable MAC filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macfilter: Option<bool>,

    /// Enable IP filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipfilter: Option<bool>,

    /// Input policy: `ACCEPT`, `DROP`, `REJECT`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_in: Option<String>,

    /// Output policy: `ACCEPT`, `DROP`, `REJECT`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_out: Option<String>,

    /// Log level for input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level_in: Option<String>,

    /// Log level for output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level_out: Option<String>,
}

impl ProxmoxClient {
    // ── Firewall rules ─────────────────────────────────────────────

    /// Lists firewall rules for a QEMU VM.
    pub async fn list_vm_firewall_rules(&self, node: &str, vmid: u32) -> Result<Vec<FirewallRule>> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/firewall/rules"),
            &format!("VM {vmid} firewall rules"),
        )
        .await
    }

    /// Creates a firewall rule for a QEMU VM.
    pub async fn create_vm_firewall_rule(
        &self,
        node: &str,
        vmid: u32,
        params: &FirewallRuleParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/qemu/{vmid}/firewall/rules"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("VM {vmid} firewall rule")).await?;
        Ok(())
    }

    /// Returns a specific firewall rule by position.
    pub async fn get_vm_firewall_rule(
        &self,
        node: &str,
        vmid: u32,
        pos: u32,
    ) -> Result<FirewallRule> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/firewall/rules/{pos}"),
            &format!("VM {vmid} firewall rule {pos}"),
        )
        .await
    }

    /// Updates a firewall rule by position.
    pub async fn update_vm_firewall_rule(
        &self,
        node: &str,
        vmid: u32,
        pos: u32,
        params: &FirewallRuleParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .put(&format!("/nodes/{node}/qemu/{vmid}/firewall/rules/{pos}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("VM {vmid} firewall rule {pos}")).await?;
        Ok(())
    }

    /// Deletes a firewall rule by position.
    pub async fn delete_vm_firewall_rule(&self, node: &str, vmid: u32, pos: u32) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.delete_void(
            &format!("/nodes/{node}/qemu/{vmid}/firewall/rules/{pos}"),
            &format!("VM {vmid} firewall rule {pos}"),
        )
        .await
    }

    // ── Firewall options ───────────────────────────────────────────

    /// Returns firewall options for a QEMU VM.
    pub async fn get_vm_firewall_options(&self, node: &str, vmid: u32) -> Result<FirewallOptions> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/firewall/options"),
            &format!("VM {vmid} firewall options"),
        )
        .await
    }

    /// Sets firewall options for a QEMU VM.
    pub async fn set_vm_firewall_options(
        &self,
        node: &str,
        vmid: u32,
        params: &FirewallOptions,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .put(&format!("/nodes/{node}/qemu/{vmid}/firewall/options"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("VM {vmid} firewall options")).await?;
        Ok(())
    }

    // ── Firewall log ───────────────────────────────────────────────

    /// Returns firewall log entries for a QEMU VM.
    pub async fn get_vm_firewall_log(
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
            &format!("/nodes/{node}/qemu/{vmid}/firewall/log"),
            &[
                ("start", start_str.as_deref()),
                ("limit", limit_str.as_deref()),
            ],
        );
        self.get_parsed(&path, &format!("VM {vmid} firewall log"))
            .await
    }

    /// Lists firewall references for a QEMU VM.
    pub async fn list_vm_firewall_refs(&self, node: &str, vmid: u32) -> Result<Vec<FirewallRef>> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/firewall/refs"),
            &format!("VM {vmid} firewall refs"),
        )
        .await
    }

    // ── Firewall aliases ───────────────────────────────────────────

    /// Lists firewall aliases for a QEMU VM.
    pub async fn list_vm_firewall_aliases(
        &self,
        node: &str,
        vmid: u32,
    ) -> Result<Vec<FirewallAlias>> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/firewall/aliases"),
            &format!("VM {vmid} firewall aliases"),
        )
        .await
    }

    /// Creates a firewall alias for a QEMU VM.
    pub async fn create_vm_firewall_alias(
        &self,
        node: &str,
        vmid: u32,
        params: &FirewallAliasParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/qemu/{vmid}/firewall/aliases"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("VM {vmid} firewall alias")).await?;
        Ok(())
    }

    /// Returns a specific firewall alias.
    pub async fn get_vm_firewall_alias(
        &self,
        node: &str,
        vmid: u32,
        name: &str,
    ) -> Result<FirewallAlias> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        validate_resource_id(name)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/firewall/aliases/{name}"),
            &format!("VM {vmid} firewall alias {name}"),
        )
        .await
    }

    /// Updates a firewall alias.
    pub async fn update_vm_firewall_alias(
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
            .put(&format!(
                "/nodes/{node}/qemu/{vmid}/firewall/aliases/{name}"
            ))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("VM {vmid} firewall alias {name}")).await?;
        Ok(())
    }

    /// Deletes a firewall alias.
    pub async fn delete_vm_firewall_alias(&self, node: &str, vmid: u32, name: &str) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        validate_resource_id(name)?;
        self.delete_void(
            &format!("/nodes/{node}/qemu/{vmid}/firewall/aliases/{name}"),
            &format!("VM {vmid} firewall alias {name}"),
        )
        .await
    }

    // ── Firewall IP sets ───────────────────────────────────────────

    /// Lists firewall IP sets for a QEMU VM.
    pub async fn list_vm_firewall_ipsets(
        &self,
        node: &str,
        vmid: u32,
    ) -> Result<Vec<FirewallIpSet>> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/firewall/ipset"),
            &format!("VM {vmid} firewall ipsets"),
        )
        .await
    }

    /// Creates a firewall IP set for a QEMU VM.
    pub async fn create_vm_firewall_ipset(
        &self,
        node: &str,
        vmid: u32,
        params: &FirewallIpSetCreateParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/qemu/{vmid}/firewall/ipset"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("VM {vmid} firewall ipset")).await?;
        Ok(())
    }

    /// Lists entries in a firewall IP set.
    pub async fn list_vm_firewall_ipset_entries(
        &self,
        node: &str,
        vmid: u32,
        name: &str,
    ) -> Result<Vec<FirewallIpSetEntry>> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        validate_resource_id(name)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/firewall/ipset/{name}"),
            &format!("VM {vmid} firewall ipset {name}"),
        )
        .await
    }

    /// Adds an entry to a firewall IP set.
    pub async fn add_vm_firewall_ipset_entry(
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
            .post(&format!("/nodes/{node}/qemu/{vmid}/firewall/ipset/{name}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("VM {vmid} firewall ipset {name} entry")).await?;
        Ok(())
    }

    /// Returns a specific IP set entry.
    pub async fn get_vm_firewall_ipset_entry(
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
                "/nodes/{node}/qemu/{vmid}/firewall/ipset/{name}/{}",
                encode(cidr)
            ),
            &format!("VM {vmid} firewall ipset {name} {cidr}"),
        )
        .await
    }

    /// Updates an IP set entry.
    pub async fn update_vm_firewall_ipset_entry(
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
                "/nodes/{node}/qemu/{vmid}/firewall/ipset/{name}/{}",
                encode(cidr)
            ))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("VM {vmid} firewall ipset {name} {cidr}")).await?;
        Ok(())
    }

    /// Deletes an IP set entry.
    pub async fn delete_vm_firewall_ipset_entry(
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
                "/nodes/{node}/qemu/{vmid}/firewall/ipset/{name}/{}",
                encode(cidr)
            ),
            &format!("VM {vmid} firewall ipset {name} {cidr}"),
        )
        .await
    }

    /// Deletes a firewall IP set.
    pub async fn delete_vm_firewall_ipset(&self, node: &str, vmid: u32, name: &str) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        validate_resource_id(name)?;
        self.delete_void(
            &format!("/nodes/{node}/qemu/{vmid}/firewall/ipset/{name}"),
            &format!("VM {vmid} firewall ipset {name}"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn firewall_rule_serde_roundtrip() {
        let json = r#"{
            "pos": 0,
            "type": "in",
            "action": "ACCEPT",
            "enable": 1,
            "source": "10.0.0.0/24",
            "proto": "tcp",
            "dport": "22",
            "comment": "Allow SSH"
        }"#;
        let rule: FirewallRule = serde_json::from_str(json).unwrap();
        assert_eq!(rule.pos, Some(0));
        assert_eq!(rule.rule_type.as_deref(), Some("in"));
        assert_eq!(rule.action.as_deref(), Some("ACCEPT"));

        let serialized = serde_json::to_string(&rule).unwrap();
        let deserialized: FirewallRule = serde_json::from_str(&serialized).unwrap();
        assert_eq!(rule, deserialized);
    }

    #[test]
    fn firewall_options_serde_roundtrip() {
        let json = r#"{
            "enable": true,
            "dhcp": true,
            "policy_in": "DROP",
            "policy_out": "ACCEPT"
        }"#;
        let opts: FirewallOptions = serde_json::from_str(json).unwrap();
        assert_eq!(opts.enable, Some(true));
        assert_eq!(opts.policy_in.as_deref(), Some("DROP"));

        let serialized = serde_json::to_string(&opts).unwrap();
        let deserialized: FirewallOptions = serde_json::from_str(&serialized).unwrap();
        assert_eq!(opts, deserialized);
    }

    #[test]
    fn firewall_alias_serde_roundtrip() {
        let json = r#"{
            "name": "myalias",
            "cidr": "10.0.0.1/32",
            "comment": "Test alias"
        }"#;
        let alias: FirewallAlias = serde_json::from_str(json).unwrap();
        assert_eq!(alias.name.as_deref(), Some("myalias"));

        let serialized = serde_json::to_string(&alias).unwrap();
        let deserialized: FirewallAlias = serde_json::from_str(&serialized).unwrap();
        assert_eq!(alias, deserialized);
    }

    #[test]
    fn firewall_ipset_entry_serde_roundtrip() {
        let json = r#"{
            "cidr": "10.0.0.0/24",
            "nomatch": 0,
            "comment": "Internal network"
        }"#;
        let entry: FirewallIpSetEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.cidr.as_deref(), Some("10.0.0.0/24"));

        let serialized = serde_json::to_string(&entry).unwrap();
        let deserialized: FirewallIpSetEntry = serde_json::from_str(&serialized).unwrap();
        assert_eq!(entry, deserialized);
    }
}
