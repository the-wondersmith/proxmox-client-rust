use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_node_name;

/// A QEMU CPU model.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CpuModel {
    /// CPU model name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// CPU vendor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    /// Whether this is a custom CPU model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<i32>,
}

/// A QEMU machine type.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MachineType {
    /// Machine type identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Machine type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub machine_type: Option<String>,

    /// Machine version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl ProxmoxClient {
    /// Lists available QEMU CPU models on a node.
    ///
    /// `GET /nodes/{node}/capabilities/qemu/cpu`
    pub async fn list_qemu_cpu_models(&self, node: &str) -> Result<Vec<CpuModel>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/capabilities/qemu/cpu"),
            &format!("node {node} QEMU CPU models"),
        )
        .await
    }

    /// Lists available QEMU CPU flags on a node.
    ///
    /// `GET /nodes/{node}/capabilities/qemu/cpu-flags`
    pub async fn list_qemu_cpu_flags(&self, node: &str) -> Result<Vec<Value>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/capabilities/qemu/cpu-flags"),
            &format!("node {node} QEMU CPU flags"),
        )
        .await
    }

    /// Lists available QEMU machine types on a node.
    ///
    /// `GET /nodes/{node}/capabilities/qemu/machines`
    pub async fn list_qemu_machine_types(&self, node: &str) -> Result<Vec<MachineType>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/capabilities/qemu/machines"),
            &format!("node {node} QEMU machine types"),
        )
        .await
    }

    /// Gets QEMU migration capabilities on a node.
    ///
    /// `GET /nodes/{node}/capabilities/qemu/migration`
    pub async fn get_qemu_migration_caps(&self, node: &str) -> Result<Value> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/capabilities/qemu/migration"),
            &format!("node {node} QEMU migration caps"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_model_serde_roundtrip() {
        let model = CpuModel {
            name: Some("kvm64".to_string()),
            vendor: Some("AuthenticAMD".to_string()),
            custom: Some(0),
        };

        let json = serde_json::to_string(&model).unwrap();
        let deserialized: CpuModel = serde_json::from_str(&json).unwrap();
        assert_eq!(model, deserialized);
    }

    #[test]
    fn cpu_model_skip_serializing_none() {
        let model = CpuModel::default();
        let json = serde_json::to_value(&model).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty(), "default CpuModel should serialize to {{}}");
    }

    #[test]
    fn machine_type_serde_roundtrip() {
        let mt = MachineType {
            id: Some("pc-i440fx-8.1".to_string()),
            machine_type: Some("i440fx".to_string()),
            version: Some("8.1".to_string()),
        };

        let json = serde_json::to_string(&mt).unwrap();
        let deserialized: MachineType = serde_json::from_str(&json).unwrap();
        assert_eq!(mt, deserialized);
    }

    #[test]
    fn machine_type_type_rename() {
        let json = r#"{"id": "pc", "type": "i440fx"}"#;
        let mt: MachineType = serde_json::from_str(json).unwrap();
        assert_eq!(mt.machine_type.as_deref(), Some("i440fx"));

        let serialized = serde_json::to_value(&mt).unwrap();
        assert!(serialized.get("type").is_some());
        assert!(serialized.get("machine_type").is_none());
    }

    #[test]
    fn cpu_model_unknown_fields_ignored() {
        let json = r#"{"name": "host", "unknownField": true}"#;
        let model: CpuModel = serde_json::from_str(json).unwrap();
        assert_eq!(model.name.as_deref(), Some("host"));
    }
}
