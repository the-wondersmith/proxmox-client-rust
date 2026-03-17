use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;

/// Proxmox VE version information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Version {
    /// The Proxmox VE version string (e.g., `8.1.3`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// The release version (e.g., `8.1`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,

    /// Repository ID / package version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repoid: Option<String>,

    /// Console viewer type (e.g., `applet`, `vv`, `html5`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console: Option<String>,
}

impl ProxmoxClient {
    /// Returns the Proxmox VE API version.
    pub async fn version(&self) -> Result<Version> {
        self.get_parsed("/version", "version").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_deserialize() {
        let json = r#"{
            "version": "8.1.3",
            "release": "8.1",
            "repoid": "d3a7bcaa9962cd93",
            "console": "applet"
        }"#;
        let version: Version = serde_json::from_str(json).unwrap();
        assert_eq!(version.version.as_deref(), Some("8.1.3"));
        assert_eq!(version.release.as_deref(), Some("8.1"));
        assert_eq!(version.repoid.as_deref(), Some("d3a7bcaa9962cd93"));
        assert_eq!(version.console.as_deref(), Some("applet"));
    }

    #[test]
    fn version_serde_roundtrip() {
        let version = Version {
            version: Some("8.1.3".to_string()),
            release: Some("8.1".to_string()),
            repoid: Some("abc123".to_string()),
            console: None,
        };
        let json = serde_json::to_string(&version).unwrap();
        let deserialized: Version = serde_json::from_str(&json).unwrap();
        assert_eq!(version, deserialized);
    }

    #[test]
    fn version_skip_none() {
        let version = Version {
            version: Some("8.1".to_string()),
            release: None,
            repoid: None,
            console: None,
        };
        let json = serde_json::to_value(&version).unwrap();
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("release"));
        assert!(!obj.contains_key("repoid"));
    }
}
