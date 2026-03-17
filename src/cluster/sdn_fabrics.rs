use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_resource_id;

/// An SDN fabric.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SdnFabric {
    /// Fabric ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Fabric type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub fabric_type: Option<String>,

    /// Additional properties.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Parameters for creating an SDN fabric.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct SdnFabricCreateParams {
    /// Fabric ID (required).
    pub id: String,

    /// Fabric type (required).
    #[serde(rename = "type")]
    pub fabric_type: String,

    /// Additional properties.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl SdnFabricCreateParams {
    /// Creates new parameters with required fields.
    pub fn new(id: impl Into<String>, fabric_type: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            fabric_type: fabric_type.into(),
            ..Default::default()
        }
    }
}

/// An SDN fabric node.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SdnFabricNode {
    /// Node name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,

    /// Fabric ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabric_id: Option<String>,

    /// Additional properties.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Parameters for adding a node to an SDN fabric.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct SdnFabricNodeCreateParams {
    /// Node name (required).
    pub node: String,

    /// Additional properties.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl SdnFabricNodeCreateParams {
    /// Creates new parameters with required fields.
    pub fn new(node: impl Into<String>) -> Self {
        Self {
            node: node.into(),
            ..Default::default()
        }
    }
}

impl ProxmoxClient {
    // --- SDN Fabrics: All ---

    /// Lists all SDN fabrics and their details.
    ///
    /// `GET /cluster/sdn/fabrics/all`
    pub async fn list_all_sdn_fabrics(&self) -> Result<Vec<Value>> {
        self.get_parsed("/cluster/sdn/fabrics/all", "SDN fabrics all")
            .await
    }

    // --- SDN Fabrics: Fabric CRUD ---

    /// Lists SDN fabrics.
    ///
    /// `GET /cluster/sdn/fabrics/fabric`
    pub async fn list_sdn_fabrics(&self) -> Result<Vec<SdnFabric>> {
        self.get_parsed("/cluster/sdn/fabrics/fabric", "SDN fabrics")
            .await
    }

    /// Creates an SDN fabric.
    ///
    /// `POST /cluster/sdn/fabrics/fabric`
    pub async fn create_sdn_fabric(&self, params: &SdnFabricCreateParams) -> Result<()> {
        let response = self
            .post("/cluster/sdn/fabrics/fabric")?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, "SDN fabric creation").await?;
        Ok(())
    }

    /// Gets a specific SDN fabric.
    ///
    /// `GET /cluster/sdn/fabrics/fabric/{id}`
    pub async fn get_sdn_fabric(&self, id: &str) -> Result<SdnFabric> {
        validate_resource_id(id)?;
        self.get_parsed(
            &format!("/cluster/sdn/fabrics/fabric/{id}"),
            &format!("SDN fabric {id}"),
        )
        .await
    }

    /// Updates an SDN fabric.
    ///
    /// `PUT /cluster/sdn/fabrics/fabric/{id}`
    pub async fn update_sdn_fabric(&self, id: &str, params: &Value) -> Result<()> {
        validate_resource_id(id)?;
        let response = self
            .put(&format!("/cluster/sdn/fabrics/fabric/{id}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("SDN fabric {id}")).await?;
        Ok(())
    }

    /// Deletes an SDN fabric.
    ///
    /// `DELETE /cluster/sdn/fabrics/fabric/{id}`
    pub async fn delete_sdn_fabric(&self, id: &str) -> Result<()> {
        validate_resource_id(id)?;
        self.delete_void(
            &format!("/cluster/sdn/fabrics/fabric/{id}"),
            &format!("SDN fabric {id}"),
        )
        .await
    }

    // --- SDN Fabrics: Node management ---

    /// Lists all SDN fabric node assignments.
    ///
    /// `GET /cluster/sdn/fabrics/node`
    pub async fn list_all_sdn_fabric_nodes(&self) -> Result<Vec<SdnFabricNode>> {
        self.get_parsed("/cluster/sdn/fabrics/node", "SDN fabric nodes")
            .await
    }

    /// Lists nodes in an SDN fabric.
    ///
    /// `GET /cluster/sdn/fabrics/node/{fabric_id}`
    pub async fn list_sdn_fabric_nodes(&self, fabric_id: &str) -> Result<Vec<SdnFabricNode>> {
        validate_resource_id(fabric_id)?;
        self.get_parsed(
            &format!("/cluster/sdn/fabrics/node/{fabric_id}"),
            &format!("SDN fabric {fabric_id} nodes"),
        )
        .await
    }

    /// Adds a node to an SDN fabric.
    ///
    /// `POST /cluster/sdn/fabrics/node/{fabric_id}`
    pub async fn add_sdn_fabric_node(
        &self,
        fabric_id: &str,
        params: &SdnFabricNodeCreateParams,
    ) -> Result<()> {
        validate_resource_id(fabric_id)?;
        let response = self
            .post(&format!("/cluster/sdn/fabrics/node/{fabric_id}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("SDN fabric {fabric_id} node add")).await?;
        Ok(())
    }

    /// Gets a specific node in an SDN fabric.
    ///
    /// `GET /cluster/sdn/fabrics/node/{fabric_id}/{node_id}`
    pub async fn get_sdn_fabric_node(
        &self,
        fabric_id: &str,
        node_id: &str,
    ) -> Result<SdnFabricNode> {
        validate_resource_id(fabric_id)?;
        validate_resource_id(node_id)?;
        self.get_parsed(
            &format!("/cluster/sdn/fabrics/node/{fabric_id}/{node_id}"),
            &format!("SDN fabric {fabric_id} node {node_id}"),
        )
        .await
    }

    /// Updates a node in an SDN fabric.
    ///
    /// `PUT /cluster/sdn/fabrics/node/{fabric_id}/{node_id}`
    pub async fn update_sdn_fabric_node(
        &self,
        fabric_id: &str,
        node_id: &str,
        params: &Value,
    ) -> Result<()> {
        validate_resource_id(fabric_id)?;
        validate_resource_id(node_id)?;
        let response = self
            .put(&format!("/cluster/sdn/fabrics/node/{fabric_id}/{node_id}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("SDN fabric {fabric_id} node {node_id}")).await?;
        Ok(())
    }

    /// Deletes a node from an SDN fabric.
    ///
    /// `DELETE /cluster/sdn/fabrics/node/{fabric_id}/{node_id}`
    pub async fn delete_sdn_fabric_node(&self, fabric_id: &str, node_id: &str) -> Result<()> {
        validate_resource_id(fabric_id)?;
        validate_resource_id(node_id)?;
        self.delete_void(
            &format!("/cluster/sdn/fabrics/node/{fabric_id}/{node_id}"),
            &format!("SDN fabric {fabric_id} node {node_id}"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sdn_fabric_serde_roundtrip() {
        let fabric = SdnFabric {
            id: Some("ospf1".to_string()),
            fabric_type: Some("ospf".to_string()),
            extra: HashMap::new(),
        };

        let json = serde_json::to_string(&fabric).unwrap();
        let deserialized: SdnFabric = serde_json::from_str(&json).unwrap();
        assert_eq!(fabric, deserialized);
    }

    #[test]
    fn sdn_fabric_type_rename() {
        let json = r#"{"id": "ospf1", "type": "ospf"}"#;
        let fabric: SdnFabric = serde_json::from_str(json).unwrap();
        assert_eq!(fabric.fabric_type.as_deref(), Some("ospf"));

        let serialized = serde_json::to_value(&fabric).unwrap();
        assert!(serialized.get("type").is_some());
        assert!(serialized.get("fabric_type").is_none());
    }

    #[test]
    fn sdn_fabric_create_params_serialization() {
        let params = SdnFabricCreateParams {
            id: "ospf1".to_string(),
            fabric_type: "ospf".to_string(),
            extra: HashMap::new(),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["id"], "ospf1");
        assert_eq!(json["type"], "ospf");
    }

    #[test]
    fn sdn_fabric_node_serde_roundtrip() {
        let node = SdnFabricNode {
            node: Some("pve1".to_string()),
            fabric_id: Some("ospf1".to_string()),
            extra: HashMap::new(),
        };

        let json = serde_json::to_string(&node).unwrap();
        let deserialized: SdnFabricNode = serde_json::from_str(&json).unwrap();
        assert_eq!(node, deserialized);
    }

    #[test]
    fn sdn_fabric_node_create_params_serialization() {
        let params = SdnFabricNodeCreateParams {
            node: "pve1".to_string(),
            extra: HashMap::new(),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["node"], "pve1");
    }
}
