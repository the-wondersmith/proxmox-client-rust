use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_node_name;

/// Subscription information for a node.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SubscriptionInfo {
    /// Subscription status (e.g., `Active`, `New`, `NotFound`, `Invalid`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Server ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverid: Option<String>,

    /// Last check timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checktime: Option<String>,

    /// Subscription key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// Subscription level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,

    /// Product name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub productname: Option<String>,

    /// Registration date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regdate: Option<String>,

    /// Next due date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nextduedate: Option<String>,

    /// Subscription URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Status message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Parameters for setting a subscription key.
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub struct SubscriptionKeyParams {
    /// Subscription key.
    pub key: String,
}

impl SubscriptionKeyParams {
    /// Creates a new `SubscriptionKeyParams` with the required fields.
    pub fn new(key: impl Into<String>) -> Self {
        Self { key: key.into() }
    }
}

impl ProxmoxClient {
    /// Gets the subscription status of a node.
    ///
    /// `GET /nodes/{node}/subscription`
    pub async fn get_subscription(&self, node: &str) -> Result<SubscriptionInfo> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/subscription"),
            &format!("node {node} subscription"),
        )
        .await
    }

    /// Checks/updates the subscription status.
    ///
    /// `POST /nodes/{node}/subscription`
    pub async fn check_subscription(&self, node: &str) -> Result<()> {
        validate_node_name(node)?;
        let response = self
            .post(&format!("/nodes/{node}/subscription"))?
            .send()
            .await?;
        Self::handle_error(response, &format!("node {node} subscription check")).await?;
        Ok(())
    }

    /// Sets the subscription key.
    ///
    /// `PUT /nodes/{node}/subscription`
    pub async fn set_subscription_key(
        &self,
        node: &str,
        params: &SubscriptionKeyParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        let response = self
            .put(&format!("/nodes/{node}/subscription"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("node {node} subscription key")).await?;
        Ok(())
    }

    /// Deletes the subscription key.
    ///
    /// `DELETE /nodes/{node}/subscription`
    pub async fn delete_subscription(&self, node: &str) -> Result<()> {
        validate_node_name(node)?;
        self.delete_void(
            &format!("/nodes/{node}/subscription"),
            &format!("node {node} subscription"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subscription_info_serde_roundtrip() {
        let info = SubscriptionInfo {
            status: Some("Active".to_string()),
            serverid: Some("abc123".to_string()),
            key: Some("pve1c-1234567890".to_string()),
            level: Some("c".to_string()),
            productname: Some("Proxmox VE Community".to_string()),
            regdate: Some("2024-01-01 00:00:00".to_string()),
            nextduedate: Some("2025-01-01 00:00:00".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&info).unwrap();
        let deserialized: SubscriptionInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(info, deserialized);
    }

    #[test]
    fn subscription_info_skip_serializing_none() {
        let info = SubscriptionInfo::default();
        let json = serde_json::to_value(&info).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default SubscriptionInfo should serialize to {{}}"
        );
    }

    #[test]
    fn subscription_key_params_serialization() {
        let params = SubscriptionKeyParams {
            key: "pve1c-1234567890".to_string(),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["key"], "pve1c-1234567890");
    }

    #[test]
    fn subscription_info_unknown_fields_ignored() {
        let json = r#"{"status": "Active", "unknownField": true}"#;
        let info: SubscriptionInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.status.as_deref(), Some("Active"));
    }
}
