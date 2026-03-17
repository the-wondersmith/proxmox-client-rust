use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::{validate_resource_id, validate_user_id};

/// A TFA (two-factor authentication) entry for a user.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TfaEntry {
    /// Unique TFA entry ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// TFA type (e.g., `totp`, `u2f`, `webauthn`, `recovery`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub tfa_type: Option<String>,

    /// Human-readable description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Unix timestamp when this entry was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,

    /// Whether this TFA entry is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub enable: Option<bool>,
}

/// A user's TFA entries listing (includes userid).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserTfaList {
    /// The user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userid: Option<String>,

    /// List of TFA entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<TfaEntry>>,
}

/// Parameters for adding a new TFA entry.
#[derive(Clone, Default, Serialize)]
#[non_exhaustive]
pub struct TfaCreateParams {
    /// TFA type (required, e.g., `totp`, `u2f`, `webauthn`, `recovery`).
    #[serde(rename = "type")]
    pub tfa_type: String,

    /// Human-readable description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// TOTP URI or secret (for TOTP type).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totp: Option<String>,

    /// The current password (required for verification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// A TFA challenge response value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl TfaCreateParams {
    /// Creates a new `TfaCreateParams` with the required fields.
    pub fn new(tfa_type: impl Into<String>) -> Self {
        Self {
            tfa_type: tfa_type.into(),
            ..Default::default()
        }
    }
}

impl std::fmt::Debug for TfaCreateParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TfaCreateParams")
            .field("tfa_type", &self.tfa_type)
            .field("description", &self.description)
            .field("totp", &"<redacted>")
            .field("password", &"<redacted>")
            .field("value", &self.value)
            .finish()
    }
}

/// Parameters for updating an existing TFA entry.
#[derive(Clone, Default, Serialize)]
#[non_exhaustive]
pub struct TfaUpdateParams {
    /// Human-readable description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether this TFA entry is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub enable: Option<bool>,

    /// The current password (required for verification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl std::fmt::Debug for TfaUpdateParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TfaUpdateParams")
            .field("description", &self.description)
            .field("enable", &self.enable)
            .field("password", &"<redacted>")
            .finish()
    }
}

impl ProxmoxClient {
    /// Lists all TFA types/entries across all users.
    ///
    /// `GET /access/tfa`
    pub async fn list_tfa(&self) -> Result<Vec<UserTfaList>> {
        self.get_parsed("/access/tfa", "tfa").await
    }

    /// Lists TFA entries for a specific user.
    ///
    /// `GET /access/tfa/{userid}`
    pub async fn list_user_tfa(&self, userid: &str) -> Result<Vec<TfaEntry>> {
        validate_user_id(userid)?;
        self.get_parsed(
            &format!("/access/tfa/{userid}"),
            &format!("user {userid} tfa"),
        )
        .await
    }

    /// Adds a new TFA entry for a user.
    ///
    /// `POST /access/tfa/{userid}`
    pub async fn create_user_tfa(
        &self,
        userid: &str,
        params: &TfaCreateParams,
    ) -> Result<serde_json::Value> {
        validate_user_id(userid)?;
        self.post_parsed(
            &format!("/access/tfa/{userid}"),
            params,
            &format!("user {userid} tfa creation"),
        )
        .await
    }

    /// Gets a specific TFA entry for a user.
    ///
    /// `GET /access/tfa/{userid}/{id}`
    pub async fn get_user_tfa_entry(&self, userid: &str, tfa_id: &str) -> Result<TfaEntry> {
        validate_user_id(userid)?;
        validate_resource_id(tfa_id)?;
        self.get_parsed(
            &format!("/access/tfa/{userid}/{tfa_id}"),
            &format!("user {userid} tfa {tfa_id}"),
        )
        .await
    }

    /// Updates a specific TFA entry for a user.
    ///
    /// `PUT /access/tfa/{userid}/{id}`
    pub async fn update_user_tfa_entry(
        &self,
        userid: &str,
        tfa_id: &str,
        params: &TfaUpdateParams,
    ) -> Result<()> {
        validate_user_id(userid)?;
        validate_resource_id(tfa_id)?;
        let response = self
            .put(&format!("/access/tfa/{userid}/{tfa_id}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("user {userid} tfa {tfa_id}")).await?;
        Ok(())
    }

    /// Deletes a specific TFA entry for a user.
    ///
    /// `DELETE /access/tfa/{userid}/{id}`
    pub async fn delete_user_tfa_entry(&self, userid: &str, tfa_id: &str) -> Result<()> {
        validate_user_id(userid)?;
        validate_resource_id(tfa_id)?;
        self.delete_void(
            &format!("/access/tfa/{userid}/{tfa_id}"),
            &format!("user {userid} tfa {tfa_id}"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tfa_entry_serde_roundtrip() {
        let entry = TfaEntry {
            id: Some("tfa-123".to_string()),
            tfa_type: Some("totp".to_string()),
            description: Some("My TOTP app".to_string()),
            created: Some(1700000000),
            enable: Some(true),
        };

        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: TfaEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(entry, deserialized);
    }

    #[test]
    fn tfa_entry_type_field_rename() {
        let entry = TfaEntry {
            tfa_type: Some("totp".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_value(&entry).unwrap();
        assert!(
            json.get("type").is_some(),
            "tfa_type must serialize as \"type\""
        );
        assert!(
            json.get("tfa_type").is_none(),
            "must not serialize as \"tfa_type\""
        );
    }

    #[test]
    fn tfa_entry_skip_serializing_none() {
        let entry = TfaEntry::default();
        let json = serde_json::to_value(&entry).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty(), "default TfaEntry should serialize to {{}}");
    }

    #[test]
    fn tfa_entry_deserialize_from_api() {
        let json = r#"{
            "id": "tfa-001",
            "type": "u2f",
            "description": "Security Key",
            "created": 1700000000,
            "enable": 1
        }"#;
        let entry: TfaEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.id.as_deref(), Some("tfa-001"));
        assert_eq!(entry.tfa_type.as_deref(), Some("u2f"));
        assert_eq!(entry.description.as_deref(), Some("Security Key"));
        assert_eq!(entry.created, Some(1700000000));
        assert_eq!(entry.enable, Some(true));
    }

    #[test]
    fn tfa_entry_unknown_fields_ignored() {
        let json = r#"{"id": "tfa-1", "unknownField": 42}"#;
        let entry: TfaEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.id.as_deref(), Some("tfa-1"));
    }

    #[test]
    fn tfa_create_params_serialization() {
        let params = TfaCreateParams {
            tfa_type: "totp".to_string(),
            description: Some("My authenticator".to_string()),
            totp: Some("otpauth://totp/test?secret=BASE32SECRET".to_string()),
            password: Some("mypassword".to_string()),
            value: None,
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["type"], "totp");
        assert_eq!(json["description"], "My authenticator");
        assert_eq!(json["totp"], "otpauth://totp/test?secret=BASE32SECRET");
        assert_eq!(json["password"], "mypassword");
        assert!(!json.as_object().unwrap().contains_key("value"));
    }

    #[test]
    fn tfa_create_params_type_rename() {
        let params = TfaCreateParams::new("webauthn");
        let json = serde_json::to_value(&params).unwrap();
        assert!(
            json.get("type").is_some(),
            "tfa_type must serialize as \"type\""
        );
        assert!(
            json.get("tfa_type").is_none(),
            "must not serialize as \"tfa_type\""
        );
    }

    #[test]
    fn tfa_update_params_serialization() {
        let params = TfaUpdateParams {
            description: Some("Updated description".to_string()),
            enable: Some(false),
            password: Some("verify".to_string()),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["description"], "Updated description");
        assert_eq!(json["enable"], 0);
        assert_eq!(json["password"], "verify");
    }

    #[test]
    fn tfa_update_params_skip_none() {
        let params = TfaUpdateParams::default();
        let json = serde_json::to_value(&params).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default TfaUpdateParams should serialize to {{}}"
        );
    }

    #[test]
    fn user_tfa_list_serde_roundtrip() {
        let list = UserTfaList {
            userid: Some("root@pam".to_string()),
            entries: Some(vec![TfaEntry {
                id: Some("tfa-1".to_string()),
                tfa_type: Some("totp".to_string()),
                description: Some("TOTP".to_string()),
                created: Some(1700000000),
                enable: Some(true),
            }]),
        };

        let json = serde_json::to_string(&list).unwrap();
        let deserialized: UserTfaList = serde_json::from_str(&json).unwrap();
        assert_eq!(list, deserialized);
    }

    #[test]
    fn tfa_create_params_debug_redacts_secrets() {
        let params = TfaCreateParams {
            tfa_type: "totp".to_string(),
            description: None,
            totp: Some("otpauth://secret".to_string()),
            password: Some("my-password".to_string()),
            value: None,
        };
        let debug = format!("{params:?}");
        assert!(
            !debug.contains("otpauth://secret"),
            "totp must be redacted in Debug output"
        );
        assert!(
            !debug.contains("my-password"),
            "password must be redacted in Debug output"
        );
        assert!(debug.contains("<redacted>"));
    }

    #[test]
    fn tfa_update_params_debug_redacts_password() {
        let params = TfaUpdateParams {
            description: Some("test".to_string()),
            enable: Some(true),
            password: Some("secret-pass".to_string()),
        };
        let debug = format!("{params:?}");
        assert!(
            !debug.contains("secret-pass"),
            "password must be redacted in Debug output"
        );
        assert!(debug.contains("<redacted>"));
    }
}
