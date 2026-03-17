use serde::{Deserialize, Serialize};
use urlencoding::encode;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_pool_id;

/// A Proxmox VE resource pool.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Pool {
    /// Pool ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poolid: Option<String>,

    /// Comment / description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Pool members (VMs and storage).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<PoolMember>>,
}

/// A member of a resource pool (VM, container, or storage).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PoolMember {
    /// Member ID (VMID or storage name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Node the member resides on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,

    /// Member type (`qemu`, `lxc`, or `storage`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,

    /// VMID for VM/container members.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmid: Option<u32>,

    /// Storage name for storage members.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,

    /// Current status of the member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Parameters for creating a new pool.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct PoolCreateParams {
    /// Pool ID (required).
    pub poolid: String,

    /// Comment / description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl PoolCreateParams {
    /// Creates new pool creation parameters with the given pool ID.
    pub fn new(poolid: impl Into<String>) -> Self {
        Self {
            poolid: poolid.into(),
            ..Default::default()
        }
    }
}

/// Parameters for updating an existing pool.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct PoolUpdateParams {
    /// Pool ID (required for non-deprecated `PUT /pools` endpoint).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poolid: Option<String>,

    /// Comment / description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Comma-separated list of VMIDs to add or remove.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vms: Option<String>,

    /// Comma-separated list of storage IDs to add or remove.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,

    /// Whether to remove the specified VMs/storage instead of adding them.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub delete: Option<bool>,
}

impl ProxmoxClient {
    /// Lists all resource pools.
    ///
    /// `GET /pools`
    pub async fn list_pools(&self) -> Result<Vec<Pool>> {
        self.get_parsed("/pools", "pools").await
    }

    /// Creates a new resource pool.
    ///
    /// `POST /pools`
    pub async fn create_pool(&self, params: &PoolCreateParams) -> Result<()> {
        validate_pool_id(&params.poolid)?;
        let response = self.post("/pools")?.json(params).send().await?;
        Self::handle_error(response, "pool creation").await?;
        Ok(())
    }

    /// Gets a single pool by pool ID.
    ///
    /// `GET /pools/{poolid}`
    pub async fn get_pool(&self, poolid: &str) -> Result<Pool> {
        validate_pool_id(poolid)?;
        self.get_parsed(&format!("/pools/{poolid}"), &format!("pool {poolid}"))
            .await
    }

    /// Updates an existing pool.
    ///
    /// `PUT /pools/{poolid}`
    pub async fn update_pool(&self, poolid: &str, params: &PoolUpdateParams) -> Result<()> {
        validate_pool_id(poolid)?;
        let response = self
            .put(&format!("/pools/{poolid}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("pool {poolid}")).await?;
        Ok(())
    }

    /// Deletes a pool.
    ///
    /// `DELETE /pools/{poolid}`
    pub async fn delete_pool(&self, poolid: &str) -> Result<()> {
        validate_pool_id(poolid)?;
        self.delete_void(&format!("/pools/{poolid}"), &format!("pool {poolid}"))
            .await
    }

    /// Updates an existing pool (non-deprecated endpoint).
    ///
    /// `PUT /pools`
    ///
    /// The `poolid` field must be set in `params`.
    pub async fn update_pool_params(&self, params: &PoolUpdateParams) -> Result<()> {
        let response = self.put("/pools")?.json(params).send().await?;
        Self::handle_error(response, "pool update").await?;
        Ok(())
    }

    /// Deletes a pool (non-deprecated endpoint).
    ///
    /// `DELETE /pools?poolid={poolid}`
    pub async fn delete_pool_params(&self, poolid: &str) -> Result<()> {
        validate_pool_id(poolid)?;
        let response = self
            .delete(&format!("/pools?poolid={}", encode(poolid)))?
            .send()
            .await?;
        Self::handle_error(response, &format!("pool {poolid}")).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool_serde_roundtrip() {
        let pool = Pool {
            poolid: Some("production".to_string()),
            comment: Some("Production VMs".to_string()),
            members: Some(vec![
                PoolMember {
                    id: Some("100".to_string()),
                    node: Some("pve1".to_string()),
                    member_type: Some("qemu".to_string()),
                    vmid: Some(100),
                    storage: None,
                    status: Some("running".to_string()),
                },
                PoolMember {
                    id: Some("local-lvm".to_string()),
                    node: Some("pve1".to_string()),
                    member_type: Some("storage".to_string()),
                    vmid: None,
                    storage: Some("local-lvm".to_string()),
                    status: Some("available".to_string()),
                },
            ]),
        };

        let json = serde_json::to_string(&pool).unwrap();
        let deserialized: Pool = serde_json::from_str(&json).unwrap();
        assert_eq!(pool, deserialized);
    }

    #[test]
    fn pool_skip_serializing_none() {
        let pool = Pool::default();
        let json = serde_json::to_value(&pool).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty(), "default Pool should serialize to {{}}");
    }

    #[test]
    fn pool_deserialize_from_api() {
        let json = r#"{
            "poolid": "dev-pool",
            "comment": "Development resources",
            "members": [
                {
                    "id": "101",
                    "node": "pve1",
                    "type": "qemu",
                    "vmid": 101,
                    "status": "stopped"
                }
            ]
        }"#;
        let pool: Pool = serde_json::from_str(json).unwrap();
        assert_eq!(pool.poolid.as_deref(), Some("dev-pool"));
        assert_eq!(pool.comment.as_deref(), Some("Development resources"));
        let members = pool.members.unwrap();
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].vmid, Some(101));
        assert_eq!(members[0].member_type.as_deref(), Some("qemu"));
    }

    #[test]
    fn pool_unknown_fields_ignored() {
        let json = r#"{"poolid": "test", "unknownField": 42}"#;
        let pool: Pool = serde_json::from_str(json).unwrap();
        assert_eq!(pool.poolid.as_deref(), Some("test"));
    }

    #[test]
    fn pool_member_serde_roundtrip() {
        let member = PoolMember {
            id: Some("200".to_string()),
            node: Some("pve2".to_string()),
            member_type: Some("lxc".to_string()),
            vmid: Some(200),
            storage: None,
            status: Some("running".to_string()),
        };

        let json = serde_json::to_string(&member).unwrap();
        let deserialized: PoolMember = serde_json::from_str(&json).unwrap();
        assert_eq!(member, deserialized);
    }

    #[test]
    fn pool_member_type_field_rename() {
        let member = PoolMember {
            member_type: Some("qemu".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_value(&member).unwrap();
        assert!(
            json.get("type").is_some(),
            "type field must serialize as 'type'"
        );
    }

    #[test]
    fn pool_member_skip_serializing_none() {
        let member = PoolMember::default();
        let json = serde_json::to_value(&member).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default PoolMember should serialize to {{}}"
        );
    }

    #[test]
    fn pool_create_params_serialization() {
        let params = PoolCreateParams {
            poolid: "newpool".to_string(),
            comment: Some("A new pool".to_string()),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["poolid"], "newpool");
        assert_eq!(json["comment"], "A new pool");
    }

    #[test]
    fn pool_create_params_skip_none() {
        let params = PoolCreateParams::new("minimal");
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["poolid"], "minimal");
        assert!(!json.as_object().unwrap().contains_key("comment"));
    }

    #[test]
    fn pool_update_params_serialization() {
        let params = PoolUpdateParams {
            poolid: Some("mypool".to_string()),
            comment: Some("Updated comment".to_string()),
            vms: Some("100,101".to_string()),
            storage: Some("local-lvm".to_string()),
            delete: Some(false),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["comment"], "Updated comment");
        assert_eq!(json["vms"], "100,101");
        assert_eq!(json["storage"], "local-lvm");
        assert_eq!(json["delete"], 0);
    }

    #[test]
    fn pool_update_params_skip_none() {
        let params = PoolUpdateParams::default();
        let json = serde_json::to_value(&params).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default PoolUpdateParams should serialize to {{}}"
        );
    }

    #[test]
    fn pool_update_params_delete_members() {
        let params = PoolUpdateParams {
            vms: Some("100".to_string()),
            delete: Some(true),
            ..Default::default()
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["vms"], "100");
        assert_eq!(json["delete"], 1);
        assert!(!json.as_object().unwrap().contains_key("comment"));
        assert!(!json.as_object().unwrap().contains_key("storage"));
    }
}
