use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use urlencoding::encode;

use crate::validation::validate_resource_id;

// Re-export unified types.
pub use crate::firewall_types::{
    FirewallAlias, FirewallAliasParams, FirewallIpSet, FirewallIpSetCreateParams,
    FirewallIpSetEntry, FirewallIpSetEntryParams, FirewallRef, FirewallRule, FirewallRuleParams,
    SecurityGroupCreateParams,
};

/// Backward compatibility alias.
pub type ClusterFirewallRule = FirewallRule;
/// Backward compatibility alias.
pub type ClusterFirewallRuleCreateParams = FirewallRuleParams;
/// Backward compatibility alias.
pub type ClusterFirewallAlias = FirewallAlias;
/// Backward compatibility alias.
pub type ClusterFirewallIpSet = FirewallIpSet;
/// Backward compatibility alias.
pub type ClusterFirewallIpSetEntry = FirewallIpSetEntry;

/// Cluster-wide firewall options.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ClusterFirewallOptions {
    /// Whether the firewall is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub enable: Option<bool>,

    /// Default input policy (ACCEPT, DROP, REJECT).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_in: Option<String>,

    /// Default output policy (ACCEPT, DROP, REJECT).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_out: Option<String>,

    /// Log rate limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_ratelimit: Option<String>,

    /// Whether ebtables is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub ebtables: Option<bool>,
}

/// A firewall security group.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SecurityGroup {
    /// Security group name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    /// Comment / description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Digest for change detection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

/// A firewall macro definition.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FirewallMacro {
    /// Macro name.
    #[serde(rename = "macro", skip_serializing_if = "Option::is_none")]
    pub macro_name: Option<String>,

    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descr: Option<String>,
}

impl ProxmoxClient {
    // --- Firewall Options ---

    /// Gets cluster firewall options.
    ///
    /// `GET /cluster/firewall/options`
    pub async fn get_cluster_firewall_options(&self) -> Result<ClusterFirewallOptions> {
        self.get_parsed("/cluster/firewall/options", "cluster firewall options")
            .await
    }

    /// Sets cluster firewall options.
    ///
    /// `PUT /cluster/firewall/options`
    pub async fn set_cluster_firewall_options(
        &self,
        params: &ClusterFirewallOptions,
    ) -> Result<()> {
        let response = self
            .put("/cluster/firewall/options")?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, "cluster firewall options").await?;
        Ok(())
    }

    // --- Security Groups ---

    /// Lists firewall security groups.
    ///
    /// `GET /cluster/firewall/groups`
    pub async fn list_security_groups(&self) -> Result<Vec<SecurityGroup>> {
        self.get_parsed("/cluster/firewall/groups", "security groups")
            .await
    }

    /// Creates a new firewall security group.
    ///
    /// `POST /cluster/firewall/groups`
    pub async fn create_security_group(&self, params: &SecurityGroupCreateParams) -> Result<()> {
        let response = self
            .post("/cluster/firewall/groups")?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, "security group creation").await?;
        Ok(())
    }

    /// Lists rules in a security group.
    ///
    /// `GET /cluster/firewall/groups/{group}`
    pub async fn list_security_group_rules(&self, group: &str) -> Result<Vec<FirewallRule>> {
        validate_resource_id(group)?;
        self.get_parsed(
            &format!("/cluster/firewall/groups/{group}"),
            &format!("security group {group} rules"),
        )
        .await
    }

    /// Creates a rule in a security group.
    ///
    /// `POST /cluster/firewall/groups/{group}`
    pub async fn create_security_group_rule(
        &self,
        group: &str,
        params: &FirewallRuleParams,
    ) -> Result<()> {
        validate_resource_id(group)?;
        let response = self
            .post(&format!("/cluster/firewall/groups/{group}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("security group {group} rule creation")).await?;
        Ok(())
    }

    /// Gets a specific rule in a security group.
    ///
    /// `GET /cluster/firewall/groups/{group}/{pos}`
    pub async fn get_security_group_rule(&self, group: &str, pos: i64) -> Result<FirewallRule> {
        validate_resource_id(group)?;
        self.get_parsed(
            &format!("/cluster/firewall/groups/{group}/{pos}"),
            &format!("security group {group} rule {pos}"),
        )
        .await
    }

    /// Updates a rule in a security group.
    ///
    /// `PUT /cluster/firewall/groups/{group}/{pos}`
    pub async fn update_security_group_rule(
        &self,
        group: &str,
        pos: i64,
        params: &FirewallRuleParams,
    ) -> Result<()> {
        validate_resource_id(group)?;
        let response = self
            .put(&format!("/cluster/firewall/groups/{group}/{pos}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("security group {group} rule {pos}")).await?;
        Ok(())
    }

    /// Deletes a rule from a security group.
    ///
    /// `DELETE /cluster/firewall/groups/{group}/{pos}`
    pub async fn delete_security_group_rule(&self, group: &str, pos: i64) -> Result<()> {
        validate_resource_id(group)?;
        self.delete_void(
            &format!("/cluster/firewall/groups/{group}/{pos}"),
            &format!("security group {group} rule {pos}"),
        )
        .await
    }

    /// Deletes a security group.
    ///
    /// `DELETE /cluster/firewall/groups/{group}`
    pub async fn delete_security_group(&self, group: &str) -> Result<()> {
        validate_resource_id(group)?;
        self.delete_void(
            &format!("/cluster/firewall/groups/{group}"),
            &format!("security group {group}"),
        )
        .await
    }

    // --- Cluster Firewall Rules ---

    /// Lists cluster-level firewall rules.
    ///
    /// `GET /cluster/firewall/rules`
    pub async fn list_cluster_firewall_rules(&self) -> Result<Vec<FirewallRule>> {
        self.get_parsed("/cluster/firewall/rules", "cluster firewall rules")
            .await
    }

    /// Creates a cluster-level firewall rule.
    ///
    /// `POST /cluster/firewall/rules`
    pub async fn create_cluster_firewall_rule(&self, params: &FirewallRuleParams) -> Result<()> {
        let response = self
            .post("/cluster/firewall/rules")?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, "cluster firewall rule creation").await?;
        Ok(())
    }

    /// Gets a specific cluster-level firewall rule.
    ///
    /// `GET /cluster/firewall/rules/{pos}`
    pub async fn get_cluster_firewall_rule(&self, pos: i64) -> Result<FirewallRule> {
        self.get_parsed(
            &format!("/cluster/firewall/rules/{pos}"),
            &format!("cluster firewall rule {pos}"),
        )
        .await
    }

    /// Updates a cluster-level firewall rule.
    ///
    /// `PUT /cluster/firewall/rules/{pos}`
    pub async fn update_cluster_firewall_rule(
        &self,
        pos: i64,
        params: &FirewallRuleParams,
    ) -> Result<()> {
        let response = self
            .put(&format!("/cluster/firewall/rules/{pos}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("cluster firewall rule {pos}")).await?;
        Ok(())
    }

    /// Deletes a cluster-level firewall rule.
    ///
    /// `DELETE /cluster/firewall/rules/{pos}`
    pub async fn delete_cluster_firewall_rule(&self, pos: i64) -> Result<()> {
        self.delete_void(
            &format!("/cluster/firewall/rules/{pos}"),
            &format!("cluster firewall rule {pos}"),
        )
        .await
    }

    // --- Firewall Aliases ---

    /// Lists firewall aliases.
    ///
    /// `GET /cluster/firewall/aliases`
    pub async fn list_cluster_firewall_aliases(&self) -> Result<Vec<FirewallAlias>> {
        self.get_parsed("/cluster/firewall/aliases", "firewall aliases")
            .await
    }

    /// Creates a firewall alias.
    ///
    /// `POST /cluster/firewall/aliases`
    pub async fn create_cluster_firewall_alias(&self, params: &FirewallAliasParams) -> Result<()> {
        let response = self
            .post("/cluster/firewall/aliases")?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, "firewall alias creation").await?;
        Ok(())
    }

    /// Gets a specific firewall alias.
    ///
    /// `GET /cluster/firewall/aliases/{name}`
    pub async fn get_cluster_firewall_alias(&self, name: &str) -> Result<FirewallAlias> {
        validate_resource_id(name)?;
        self.get_parsed(
            &format!("/cluster/firewall/aliases/{name}"),
            &format!("firewall alias {name}"),
        )
        .await
    }

    /// Updates a firewall alias.
    ///
    /// `PUT /cluster/firewall/aliases/{name}`
    pub async fn update_cluster_firewall_alias(
        &self,
        name: &str,
        params: &FirewallAliasParams,
    ) -> Result<()> {
        validate_resource_id(name)?;
        let response = self
            .put(&format!("/cluster/firewall/aliases/{name}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("firewall alias {name}")).await?;
        Ok(())
    }

    /// Deletes a firewall alias.
    ///
    /// `DELETE /cluster/firewall/aliases/{name}`
    pub async fn delete_cluster_firewall_alias(&self, name: &str) -> Result<()> {
        validate_resource_id(name)?;
        self.delete_void(
            &format!("/cluster/firewall/aliases/{name}"),
            &format!("firewall alias {name}"),
        )
        .await
    }

    // --- Firewall IP Sets ---

    /// Lists firewall IP sets.
    ///
    /// `GET /cluster/firewall/ipset`
    pub async fn list_cluster_firewall_ipsets(&self) -> Result<Vec<FirewallIpSet>> {
        self.get_parsed("/cluster/firewall/ipset", "firewall IP sets")
            .await
    }

    /// Creates a firewall IP set.
    ///
    /// `POST /cluster/firewall/ipset`
    pub async fn create_cluster_firewall_ipset(
        &self,
        params: &FirewallIpSetCreateParams,
    ) -> Result<()> {
        let response = self
            .post("/cluster/firewall/ipset")?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, "firewall IP set creation").await?;
        Ok(())
    }

    /// Lists entries in a firewall IP set.
    ///
    /// `GET /cluster/firewall/ipset/{name}`
    pub async fn list_cluster_firewall_ipset_entries(
        &self,
        name: &str,
    ) -> Result<Vec<FirewallIpSetEntry>> {
        validate_resource_id(name)?;
        self.get_parsed(
            &format!("/cluster/firewall/ipset/{name}"),
            &format!("firewall IP set {name} entries"),
        )
        .await
    }

    /// Adds an entry to a firewall IP set.
    ///
    /// `POST /cluster/firewall/ipset/{name}`
    pub async fn add_cluster_firewall_ipset_entry(
        &self,
        name: &str,
        params: &FirewallIpSetEntryParams,
    ) -> Result<()> {
        validate_resource_id(name)?;
        let response = self
            .post(&format!("/cluster/firewall/ipset/{name}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("firewall IP set {name} entry")).await?;
        Ok(())
    }

    /// Gets a specific entry in a firewall IP set.
    ///
    /// `GET /cluster/firewall/ipset/{name}/{cidr}`
    pub async fn get_cluster_firewall_ipset_entry(
        &self,
        name: &str,
        cidr: &str,
    ) -> Result<FirewallIpSetEntry> {
        validate_resource_id(name)?;
        self.get_parsed(
            &format!("/cluster/firewall/ipset/{name}/{}", encode(cidr)),
            &format!("firewall IP set {name} entry {cidr}"),
        )
        .await
    }

    /// Updates an entry in a firewall IP set.
    ///
    /// `PUT /cluster/firewall/ipset/{name}/{cidr}`
    pub async fn update_cluster_firewall_ipset_entry(
        &self,
        name: &str,
        cidr: &str,
        params: &FirewallIpSetEntryParams,
    ) -> Result<()> {
        validate_resource_id(name)?;
        let response = self
            .put(&format!("/cluster/firewall/ipset/{name}/{}", encode(cidr)))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("firewall IP set {name} entry {cidr}")).await?;
        Ok(())
    }

    /// Deletes an entry from a firewall IP set.
    ///
    /// `DELETE /cluster/firewall/ipset/{name}/{cidr}`
    pub async fn delete_cluster_firewall_ipset_entry(&self, name: &str, cidr: &str) -> Result<()> {
        validate_resource_id(name)?;
        self.delete_void(
            &format!("/cluster/firewall/ipset/{name}/{}", encode(cidr)),
            &format!("firewall IP set {name} entry {cidr}"),
        )
        .await
    }

    /// Deletes a firewall IP set.
    ///
    /// `DELETE /cluster/firewall/ipset/{name}`
    pub async fn delete_cluster_firewall_ipset(&self, name: &str) -> Result<()> {
        validate_resource_id(name)?;
        self.delete_void(
            &format!("/cluster/firewall/ipset/{name}"),
            &format!("firewall IP set {name}"),
        )
        .await
    }

    // --- Firewall Refs and Macros ---

    /// Lists available firewall references (aliases, IP sets, etc.).
    ///
    /// `GET /cluster/firewall/refs`
    pub async fn list_cluster_firewall_refs(&self) -> Result<Vec<FirewallRef>> {
        self.get_parsed("/cluster/firewall/refs", "firewall refs")
            .await
    }

    /// Lists available firewall macros.
    ///
    /// `GET /cluster/firewall/macros`
    pub async fn list_cluster_firewall_macros(&self) -> Result<Vec<FirewallMacro>> {
        self.get_parsed("/cluster/firewall/macros", "firewall macros")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cluster_firewall_options_serde_roundtrip() {
        let options = ClusterFirewallOptions {
            enable: Some(true),
            policy_in: Some("DROP".to_string()),
            policy_out: Some("ACCEPT".to_string()),
            log_ratelimit: Some("enable=1,rate=1/second,burst=5".to_string()),
            ebtables: Some(true),
        };

        let json = serde_json::to_string(&options).unwrap();
        let deserialized: ClusterFirewallOptions = serde_json::from_str(&json).unwrap();
        assert_eq!(options, deserialized);
    }

    #[test]
    fn cluster_firewall_options_skip_serializing_none() {
        let options = ClusterFirewallOptions::default();
        let json = serde_json::to_value(&options).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default ClusterFirewallOptions should serialize to {{}}"
        );
    }

    #[test]
    fn security_group_serde_roundtrip() {
        let group = SecurityGroup {
            group: Some("webservers".to_string()),
            comment: Some("Web server rules".to_string()),
            digest: Some("abc123".to_string()),
        };

        let json = serde_json::to_string(&group).unwrap();
        let deserialized: SecurityGroup = serde_json::from_str(&json).unwrap();
        assert_eq!(group, deserialized);
    }

    #[test]
    fn cluster_firewall_rule_serde_roundtrip() {
        let rule = FirewallRule {
            pos: Some(0),
            rule_type: Some("in".to_string()),
            action: Some("ACCEPT".to_string()),
            source: Some("10.0.0.0/24".to_string()),
            dest: None,
            proto: Some("tcp".to_string()),
            sport: None,
            dport: Some("22".to_string()),
            comment: Some("Allow SSH".to_string()),
            enable: Some(true),
            log: None,
            macro_name: None,
            iface: None,
            digest: None,
            icmp_type: None,
        };

        let json = serde_json::to_string(&rule).unwrap();
        let deserialized: FirewallRule = serde_json::from_str(&json).unwrap();
        assert_eq!(rule, deserialized);
    }

    #[test]
    fn cluster_firewall_rule_type_rename() {
        let json = r#"{"type": "in", "action": "ACCEPT", "pos": 0}"#;
        let rule: FirewallRule = serde_json::from_str(json).unwrap();
        assert_eq!(rule.rule_type.as_deref(), Some("in"));

        let serialized = serde_json::to_value(&rule).unwrap();
        assert!(serialized.get("type").is_some());
        assert!(serialized.get("rule_type").is_none());
    }

    #[test]
    fn cluster_firewall_rule_macro_rename() {
        let json = r#"{"type": "in", "action": "ACCEPT", "macro": "SSH"}"#;
        let rule: FirewallRule = serde_json::from_str(json).unwrap();
        assert_eq!(rule.macro_name.as_deref(), Some("SSH"));

        let serialized = serde_json::to_value(&rule).unwrap();
        assert!(serialized.get("macro").is_some());
        assert!(serialized.get("macro_name").is_none());
    }

    #[test]
    fn cluster_firewall_rule_create_params_serialization() {
        let mut params = FirewallRuleParams::new("in", "ACCEPT");
        params.proto = Some("tcp".to_string());
        params.dport = Some("443".to_string());
        params.comment = Some("Allow HTTPS".to_string());
        params.enable = Some(true);
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["type"], "in");
        assert_eq!(json["action"], "ACCEPT");
        assert_eq!(json["proto"], "tcp");
        assert_eq!(json["dport"], "443");
        assert!(!json.as_object().unwrap().contains_key("source"));
    }

    #[test]
    fn cluster_firewall_alias_serde_roundtrip() {
        let alias = FirewallAlias {
            name: Some("local_network".to_string()),
            cidr: Some("10.0.0.0/24".to_string()),
            comment: Some("Local network".to_string()),
            digest: None,
        };

        let json = serde_json::to_string(&alias).unwrap();
        let deserialized: FirewallAlias = serde_json::from_str(&json).unwrap();
        assert_eq!(alias, deserialized);
    }

    #[test]
    fn cluster_firewall_ipset_serde_roundtrip() {
        let ipset = FirewallIpSet {
            name: Some("blocklist".to_string()),
            comment: Some("Blocked IPs".to_string()),
            digest: None,
        };

        let json = serde_json::to_string(&ipset).unwrap();
        let deserialized: FirewallIpSet = serde_json::from_str(&json).unwrap();
        assert_eq!(ipset, deserialized);
    }

    #[test]
    fn cluster_firewall_ipset_entry_serde_roundtrip() {
        let entry = FirewallIpSetEntry {
            cidr: Some("192.168.1.0/24".to_string()),
            comment: Some("LAN".to_string()),
            nomatch: Some(false),
            digest: None,
        };

        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: FirewallIpSetEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(entry, deserialized);
    }

    #[test]
    fn firewall_ref_serde_roundtrip() {
        let fref = FirewallRef {
            ref_type: Some("alias".to_string()),
            name: Some("local_network".to_string()),
            comment: Some("Local network alias".to_string()),
        };

        let json = serde_json::to_string(&fref).unwrap();
        let deserialized: FirewallRef = serde_json::from_str(&json).unwrap();
        assert_eq!(fref, deserialized);
    }

    #[test]
    fn firewall_ref_type_rename() {
        let json = r#"{"type": "alias", "name": "test"}"#;
        let fref: FirewallRef = serde_json::from_str(json).unwrap();
        assert_eq!(fref.ref_type.as_deref(), Some("alias"));
    }

    #[test]
    fn firewall_macro_serde_roundtrip() {
        let m = FirewallMacro {
            macro_name: Some("SSH".to_string()),
            descr: Some("Secure Shell".to_string()),
        };

        let json = serde_json::to_string(&m).unwrap();
        let deserialized: FirewallMacro = serde_json::from_str(&json).unwrap();
        assert_eq!(m, deserialized);
    }

    #[test]
    fn firewall_macro_name_rename() {
        let json = r#"{"macro": "HTTP", "descr": "Hypertext Transfer Protocol"}"#;
        let m: FirewallMacro = serde_json::from_str(json).unwrap();
        assert_eq!(m.macro_name.as_deref(), Some("HTTP"));

        let serialized = serde_json::to_value(&m).unwrap();
        assert!(serialized.get("macro").is_some());
        assert!(serialized.get("macro_name").is_none());
    }

    #[test]
    fn cluster_firewall_rule_unknown_fields_ignored() {
        let json = r#"{"type": "in", "action": "DROP", "unknownField": true}"#;
        let rule: FirewallRule = serde_json::from_str(json).unwrap();
        assert_eq!(rule.action.as_deref(), Some("DROP"));
    }

    #[test]
    fn backward_compat_type_aliases() {
        // Verify that type aliases work for backward compatibility.
        let rule = ClusterFirewallRule::default();
        assert!(rule.pos.is_none());

        let alias = ClusterFirewallAlias::default();
        assert!(alias.name.is_none());

        let ipset = ClusterFirewallIpSet::default();
        assert!(ipset.name.is_none());

        let entry = ClusterFirewallIpSetEntry::default();
        assert!(entry.cidr.is_none());
    }
}
