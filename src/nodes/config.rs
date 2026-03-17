use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_node_name;

/// Node configuration.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NodeConfig {
    /// Node description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// ACME account name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acme: Option<String>,

    /// ACME domain configuration (index 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acmedomain0: Option<String>,

    /// ACME domain configuration (index 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acmedomain1: Option<String>,

    /// ACME domain configuration (index 2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acmedomain2: Option<String>,

    /// ACME domain configuration (index 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acmedomain3: Option<String>,

    /// ACME domain configuration (index 4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acmedomain4: Option<String>,

    /// ACME domain configuration (index 5).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acmedomain5: Option<String>,

    /// Delay in seconds before starting all VMs/CTs on boot.
    #[serde(
        rename = "startall-onboot-delay",
        skip_serializing_if = "Option::is_none"
    )]
    pub startall_onboot_delay: Option<i64>,

    /// MAC address for Wake-on-LAN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wakeonlan: Option<String>,

    /// Configuration digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

/// Parameters for updating node configuration.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct NodeConfigUpdateParams {
    /// Node description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// ACME account name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acme: Option<String>,

    /// ACME domain configuration (index 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acmedomain0: Option<String>,

    /// ACME domain configuration (index 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acmedomain1: Option<String>,

    /// ACME domain configuration (index 2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acmedomain2: Option<String>,

    /// ACME domain configuration (index 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acmedomain3: Option<String>,

    /// ACME domain configuration (index 4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acmedomain4: Option<String>,

    /// ACME domain configuration (index 5).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acmedomain5: Option<String>,

    /// Delay in seconds before starting all VMs/CTs on boot.
    #[serde(
        rename = "startall-onboot-delay",
        skip_serializing_if = "Option::is_none"
    )]
    pub startall_onboot_delay: Option<i64>,

    /// MAC address for Wake-on-LAN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wakeonlan: Option<String>,

    /// List of settings to delete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<String>,

    /// Prevent changes if current config digest differs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

impl ProxmoxClient {
    /// Gets the node configuration.
    ///
    /// `GET /nodes/{node}/config`
    pub async fn get_node_config(&self, node: &str) -> Result<NodeConfig> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/config"),
            &format!("node {node} config"),
        )
        .await
    }

    /// Updates the node configuration.
    ///
    /// `PUT /nodes/{node}/config`
    pub async fn update_node_config(
        &self,
        node: &str,
        params: &NodeConfigUpdateParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        let response = self
            .put(&format!("/nodes/{node}/config"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("node {node} config")).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_config_serde_roundtrip() {
        let config = NodeConfig {
            description: Some("Production node".to_string()),
            acme: Some("default".to_string()),
            acmedomain0: Some("pve.example.com".to_string()),
            startall_onboot_delay: Some(30),
            wakeonlan: Some("AA:BB:CC:DD:EE:FF".to_string()),
            digest: Some("abc123".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: NodeConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config, deserialized);
    }

    #[test]
    fn node_config_skip_serializing_none() {
        let config = NodeConfig::default();
        let json = serde_json::to_value(&config).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default NodeConfig should serialize to {{}}"
        );
    }

    #[test]
    fn node_config_update_params_serialization() {
        let params = NodeConfigUpdateParams {
            description: Some("Updated description".to_string()),
            startall_onboot_delay: Some(60),
            ..Default::default()
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["description"], "Updated description");
        assert_eq!(json["startall-onboot-delay"], 60);
        assert!(!json.as_object().unwrap().contains_key("acme"));
    }

    #[test]
    fn node_config_unknown_fields_ignored() {
        let json = r#"{"description": "test", "unknownField": true}"#;
        let config: NodeConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.description.as_deref(), Some("test"));
    }
}
