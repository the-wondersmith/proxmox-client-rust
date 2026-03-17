use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_role_id;

/// A Proxmox VE role definition.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Role {
    /// Role ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roleid: Option<String>,

    /// Comma-separated list of privileges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privs: Option<String>,

    /// Whether this is a built-in special role.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub special: Option<bool>,
}

/// Parameters for creating a new role.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct RoleCreateParams {
    /// Role ID (required).
    pub roleid: String,

    /// Comma-separated list of privileges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privs: Option<String>,
}

impl RoleCreateParams {
    /// Creates a new `RoleCreateParams` with the required fields.
    pub fn new(roleid: impl Into<String>) -> Self {
        Self {
            roleid: roleid.into(),
            ..Default::default()
        }
    }
}

/// Parameters for updating an existing role.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct RoleUpdateParams {
    /// Comma-separated list of privileges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privs: Option<String>,

    /// Whether to append privileges instead of replacing.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub append: Option<bool>,
}

impl ProxmoxClient {
    /// Lists all roles.
    ///
    /// `GET /access/roles`
    pub async fn list_roles(&self) -> Result<Vec<Role>> {
        self.get_parsed("/access/roles", "roles").await
    }

    /// Creates a new role.
    ///
    /// `POST /access/roles`
    pub async fn create_role(&self, params: &RoleCreateParams) -> Result<()> {
        validate_role_id(&params.roleid)?;
        let response = self.post("/access/roles")?.json(params).send().await?;
        Self::handle_error(response, "role creation").await?;
        Ok(())
    }

    /// Gets a single role by role ID.
    ///
    /// `GET /access/roles/{roleid}`
    pub async fn get_role(&self, roleid: &str) -> Result<Role> {
        validate_role_id(roleid)?;
        self.get_parsed(
            &format!("/access/roles/{roleid}"),
            &format!("role {roleid}"),
        )
        .await
    }

    /// Updates an existing role.
    ///
    /// `PUT /access/roles/{roleid}`
    pub async fn update_role(&self, roleid: &str, params: &RoleUpdateParams) -> Result<()> {
        validate_role_id(roleid)?;
        let response = self
            .put(&format!("/access/roles/{roleid}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("role {roleid}")).await?;
        Ok(())
    }

    /// Deletes a role.
    ///
    /// `DELETE /access/roles/{roleid}`
    pub async fn delete_role(&self, roleid: &str) -> Result<()> {
        validate_role_id(roleid)?;
        self.delete_void(
            &format!("/access/roles/{roleid}"),
            &format!("role {roleid}"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_serde_roundtrip() {
        let role = Role {
            roleid: Some("PVEAdmin".to_string()),
            privs: Some("VM.Allocate,VM.Audit,VM.Config.CDROM".to_string()),
            special: Some(true),
        };

        let json = serde_json::to_string(&role).unwrap();
        let deserialized: Role = serde_json::from_str(&json).unwrap();
        assert_eq!(role, deserialized);
    }

    #[test]
    fn role_skip_serializing_none() {
        let role = Role::default();
        let json = serde_json::to_value(&role).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty(), "default Role should serialize to {{}}");
    }

    #[test]
    fn role_deserialize_from_api() {
        let json = r#"{
            "roleid": "PVEVMAdmin",
            "privs": "VM.Allocate,VM.Audit,VM.Backup,VM.Clone",
            "special": 1
        }"#;
        let role: Role = serde_json::from_str(json).unwrap();
        assert_eq!(role.roleid.as_deref(), Some("PVEVMAdmin"));
        assert_eq!(role.special, Some(true));
    }

    #[test]
    fn role_unknown_fields_ignored() {
        let json = r#"{"roleid": "test", "unknownField": true}"#;
        let role: Role = serde_json::from_str(json).unwrap();
        assert_eq!(role.roleid.as_deref(), Some("test"));
    }

    #[test]
    fn role_create_params_serialization() {
        let params = RoleCreateParams {
            roleid: "CustomRole".to_string(),
            privs: Some("VM.Audit,VM.Console".to_string()),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["roleid"], "CustomRole");
        assert_eq!(json["privs"], "VM.Audit,VM.Console");
    }

    #[test]
    fn role_update_params_serialization() {
        let params = RoleUpdateParams {
            privs: Some("VM.Audit".to_string()),
            append: Some(true),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["privs"], "VM.Audit");
        assert_eq!(json["append"], 1);
    }

    #[test]
    fn role_update_params_skip_none() {
        let params = RoleUpdateParams::default();
        let json = serde_json::to_value(&params).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default RoleUpdateParams should serialize to {{}}"
        );
    }
}
