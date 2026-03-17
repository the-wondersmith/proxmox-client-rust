use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::{validate_ha_sid, validate_node_name, validate_resource_id};

/// An HA-managed resource.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HaResource {
    /// Service ID (e.g., `vm:100`, `ct:200`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,

    /// Resource type (`vm` or `ct`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,

    /// Current state (e.g., `started`, `stopped`, `error`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Requested state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_state: Option<String>,

    /// HA group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    /// Maximum number of restart attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_restart: Option<i64>,

    /// Maximum number of relocate attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_relocate: Option<i64>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,

    /// Status text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Parameters for creating an HA resource.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct HaResourceCreateParams {
    /// Service ID (required, e.g., `vm:100`).
    pub sid: String,

    /// Requested state (`started`, `stopped`, `disabled`, `ignored`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// HA group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    /// Maximum number of restart attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_restart: Option<i64>,

    /// Maximum number of relocate attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_relocate: Option<i64>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl HaResourceCreateParams {
    /// Creates a new `HaResourceCreateParams` with the required fields.
    pub fn new(sid: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            ..Default::default()
        }
    }
}

/// Parameters for updating an HA resource.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct HaResourceUpdateParams {
    /// Requested state (`started`, `stopped`, `disabled`, `ignored`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// HA group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    /// Maximum number of restart attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_restart: Option<i64>,

    /// Maximum number of relocate attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_relocate: Option<i64>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Comma-separated list of properties to delete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<String>,

    /// Digest for change detection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

/// An HA group definition.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HaGroup {
    /// Group ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    /// Comma-separated list of nodes with optional priority (e.g., `node1:2,node2:1`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<String>,

    /// Whether the resource is restricted to the group nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub restricted: Option<bool>,

    /// Whether failback to higher-priority node is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub nofailback: Option<bool>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Resource type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub group_type: Option<String>,

    /// Digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

/// Parameters for creating an HA group.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct HaGroupCreateParams {
    /// Group ID (required).
    pub group: String,

    /// Comma-separated list of nodes with optional priority (required).
    pub nodes: String,

    /// Whether the resource is restricted to the group nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub restricted: Option<bool>,

    /// Whether failback to higher-priority node is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub nofailback: Option<bool>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl HaGroupCreateParams {
    /// Creates a new `HaGroupCreateParams` with the required fields.
    pub fn new(group: impl Into<String>, nodes: impl Into<String>) -> Self {
        Self {
            group: group.into(),
            nodes: nodes.into(),
            ..Default::default()
        }
    }
}

/// Parameters for updating an HA group.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct HaGroupUpdateParams {
    /// Comma-separated list of nodes with optional priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<String>,

    /// Whether the resource is restricted to the group nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub restricted: Option<bool>,

    /// Whether failback to higher-priority node is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub nofailback: Option<bool>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Comma-separated list of properties to delete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<String>,

    /// Digest for change detection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

/// HA status information.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HaStatus {
    /// Quorum information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quorum: Option<serde_json::Value>,

    /// Master node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_status: Option<serde_json::Value>,
}

/// HA manager status information.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HaManagerStatus {
    /// Manager status data.
    #[serde(flatten)]
    pub data: Option<serde_json::Value>,
}

/// An HA rule.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HaRule {
    /// Rule identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,

    /// Rule type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Property name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,

    /// Rule expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// Digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,

    /// Additional properties.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Parameters for creating an HA rule.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct HaRuleCreateParams {
    /// Rule type (required).
    #[serde(rename = "type")]
    pub rule_type: String,

    /// Property name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,

    /// Rule expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl HaRuleCreateParams {
    /// Creates a new `HaRuleCreateParams` with the required fields.
    pub fn new(rule_type: impl Into<String>) -> Self {
        Self {
            rule_type: rule_type.into(),
            ..Default::default()
        }
    }
}

/// Parameters for updating an HA rule.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct HaRuleUpdateParams {
    /// Property name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,

    /// Rule expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Comma-separated list of properties to delete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<String>,

    /// Digest for change detection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

impl ProxmoxClient {
    // --- HA Resources ---

    /// Lists HA resources.
    ///
    /// `GET /cluster/ha/resources`
    pub async fn list_ha_resources(&self) -> Result<Vec<HaResource>> {
        self.get_parsed("/cluster/ha/resources", "HA resources")
            .await
    }

    /// Creates an HA resource.
    ///
    /// `POST /cluster/ha/resources`
    pub async fn create_ha_resource(&self, params: &HaResourceCreateParams) -> Result<()> {
        let response = self
            .post("/cluster/ha/resources")?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, "HA resource creation").await?;
        Ok(())
    }

    /// Gets a specific HA resource.
    ///
    /// `GET /cluster/ha/resources/{sid}`
    pub async fn get_ha_resource(&self, sid: &str) -> Result<HaResource> {
        validate_ha_sid(sid)?;
        self.get_parsed(
            &format!("/cluster/ha/resources/{sid}"),
            &format!("HA resource {sid}"),
        )
        .await
    }

    /// Updates an HA resource.
    ///
    /// `PUT /cluster/ha/resources/{sid}`
    pub async fn update_ha_resource(&self, sid: &str, params: &HaResourceUpdateParams) -> Result<()> {
        validate_ha_sid(sid)?;
        let response = self
            .put(&format!("/cluster/ha/resources/{sid}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("HA resource {sid}")).await?;
        Ok(())
    }

    /// Deletes an HA resource.
    ///
    /// `DELETE /cluster/ha/resources/{sid}`
    pub async fn delete_ha_resource(&self, sid: &str) -> Result<()> {
        validate_ha_sid(sid)?;
        self.delete_void(
            &format!("/cluster/ha/resources/{sid}"),
            &format!("HA resource {sid}"),
        )
        .await
    }

    /// Requests migration of an HA resource to another node.
    ///
    /// `POST /cluster/ha/resources/{sid}/migrate`
    pub async fn migrate_ha_resource(&self, sid: &str, node: &str) -> Result<()> {
        validate_ha_sid(sid)?;
        validate_node_name(node)?;
        let body = serde_json::json!({ "node": node });
        let response = self
            .post(&format!("/cluster/ha/resources/{sid}/migrate"))?
            .json(&body)
            .send()
            .await?;
        Self::handle_error(response, &format!("HA resource {sid} migration")).await?;
        Ok(())
    }

    /// Requests relocation of an HA resource to another node.
    ///
    /// `POST /cluster/ha/resources/{sid}/relocate`
    pub async fn relocate_ha_resource(&self, sid: &str, node: &str) -> Result<()> {
        validate_ha_sid(sid)?;
        validate_node_name(node)?;
        let body = serde_json::json!({ "node": node });
        let response = self
            .post(&format!("/cluster/ha/resources/{sid}/relocate"))?
            .json(&body)
            .send()
            .await?;
        Self::handle_error(response, &format!("HA resource {sid} relocation")).await?;
        Ok(())
    }

    // --- HA Groups ---

    /// Lists HA groups.
    ///
    /// `GET /cluster/ha/groups`
    pub async fn list_ha_groups(&self) -> Result<Vec<HaGroup>> {
        self.get_parsed("/cluster/ha/groups", "HA groups").await
    }

    /// Creates an HA group.
    ///
    /// `POST /cluster/ha/groups`
    pub async fn create_ha_group(&self, params: &HaGroupCreateParams) -> Result<()> {
        let response = self.post("/cluster/ha/groups")?.json(params).send().await?;
        Self::handle_error(response, "HA group creation").await?;
        Ok(())
    }

    /// Gets a specific HA group.
    ///
    /// `GET /cluster/ha/groups/{group}`
    pub async fn get_ha_group(&self, group: &str) -> Result<HaGroup> {
        validate_resource_id(group)?;
        self.get_parsed(
            &format!("/cluster/ha/groups/{group}"),
            &format!("HA group {group}"),
        )
        .await
    }

    /// Updates an HA group.
    ///
    /// `PUT /cluster/ha/groups/{group}`
    pub async fn update_ha_group(&self, group: &str, params: &HaGroupUpdateParams) -> Result<()> {
        validate_resource_id(group)?;
        let response = self
            .put(&format!("/cluster/ha/groups/{group}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("HA group {group}")).await?;
        Ok(())
    }

    /// Deletes an HA group.
    ///
    /// `DELETE /cluster/ha/groups/{group}`
    pub async fn delete_ha_group(&self, group: &str) -> Result<()> {
        validate_resource_id(group)?;
        self.delete_void(
            &format!("/cluster/ha/groups/{group}"),
            &format!("HA group {group}"),
        )
        .await
    }

    // --- HA Status ---

    /// Gets the current HA status.
    ///
    /// `GET /cluster/ha/status/current`
    pub async fn get_ha_status(&self) -> Result<Vec<HaStatus>> {
        self.get_parsed("/cluster/ha/status/current", "HA status")
            .await
    }

    /// Gets the HA manager status.
    ///
    /// `GET /cluster/ha/status/manager_status`
    pub async fn get_ha_manager_status(&self) -> Result<HaManagerStatus> {
        self.get_parsed("/cluster/ha/status/manager_status", "HA manager status")
            .await
    }

    // --- HA Rules ---

    /// Lists HA rules.
    ///
    /// `GET /cluster/ha/rules`
    pub async fn list_ha_rules(&self) -> Result<Vec<HaRule>> {
        self.get_parsed("/cluster/ha/rules", "HA rules").await
    }

    /// Creates an HA rule.
    ///
    /// `POST /cluster/ha/rules`
    pub async fn create_ha_rule(&self, params: &HaRuleCreateParams) -> Result<()> {
        let response = self.post("/cluster/ha/rules")?.json(params).send().await?;
        Self::handle_error(response, "HA rule creation").await?;
        Ok(())
    }

    /// Gets a specific HA rule.
    ///
    /// `GET /cluster/ha/rules/{rule}`
    pub async fn get_ha_rule(&self, rule: &str) -> Result<HaRule> {
        validate_resource_id(rule)?;
        self.get_parsed(
            &format!("/cluster/ha/rules/{rule}"),
            &format!("HA rule {rule}"),
        )
        .await
    }

    /// Updates an HA rule.
    ///
    /// `PUT /cluster/ha/rules/{rule}`
    pub async fn update_ha_rule(&self, rule: &str, params: &HaRuleUpdateParams) -> Result<()> {
        validate_resource_id(rule)?;
        let response = self
            .put(&format!("/cluster/ha/rules/{rule}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("HA rule {rule}")).await?;
        Ok(())
    }

    /// Deletes an HA rule.
    ///
    /// `DELETE /cluster/ha/rules/{rule}`
    pub async fn delete_ha_rule(&self, rule: &str) -> Result<()> {
        validate_resource_id(rule)?;
        self.delete_void(
            &format!("/cluster/ha/rules/{rule}"),
            &format!("HA rule {rule}"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ha_resource_serde_roundtrip() {
        let resource = HaResource {
            sid: Some("vm:100".to_string()),
            resource_type: Some("vm".to_string()),
            state: Some("started".to_string()),
            request_state: Some("started".to_string()),
            group: Some("prefer_pve1".to_string()),
            max_restart: Some(3),
            max_relocate: Some(1),
            comment: Some("Critical VM".to_string()),
            digest: None,
            status: None,
        };

        let json = serde_json::to_string(&resource).unwrap();
        let deserialized: HaResource = serde_json::from_str(&json).unwrap();
        assert_eq!(resource, deserialized);
    }

    #[test]
    fn ha_resource_skip_serializing_none() {
        let resource = HaResource::default();
        let json = serde_json::to_value(&resource).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default HaResource should serialize to {{}}"
        );
    }

    #[test]
    fn ha_resource_type_rename() {
        let json = r#"{"type": "vm", "sid": "vm:100"}"#;
        let resource: HaResource = serde_json::from_str(json).unwrap();
        assert_eq!(resource.resource_type.as_deref(), Some("vm"));

        let serialized = serde_json::to_value(&resource).unwrap();
        assert!(serialized.get("type").is_some());
        assert!(serialized.get("resource_type").is_none());
    }

    #[test]
    fn ha_resource_create_params_serialization() {
        let params = HaResourceCreateParams {
            sid: "vm:100".to_string(),
            state: Some("started".to_string()),
            group: Some("prefer_pve1".to_string()),
            max_restart: Some(3),
            max_relocate: Some(1),
            comment: None,
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["sid"], "vm:100");
        assert_eq!(json["state"], "started");
        assert_eq!(json["group"], "prefer_pve1");
        assert!(!json.as_object().unwrap().contains_key("comment"));
    }

    #[test]
    fn ha_group_serde_roundtrip() {
        let group = HaGroup {
            group: Some("prefer_pve1".to_string()),
            nodes: Some("pve1:2,pve2:1".to_string()),
            restricted: Some(false),
            nofailback: Some(false),
            comment: Some("Prefer pve1".to_string()),
            group_type: None,
            digest: None,
        };

        let json = serde_json::to_string(&group).unwrap();
        let deserialized: HaGroup = serde_json::from_str(&json).unwrap();
        assert_eq!(group, deserialized);
    }

    #[test]
    fn ha_group_skip_serializing_none() {
        let group = HaGroup::default();
        let json = serde_json::to_value(&group).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty(), "default HaGroup should serialize to {{}}");
    }

    #[test]
    fn ha_group_create_params_serialization() {
        let params = HaGroupCreateParams {
            group: "prefer_pve1".to_string(),
            nodes: "pve1:2,pve2:1".to_string(),
            restricted: Some(true),
            nofailback: Some(false),
            comment: None,
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["group"], "prefer_pve1");
        assert_eq!(json["nodes"], "pve1:2,pve2:1");
        assert_eq!(json["restricted"], 1);
        assert!(!json.as_object().unwrap().contains_key("comment"));
    }

    #[test]
    fn ha_resource_unknown_fields_ignored() {
        let json = r#"{"sid": "vm:100", "unknownField": true}"#;
        let resource: HaResource = serde_json::from_str(json).unwrap();
        assert_eq!(resource.sid.as_deref(), Some("vm:100"));
    }

    #[test]
    fn ha_status_serde_roundtrip() {
        let status = HaStatus {
            quorum: Some(serde_json::json!({"node": "pve1"})),
            manager_status: None,
        };

        let json = serde_json::to_string(&status).unwrap();
        let deserialized: HaStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, deserialized);
    }
}
