use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::firewall_types::{FirewallRule, FirewallRuleParams};
use crate::validation::validate_node_name;

/// Node-level firewall options.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NodeFirewallOptions {
    /// Whether the firewall is enabled on this node.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub enable: Option<bool>,

    /// Whether logging of conntrack entries is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub log_nf_conntrack: Option<bool>,

    /// Maximum number of tracked connections.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nf_conntrack_max: Option<i64>,

    /// Whether nftables is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub nftables: Option<bool>,

    /// Log level for SMUF packets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smurf_log_level: Option<String>,

    /// Log level for illegal TCP flag combinations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_flags_log_level: Option<String>,

    /// Whether logging of TCP flag violations is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub tcpflags: Option<bool>,

    /// Whether NDP is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub ndp: Option<bool>,

    /// Log level for incoming traffic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level_in: Option<String>,

    /// Log level for outgoing traffic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level_out: Option<String>,

    /// Whether SMURFS filter is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub nosmurfs: Option<bool>,

    /// Whether synflood protection is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub protection_synflood: Option<bool>,

    /// Synflood rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_synflood_rate: Option<i64>,

    /// Synflood burst.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_synflood_burst: Option<i64>,

    /// Digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

impl ProxmoxClient {
    /// Gets node-level firewall options.
    ///
    /// `GET /nodes/{node}/firewall/options`
    pub async fn get_node_firewall_options(&self, node: &str) -> Result<NodeFirewallOptions> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/firewall/options"),
            &format!("node {node} firewall options"),
        )
        .await
    }

    /// Sets node-level firewall options.
    ///
    /// `PUT /nodes/{node}/firewall/options`
    pub async fn set_node_firewall_options(
        &self,
        node: &str,
        params: &NodeFirewallOptions,
    ) -> Result<()> {
        validate_node_name(node)?;
        let response = self
            .put(&format!("/nodes/{node}/firewall/options"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("node {node} firewall options")).await?;
        Ok(())
    }

    /// Lists node-level firewall rules.
    ///
    /// `GET /nodes/{node}/firewall/rules`
    pub async fn list_node_firewall_rules(&self, node: &str) -> Result<Vec<FirewallRule>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/firewall/rules"),
            &format!("node {node} firewall rules"),
        )
        .await
    }

    /// Creates a node-level firewall rule.
    ///
    /// `POST /nodes/{node}/firewall/rules`
    pub async fn create_node_firewall_rule(
        &self,
        node: &str,
        params: &FirewallRuleParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        let response = self
            .post(&format!("/nodes/{node}/firewall/rules"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("node {node} firewall rule creation")).await?;
        Ok(())
    }

    /// Gets a specific node-level firewall rule.
    ///
    /// `GET /nodes/{node}/firewall/rules/{pos}`
    pub async fn get_node_firewall_rule(&self, node: &str, pos: u32) -> Result<FirewallRule> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/firewall/rules/{pos}"),
            &format!("node {node} firewall rule {pos}"),
        )
        .await
    }

    /// Updates a node-level firewall rule.
    ///
    /// `PUT /nodes/{node}/firewall/rules/{pos}`
    pub async fn update_node_firewall_rule(
        &self,
        node: &str,
        pos: u32,
        params: &FirewallRuleParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        let response = self
            .put(&format!("/nodes/{node}/firewall/rules/{pos}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("node {node} firewall rule {pos}")).await?;
        Ok(())
    }

    /// Deletes a node-level firewall rule.
    ///
    /// `DELETE /nodes/{node}/firewall/rules/{pos}`
    pub async fn delete_node_firewall_rule(&self, node: &str, pos: u32) -> Result<()> {
        validate_node_name(node)?;
        self.delete_void(
            &format!("/nodes/{node}/firewall/rules/{pos}"),
            &format!("node {node} firewall rule {pos}"),
        )
        .await
    }

    /// Gets the node firewall log.
    ///
    /// `GET /nodes/{node}/firewall/log`
    pub async fn get_node_firewall_log(
        &self,
        node: &str,
        start: Option<u64>,
        limit: Option<u64>,
    ) -> Result<Vec<Value>> {
        validate_node_name(node)?;
        let start_str = start.map(|s| s.to_string());
        let limit_str = limit.map(|l| l.to_string());
        let path = Self::build_query_path(
            &format!("/nodes/{node}/firewall/log"),
            &[
                ("start", start_str.as_deref()),
                ("limit", limit_str.as_deref()),
            ],
        );
        self.get_parsed(&path, &format!("node {node} firewall log"))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_firewall_options_serde_roundtrip() {
        let options = NodeFirewallOptions {
            enable: Some(true),
            nf_conntrack_max: Some(262144),
            smurf_log_level: Some("nolog".to_string()),
            tcp_flags_log_level: Some("nolog".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&options).unwrap();
        let deserialized: NodeFirewallOptions = serde_json::from_str(&json).unwrap();
        assert_eq!(options, deserialized);
    }

    #[test]
    fn node_firewall_options_skip_serializing_none() {
        let options = NodeFirewallOptions::default();
        let json = serde_json::to_value(&options).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default NodeFirewallOptions should serialize to {{}}"
        );
    }

    #[test]
    fn node_firewall_options_unknown_fields_ignored() {
        let json = r#"{"enable": 1, "unknownField": true}"#;
        let options: NodeFirewallOptions = serde_json::from_str(json).unwrap();
        assert_eq!(options.enable, Some(true));
    }
}
