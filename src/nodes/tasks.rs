use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::types::TaskStatus;
use crate::validation::validate_node_name;

use urlencoding::encode as urlencode;

/// A task list entry as returned by `GET /nodes/{node}/tasks`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TaskListItem {
    /// UPID string identifying the task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upid: Option<String>,

    /// Node this task is running on.
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

    /// End time (epoch), if finished.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endtime: Option<u64>,

    /// Task type (e.g., `qmstart`, `vzdump`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,

    /// Task ID (e.g., VMID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// User who started the task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// Task status string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// A single log line from a task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TaskLogLine {
    /// Line number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u64>,

    /// Log text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t: Option<String>,
}

/// Parameters for listing tasks.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct TaskListParams {
    /// Filter by VMID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmid: Option<u32>,

    /// Filter by task type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typefilter: Option<String>,

    /// Filter by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statusfilter: Option<String>,

    /// Start index (for paging).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,

    /// Max number of entries to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Task source filter (`archive` or `active`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Only list tasks since this epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<u64>,

    /// Only list tasks until this epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<u64>,

    /// Filter by user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userfilter: Option<String>,
}

impl ProxmoxClient {
    /// Lists tasks on a node with optional filters.
    pub async fn list_tasks(
        &self,
        node: &str,
        params: Option<&TaskListParams>,
    ) -> Result<Vec<TaskListItem>> {
        validate_node_name(node)?;
        let mut path = format!("/nodes/{node}/tasks");
        if let Some(p) = params {
            let query = serde_json::to_value(p)
                .expect("TaskListParams serialization is infallible")
                .as_object()
                .cloned()
                .unwrap_or_default();
            let pairs: Vec<String> = query
                .iter()
                .map(|(k, v)| {
                    let val = match v {
                        serde_json::Value::String(s) => s.clone(),
                        other => other.to_string(),
                    };
                    format!("{}={}", urlencode(k), urlencode(&val))
                })
                .collect();
            if !pairs.is_empty() {
                path.push('?');
                path.push_str(&pairs.join("&"));
            }
        }
        self.get_parsed(&path, &format!("node {node} tasks")).await
    }

    /// Returns the status of a specific task.
    pub async fn get_task_status(&self, node: &str, upid: &str) -> Result<TaskStatus> {
        validate_node_name(node)?;
        let encoded_upid = urlencode(upid);
        self.get_parsed(
            &format!("/nodes/{node}/tasks/{encoded_upid}/status"),
            &format!("task {upid}"),
        )
        .await
    }

    /// Returns the log output of a task.
    pub async fn get_task_log(
        &self,
        node: &str,
        upid: &str,
        start: Option<u64>,
        limit: Option<u64>,
    ) -> Result<Vec<TaskLogLine>> {
        validate_node_name(node)?;
        let encoded_upid = urlencode(upid);
        let mut path = format!("/nodes/{node}/tasks/{encoded_upid}/log");
        let mut query_parts = Vec::new();
        if let Some(s) = start {
            query_parts.push(format!("start={s}"));
        }
        if let Some(l) = limit {
            query_parts.push(format!("limit={l}"));
        }
        if !query_parts.is_empty() {
            path.push('?');
            path.push_str(&query_parts.join("&"));
        }
        self.get_parsed(&path, &format!("task {upid} log")).await
    }

    /// Stops a running task.
    pub async fn stop_task(&self, node: &str, upid: &str) -> Result<()> {
        validate_node_name(node)?;
        let encoded_upid = urlencode(upid);
        self.delete_void(
            &format!("/nodes/{node}/tasks/{encoded_upid}"),
            &format!("task {upid}"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_list_item_serde_roundtrip() {
        let json = r#"{
            "upid": "UPID:pve1:00001234:00000001:00000000:qmstart:100:root@pam:",
            "node": "pve1",
            "pid": 4660,
            "pstart": 1,
            "starttime": 1700000000,
            "endtime": 1700001000,
            "type": "qmstart",
            "id": "100",
            "user": "root@pam",
            "status": "OK"
        }"#;
        let item: TaskListItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.node.as_deref(), Some("pve1"));
        assert_eq!(item.task_type.as_deref(), Some("qmstart"));
        assert_eq!(item.endtime, Some(1700001000));

        let serialized = serde_json::to_string(&item).unwrap();
        let deserialized: TaskListItem = serde_json::from_str(&serialized).unwrap();
        assert_eq!(item, deserialized);
    }

    #[test]
    fn task_log_line_serde_roundtrip() {
        let json = r#"{"n": 1, "t": "starting VM 100"}"#;
        let line: TaskLogLine = serde_json::from_str(json).unwrap();
        assert_eq!(line.n, Some(1));
        assert_eq!(line.t.as_deref(), Some("starting VM 100"));

        let serialized = serde_json::to_string(&line).unwrap();
        let deserialized: TaskLogLine = serde_json::from_str(&serialized).unwrap();
        assert_eq!(line, deserialized);
    }

    #[test]
    fn urlencoding_encode_upid() {
        let upid = "UPID:pve1:00001234:00000001:00000000:qmstart:100:root@pam:";
        let encoded = urlencode(upid);
        assert!(!encoded.contains(':'));
        assert!(encoded.contains("%3A"));
    }
}
