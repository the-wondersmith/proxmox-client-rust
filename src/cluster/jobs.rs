use serde::{Deserialize, Serialize};

use urlencoding::encode;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_resource_id;

/// A realm sync job.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RealmSyncJob {
    /// Job ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Whether the job is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub enabled: Option<bool>,

    /// Schedule expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,

    /// Realm to sync.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realm: Option<String>,

    /// Sync scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// How to handle vanished entries.
    #[serde(rename = "remove-vanished", skip_serializing_if = "Option::is_none")]
    pub remove_vanished: Option<String>,

    /// Whether to enable new users/groups.
    #[serde(default, rename = "enable-new", skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub enable_new: Option<bool>,

    /// Last run time.
    #[serde(rename = "last-run", skip_serializing_if = "Option::is_none")]
    pub last_run: Option<i64>,

    /// Next run time.
    #[serde(rename = "next-run", skip_serializing_if = "Option::is_none")]
    pub next_run: Option<i64>,
}

/// Parameters for creating a realm sync job.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct RealmSyncJobCreateParams {
    /// Schedule expression (required).
    pub schedule: String,

    /// Realm to sync (required).
    pub realm: String,

    /// Whether the job is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub enabled: Option<bool>,

    /// Sync scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// How to handle vanished entries.
    #[serde(rename = "remove-vanished", skip_serializing_if = "Option::is_none")]
    pub remove_vanished: Option<String>,

    /// Whether to enable new users/groups.
    #[serde(default, rename = "enable-new", skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub enable_new: Option<bool>,
}

impl RealmSyncJobCreateParams {
    /// Creates a new `RealmSyncJobCreateParams` with the required fields.
    pub fn new(schedule: impl Into<String>, realm: impl Into<String>) -> Self {
        Self {
            schedule: schedule.into(),
            realm: realm.into(),
            ..Default::default()
        }
    }
}

/// Result of a schedule analysis.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ScheduleAnalysis {
    /// Timestamp of the scheduled event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,

    /// UTC time string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utc: Option<String>,
}

impl ProxmoxClient {
    // --- Realm Sync Jobs ---

    /// Lists realm sync jobs.
    ///
    /// `GET /cluster/jobs/realm-sync`
    pub async fn list_realm_sync_jobs(&self) -> Result<Vec<RealmSyncJob>> {
        self.get_parsed("/cluster/jobs/realm-sync", "realm sync jobs")
            .await
    }

    /// Creates a realm sync job.
    ///
    /// `POST /cluster/jobs/realm-sync/{id}`
    pub async fn create_realm_sync_job(
        &self,
        id: &str,
        params: &RealmSyncJobCreateParams,
    ) -> Result<()> {
        validate_resource_id(id)?;
        let response = self
            .post(&format!("/cluster/jobs/realm-sync/{id}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("realm sync job {id} creation")).await?;
        Ok(())
    }

    /// Gets a specific realm sync job.
    ///
    /// `GET /cluster/jobs/realm-sync/{id}`
    pub async fn get_realm_sync_job(&self, id: &str) -> Result<RealmSyncJob> {
        validate_resource_id(id)?;
        self.get_parsed(
            &format!("/cluster/jobs/realm-sync/{id}"),
            &format!("realm sync job {id}"),
        )
        .await
    }

    /// Updates a realm sync job.
    ///
    /// `PUT /cluster/jobs/realm-sync/{id}`
    pub async fn update_realm_sync_job(&self, id: &str, params: &serde_json::Value) -> Result<()> {
        validate_resource_id(id)?;
        let response = self
            .put(&format!("/cluster/jobs/realm-sync/{id}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("realm sync job {id}")).await?;
        Ok(())
    }

    /// Deletes a realm sync job.
    ///
    /// `DELETE /cluster/jobs/realm-sync/{id}`
    pub async fn delete_realm_sync_job(&self, id: &str) -> Result<()> {
        validate_resource_id(id)?;
        self.delete_void(
            &format!("/cluster/jobs/realm-sync/{id}"),
            &format!("realm sync job {id}"),
        )
        .await
    }

    // --- Schedule Analysis ---

    /// Analyzes a schedule expression and returns future event times.
    ///
    /// `GET /cluster/jobs/schedule-analyze`
    pub async fn analyze_schedule(&self, schedule: &str) -> Result<Vec<ScheduleAnalysis>> {
        self.get_parsed(
            &format!(
                "/cluster/jobs/schedule-analyze?schedule={}",
                encode(schedule)
            ),
            "schedule analysis",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schedule_analysis_serde_roundtrip() {
        let analysis = ScheduleAnalysis {
            timestamp: Some(1700000000),
            utc: Some("2023-11-14T22:13:20".to_string()),
        };

        let json = serde_json::to_string(&analysis).unwrap();
        let deserialized: ScheduleAnalysis = serde_json::from_str(&json).unwrap();
        assert_eq!(analysis, deserialized);
    }

    #[test]
    fn schedule_analysis_skip_serializing_none() {
        let analysis = ScheduleAnalysis::default();
        let json = serde_json::to_value(&analysis).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default ScheduleAnalysis should serialize to {{}}"
        );
    }

    #[test]
    fn schedule_analysis_unknown_fields_ignored() {
        let json = r#"{"timestamp": 1700000000, "unknownField": true}"#;
        let analysis: ScheduleAnalysis = serde_json::from_str(json).unwrap();
        assert_eq!(analysis.timestamp, Some(1700000000));
    }

    #[test]
    fn realm_sync_job_serde_roundtrip() {
        let job = RealmSyncJob {
            id: Some("test-sync".to_string()),
            enabled: Some(true),
            schedule: Some("daily".to_string()),
            realm: Some("ldap".to_string()),
            scope: Some("users".to_string()),
            comment: Some("nightly sync".to_string()),
            remove_vanished: Some("entry;properties".to_string()),
            enable_new: Some(true),
            last_run: Some(1700000000),
            next_run: Some(1700086400),
        };

        let json = serde_json::to_string(&job).unwrap();
        let deserialized: RealmSyncJob = serde_json::from_str(&json).unwrap();
        assert_eq!(job, deserialized);
    }

    #[test]
    fn realm_sync_job_skip_serializing_none() {
        let job = RealmSyncJob::default();
        let json = serde_json::to_value(&job).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default RealmSyncJob should serialize to {{}}"
        );
    }

    #[test]
    fn realm_sync_job_hyphenated_fields() {
        let json = r#"{
            "id": "sync1",
            "remove-vanished": "entry;properties",
            "enable-new": 1,
            "last-run": 1700000000,
            "next-run": 1700086400
        }"#;
        let job: RealmSyncJob = serde_json::from_str(json).unwrap();
        assert_eq!(job.remove_vanished.as_deref(), Some("entry;properties"));
        assert_eq!(job.enable_new, Some(true));
        assert_eq!(job.last_run, Some(1700000000));
        assert_eq!(job.next_run, Some(1700086400));

        let serialized = serde_json::to_value(&job).unwrap();
        assert_eq!(serialized["remove-vanished"], "entry;properties");
        assert_eq!(serialized["enable-new"], 1);
        assert_eq!(serialized["last-run"], 1700000000);
        assert_eq!(serialized["next-run"], 1700086400);
    }

    #[test]
    fn realm_sync_job_create_params_serialization() {
        let params = RealmSyncJobCreateParams {
            schedule: "daily".to_string(),
            realm: "ldap".to_string(),
            enabled: Some(true),
            scope: Some("users".to_string()),
            comment: None,
            remove_vanished: Some("entry".to_string()),
            enable_new: Some(false),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["schedule"], "daily");
        assert_eq!(json["realm"], "ldap");
        assert_eq!(json["remove-vanished"], "entry");
        assert_eq!(json["enable-new"], 0);
    }

    #[test]
    fn realm_sync_job_create_params_skip_none() {
        let params = RealmSyncJobCreateParams::new("hourly", "ad");
        let json = serde_json::to_value(&params).unwrap();
        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2);
        assert_eq!(json["schedule"], "hourly");
        assert_eq!(json["realm"], "ad");
    }
}
