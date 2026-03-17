//! Unified firewall types shared across cluster, node, VM, and container endpoints.

use serde::{Deserialize, Serialize};

/// A firewall rule.
///
/// Used for cluster-level, node-level, VM, and container firewall rules.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FirewallRule {
    /// Rule position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<i64>,

    /// Rule type: `in`, `out`, `group`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,

    /// Action: `ACCEPT`, `DROP`, `REJECT`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// Whether the rule is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub enable: Option<bool>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Source address/CIDR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Destination address/CIDR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest: Option<String>,

    /// Protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proto: Option<String>,

    /// Source port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sport: Option<String>,

    /// Destination port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dport: Option<String>,

    /// Interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iface: Option<String>,

    /// Macro name.
    #[serde(rename = "macro", skip_serializing_if = "Option::is_none")]
    pub macro_name: Option<String>,

    /// Log level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<String>,

    /// ICMP type.
    #[serde(rename = "icmp-type", skip_serializing_if = "Option::is_none")]
    pub icmp_type: Option<String>,

    /// Digest for change detection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

/// Parameters for creating or updating a firewall rule.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct FirewallRuleParams {
    /// Rule type: `in`, `out`, `group`.
    #[serde(rename = "type")]
    pub rule_type: String,

    /// Action: `ACCEPT`, `DROP`, `REJECT`.
    pub action: String,

    /// Whether the rule is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub enable: Option<bool>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Source address/CIDR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Destination address/CIDR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest: Option<String>,

    /// Protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proto: Option<String>,

    /// Source port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sport: Option<String>,

    /// Destination port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dport: Option<String>,

    /// Interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iface: Option<String>,

    /// Macro name.
    #[serde(rename = "macro", skip_serializing_if = "Option::is_none")]
    pub macro_name: Option<String>,

    /// Position to insert at.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<i64>,

    /// Log level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<String>,

    /// ICMP type.
    #[serde(rename = "icmp-type", skip_serializing_if = "Option::is_none")]
    pub icmp_type: Option<String>,
}

impl FirewallRuleParams {
    /// Creates a new `FirewallRuleParams` with the required fields.
    pub fn new(rule_type: impl Into<String>, action: impl Into<String>) -> Self {
        Self {
            rule_type: rule_type.into(),
            action: action.into(),
            ..Default::default()
        }
    }
}

/// Firewall alias (named IP/CIDR).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FirewallAlias {
    /// Alias name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// IP/CIDR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

/// Parameters for creating/updating a firewall alias.
#[derive(Debug, Clone, Default, Serialize)]
pub struct FirewallAliasParams {
    /// Alias name.
    pub name: String,

    /// IP/CIDR.
    pub cidr: String,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl FirewallAliasParams {
    /// Creates a new `FirewallAliasParams` with the required fields.
    pub fn new(name: impl Into<String>, cidr: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            cidr: cidr.into(),
            ..Default::default()
        }
    }
}

/// Firewall IP set.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FirewallIpSet {
    /// IP set name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

/// An entry in a firewall IP set.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FirewallIpSetEntry {
    /// CIDR address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,

    /// Whether this entry is a no-match exclusion.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub nomatch: Option<bool>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

/// A firewall reference (alias, ipset, etc.).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FirewallRef {
    /// Reference type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub ref_type: Option<String>,

    /// Reference name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// Parameters for creating a firewall security group.
#[derive(Debug, Clone, Default, Serialize)]
pub struct SecurityGroupCreateParams {
    /// Security group name (required).
    pub group: String,

    /// Comment / description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl SecurityGroupCreateParams {
    /// Creates a new `SecurityGroupCreateParams` with the required fields.
    pub fn new(group: impl Into<String>) -> Self {
        Self {
            group: group.into(),
            ..Default::default()
        }
    }
}

/// Parameters for creating a firewall IP set.
#[derive(Debug, Clone, Default, Serialize)]
pub struct FirewallIpSetCreateParams {
    /// IP set name (required).
    pub name: String,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl FirewallIpSetCreateParams {
    /// Creates a new `FirewallIpSetCreateParams` with the required fields.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
}

/// Parameters for adding or updating an entry in a firewall IP set.
#[derive(Debug, Clone, Default, Serialize)]
pub struct FirewallIpSetEntryParams {
    /// CIDR address (required).
    pub cidr: String,

    /// Whether this entry is a no-match exclusion.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub nomatch: Option<bool>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl FirewallIpSetEntryParams {
    /// Creates a new `FirewallIpSetEntryParams` with the required fields.
    pub fn new(cidr: impl Into<String>) -> Self {
        Self {
            cidr: cidr.into(),
            ..Default::default()
        }
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
        assert_eq!(rule.enable, Some(true));

        let serialized = serde_json::to_string(&rule).unwrap();
        let deserialized: FirewallRule = serde_json::from_str(&serialized).unwrap();
        assert_eq!(rule, deserialized);
    }

    #[test]
    fn firewall_rule_with_digest() {
        let json = r#"{
            "type": "in",
            "action": "ACCEPT",
            "digest": "abc123",
            "icmp-type": "echo-request"
        }"#;
        let rule: FirewallRule = serde_json::from_str(json).unwrap();
        assert_eq!(rule.digest.as_deref(), Some("abc123"));
        assert_eq!(rule.icmp_type.as_deref(), Some("echo-request"));
    }

    #[test]
    fn firewall_rule_default() {
        let rule = FirewallRule::default();
        let json = serde_json::to_value(&rule).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty());
    }

    #[test]
    fn firewall_rule_params_serialization() {
        let mut params = FirewallRuleParams::new("in", "ACCEPT");
        params.proto = Some("tcp".to_string());
        params.dport = Some("443".to_string());
        params.enable = Some(true);
        params.icmp_type = Some("echo-request".to_string());
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["type"], "in");
        assert_eq!(json["action"], "ACCEPT");
        assert_eq!(json["proto"], "tcp");
        assert_eq!(json["dport"], "443");
        assert_eq!(json["enable"], 1);
        assert_eq!(json["icmp-type"], "echo-request");
        assert!(!json.as_object().unwrap().contains_key("source"));
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
        assert_eq!(entry.nomatch, Some(false));

        let serialized = serde_json::to_string(&entry).unwrap();
        let deserialized: FirewallIpSetEntry = serde_json::from_str(&serialized).unwrap();
        assert_eq!(entry, deserialized);
    }

    #[test]
    fn firewall_ref_serde_roundtrip() {
        let json = r#"{"type": "alias", "name": "test"}"#;
        let fref: FirewallRef = serde_json::from_str(json).unwrap();
        assert_eq!(fref.ref_type.as_deref(), Some("alias"));

        let serialized = serde_json::to_string(&fref).unwrap();
        let deserialized: FirewallRef = serde_json::from_str(&serialized).unwrap();
        assert_eq!(fref, deserialized);
    }

    #[test]
    fn security_group_create_params_serialization() {
        let params = SecurityGroupCreateParams {
            group: "webservers".to_string(),
            comment: Some("Web server rules".to_string()),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["group"], "webservers");
        assert_eq!(json["comment"], "Web server rules");
    }

    #[test]
    fn firewall_ipset_create_params_serialization() {
        let params = FirewallIpSetCreateParams::new("blocklist");
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["name"], "blocklist");
        assert!(!json.as_object().unwrap().contains_key("comment"));
    }

    #[test]
    fn firewall_ipset_entry_params_serialization() {
        let mut params = FirewallIpSetEntryParams::new("10.0.0.0/24");
        params.nomatch = Some(true);
        params.comment = Some("Exclude".to_string());
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["cidr"], "10.0.0.0/24");
        assert_eq!(json["nomatch"], 1);
        assert_eq!(json["comment"], "Exclude");
    }
}
