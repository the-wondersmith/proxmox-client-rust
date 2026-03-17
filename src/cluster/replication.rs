use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_resource_id;

/// A replication job definition.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReplicationJob {
    /// Replication job ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Replication type (e.g., `local`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub replication_type: Option<String>,

    /// Source VM/CT (VMID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest: Option<i64>,

    /// Target node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Schedule (cron-like).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Whether the job is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,

    /// Rate limit in MB/s.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,

    /// Remove the job on node removal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_job: Option<String>,

    /// Source node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// Parameters for creating a replication job.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct ReplicationJobCreateParams {
    /// Replication job ID (required).
    pub id: String,

    /// Target node (required).
    pub target: String,

    /// Replication type (required, e.g., `local`).
    #[serde(rename = "type")]
    pub replication_type: String,

    /// Schedule (cron-like).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Whether the job is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,

    /// Rate limit in MB/s.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,

    /// Remove the job on node removal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_job: Option<String>,
}

impl ReplicationJobCreateParams {
    /// Creates a new `ReplicationJobCreateParams` with the required fields.
    pub fn new(
        id: impl Into<String>,
        target: impl Into<String>,
        replication_type: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            target: target.into(),
            replication_type: replication_type.into(),
            ..Default::default()
        }
    }
}

impl ProxmoxClient {
    /// Lists all replication jobs.
    ///
    /// `GET /cluster/replication`
    pub async fn list_replication_jobs(&self) -> Result<Vec<ReplicationJob>> {
        self.get_parsed("/cluster/replication", "replication jobs")
            .await
    }

    /// Creates a new replication job.
    ///
    /// `POST /cluster/replication`
    pub async fn create_replication_job(&self, params: &ReplicationJobCreateParams) -> Result<()> {
        let response = self
            .post("/cluster/replication")?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, "replication job creation").await?;
        Ok(())
    }

    /// Gets a specific replication job.
    ///
    /// `GET /cluster/replication/{id}`
    pub async fn get_replication_job(&self, id: &str) -> Result<ReplicationJob> {
        validate_resource_id(id)?;
        self.get_parsed(
            &format!("/cluster/replication/{id}"),
            &format!("replication job {id}"),
        )
        .await
    }

    /// Updates an existing replication job.
    ///
    /// `PUT /cluster/replication/{id}`
    pub async fn update_replication_job(&self, id: &str, params: &serde_json::Value) -> Result<()> {
        validate_resource_id(id)?;
        let response = self
            .put(&format!("/cluster/replication/{id}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("replication job {id}")).await?;
        Ok(())
    }

    /// Deletes a replication job.
    ///
    /// `DELETE /cluster/replication/{id}`
    pub async fn delete_replication_job(&self, id: &str) -> Result<()> {
        validate_resource_id(id)?;
        self.delete_void(
            &format!("/cluster/replication/{id}"),
            &format!("replication job {id}"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replication_job_serde_roundtrip() {
        let job = ReplicationJob {
            id: Some("100-0".to_string()),
            replication_type: Some("local".to_string()),
            guest: Some(100),
            target: Some("pve2".to_string()),
            schedule: Some("*/15".to_string()),
            comment: Some("Replicate VM 100".to_string()),
            disable: Some(false),
            rate: Some(10.0),
            remove_job: None,
            source: Some("pve1".to_string()),
        };

        let json = serde_json::to_string(&job).unwrap();
        let deserialized: ReplicationJob = serde_json::from_str(&json).unwrap();
        assert_eq!(job, deserialized);
    }

    #[test]
    fn replication_job_skip_serializing_none() {
        let job = ReplicationJob::default();
        let json = serde_json::to_value(&job).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default ReplicationJob should serialize to {{}}"
        );
    }

    #[test]
    fn replication_job_type_rename() {
        let json = r#"{"type": "local", "id": "100-0"}"#;
        let job: ReplicationJob = serde_json::from_str(json).unwrap();
        assert_eq!(job.replication_type.as_deref(), Some("local"));

        let serialized = serde_json::to_value(&job).unwrap();
        assert!(serialized.get("type").is_some());
        assert!(serialized.get("replication_type").is_none());
    }

    #[test]
    fn replication_job_create_params_serialization() {
        let mut params = ReplicationJobCreateParams::new("100-0", "pve2", "local");
        params.schedule = Some("*/15".to_string());
        params.comment = Some("Replicate VM 100".to_string());
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["id"], "100-0");
        assert_eq!(json["target"], "pve2");
        assert_eq!(json["type"], "local");
        assert_eq!(json["schedule"], "*/15");
        assert!(!json.as_object().unwrap().contains_key("disable"));
    }

    #[test]
    fn replication_job_unknown_fields_ignored() {
        let json = r#"{"id": "100-0", "unknownField": true}"#;
        let job: ReplicationJob = serde_json::from_str(json).unwrap();
        assert_eq!(job.id.as_deref(), Some("100-0"));
    }
}
