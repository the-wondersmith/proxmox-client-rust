use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_group_id;

/// A Proxmox VE group.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Group {
    /// Group ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupid: Option<String>,

    /// Comment / description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Group members (comma-separated user IDs).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<String>,
}

/// Parameters for creating a new group.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct GroupCreateParams {
    /// Group ID (required).
    pub groupid: String,

    /// Comment / description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl GroupCreateParams {
    /// Creates a new `GroupCreateParams` with the required fields.
    pub fn new(groupid: impl Into<String>) -> Self {
        Self {
            groupid: groupid.into(),
            ..Default::default()
        }
    }
}

/// Parameters for updating an existing group.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct GroupUpdateParams {
    /// Comment / description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl ProxmoxClient {
    /// Lists all groups.
    ///
    /// `GET /access/groups`
    pub async fn list_groups(&self) -> Result<Vec<Group>> {
        self.get_parsed("/access/groups", "groups").await
    }

    /// Creates a new group.
    ///
    /// `POST /access/groups`
    pub async fn create_group(&self, params: &GroupCreateParams) -> Result<()> {
        validate_group_id(&params.groupid)?;
        let response = self.post("/access/groups")?.json(params).send().await?;
        Self::handle_error(response, "group creation").await?;
        Ok(())
    }

    /// Gets a single group by group ID.
    ///
    /// `GET /access/groups/{groupid}`
    pub async fn get_group(&self, groupid: &str) -> Result<Group> {
        validate_group_id(groupid)?;
        self.get_parsed(
            &format!("/access/groups/{groupid}"),
            &format!("group {groupid}"),
        )
        .await
    }

    /// Updates an existing group.
    ///
    /// `PUT /access/groups/{groupid}`
    pub async fn update_group(&self, groupid: &str, params: &GroupUpdateParams) -> Result<()> {
        validate_group_id(groupid)?;
        let response = self
            .put(&format!("/access/groups/{groupid}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("group {groupid}")).await?;
        Ok(())
    }

    /// Deletes a group.
    ///
    /// `DELETE /access/groups/{groupid}`
    pub async fn delete_group(&self, groupid: &str) -> Result<()> {
        validate_group_id(groupid)?;
        self.delete_void(
            &format!("/access/groups/{groupid}"),
            &format!("group {groupid}"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_serde_roundtrip() {
        let group = Group {
            groupid: Some("admins".to_string()),
            comment: Some("System administrators".to_string()),
            members: Some("root@pam,admin@pve".to_string()),
        };

        let json = serde_json::to_string(&group).unwrap();
        let deserialized: Group = serde_json::from_str(&json).unwrap();
        assert_eq!(group, deserialized);
    }

    #[test]
    fn group_skip_serializing_none() {
        let group = Group::default();
        let json = serde_json::to_value(&group).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty(), "default Group should serialize to {{}}");
    }

    #[test]
    fn group_deserialize_from_api() {
        let json = r#"{
            "groupid": "developers",
            "comment": "Dev team",
            "members": "dev1@pve,dev2@pve"
        }"#;
        let group: Group = serde_json::from_str(json).unwrap();
        assert_eq!(group.groupid.as_deref(), Some("developers"));
        assert_eq!(group.comment.as_deref(), Some("Dev team"));
        assert_eq!(group.members.as_deref(), Some("dev1@pve,dev2@pve"));
    }

    #[test]
    fn group_unknown_fields_ignored() {
        let json = r#"{"groupid": "test", "unknownField": 42}"#;
        let group: Group = serde_json::from_str(json).unwrap();
        assert_eq!(group.groupid.as_deref(), Some("test"));
    }

    #[test]
    fn group_create_params_serialization() {
        let params = GroupCreateParams {
            groupid: "newgroup".to_string(),
            comment: Some("A new group".to_string()),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["groupid"], "newgroup");
        assert_eq!(json["comment"], "A new group");
    }

    #[test]
    fn group_update_params_serialization() {
        let params = GroupUpdateParams {
            comment: Some("Updated".to_string()),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["comment"], "Updated");
    }

    #[test]
    fn group_update_params_skip_none() {
        let params = GroupUpdateParams::default();
        let json = serde_json::to_value(&params).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default GroupUpdateParams should serialize to {{}}"
        );
    }
}
