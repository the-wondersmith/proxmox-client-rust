use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::{validate_node_name, validate_vmid};

/// Parameters for starting an LXC container.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ContainerStartParams {
    /// Skip lock check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skiplock: Option<bool>,

    /// Enable debug mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug: Option<bool>,
}

/// Parameters for stopping an LXC container.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ContainerStopParams {
    /// Skip lock check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skiplock: Option<bool>,

    /// Overrule an active shutdown task.
    #[serde(rename = "overrule-shutdown", skip_serializing_if = "Option::is_none")]
    pub overrule_shutdown: Option<bool>,
}

/// Parameters for shutting down an LXC container.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ContainerShutdownParams {
    /// Timeout in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,

    /// Force stop after timeout.
    #[serde(rename = "forceStop", skip_serializing_if = "Option::is_none")]
    pub force_stop: Option<bool>,
}

/// Current status of an LXC container.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ContainerStatus {
    /// Container ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmid: Option<u32>,

    /// Current status (`running`, `stopped`, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Container name / hostname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// CPU usage (0.0 - 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f64>,

    /// Number of CPUs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<u32>,

    /// Current memory usage in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem: Option<u64>,

    /// Maximum memory in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxmem: Option<u64>,

    /// Current swap usage in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap: Option<u64>,

    /// Maximum swap in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxswap: Option<u64>,

    /// Current disk usage in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<u64>,

    /// Maximum disk in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxdisk: Option<u64>,

    /// Uptime in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime: Option<u64>,

    /// Network in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netin: Option<u64>,

    /// Network out bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netout: Option<u64>,

    /// PID of the container init process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<u32>,

    /// HA state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ha: Option<Value>,

    /// Lock status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<String>,

    /// Tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,

    /// Container type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub ct_type: Option<String>,
}

impl ProxmoxClient {
    /// Returns the current status of an LXC container.
    pub async fn get_container_status(&self, node: &str, vmid: u32) -> Result<ContainerStatus> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/lxc/{vmid}/status/current"),
            &format!("container {vmid} status"),
        )
        .await
    }

    /// Starts an LXC container.
    pub async fn start_container(
        &self,
        node: &str,
        vmid: u32,
        params: Option<&ContainerStartParams>,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let path = format!("/nodes/{node}/lxc/{vmid}/status/start");
        self.post_with_optional_body(&path, params, &format!("start container {vmid}"))
            .await
    }

    /// Stops an LXC container immediately.
    pub async fn stop_container(
        &self,
        node: &str,
        vmid: u32,
        params: Option<&ContainerStopParams>,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let path = format!("/nodes/{node}/lxc/{vmid}/status/stop");
        self.post_with_optional_body(&path, params, &format!("stop container {vmid}"))
            .await
    }

    /// Sends a shutdown signal to an LXC container.
    pub async fn shutdown_container(
        &self,
        node: &str,
        vmid: u32,
        params: Option<&ContainerShutdownParams>,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let path = format!("/nodes/{node}/lxc/{vmid}/status/shutdown");
        self.post_with_optional_body(&path, params, &format!("shutdown container {vmid}"))
            .await
    }

    /// Reboots an LXC container.
    pub async fn reboot_container(
        &self,
        node: &str,
        vmid: u32,
        timeout: Option<u64>,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let path = format!("/nodes/{node}/lxc/{vmid}/status/reboot");
        if let Some(t) = timeout {
            let params = serde_json::json!({ "timeout": t });
            self.post_parsed(&path, &params, &format!("reboot container {vmid}"))
                .await
        } else {
            let response = self.post(&path)?.send().await?;
            Self::parse_response(response, &format!("reboot container {vmid}")).await
        }
    }

    /// Suspends an LXC container.
    pub async fn suspend_container(&self, node: &str, vmid: u32) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/lxc/{vmid}/status/suspend"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("suspend container {vmid}")).await
    }

    /// Resumes a suspended LXC container.
    pub async fn resume_container(&self, node: &str, vmid: u32) -> Result<String> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/lxc/{vmid}/status/resume"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("resume container {vmid}")).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_status_serde_roundtrip() {
        let json = r#"{
            "vmid": 200,
            "status": "running",
            "name": "test-ct",
            "cpu": 0.05,
            "cpus": 2,
            "mem": 268435456,
            "maxmem": 1073741824,
            "swap": 0,
            "maxswap": 536870912,
            "disk": 1073741824,
            "maxdisk": 8589934592,
            "uptime": 3600,
            "pid": 54321,
            "type": "lxc"
        }"#;
        let status: ContainerStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status.vmid, Some(200));
        assert_eq!(status.status.as_deref(), Some("running"));
        assert_eq!(status.ct_type.as_deref(), Some("lxc"));

        let serialized = serde_json::to_string(&status).unwrap();
        let deserialized: ContainerStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(status, deserialized);
    }
}
