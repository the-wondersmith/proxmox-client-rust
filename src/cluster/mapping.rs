use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_resource_id;

/// A PCI device mapping.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PciMapping {
    /// Mapping ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Mapping entries (node-specific device paths).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map: Option<Vec<String>>,

    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether mediated devices are used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdev: Option<i32>,
}

/// Parameters for creating a PCI mapping.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct PciMappingCreateParams {
    /// Mapping ID (required).
    pub id: String,

    /// Mapping entries (required).
    pub map: Vec<String>,

    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether mediated devices are used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdev: Option<i32>,
}

impl PciMappingCreateParams {
    /// Creates a new `PciMappingCreateParams` with the required fields.
    pub fn new(id: impl Into<String>, map: Vec<String>) -> Self {
        Self {
            id: id.into(),
            map,
            ..Default::default()
        }
    }
}

/// A USB device mapping.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UsbMapping {
    /// Mapping ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Mapping entries (node-specific device paths).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map: Option<Vec<String>>,

    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Parameters for creating a USB mapping.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct UsbMappingCreateParams {
    /// Mapping ID (required).
    pub id: String,

    /// Mapping entries (required).
    pub map: Vec<String>,

    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl UsbMappingCreateParams {
    /// Creates a new `UsbMappingCreateParams` with the required fields.
    pub fn new(id: impl Into<String>, map: Vec<String>) -> Self {
        Self {
            id: id.into(),
            map,
            ..Default::default()
        }
    }
}

/// A directory mapping.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DirMapping {
    /// Mapping ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Mapping entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map: Option<Vec<String>>,

    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Parameters for creating a directory mapping.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct DirMappingCreateParams {
    /// Mapping ID (required).
    pub id: String,

    /// Mapping entries (required).
    pub map: Vec<String>,

    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl DirMappingCreateParams {
    /// Creates a new `DirMappingCreateParams` with the required fields.
    pub fn new(id: impl Into<String>, map: Vec<String>) -> Self {
        Self {
            id: id.into(),
            map,
            ..Default::default()
        }
    }
}

impl ProxmoxClient {
    // --- PCI Mappings ---

    /// Lists all PCI device mappings.
    ///
    /// `GET /cluster/mapping/pci`
    pub async fn list_pci_mappings(&self) -> Result<Vec<PciMapping>> {
        self.get_parsed("/cluster/mapping/pci", "PCI mappings")
            .await
    }

    /// Creates a new PCI device mapping.
    ///
    /// `POST /cluster/mapping/pci`
    pub async fn create_pci_mapping(&self, params: &PciMappingCreateParams) -> Result<()> {
        let response = self
            .post("/cluster/mapping/pci")?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, "PCI mapping creation").await?;
        Ok(())
    }

    /// Gets a specific PCI device mapping.
    ///
    /// `GET /cluster/mapping/pci/{id}`
    pub async fn get_pci_mapping(&self, id: &str) -> Result<PciMapping> {
        validate_resource_id(id)?;
        self.get_parsed(
            &format!("/cluster/mapping/pci/{id}"),
            &format!("PCI mapping {id}"),
        )
        .await
    }

    /// Updates a PCI device mapping.
    ///
    /// `PUT /cluster/mapping/pci/{id}`
    pub async fn update_pci_mapping(&self, id: &str, params: &serde_json::Value) -> Result<()> {
        validate_resource_id(id)?;
        let response = self
            .put(&format!("/cluster/mapping/pci/{id}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("PCI mapping {id}")).await?;
        Ok(())
    }

    /// Deletes a PCI device mapping.
    ///
    /// `DELETE /cluster/mapping/pci/{id}`
    pub async fn delete_pci_mapping(&self, id: &str) -> Result<()> {
        validate_resource_id(id)?;
        self.delete_void(
            &format!("/cluster/mapping/pci/{id}"),
            &format!("PCI mapping {id}"),
        )
        .await
    }

    // --- USB Mappings ---

    /// Lists all USB device mappings.
    ///
    /// `GET /cluster/mapping/usb`
    pub async fn list_usb_mappings(&self) -> Result<Vec<UsbMapping>> {
        self.get_parsed("/cluster/mapping/usb", "USB mappings")
            .await
    }

    /// Creates a new USB device mapping.
    ///
    /// `POST /cluster/mapping/usb`
    pub async fn create_usb_mapping(&self, params: &UsbMappingCreateParams) -> Result<()> {
        let response = self
            .post("/cluster/mapping/usb")?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, "USB mapping creation").await?;
        Ok(())
    }

    /// Gets a specific USB device mapping.
    ///
    /// `GET /cluster/mapping/usb/{id}`
    pub async fn get_usb_mapping(&self, id: &str) -> Result<UsbMapping> {
        validate_resource_id(id)?;
        self.get_parsed(
            &format!("/cluster/mapping/usb/{id}"),
            &format!("USB mapping {id}"),
        )
        .await
    }

    /// Updates a USB device mapping.
    ///
    /// `PUT /cluster/mapping/usb/{id}`
    pub async fn update_usb_mapping(&self, id: &str, params: &serde_json::Value) -> Result<()> {
        validate_resource_id(id)?;
        let response = self
            .put(&format!("/cluster/mapping/usb/{id}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("USB mapping {id}")).await?;
        Ok(())
    }

    /// Deletes a USB device mapping.
    ///
    /// `DELETE /cluster/mapping/usb/{id}`
    pub async fn delete_usb_mapping(&self, id: &str) -> Result<()> {
        validate_resource_id(id)?;
        self.delete_void(
            &format!("/cluster/mapping/usb/{id}"),
            &format!("USB mapping {id}"),
        )
        .await
    }

    // --- Directory Mappings ---

    /// Lists all directory mappings.
    ///
    /// `GET /cluster/mapping/dir`
    pub async fn list_dir_mappings(&self) -> Result<Vec<DirMapping>> {
        self.get_parsed("/cluster/mapping/dir", "directory mappings")
            .await
    }

    /// Creates a new directory mapping.
    ///
    /// `POST /cluster/mapping/dir`
    pub async fn create_dir_mapping(&self, params: &DirMappingCreateParams) -> Result<()> {
        let response = self
            .post("/cluster/mapping/dir")?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, "directory mapping creation").await?;
        Ok(())
    }

    /// Gets a specific directory mapping.
    ///
    /// `GET /cluster/mapping/dir/{id}`
    pub async fn get_dir_mapping(&self, id: &str) -> Result<DirMapping> {
        validate_resource_id(id)?;
        self.get_parsed(
            &format!("/cluster/mapping/dir/{id}"),
            &format!("directory mapping {id}"),
        )
        .await
    }

    /// Updates a directory mapping.
    ///
    /// `PUT /cluster/mapping/dir/{id}`
    pub async fn update_dir_mapping(&self, id: &str, params: &serde_json::Value) -> Result<()> {
        validate_resource_id(id)?;
        let response = self
            .put(&format!("/cluster/mapping/dir/{id}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("directory mapping {id}")).await?;
        Ok(())
    }

    /// Deletes a directory mapping.
    ///
    /// `DELETE /cluster/mapping/dir/{id}`
    pub async fn delete_dir_mapping(&self, id: &str) -> Result<()> {
        validate_resource_id(id)?;
        self.delete_void(
            &format!("/cluster/mapping/dir/{id}"),
            &format!("directory mapping {id}"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pci_mapping_serde_roundtrip() {
        let mapping = PciMapping {
            id: Some("gpu0".to_string()),
            map: Some(vec!["node=pve1,path=0000:01:00.0,id=10de:1b06".to_string()]),
            description: Some("GPU passthrough".to_string()),
            mdev: Some(0),
        };

        let json = serde_json::to_string(&mapping).unwrap();
        let deserialized: PciMapping = serde_json::from_str(&json).unwrap();
        assert_eq!(mapping, deserialized);
    }

    #[test]
    fn pci_mapping_skip_serializing_none() {
        let mapping = PciMapping::default();
        let json = serde_json::to_value(&mapping).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default PciMapping should serialize to {{}}"
        );
    }

    #[test]
    fn pci_mapping_create_params_serialization() {
        let params = PciMappingCreateParams {
            id: "gpu0".to_string(),
            map: vec!["node=pve1,path=0000:01:00.0".to_string()],
            description: Some("GPU".to_string()),
            mdev: None,
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["id"], "gpu0");
        assert!(!json.as_object().unwrap().contains_key("mdev"));
    }

    #[test]
    fn usb_mapping_serde_roundtrip() {
        let mapping = UsbMapping {
            id: Some("keyboard0".to_string()),
            map: Some(vec!["node=pve1,path=1-2,id=046d:c534".to_string()]),
            description: Some("Wireless keyboard".to_string()),
        };

        let json = serde_json::to_string(&mapping).unwrap();
        let deserialized: UsbMapping = serde_json::from_str(&json).unwrap();
        assert_eq!(mapping, deserialized);
    }

    #[test]
    fn usb_mapping_skip_serializing_none() {
        let mapping = UsbMapping::default();
        let json = serde_json::to_value(&mapping).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default UsbMapping should serialize to {{}}"
        );
    }

    #[test]
    fn usb_mapping_create_params_serialization() {
        let params = UsbMappingCreateParams {
            id: "keyboard0".to_string(),
            map: vec!["node=pve1,path=1-2".to_string()],
            description: Some("Keyboard".to_string()),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["id"], "keyboard0");
        assert_eq!(json["description"], "Keyboard");
    }

    #[test]
    fn pci_mapping_unknown_fields_ignored() {
        let json = r#"{"id": "gpu0", "unknownField": true}"#;
        let mapping: PciMapping = serde_json::from_str(json).unwrap();
        assert_eq!(mapping.id.as_deref(), Some("gpu0"));
    }
}
