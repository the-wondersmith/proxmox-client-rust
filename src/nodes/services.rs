use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::{validate_node_name, validate_resource_id};

/// A service on a node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NodeService {
    /// Service ID (e.g., `pveproxy`, `pvedaemon`, `sshd`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,

    /// Service name / description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Service description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,

    /// Current state (`running`, `stopped`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// State of a service.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServiceState {
    /// Unit file state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_file_state: Option<String>,

    /// Active state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_state: Option<String>,

    /// Sub state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_state: Option<String>,

    /// Service name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ProxmoxClient {
    /// Lists services on a node.
    pub async fn list_services(&self, node: &str) -> Result<Vec<NodeService>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/services"),
            &format!("node {node} services"),
        )
        .await
    }

    /// Returns the service directory index for a specific service.
    ///
    /// `GET /nodes/{node}/services/{service}`
    pub async fn get_service_info(
        &self,
        node: &str,
        service: &str,
    ) -> Result<Vec<serde_json::Value>> {
        validate_node_name(node)?;
        validate_resource_id(service)?;
        self.get_parsed(
            &format!("/nodes/{node}/services/{service}"),
            &format!("service {service} info"),
        )
        .await
    }

    /// Returns the state of a specific service.
    pub async fn get_service_state(&self, node: &str, service: &str) -> Result<ServiceState> {
        validate_node_name(node)?;
        validate_resource_id(service)?;
        self.get_parsed(
            &format!("/nodes/{node}/services/{service}/state"),
            &format!("service {service}"),
        )
        .await
    }

    /// Starts a service on a node.
    pub async fn start_service(&self, node: &str, service: &str) -> Result<String> {
        validate_node_name(node)?;
        validate_resource_id(service)?;
        let response = self
            .post(&format!("/nodes/{node}/services/{service}/start"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("start service {service}")).await
    }

    /// Stops a service on a node.
    pub async fn stop_service(&self, node: &str, service: &str) -> Result<String> {
        validate_node_name(node)?;
        validate_resource_id(service)?;
        let response = self
            .post(&format!("/nodes/{node}/services/{service}/stop"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("stop service {service}")).await
    }

    /// Restarts a service on a node.
    pub async fn restart_service(&self, node: &str, service: &str) -> Result<String> {
        validate_node_name(node)?;
        validate_resource_id(service)?;
        let response = self
            .post(&format!("/nodes/{node}/services/{service}/restart"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("restart service {service}")).await
    }

    /// Reloads a service on a node.
    pub async fn reload_service(&self, node: &str, service: &str) -> Result<String> {
        validate_node_name(node)?;
        validate_resource_id(service)?;
        let response = self
            .post(&format!("/nodes/{node}/services/{service}/reload"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("reload service {service}")).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_service_serde_roundtrip() {
        let json = r#"{
            "service": "pveproxy",
            "name": "pveproxy",
            "desc": "PVE API Proxy Server",
            "state": "running"
        }"#;
        let svc: NodeService = serde_json::from_str(json).unwrap();
        assert_eq!(svc.service.as_deref(), Some("pveproxy"));
        assert_eq!(svc.state.as_deref(), Some("running"));

        let serialized = serde_json::to_string(&svc).unwrap();
        let deserialized: NodeService = serde_json::from_str(&serialized).unwrap();
        assert_eq!(svc, deserialized);
    }

    #[test]
    fn service_state_serde_roundtrip() {
        let json = r#"{
            "active_state": "active",
            "sub_state": "running",
            "name": "pveproxy.service",
            "description": "PVE API Proxy Server"
        }"#;
        let state: ServiceState = serde_json::from_str(json).unwrap();
        assert_eq!(state.active_state.as_deref(), Some("active"));

        let serialized = serde_json::to_string(&state).unwrap();
        let deserialized: ServiceState = serde_json::from_str(&serialized).unwrap();
        assert_eq!(state, deserialized);
    }
}
