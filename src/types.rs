use std::fmt;

use serde::{Deserialize, Serialize};

/// A Proxmox Unique Process ID (UPID) identifying an async task.
///
/// Format: `UPID:<node>:<pid>:<pstart>:<starttime>:<type>:<id>:<user>`
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Upid(String);

impl Upid {
    /// Returns the UPID as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Upid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Upid").field(&self.0).finish()
    }
}

impl fmt::Display for Upid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for Upid {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<Upid> for String {
    fn from(u: Upid) -> Self {
        u.0
    }
}

impl AsRef<str> for Upid {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Status of an async task.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TaskStatus {
    /// The UPID of the task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upid: Option<String>,

    /// The node this task is running on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,

    /// Process ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<u32>,

    /// Process start time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pstart: Option<u64>,

    /// Task start time (epoch).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starttime: Option<u64>,

    /// Task type (e.g., `qmstart`, `vzdump`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,

    /// Task ID (e.g., VMID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// User who started the task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// Task status string (e.g., `running`, `OK`, `ERROR: ...`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Exit status string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exitstatus: Option<String>,
}

impl TaskStatus {
    /// Returns `true` if the task is still running.
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.status.as_deref() == Some("running")
    }

    /// Returns `true` if the task completed successfully.
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.exitstatus.as_deref() == Some("OK")
    }
}

/// Proxmox API response envelope.
///
/// All Proxmox API responses are wrapped in `{"data": ...}`.
#[derive(Debug, Deserialize)]
pub(crate) struct ProxmoxResponse<T> {
    /// The actual response data.
    pub data: T,

    /// Optional error messages from the server.
    #[serde(default)]
    pub errors: Option<std::collections::HashMap<String, String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upid_display() {
        let upid =
            Upid::from("UPID:pve1:00001234:00000001:00000000:qmstart:100:root@pam:".to_string());
        assert_eq!(upid.to_string(), upid.as_str());
    }

    #[test]
    fn upid_from_string() {
        let s = "UPID:pve1:00001234".to_string();
        let upid = Upid::from(s.clone());
        assert_eq!(upid.as_str(), s);
    }

    #[test]
    fn upid_into_string() {
        let upid = Upid::from("test-upid".to_string());
        let s: String = upid.into();
        assert_eq!(s, "test-upid");
    }

    #[test]
    fn task_status_is_running() {
        let status = TaskStatus {
            status: Some("running".to_string()),
            exitstatus: None,
            upid: None,
            node: None,
            pid: None,
            pstart: None,
            starttime: None,
            task_type: None,
            id: None,
            user: None,
        };
        assert!(status.is_running());
        assert!(!status.is_ok());
    }

    #[test]
    fn task_status_is_ok() {
        let status = TaskStatus {
            status: Some("stopped".to_string()),
            exitstatus: Some("OK".to_string()),
            upid: None,
            node: None,
            pid: None,
            pstart: None,
            starttime: None,
            task_type: None,
            id: None,
            user: None,
        };
        assert!(!status.is_running());
        assert!(status.is_ok());
    }

    #[test]
    fn proxmox_response_deserialize() {
        let json = r#"{"data": {"status": "running"}}"#;
        let resp: ProxmoxResponse<TaskStatus> = serde_json::from_str(json).unwrap();
        assert!(resp.data.is_running());
        assert!(resp.errors.is_none());
    }

    #[test]
    fn proxmox_response_with_errors() {
        let json = r#"{"data": null, "errors": {"field": "invalid value"}}"#;
        let resp: ProxmoxResponse<Option<String>> = serde_json::from_str(json).unwrap();
        assert!(resp.data.is_none());
        assert!(resp.errors.unwrap().contains_key("field"));
    }

    #[test]
    fn task_status_none_is_not_running() {
        let status = TaskStatus {
            status: None,
            exitstatus: None,
            upid: None,
            node: None,
            pid: None,
            pstart: None,
            starttime: None,
            task_type: None,
            id: None,
            user: None,
        };
        assert!(!status.is_running());
    }

    #[test]
    fn task_status_none_exitstatus_is_not_ok() {
        let status = TaskStatus {
            status: Some("stopped".to_string()),
            exitstatus: None,
            upid: None,
            node: None,
            pid: None,
            pstart: None,
            starttime: None,
            task_type: None,
            id: None,
            user: None,
        };
        assert!(!status.is_ok());
    }

    #[test]
    fn task_status_error_exitstatus() {
        let status = TaskStatus {
            status: Some("stopped".to_string()),
            exitstatus: Some("command failed: exit code 1".to_string()),
            upid: None,
            node: None,
            pid: None,
            pstart: None,
            starttime: None,
            task_type: None,
            id: None,
            user: None,
        };
        assert!(!status.is_running());
        assert!(!status.is_ok());
    }

    #[test]
    fn upid_equality() {
        let a =
            Upid::from("UPID:pve1:00001234:00000001:00000000:qmstart:100:root@pam:".to_string());
        let b =
            Upid::from("UPID:pve1:00001234:00000001:00000000:qmstart:100:root@pam:".to_string());
        assert_eq!(a, b);
    }

    #[test]
    fn upid_inequality() {
        let a = Upid::from("UPID:pve1:00001234".to_string());
        let b = Upid::from("UPID:pve2:00005678".to_string());
        assert_ne!(a, b);
    }

    #[test]
    fn upid_hash_consistent_with_eq() {
        use std::collections::HashSet;
        let a = Upid::from("UPID:pve1:00001234".to_string());
        let b = Upid::from("UPID:pve1:00001234".to_string());
        let mut set = HashSet::new();
        set.insert(a);
        set.insert(b);
        assert_eq!(set.len(), 1, "equal UPIDs must hash to the same bucket");
    }

    #[test]
    fn task_status_serde_roundtrip() {
        let json = r#"{
            "upid": "UPID:pve1:00001234:00000001:00000000:qmstart:100:root@pam:",
            "node": "pve1",
            "pid": 4660,
            "pstart": 1,
            "starttime": 1700000000,
            "type": "qmstart",
            "id": "100",
            "user": "root@pam",
            "status": "running"
        }"#;
        let status: TaskStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status.node.as_deref(), Some("pve1"));
        assert_eq!(status.task_type.as_deref(), Some("qmstart"));
        assert!(status.is_running());
    }
}
