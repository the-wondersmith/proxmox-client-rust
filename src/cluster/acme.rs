use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_resource_id;

/// An ACME account.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AcmeAccount {
    /// Account name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Account contact (email).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,

    /// ACME directory URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,

    /// Account location URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Account status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Terms of Service URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos: Option<String>,

    /// Account data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<serde_json::Value>,
}

/// Parameters for registering an ACME account.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct AcmeAccountCreateParams {
    /// Account name (required).
    pub name: String,

    /// Contact email address (required).
    pub contact: String,

    /// ACME directory URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,

    /// Accept terms of service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_url: Option<String>,
}

impl AcmeAccountCreateParams {
    /// Creates new parameters with required fields.
    pub fn new(name: impl Into<String>, contact: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            contact: contact.into(),
            ..Default::default()
        }
    }
}

/// An ACME plugin configuration.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AcmePlugin {
    /// Plugin ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin: Option<String>,

    /// Plugin type (e.g., `standalone`, `dns`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub plugin_type: Option<String>,

    /// DNS API name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,

    /// Plugin data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// Validation delay in seconds.
    #[serde(rename = "validation-delay", skip_serializing_if = "Option::is_none")]
    pub validation_delay: Option<i64>,

    /// Whether the plugin is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,

    /// Digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

/// Parameters for creating an ACME plugin.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct AcmePluginCreateParams {
    /// Plugin ID (required).
    pub id: String,

    /// Plugin type (required, e.g., `standalone`, `dns`).
    #[serde(rename = "type")]
    pub plugin_type: String,

    /// DNS API name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,

    /// Plugin data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// Validation delay in seconds.
    #[serde(rename = "validation-delay", skip_serializing_if = "Option::is_none")]
    pub validation_delay: Option<i64>,

    /// Whether the plugin is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,
}

impl AcmePluginCreateParams {
    /// Creates new parameters with required fields.
    pub fn new(id: impl Into<String>, plugin_type: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            plugin_type: plugin_type.into(),
            ..Default::default()
        }
    }
}

/// An ACME directory entry.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AcmeDirectory {
    /// Directory name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Directory URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ProxmoxClient {
    // --- ACME Accounts ---

    /// Lists ACME accounts.
    ///
    /// `GET /cluster/acme/account`
    pub async fn list_acme_accounts(&self) -> Result<Vec<AcmeAccount>> {
        self.get_parsed("/cluster/acme/account", "ACME accounts")
            .await
    }

    /// Registers a new ACME account.
    ///
    /// `POST /cluster/acme/account`
    pub async fn create_acme_account(&self, params: &AcmeAccountCreateParams) -> Result<String> {
        self.post_parsed("/cluster/acme/account", params, "ACME account registration")
            .await
    }

    /// Gets a specific ACME account.
    ///
    /// `GET /cluster/acme/account/{name}`
    pub async fn get_acme_account(&self, name: &str) -> Result<AcmeAccount> {
        validate_resource_id(name)?;
        self.get_parsed(
            &format!("/cluster/acme/account/{name}"),
            &format!("ACME account {name}"),
        )
        .await
    }

    /// Updates an ACME account.
    ///
    /// `PUT /cluster/acme/account/{name}`
    pub async fn update_acme_account(
        &self,
        name: &str,
        params: &serde_json::Value,
    ) -> Result<String> {
        validate_resource_id(name)?;
        self.put_parsed(
            &format!("/cluster/acme/account/{name}"),
            params,
            &format!("ACME account {name}"),
        )
        .await
    }

    /// Deactivates an ACME account.
    ///
    /// `DELETE /cluster/acme/account/{name}`
    pub async fn deactivate_acme_account(&self, name: &str) -> Result<()> {
        validate_resource_id(name)?;
        self.delete_void(
            &format!("/cluster/acme/account/{name}"),
            &format!("ACME account {name}"),
        )
        .await
    }

    // --- ACME Plugins ---

    /// Lists ACME plugins.
    ///
    /// `GET /cluster/acme/plugins`
    pub async fn list_acme_plugins(&self) -> Result<Vec<AcmePlugin>> {
        self.get_parsed("/cluster/acme/plugins", "ACME plugins")
            .await
    }

    /// Creates a new ACME plugin.
    ///
    /// `POST /cluster/acme/plugins`
    pub async fn create_acme_plugin(&self, params: &AcmePluginCreateParams) -> Result<()> {
        let response = self
            .post("/cluster/acme/plugins")?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, "ACME plugin creation").await?;
        Ok(())
    }

    /// Gets a specific ACME plugin.
    ///
    /// `GET /cluster/acme/plugins/{id}`
    pub async fn get_acme_plugin(&self, id: &str) -> Result<AcmePlugin> {
        validate_resource_id(id)?;
        self.get_parsed(
            &format!("/cluster/acme/plugins/{id}"),
            &format!("ACME plugin {id}"),
        )
        .await
    }

    /// Updates an ACME plugin.
    ///
    /// `PUT /cluster/acme/plugins/{id}`
    pub async fn update_acme_plugin(&self, id: &str, params: &serde_json::Value) -> Result<()> {
        validate_resource_id(id)?;
        let response = self
            .put(&format!("/cluster/acme/plugins/{id}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("ACME plugin {id}")).await?;
        Ok(())
    }

    /// Deletes an ACME plugin.
    ///
    /// `DELETE /cluster/acme/plugins/{id}`
    pub async fn delete_acme_plugin(&self, id: &str) -> Result<()> {
        validate_resource_id(id)?;
        self.delete_void(
            &format!("/cluster/acme/plugins/{id}"),
            &format!("ACME plugin {id}"),
        )
        .await
    }

    // --- ACME Info ---

    /// Gets the ACME Terms of Service URL.
    ///
    /// `GET /cluster/acme/tos`
    pub async fn get_acme_tos(&self) -> Result<Option<String>> {
        self.get_parsed("/cluster/acme/tos", "ACME ToS").await
    }

    /// Lists available ACME directories.
    ///
    /// `GET /cluster/acme/directories`
    pub async fn list_acme_directories(&self) -> Result<Vec<AcmeDirectory>> {
        self.get_parsed("/cluster/acme/directories", "ACME directories")
            .await
    }

    /// Gets ACME challenge schemas.
    ///
    /// `GET /cluster/acme/challenge-schema`
    pub async fn get_acme_challenge_schema(&self) -> Result<Vec<serde_json::Value>> {
        self.get_parsed("/cluster/acme/challenge-schema", "ACME challenge schema")
            .await
    }

    /// Gets ACME metadata.
    ///
    /// `GET /cluster/acme/meta`
    pub async fn get_acme_meta(&self) -> Result<serde_json::Value> {
        self.get_parsed("/cluster/acme/meta", "ACME metadata").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acme_account_serde_roundtrip() {
        let account = AcmeAccount {
            name: Some("default".to_string()),
            contact: Some("admin@example.com".to_string()),
            directory: Some("https://acme-v02.api.letsencrypt.org/directory".to_string()),
            location: Some("https://acme-v02.api.letsencrypt.org/acme/acct/12345".to_string()),
            status: Some("valid".to_string()),
            tos: Some(
                "https://letsencrypt.org/documents/LE-SA-v1.3-September-21-2022.pdf".to_string(),
            ),
            account: None,
        };

        let json = serde_json::to_string(&account).unwrap();
        let deserialized: AcmeAccount = serde_json::from_str(&json).unwrap();
        assert_eq!(account, deserialized);
    }

    #[test]
    fn acme_account_skip_serializing_none() {
        let account = AcmeAccount::default();
        let json = serde_json::to_value(&account).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default AcmeAccount should serialize to {{}}"
        );
    }

    #[test]
    fn acme_account_create_params_serialization() {
        let params = AcmeAccountCreateParams {
            name: "default".to_string(),
            contact: "admin@example.com".to_string(),
            directory: Some("https://acme-v02.api.letsencrypt.org/directory".to_string()),
            tos_url: None,
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["name"], "default");
        assert_eq!(json["contact"], "admin@example.com");
        assert!(!json.as_object().unwrap().contains_key("tos_url"));
    }

    #[test]
    fn acme_plugin_serde_roundtrip() {
        let plugin = AcmePlugin {
            plugin: Some("standalone".to_string()),
            plugin_type: Some("standalone".to_string()),
            api: None,
            data: None,
            validation_delay: Some(30),
            disable: Some(false),
            digest: None,
        };

        let json = serde_json::to_string(&plugin).unwrap();
        let deserialized: AcmePlugin = serde_json::from_str(&json).unwrap();
        assert_eq!(plugin, deserialized);
    }

    #[test]
    fn acme_plugin_type_rename() {
        let json = r#"{"type": "dns", "plugin": "cloudflare"}"#;
        let plugin: AcmePlugin = serde_json::from_str(json).unwrap();
        assert_eq!(plugin.plugin_type.as_deref(), Some("dns"));

        let serialized = serde_json::to_value(&plugin).unwrap();
        assert!(serialized.get("type").is_some());
        assert!(serialized.get("plugin_type").is_none());
    }

    #[test]
    fn acme_plugin_create_params_serialization() {
        let mut params = AcmePluginCreateParams::new("cloudflare", "dns");
        params.api = Some("cf".to_string());
        params.data = Some("CF_Token=xxx".to_string());
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["id"], "cloudflare");
        assert_eq!(json["type"], "dns");
        assert_eq!(json["api"], "cf");
        assert!(!json.as_object().unwrap().contains_key("disable"));
    }

    #[test]
    fn acme_directory_serde_roundtrip() {
        let dir = AcmeDirectory {
            name: Some("Let's Encrypt V2".to_string()),
            url: Some("https://acme-v02.api.letsencrypt.org/directory".to_string()),
        };

        let json = serde_json::to_string(&dir).unwrap();
        let deserialized: AcmeDirectory = serde_json::from_str(&json).unwrap();
        assert_eq!(dir, deserialized);
    }

    #[test]
    fn acme_account_unknown_fields_ignored() {
        let json = r#"{"name": "default", "unknownField": true}"#;
        let account: AcmeAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.name.as_deref(), Some("default"));
    }

    #[test]
    fn acme_plugin_validation_delay_rename() {
        let json = r#"{"plugin": "test", "validation-delay": 60}"#;
        let plugin: AcmePlugin = serde_json::from_str(json).unwrap();
        assert_eq!(plugin.validation_delay, Some(60));

        let serialized = serde_json::to_value(&plugin).unwrap();
        assert!(serialized.get("validation-delay").is_some());
        assert!(serialized.get("validation_delay").is_none());
    }
}
