use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;

/// An access control list entry.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AclEntry {
    /// The object path (e.g., `/vms/100`, `/storage/local`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// The role ID assigned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roleid: Option<String>,

    /// The principal type: `user`, `group`, or `token`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub ugid_type: Option<String>,

    /// The user, group, or token ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ugid: Option<String>,

    /// Whether the permission propagates to child objects.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub propagate: Option<bool>,
}

/// Parameters for updating the ACL.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct AclUpdateParams {
    /// The object path (required).
    pub path: String,

    /// Comma-separated list of role IDs (required).
    pub roles: String,

    /// Comma-separated list of user IDs (`user@realm`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<String>,

    /// Comma-separated list of group IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<String>,

    /// Comma-separated list of API token IDs (`user@realm!tokenid`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<String>,

    /// Whether the permission propagates to child objects.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub propagate: Option<bool>,

    /// Whether to remove the specified ACL entries instead of adding them.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub delete: Option<bool>,
}

impl AclUpdateParams {
    /// Creates a new `AclUpdateParams` with the required fields.
    pub fn new(path: impl Into<String>, roles: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            roles: roles.into(),
            ..Default::default()
        }
    }
}

impl ProxmoxClient {
    /// Gets the access control list.
    ///
    /// `GET /access/acl`
    pub async fn get_acl(&self) -> Result<Vec<AclEntry>> {
        self.get_parsed("/access/acl", "acl").await
    }

    /// Updates the access control list.
    ///
    /// `PUT /access/acl`
    pub async fn update_acl(&self, params: &AclUpdateParams) -> Result<()> {
        let response = self.put("/access/acl")?.json(params).send().await?;
        Self::handle_error(response, "acl update").await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acl_entry_serde_roundtrip() {
        let entry = AclEntry {
            path: Some("/vms/100".to_string()),
            roleid: Some("PVEVMAdmin".to_string()),
            ugid_type: Some("user".to_string()),
            ugid: Some("admin@pve".to_string()),
            propagate: Some(true),
        };

        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: AclEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(entry, deserialized);
    }

    #[test]
    fn acl_entry_type_field_rename() {
        let entry = AclEntry {
            ugid_type: Some("group".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_value(&entry).unwrap();
        assert!(
            json.get("type").is_some(),
            "ugid_type must serialize as \"type\""
        );
        assert!(
            json.get("ugid_type").is_none(),
            "must not serialize as \"ugid_type\""
        );
    }

    #[test]
    fn acl_entry_skip_serializing_none() {
        let entry = AclEntry::default();
        let json = serde_json::to_value(&entry).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty(), "default AclEntry should serialize to {{}}");
    }

    #[test]
    fn acl_entry_deserialize_from_api() {
        let json = r#"{
            "path": "/storage/local",
            "roleid": "PVEDatastoreAdmin",
            "type": "group",
            "ugid": "admins",
            "propagate": 1
        }"#;
        let entry: AclEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.path.as_deref(), Some("/storage/local"));
        assert_eq!(entry.roleid.as_deref(), Some("PVEDatastoreAdmin"));
        assert_eq!(entry.ugid_type.as_deref(), Some("group"));
        assert_eq!(entry.ugid.as_deref(), Some("admins"));
        assert_eq!(entry.propagate, Some(true));
    }

    #[test]
    fn acl_entry_unknown_fields_ignored() {
        let json = r#"{"path": "/", "unknownField": true}"#;
        let entry: AclEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.path.as_deref(), Some("/"));
    }

    #[test]
    fn acl_update_params_serialization() {
        let params = AclUpdateParams {
            path: "/vms/200".to_string(),
            roles: "PVEVMAdmin".to_string(),
            users: Some("admin@pve".to_string()),
            groups: None,
            tokens: None,
            propagate: Some(true),
            delete: None,
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["path"], "/vms/200");
        assert_eq!(json["roles"], "PVEVMAdmin");
        assert_eq!(json["users"], "admin@pve");
        assert_eq!(json["propagate"], 1);
        assert!(!json.as_object().unwrap().contains_key("groups"));
        assert!(!json.as_object().unwrap().contains_key("tokens"));
        assert!(!json.as_object().unwrap().contains_key("delete"));
    }

    #[test]
    fn acl_update_params_delete_flag() {
        let mut params = AclUpdateParams::new("/vms/100", "PVEAdmin");
        params.users = Some("old@pve".to_string());
        params.delete = Some(true);
        let params = params;
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["delete"], 1);
    }
}
