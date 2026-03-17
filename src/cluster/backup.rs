use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_resource_id;

/// A backup job definition.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BackupJob {
    /// Backup job ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Backup type (`snapshot`, `suspend`, `stop`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<String>,

    /// Schedule (cron-like).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,

    /// Whether the job is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub enabled: Option<bool>,

    /// Storage for the backup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,

    /// Compression type (e.g., `zstd`, `lzo`, `gzip`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<String>,

    /// Backup mode (`include`, `exclude`, `all`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// Comma-separated list of VMIDs to back up.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmid: Option<String>,

    /// Comma-separated list of VMIDs to exclude.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<String>,

    /// Node to run the backup on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,

    /// Email address for notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailto: Option<String>,

    /// Email notification type (`always`, `failure`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailnotification: Option<String>,

    /// Maximum number of backups to keep.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxfiles: Option<i64>,

    /// Prune backups configuration.
    #[serde(rename = "prune-backups", skip_serializing_if = "Option::is_none")]
    pub prune_backups: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Pool restriction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,

    /// Whether to repeat missed backups.
    #[serde(default, rename = "repeat-missed", skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub repeat_missed: Option<bool>,

    /// Notification mode.
    #[serde(rename = "notification-mode", skip_serializing_if = "Option::is_none")]
    pub notification_mode: Option<String>,

    /// Notification target.
    #[serde(
        rename = "notification-target",
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_target: Option<String>,

    /// Whether backups are protected from pruning.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub protected: Option<bool>,

    /// Performance options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance: Option<String>,

    /// Notes template.
    #[serde(rename = "notes-template", skip_serializing_if = "Option::is_none")]
    pub notes_template: Option<String>,

    /// Fleecing configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleecing: Option<String>,
}

/// Parameters for creating a backup job.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct BackupJobCreateParams {
    /// Schedule (cron-like, required).
    pub schedule: String,

    /// Backup type (`snapshot`, `suspend`, `stop`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<String>,

    /// Whether the job is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub enabled: Option<bool>,

    /// Storage for the backup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,

    /// Compression type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<String>,

    /// Backup mode (`include`, `exclude`, `all`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// Comma-separated list of VMIDs to back up.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmid: Option<String>,

    /// Comma-separated list of VMIDs to exclude.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<String>,

    /// Node to run the backup on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,

    /// Email address for notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailto: Option<String>,

    /// Email notification type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailnotification: Option<String>,

    /// Maximum number of backups to keep.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxfiles: Option<i64>,

    /// Prune backups configuration.
    #[serde(rename = "prune-backups", skip_serializing_if = "Option::is_none")]
    pub prune_backups: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Pool restriction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,

    /// Whether to repeat missed backups.
    #[serde(default, rename = "repeat-missed", skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub repeat_missed: Option<bool>,

    /// Notification mode.
    #[serde(rename = "notification-mode", skip_serializing_if = "Option::is_none")]
    pub notification_mode: Option<String>,

    /// Notification target.
    #[serde(
        rename = "notification-target",
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_target: Option<String>,
}

impl BackupJobCreateParams {
    /// Creates a new `BackupJobCreateParams` with the required fields.
    pub fn new(schedule: impl Into<String>) -> Self {
        Self {
            schedule: schedule.into(),
            ..Default::default()
        }
    }
}

/// Information about included volumes in a backup job.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BackupIncludedVolume {
    /// VMID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmid: Option<i64>,

    /// Guest name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Guest type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub guest_type: Option<String>,

    /// Included volumes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<serde_json::Value>,
}

/// A guest that is not backed up.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NotBackedUpGuest {
    /// VMID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmid: Option<i64>,

    /// Guest name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Guest type (`qemu` or `lxc`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub guest_type: Option<String>,
}

impl ProxmoxClient {
    /// Lists all backup jobs.
    ///
    /// `GET /cluster/backup`
    pub async fn list_backup_jobs(&self) -> Result<Vec<BackupJob>> {
        self.get_parsed("/cluster/backup", "backup jobs").await
    }

    /// Creates a new backup job.
    ///
    /// `POST /cluster/backup`
    pub async fn create_backup_job(&self, params: &BackupJobCreateParams) -> Result<()> {
        let response = self.post("/cluster/backup")?.json(params).send().await?;
        Self::handle_error(response, "backup job creation").await?;
        Ok(())
    }

    /// Gets a specific backup job.
    ///
    /// `GET /cluster/backup/{id}`
    pub async fn get_backup_job(&self, id: &str) -> Result<BackupJob> {
        validate_resource_id(id)?;
        self.get_parsed(
            &format!("/cluster/backup/{id}"),
            &format!("backup job {id}"),
        )
        .await
    }

    /// Updates a backup job.
    ///
    /// `PUT /cluster/backup/{id}`
    pub async fn update_backup_job(&self, id: &str, params: &serde_json::Value) -> Result<()> {
        validate_resource_id(id)?;
        let response = self
            .put(&format!("/cluster/backup/{id}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("backup job {id}")).await?;
        Ok(())
    }

    /// Deletes a backup job.
    ///
    /// `DELETE /cluster/backup/{id}`
    pub async fn delete_backup_job(&self, id: &str) -> Result<()> {
        validate_resource_id(id)?;
        self.delete_void(
            &format!("/cluster/backup/{id}"),
            &format!("backup job {id}"),
        )
        .await
    }

    /// Gets the included volumes for a backup job.
    ///
    /// `GET /cluster/backup/{id}/included_volumes`
    pub async fn get_backup_included_volumes(&self, id: &str) -> Result<serde_json::Value> {
        validate_resource_id(id)?;
        self.get_parsed(
            &format!("/cluster/backup/{id}/included_volumes"),
            &format!("backup job {id} included volumes"),
        )
        .await
    }

    /// Lists guests that are not covered by any backup job.
    ///
    /// `GET /cluster/backup-info/not-backed-up`
    pub async fn list_not_backed_up_guests(&self) -> Result<Vec<NotBackedUpGuest>> {
        self.get_parsed("/cluster/backup-info/not-backed-up", "not-backed-up guests")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backup_job_serde_roundtrip() {
        let job = BackupJob {
            id: Some("backup-abc123".to_string()),
            backup_type: Some("snapshot".to_string()),
            schedule: Some("0 2 * * *".to_string()),
            enabled: Some(true),
            storage: Some("local".to_string()),
            compress: Some("zstd".to_string()),
            mode: Some("include".to_string()),
            vmid: Some("100,101,102".to_string()),
            exclude: None,
            node: None,
            mailto: Some("admin@example.com".to_string()),
            mailnotification: Some("always".to_string()),
            maxfiles: None,
            prune_backups: Some("keep-daily=7,keep-weekly=4".to_string()),
            comment: Some("Daily backup".to_string()),
            pool: None,
            repeat_missed: None,
            notification_mode: None,
            notification_target: None,
            protected: None,
            performance: None,
            notes_template: None,
            fleecing: None,
        };

        let json = serde_json::to_string(&job).unwrap();
        let deserialized: BackupJob = serde_json::from_str(&json).unwrap();
        assert_eq!(job, deserialized);
    }

    #[test]
    fn backup_job_skip_serializing_none() {
        let job = BackupJob::default();
        let json = serde_json::to_value(&job).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty(), "default BackupJob should serialize to {{}}");
    }

    #[test]
    fn backup_job_type_rename() {
        let json = r#"{"type": "snapshot", "id": "backup-1"}"#;
        let job: BackupJob = serde_json::from_str(json).unwrap();
        assert_eq!(job.backup_type.as_deref(), Some("snapshot"));

        let serialized = serde_json::to_value(&job).unwrap();
        assert!(serialized.get("type").is_some());
        assert!(serialized.get("backup_type").is_none());
    }

    #[test]
    fn backup_job_hyphenated_fields() {
        let json = r#"{"id": "test", "prune-backups": "keep-daily=7", "repeat-missed": 1, "notification-mode": "auto"}"#;
        let job: BackupJob = serde_json::from_str(json).unwrap();
        assert_eq!(job.prune_backups.as_deref(), Some("keep-daily=7"));
        assert_eq!(job.repeat_missed, Some(true));
        assert_eq!(job.notification_mode.as_deref(), Some("auto"));
    }

    #[test]
    fn backup_job_create_params_serialization() {
        let mut params = BackupJobCreateParams::new("0 2 * * *");
        params.backup_type = Some("snapshot".to_string());
        params.enabled = Some(true);
        params.storage = Some("local".to_string());
        params.compress = Some("zstd".to_string());
        params.mode = Some("all".to_string());
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["schedule"], "0 2 * * *");
        assert_eq!(json["type"], "snapshot");
        assert_eq!(json["storage"], "local");
        assert!(!json.as_object().unwrap().contains_key("vmid"));
    }

    #[test]
    fn not_backed_up_guest_serde_roundtrip() {
        let guest = NotBackedUpGuest {
            vmid: Some(200),
            name: Some("web-server".to_string()),
            guest_type: Some("qemu".to_string()),
        };

        let json = serde_json::to_string(&guest).unwrap();
        let deserialized: NotBackedUpGuest = serde_json::from_str(&json).unwrap();
        assert_eq!(guest, deserialized);
    }

    #[test]
    fn not_backed_up_guest_type_rename() {
        let json = r#"{"type": "lxc", "vmid": 200}"#;
        let guest: NotBackedUpGuest = serde_json::from_str(json).unwrap();
        assert_eq!(guest.guest_type.as_deref(), Some("lxc"));
    }

    #[test]
    fn backup_included_volume_serde_roundtrip() {
        let volume = BackupIncludedVolume {
            vmid: Some(100),
            name: Some("test-vm".to_string()),
            guest_type: Some("qemu".to_string()),
            volumes: None,
        };

        let json = serde_json::to_string(&volume).unwrap();
        let deserialized: BackupIncludedVolume = serde_json::from_str(&json).unwrap();
        assert_eq!(volume, deserialized);
    }

    #[test]
    fn backup_job_unknown_fields_ignored() {
        let json = r#"{"id": "test", "unknownField": true}"#;
        let job: BackupJob = serde_json::from_str(json).unwrap();
        assert_eq!(job.id.as_deref(), Some("test"));
    }
}
