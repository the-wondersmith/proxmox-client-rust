use serde::Serialize;

use crate::client::ProxmoxClient;
use crate::error::Result;

/// Parameters for bulk-starting VMs/containers.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct BulkStartParams {
    /// Target node (required).
    pub node: String,

    /// Comma-separated list of VMIDs to start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vms: Option<String>,
}

impl BulkStartParams {
    /// Creates new parameters with required fields.
    pub fn new(node: impl Into<String>) -> Self {
        Self {
            node: node.into(),
            ..Default::default()
        }
    }
}

/// Parameters for bulk-shutting-down VMs/containers.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct BulkShutdownParams {
    /// Target node (required).
    pub node: String,

    /// Comma-separated list of VMIDs to shut down.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vms: Option<String>,

    /// Force stop (1 = force).
    #[serde(rename = "force-stop", skip_serializing_if = "Option::is_none")]
    pub force_stop: Option<i32>,

    /// Timeout in seconds for graceful shutdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

impl BulkShutdownParams {
    /// Creates new parameters with required fields.
    pub fn new(node: impl Into<String>) -> Self {
        Self {
            node: node.into(),
            ..Default::default()
        }
    }
}

/// Parameters for bulk-suspending VMs/containers.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct BulkSuspendParams {
    /// Target node (required).
    pub node: String,

    /// Comma-separated list of VMIDs to suspend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vms: Option<String>,
}

impl BulkSuspendParams {
    /// Creates new parameters with required fields.
    pub fn new(node: impl Into<String>) -> Self {
        Self {
            node: node.into(),
            ..Default::default()
        }
    }
}

/// Parameters for bulk-migrating VMs/containers.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct BulkMigrateParams {
    /// Source node (required).
    pub node: String,

    /// Target node (required).
    pub target: String,

    /// Comma-separated list of VMIDs to migrate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vms: Option<String>,

    /// Use online migration.
    #[serde(rename = "with-local-disks", skip_serializing_if = "Option::is_none")]
    pub with_local_disks: Option<i32>,
}

impl BulkMigrateParams {
    /// Creates new parameters with required fields.
    pub fn new(node: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            node: node.into(),
            target: target.into(),
            ..Default::default()
        }
    }
}

impl ProxmoxClient {
    /// Bulk-starts VMs/containers across the cluster.
    ///
    /// `POST /cluster/bulk-action/guest/start`
    pub async fn bulk_start_vms(&self, params: &BulkStartParams) -> Result<String> {
        self.post_parsed("/cluster/bulk-action/guest/start", params, "bulk start")
            .await
    }

    /// Bulk-shuts-down VMs/containers across the cluster.
    ///
    /// `POST /cluster/bulk-action/guest/shutdown`
    pub async fn bulk_shutdown_vms(&self, params: &BulkShutdownParams) -> Result<String> {
        self.post_parsed(
            "/cluster/bulk-action/guest/shutdown",
            params,
            "bulk shutdown",
        )
        .await
    }

    /// Bulk-suspends VMs/containers across the cluster.
    ///
    /// `POST /cluster/bulk-action/guest/suspend`
    pub async fn bulk_suspend_vms(&self, params: &BulkSuspendParams) -> Result<String> {
        self.post_parsed("/cluster/bulk-action/guest/suspend", params, "bulk suspend")
            .await
    }

    /// Bulk-migrates VMs/containers to another node.
    ///
    /// `POST /cluster/bulk-action/guest/migrate`
    pub async fn bulk_migrate_vms(&self, params: &BulkMigrateParams) -> Result<String> {
        self.post_parsed("/cluster/bulk-action/guest/migrate", params, "bulk migrate")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bulk_start_params_serialization() {
        let mut params = BulkStartParams::new("pve1");
        params.vms = Some("100,101,102".to_string());
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["node"], "pve1");
        assert_eq!(json["vms"], "100,101,102");
    }

    #[test]
    fn bulk_start_params_skip_none() {
        let params = BulkStartParams::new("pve1");
        let json = serde_json::to_value(&params).unwrap();
        let obj = json.as_object().unwrap();
        assert_eq!(
            obj.len(),
            1,
            "BulkStartParams with only node should have 1 field"
        );
        assert_eq!(json["node"], "pve1");
    }

    #[test]
    fn bulk_shutdown_params_serialization() {
        let mut params = BulkShutdownParams::new("pve1");
        params.vms = Some("100,101".to_string());
        params.force_stop = Some(1);
        params.timeout = Some(60);
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["node"], "pve1");
        assert_eq!(json["vms"], "100,101");
        assert_eq!(json["force-stop"], 1);
        assert_eq!(json["timeout"], 60);
    }

    #[test]
    fn bulk_suspend_params_serialization() {
        let mut params = BulkSuspendParams::new("pve1");
        params.vms = Some("100,101".to_string());
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["node"], "pve1");
        assert_eq!(json["vms"], "100,101");
    }

    #[test]
    fn bulk_migrate_params_serialization() {
        let mut params = BulkMigrateParams::new("pve1", "pve2");
        params.vms = Some("100,101".to_string());
        params.with_local_disks = Some(1);
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["node"], "pve1");
        assert_eq!(json["vms"], "100,101");
        assert_eq!(json["target"], "pve2");
        assert_eq!(json["with-local-disks"], 1);
    }
}
