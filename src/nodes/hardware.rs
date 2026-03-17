use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_node_name;

/// A PCI device on a node.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PciDevice {
    /// PCI device ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Vendor name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,

    /// Device name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,

    /// PCI class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,

    /// IOMMU group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iommugroup: Option<i64>,

    /// Whether mediated device types are available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdev: Option<i32>,

    /// Vendor ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    /// Device ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,

    /// Subsystem vendor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subsystem_vendor: Option<String>,

    /// Subsystem device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subsystem_device: Option<String>,
}

/// A mediated device type for a PCI device.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MediatedDevice {
    /// Mediated device type ID.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_id: Option<String>,

    /// Number of available instances.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<i64>,

    /// Description of the mediated device type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// A USB device on a node.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UsbDevice {
    /// USB bus number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub busnum: Option<i64>,

    /// USB device number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devnum: Option<i64>,

    /// Vendor ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendid: Option<String>,

    /// Product ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prodid: Option<String>,

    /// Manufacturer string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,

    /// Product string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,

    /// USB class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<i64>,

    /// USB speed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<String>,

    /// USB level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,

    /// USB port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

    /// USB serial number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,

    /// USB bus unique ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usbpath: Option<String>,
}

impl ProxmoxClient {
    /// Lists PCI devices on a node.
    ///
    /// `GET /nodes/{node}/hardware/pci`
    pub async fn list_pci_devices(&self, node: &str) -> Result<Vec<PciDevice>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/hardware/pci"),
            &format!("node {node} PCI devices"),
        )
        .await
    }

    /// Lists mediated device types for a PCI device.
    ///
    /// `GET /nodes/{node}/hardware/pci/{pciid}/mdev`
    pub async fn list_mediated_devices(
        &self,
        node: &str,
        pciid: &str,
    ) -> Result<Vec<MediatedDevice>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/hardware/pci/{pciid}/mdev"),
            &format!("node {node} PCI {pciid} mdev"),
        )
        .await
    }

    /// Returns details of a specific PCI device on a node.
    ///
    /// `GET /nodes/{node}/hardware/pci/{pciid}`
    pub async fn get_pci_device(&self, node: &str, pciid: &str) -> Result<PciDevice> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/hardware/pci/{pciid}"),
            &format!("node {node} PCI device {pciid}"),
        )
        .await
    }

    /// Lists USB devices on a node.
    ///
    /// `GET /nodes/{node}/hardware/usb`
    pub async fn list_usb_devices(&self, node: &str) -> Result<Vec<UsbDevice>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/hardware/usb"),
            &format!("node {node} USB devices"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pci_device_serde_roundtrip() {
        let device = PciDevice {
            id: Some("0000:01:00.0".to_string()),
            vendor_name: Some("NVIDIA Corporation".to_string()),
            device_name: Some("GP106".to_string()),
            class: Some("0x030000".to_string()),
            iommugroup: Some(1),
            mdev: Some(0),
            ..Default::default()
        };

        let json = serde_json::to_string(&device).unwrap();
        let deserialized: PciDevice = serde_json::from_str(&json).unwrap();
        assert_eq!(device, deserialized);
    }

    #[test]
    fn pci_device_skip_serializing_none() {
        let device = PciDevice::default();
        let json = serde_json::to_value(&device).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty(), "default PciDevice should serialize to {{}}");
    }

    #[test]
    fn mediated_device_serde_roundtrip() {
        let mdev = MediatedDevice {
            type_id: Some("nvidia-256".to_string()),
            available: Some(4),
            description: Some("GRID M60-2Q".to_string()),
        };

        let json = serde_json::to_string(&mdev).unwrap();
        let deserialized: MediatedDevice = serde_json::from_str(&json).unwrap();
        assert_eq!(mdev, deserialized);
    }

    #[test]
    fn mediated_device_type_rename() {
        let json = r#"{"type": "nvidia-256", "available": 4}"#;
        let mdev: MediatedDevice = serde_json::from_str(json).unwrap();
        assert_eq!(mdev.type_id.as_deref(), Some("nvidia-256"));

        let serialized = serde_json::to_value(&mdev).unwrap();
        assert!(serialized.get("type").is_some());
        assert!(serialized.get("type_id").is_none());
    }

    #[test]
    fn usb_device_serde_roundtrip() {
        let device = UsbDevice {
            busnum: Some(1),
            devnum: Some(2),
            vendid: Some("046d".to_string()),
            prodid: Some("c534".to_string()),
            manufacturer: Some("Logitech".to_string()),
            product: Some("USB Receiver".to_string()),
            class: Some(0),
            speed: Some("12".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&device).unwrap();
        let deserialized: UsbDevice = serde_json::from_str(&json).unwrap();
        assert_eq!(device, deserialized);
    }

    #[test]
    fn usb_device_skip_serializing_none() {
        let device = UsbDevice::default();
        let json = serde_json::to_value(&device).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty(), "default UsbDevice should serialize to {{}}");
    }

    #[test]
    fn pci_device_unknown_fields_ignored() {
        let json = r#"{"id": "0000:00:00.0", "unknownField": true}"#;
        let device: PciDevice = serde_json::from_str(json).unwrap();
        assert_eq!(device.id.as_deref(), Some("0000:00:00.0"));
    }
}
