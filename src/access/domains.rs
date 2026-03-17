use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_realm_id;

/// A Proxmox VE authentication realm / domain.
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Realm {
    /// Realm identifier (e.g., `pam`, `pve`, `ldap-corp`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realm: Option<String>,

    /// Realm type (e.g., `pam`, `pve`, `ldap`, `ad`, `openid`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub realm_type: Option<String>,

    /// Comment / description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Whether this is the default realm for login.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub default: Option<bool>,

    /// TFA configuration string (e.g., `type=totp`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfa: Option<String>,

    /// LDAP/AD base domain name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_dn: Option<String>,

    /// LDAP/AD bind domain name (for lookups).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_dn: Option<String>,

    /// LDAP/AD server address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server1: Option<String>,

    /// LDAP/AD fallback server address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server2: Option<String>,

    /// LDAP/AD port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,

    /// Whether to use TLS for the connection.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub secure: Option<bool>,

    /// LDAP user attribute name (e.g., `uid`, `sAMAccountName`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attr: Option<String>,

    /// OpenID Connect issuer URL.
    #[serde(rename = "issuer-url", skip_serializing_if = "Option::is_none")]
    pub issuer_url: Option<String>,

    /// OpenID Connect client ID.
    #[serde(rename = "client-id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

    /// OpenID Connect client key.
    #[serde(rename = "client-key", skip_serializing_if = "Option::is_none")]
    pub client_key: Option<String>,

    /// Autocreate users on login.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub autocreate: Option<bool>,

    /// Username claim for OpenID Connect.
    #[serde(rename = "username-claim", skip_serializing_if = "Option::is_none")]
    pub username_claim: Option<String>,
}

impl std::fmt::Debug for Realm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Realm")
            .field("realm", &self.realm)
            .field("realm_type", &self.realm_type)
            .field("comment", &self.comment)
            .field("default", &self.default)
            .field("tfa", &self.tfa)
            .field("base_dn", &self.base_dn)
            .field("bind_dn", &self.bind_dn)
            .field("server1", &self.server1)
            .field("server2", &self.server2)
            .field("port", &self.port)
            .field("secure", &self.secure)
            .field("user_attr", &self.user_attr)
            .field("issuer_url", &self.issuer_url)
            .field("client_id", &self.client_id)
            .field("client_key", &"<redacted>")
            .field("autocreate", &self.autocreate)
            .field("username_claim", &self.username_claim)
            .finish()
    }
}

/// Parameters for creating a new authentication realm.
#[derive(Clone, Default, Serialize)]
#[non_exhaustive]
pub struct RealmCreateParams {
    /// Realm identifier (required).
    pub realm: String,

    /// Realm type (required, e.g., `ldap`, `ad`, `openid`).
    #[serde(rename = "type")]
    pub realm_type: String,

    /// Comment / description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Whether this is the default realm for login.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub default: Option<bool>,

    /// TFA configuration string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfa: Option<String>,

    /// LDAP/AD base domain name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_dn: Option<String>,

    /// LDAP/AD bind domain name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_dn: Option<String>,

    /// LDAP/AD server address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server1: Option<String>,

    /// LDAP/AD fallback server address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server2: Option<String>,

    /// LDAP/AD port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,

    /// Whether to use TLS for the connection.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub secure: Option<bool>,

    /// LDAP user attribute name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attr: Option<String>,

    /// OpenID Connect issuer URL.
    #[serde(rename = "issuer-url", skip_serializing_if = "Option::is_none")]
    pub issuer_url: Option<String>,

    /// OpenID Connect client ID.
    #[serde(rename = "client-id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

    /// OpenID Connect client key.
    #[serde(rename = "client-key", skip_serializing_if = "Option::is_none")]
    pub client_key: Option<String>,

    /// Autocreate users on login.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub autocreate: Option<bool>,

    /// Username claim for OpenID Connect.
    #[serde(rename = "username-claim", skip_serializing_if = "Option::is_none")]
    pub username_claim: Option<String>,
}

impl RealmCreateParams {
    /// Creates a new `RealmCreateParams` with the required fields.
    pub fn new(realm: impl Into<String>, realm_type: impl Into<String>) -> Self {
        Self {
            realm: realm.into(),
            realm_type: realm_type.into(),
            ..Default::default()
        }
    }
}

impl std::fmt::Debug for RealmCreateParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RealmCreateParams")
            .field("realm", &self.realm)
            .field("realm_type", &self.realm_type)
            .field("comment", &self.comment)
            .field("default", &self.default)
            .field("tfa", &self.tfa)
            .field("base_dn", &self.base_dn)
            .field("bind_dn", &self.bind_dn)
            .field("server1", &self.server1)
            .field("server2", &self.server2)
            .field("port", &self.port)
            .field("secure", &self.secure)
            .field("user_attr", &self.user_attr)
            .field("issuer_url", &self.issuer_url)
            .field("client_id", &self.client_id)
            .field("client_key", &"<redacted>")
            .field("autocreate", &self.autocreate)
            .field("username_claim", &self.username_claim)
            .finish()
    }
}

/// Parameters for updating an existing authentication realm.
#[derive(Clone, Default, Serialize)]
#[non_exhaustive]
pub struct RealmUpdateParams {
    /// Comment / description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Whether this is the default realm for login.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub default: Option<bool>,

    /// TFA configuration string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfa: Option<String>,

    /// LDAP/AD base domain name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_dn: Option<String>,

    /// LDAP/AD bind domain name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_dn: Option<String>,

    /// LDAP/AD server address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server1: Option<String>,

    /// LDAP/AD fallback server address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server2: Option<String>,

    /// LDAP/AD port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,

    /// Whether to use TLS for the connection.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub secure: Option<bool>,

    /// LDAP user attribute name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attr: Option<String>,

    /// OpenID Connect issuer URL.
    #[serde(rename = "issuer-url", skip_serializing_if = "Option::is_none")]
    pub issuer_url: Option<String>,

    /// OpenID Connect client ID.
    #[serde(rename = "client-id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

    /// OpenID Connect client key.
    #[serde(rename = "client-key", skip_serializing_if = "Option::is_none")]
    pub client_key: Option<String>,

    /// Autocreate users on login.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub autocreate: Option<bool>,

    /// Username claim for OpenID Connect.
    #[serde(rename = "username-claim", skip_serializing_if = "Option::is_none")]
    pub username_claim: Option<String>,
}

impl std::fmt::Debug for RealmUpdateParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RealmUpdateParams")
            .field("comment", &self.comment)
            .field("default", &self.default)
            .field("tfa", &self.tfa)
            .field("base_dn", &self.base_dn)
            .field("bind_dn", &self.bind_dn)
            .field("server1", &self.server1)
            .field("server2", &self.server2)
            .field("port", &self.port)
            .field("secure", &self.secure)
            .field("user_attr", &self.user_attr)
            .field("issuer_url", &self.issuer_url)
            .field("client_id", &self.client_id)
            .field("client_key", &"<redacted>")
            .field("autocreate", &self.autocreate)
            .field("username_claim", &self.username_claim)
            .finish()
    }
}

/// Parameters for syncing an LDAP/AD realm.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct RealmSyncParams {
    /// Dry-run mode -- if set, show what would happen without making changes.
    #[serde(default, rename = "dry-run", skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub dry_run: Option<bool>,

    /// Whether to enable new users found during sync.
    #[serde(default, rename = "enable-new", skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub enable_new: Option<bool>,

    /// Whether to perform a full sync (remove users/groups not found in the directory).
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub full: Option<bool>,

    /// Whether to purge ACLs for removed users/groups.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub purge: Option<bool>,

    /// Sync scope: `users`, `groups`, or `both`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

impl ProxmoxClient {
    /// Lists all authentication realms.
    ///
    /// `GET /access/domains`
    pub async fn list_realms(&self) -> Result<Vec<Realm>> {
        self.get_parsed("/access/domains", "realms").await
    }

    /// Creates a new authentication realm.
    ///
    /// `POST /access/domains`
    pub async fn create_realm(&self, params: &RealmCreateParams) -> Result<()> {
        validate_realm_id(&params.realm)?;
        let response = self.post("/access/domains")?.json(params).send().await?;
        Self::handle_error(response, "realm creation").await?;
        Ok(())
    }

    /// Gets a single authentication realm configuration.
    ///
    /// `GET /access/domains/{realm}`
    pub async fn get_realm(&self, realm: &str) -> Result<Realm> {
        validate_realm_id(realm)?;
        self.get_parsed(
            &format!("/access/domains/{realm}"),
            &format!("realm {realm}"),
        )
        .await
    }

    /// Updates an existing authentication realm.
    ///
    /// `PUT /access/domains/{realm}`
    pub async fn update_realm(&self, realm: &str, params: &RealmUpdateParams) -> Result<()> {
        validate_realm_id(realm)?;
        let response = self
            .put(&format!("/access/domains/{realm}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("realm {realm}")).await?;
        Ok(())
    }

    /// Deletes an authentication realm.
    ///
    /// `DELETE /access/domains/{realm}`
    pub async fn delete_realm(&self, realm: &str) -> Result<()> {
        validate_realm_id(realm)?;
        self.delete_void(
            &format!("/access/domains/{realm}"),
            &format!("realm {realm}"),
        )
        .await
    }

    /// Syncs users and/or groups from an LDAP/AD realm.
    ///
    /// `POST /access/domains/{realm}/sync`
    pub async fn sync_realm(&self, realm: &str, params: &RealmSyncParams) -> Result<String> {
        validate_realm_id(realm)?;
        self.post_parsed(
            &format!("/access/domains/{realm}/sync"),
            params,
            &format!("realm {realm} sync"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn realm_serde_roundtrip() {
        let realm = Realm {
            realm: Some("pam".to_string()),
            realm_type: Some("pam".to_string()),
            comment: Some("Linux PAM".to_string()),
            default: Some(false),
            tfa: None,
            base_dn: None,
            bind_dn: None,
            server1: None,
            server2: None,
            port: None,
            secure: None,
            user_attr: None,
            issuer_url: None,
            client_id: None,
            client_key: None,
            autocreate: None,
            username_claim: None,
        };

        let json = serde_json::to_string(&realm).unwrap();
        let deserialized: Realm = serde_json::from_str(&json).unwrap();
        assert_eq!(realm, deserialized);
    }

    #[test]
    fn realm_type_field_rename() {
        let realm = Realm {
            realm_type: Some("ldap".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_value(&realm).unwrap();
        assert!(
            json.get("type").is_some(),
            "realm_type must serialize as \"type\""
        );
        assert!(
            json.get("realm_type").is_none(),
            "must not serialize as \"realm_type\""
        );
    }

    #[test]
    fn realm_skip_serializing_none() {
        let realm = Realm::default();
        let json = serde_json::to_value(&realm).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty(), "default Realm should serialize to {{}}");
    }

    #[test]
    fn realm_deserialize_from_api() {
        let json = r#"{
            "realm": "ldap-corp",
            "type": "ldap",
            "comment": "Corporate LDAP",
            "default": 0,
            "server1": "ldap.corp.example.com",
            "port": 636,
            "secure": 1,
            "base_dn": "dc=corp,dc=example,dc=com",
            "user_attr": "uid"
        }"#;
        let realm: Realm = serde_json::from_str(json).unwrap();
        assert_eq!(realm.realm.as_deref(), Some("ldap-corp"));
        assert_eq!(realm.realm_type.as_deref(), Some("ldap"));
        assert_eq!(realm.server1.as_deref(), Some("ldap.corp.example.com"));
        assert_eq!(realm.port, Some(636));
        assert_eq!(realm.secure, Some(true));
        assert_eq!(realm.user_attr.as_deref(), Some("uid"));
    }

    #[test]
    fn realm_openid_fields() {
        let json = r#"{
            "realm": "myoidc",
            "type": "openid",
            "issuer-url": "https://auth.example.com",
            "client-id": "proxmox-client",
            "client-key": "secret123",
            "username-claim": "email",
            "autocreate": 1
        }"#;
        let realm: Realm = serde_json::from_str(json).unwrap();
        assert_eq!(
            realm.issuer_url.as_deref(),
            Some("https://auth.example.com")
        );
        assert_eq!(realm.client_id.as_deref(), Some("proxmox-client"));
        assert_eq!(realm.client_key.as_deref(), Some("secret123"));
        assert_eq!(realm.username_claim.as_deref(), Some("email"));
        assert_eq!(realm.autocreate, Some(true));
    }

    #[test]
    fn realm_unknown_fields_ignored() {
        let json = r#"{"realm": "pam", "unknownField": 42}"#;
        let realm: Realm = serde_json::from_str(json).unwrap();
        assert_eq!(realm.realm.as_deref(), Some("pam"));
    }

    #[test]
    fn realm_create_params_serialization() {
        let mut params = RealmCreateParams::new("myldap", "ldap");
        params.comment = Some("My LDAP".to_string());
        params.server1 = Some("ldap.example.com".to_string());
        params.base_dn = Some("dc=example,dc=com".to_string());
        let params = params;
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["realm"], "myldap");
        assert_eq!(json["type"], "ldap");
        assert_eq!(json["comment"], "My LDAP");
        assert_eq!(json["server1"], "ldap.example.com");
        assert_eq!(json["base_dn"], "dc=example,dc=com");
        assert!(!json.as_object().unwrap().contains_key("port"));
    }

    #[test]
    fn realm_update_params_serialization() {
        let params = RealmUpdateParams {
            comment: Some("Updated LDAP".to_string()),
            default: Some(true),
            ..Default::default()
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["comment"], "Updated LDAP");
        assert_eq!(json["default"], 1);
        assert!(!json.as_object().unwrap().contains_key("server1"));
    }

    #[test]
    fn realm_sync_params_serialization() {
        let params = RealmSyncParams {
            dry_run: Some(true),
            enable_new: Some(true),
            full: Some(false),
            purge: None,
            scope: Some("both".to_string()),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["dry-run"], 1);
        assert_eq!(json["enable-new"], 1);
        assert_eq!(json["full"], 0);
        assert!(!json.as_object().unwrap().contains_key("purge"));
        assert_eq!(json["scope"], "both");
    }

    #[test]
    fn realm_sync_params_skip_none() {
        let params = RealmSyncParams::default();
        let json = serde_json::to_value(&params).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default RealmSyncParams should serialize to {{}}"
        );
    }

    #[test]
    fn realm_debug_redacts_client_key() {
        let realm = Realm {
            client_key: Some("super-secret-key".to_string()),
            ..Default::default()
        };
        let debug = format!("{realm:?}");
        assert!(
            !debug.contains("super-secret-key"),
            "client_key must be redacted in Debug output"
        );
        assert!(debug.contains("<redacted>"));
    }

    #[test]
    fn realm_create_params_debug_redacts_client_key() {
        let mut params = RealmCreateParams::new("test", "openid");
        params.client_key = Some("secret-key-123".to_string());
        let debug = format!("{params:?}");
        assert!(
            !debug.contains("secret-key-123"),
            "client_key must be redacted in Debug output"
        );
        assert!(debug.contains("<redacted>"));
    }

    #[test]
    fn realm_update_params_debug_redacts_client_key() {
        let params = RealmUpdateParams {
            client_key: Some("secret-update-key".to_string()),
            ..Default::default()
        };
        let debug = format!("{params:?}");
        assert!(
            !debug.contains("secret-update-key"),
            "client_key must be redacted in Debug output"
        );
        assert!(debug.contains("<redacted>"));
    }
}
