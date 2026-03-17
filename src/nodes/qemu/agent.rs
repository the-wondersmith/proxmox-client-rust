use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use urlencoding::encode;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::{validate_node_name, validate_vmid};

/// Parameters for executing a command via the guest agent.
#[derive(Debug, Clone, Default, Serialize)]
pub struct AgentExecParams {
    /// Command to execute.
    pub command: String,

    /// Optional arguments.
    #[serde(rename = "input-data", skip_serializing_if = "Option::is_none")]
    pub input_data: Option<String>,
}

impl AgentExecParams {
    /// Creates a new `AgentExecParams` with the required fields.
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            ..Default::default()
        }
    }
}

/// Response from executing a command via the guest agent.
///
/// Contains the PID of the started process which can be used to poll
/// for completion via [`ProxmoxClient::agent_exec_status`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AgentExecResponse {
    /// PID of the started process in the guest.
    pub pid: i64,
}

/// Status of a guest agent exec command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AgentExecStatus {
    /// Whether the command has exited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exited: Option<bool>,

    /// Exit code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exitcode: Option<i32>,

    /// Standard output.
    #[serde(rename = "out-data", skip_serializing_if = "Option::is_none")]
    pub out_data: Option<String>,

    /// Standard error.
    #[serde(rename = "err-data", skip_serializing_if = "Option::is_none")]
    pub err_data: Option<String>,

    /// Whether output was truncated.
    #[serde(rename = "out-truncated", skip_serializing_if = "Option::is_none")]
    pub out_truncated: Option<bool>,

    /// Whether error output was truncated.
    #[serde(rename = "err-truncated", skip_serializing_if = "Option::is_none")]
    pub err_truncated: Option<bool>,
}

/// Parameters for reading a file via the guest agent.
#[derive(Debug, Clone, Default, Serialize)]
pub struct AgentFileReadParams {
    /// File path to read.
    pub file: String,
}

impl AgentFileReadParams {
    /// Creates a new `AgentFileReadParams` with the required fields.
    pub fn new(file: impl Into<String>) -> Self {
        Self {
            file: file.into(),
        }
    }
}

/// Parameters for writing a file via the guest agent.
#[derive(Debug, Clone, Default, Serialize)]
pub struct AgentFileWriteParams {
    /// File path to write.
    pub file: String,

    /// File content.
    pub content: String,
}

impl AgentFileWriteParams {
    /// Creates a new `AgentFileWriteParams` with the required fields.
    pub fn new(file: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            file: file.into(),
            content: content.into(),
        }
    }
}

/// Parameters for setting a user password via the guest agent.
#[derive(Clone, Default, Serialize)]
pub struct AgentSetPasswordParams {
    /// Username.
    pub username: String,

    /// Password.
    pub password: String,

    /// Whether the password is already crypted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crypted: Option<bool>,
}

impl AgentSetPasswordParams {
    /// Creates a new `AgentSetPasswordParams` with the required fields.
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
            ..Default::default()
        }
    }
}

impl fmt::Debug for AgentSetPasswordParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AgentSetPasswordParams")
            .field("username", &self.username)
            .field("password", &"<redacted>")
            .field("crypted", &self.crypted)
            .finish()
    }
}

impl ProxmoxClient {
    // ── Guest agent: exec ──────────────────────────────────────────

    /// Executes a command in the guest via the QEMU agent.
    ///
    /// Returns the PID of the started process. Use [`Self::agent_exec_status`]
    /// to poll for completion and retrieve output.
    pub async fn agent_exec(
        &self,
        node: &str,
        vmid: u32,
        params: &AgentExecParams,
    ) -> Result<AgentExecResponse> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.post_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/agent/exec"),
            params,
            &format!("VM {vmid} agent exec"),
        )
        .await
    }

    /// Returns the status of a guest agent exec command.
    pub async fn agent_exec_status(
        &self,
        node: &str,
        vmid: u32,
        pid: u32,
    ) -> Result<AgentExecStatus> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/agent/exec-status?pid={pid}"),
            &format!("VM {vmid} agent exec-status"),
        )
        .await
    }

    // ── Guest agent: file operations ───────────────────────────────

    /// Reads a file from the guest via the QEMU agent.
    pub async fn agent_file_read(&self, node: &str, vmid: u32, file: &str) -> Result<Value> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!(
                "/nodes/{node}/qemu/{vmid}/agent/file-read?file={}",
                encode(file)
            ),
            &format!("VM {vmid} agent file-read"),
        )
        .await
    }

    /// Writes a file to the guest via the QEMU agent.
    pub async fn agent_file_write(
        &self,
        node: &str,
        vmid: u32,
        params: &AgentFileWriteParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/qemu/{vmid}/agent/file-write"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("VM {vmid} agent file-write")).await?;
        Ok(())
    }

    // ── Guest agent: info queries ──────────────────────────────────

    /// Returns filesystem information from the guest.
    ///
    /// Response is OS-dependent. Example JSON:
    /// ```json
    /// [{"name": "/dev/sda1", "mountpoint": "/", "type": "ext4", "total-bytes": 10737418240, "used-bytes": 3221225472}]
    /// ```
    pub async fn agent_get_fsinfo(&self, node: &str, vmid: u32) -> Result<Value> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/agent/get-fsinfo"),
            &format!("VM {vmid} agent fsinfo"),
        )
        .await
    }

    /// Returns the hostname from the guest.
    ///
    /// Example JSON: `{"host-name": "myvm"}`
    pub async fn agent_get_hostname(&self, node: &str, vmid: u32) -> Result<Value> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/agent/get-host-name"),
            &format!("VM {vmid} agent hostname"),
        )
        .await
    }

    /// Returns memory block info from the guest.
    pub async fn agent_get_memory_block_info(&self, node: &str, vmid: u32) -> Result<Value> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/agent/get-memory-block-info"),
            &format!("VM {vmid} agent memory-block-info"),
        )
        .await
    }

    /// Returns memory blocks from the guest.
    pub async fn agent_get_memory_blocks(&self, node: &str, vmid: u32) -> Result<Value> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/agent/get-memory-blocks"),
            &format!("VM {vmid} agent memory-blocks"),
        )
        .await
    }

    /// Returns OS information from the guest.
    ///
    /// Example JSON: `{"id": "ubuntu", "version-id": "22.04", "pretty-name": "Ubuntu 22.04 LTS", "kernel-release": "5.15.0-91-generic"}`
    pub async fn agent_get_osinfo(&self, node: &str, vmid: u32) -> Result<Value> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/agent/get-osinfo"),
            &format!("VM {vmid} agent osinfo"),
        )
        .await
    }

    /// Returns the time from the guest as seconds since epoch.
    ///
    /// Example JSON: `{"time": 1700000000}`
    pub async fn agent_get_time(&self, node: &str, vmid: u32) -> Result<Value> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/agent/get-time"),
            &format!("VM {vmid} agent time"),
        )
        .await
    }

    /// Returns timezone information from the guest.
    ///
    /// Example JSON: `{"zone": "Europe/Berlin", "offset": 3600}`
    pub async fn agent_get_timezone(&self, node: &str, vmid: u32) -> Result<Value> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/agent/get-timezone"),
            &format!("VM {vmid} agent timezone"),
        )
        .await
    }

    /// Returns logged-in user information from the guest.
    ///
    /// Example JSON: `[{"user": "root", "login-time": 1700000000}]`
    pub async fn agent_get_users(&self, node: &str, vmid: u32) -> Result<Value> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/agent/get-users"),
            &format!("VM {vmid} agent users"),
        )
        .await
    }

    /// Returns vCPU information from the guest.
    ///
    /// Example JSON: `[{"online": true, "can-offline": false, "logical-id": 0}]`
    pub async fn agent_get_vcpus(&self, node: &str, vmid: u32) -> Result<Value> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/agent/get-vcpus"),
            &format!("VM {vmid} agent vcpus"),
        )
        .await
    }

    /// Returns guest agent info.
    pub async fn agent_info(&self, node: &str, vmid: u32) -> Result<Value> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/agent/info"),
            &format!("VM {vmid} agent info"),
        )
        .await
    }

    /// Returns network interfaces from the guest.
    ///
    /// Example JSON: `[{"name": "eth0", "hardware-address": "00:11:22:33:44:55", "ip-addresses": [...]}]`
    pub async fn agent_network_get_interfaces(&self, node: &str, vmid: u32) -> Result<Value> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        self.get_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/agent/network-get-interfaces"),
            &format!("VM {vmid} agent network-interfaces"),
        )
        .await
    }

    // ── Guest agent: actions ───────────────────────────────────────

    /// Pings the guest agent.
    pub async fn agent_ping(&self, node: &str, vmid: u32) -> Result<Value> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/qemu/{vmid}/agent/ping"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("VM {vmid} agent ping")).await
    }

    /// Sets a user password in the guest.
    pub async fn agent_set_user_password(
        &self,
        node: &str,
        vmid: u32,
        params: &AgentSetPasswordParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!(
                "/nodes/{node}/qemu/{vmid}/agent/set-user-password"
            ))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("VM {vmid} agent set-user-password")).await?;
        Ok(())
    }

    /// Shuts down the guest via the agent.
    pub async fn agent_shutdown(&self, node: &str, vmid: u32) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/qemu/{vmid}/agent/shutdown"))?
            .send()
            .await?;
        Self::handle_error(response, &format!("VM {vmid} agent shutdown")).await?;
        Ok(())
    }

    /// Hibernates the guest (suspend to disk).
    pub async fn agent_suspend_disk(&self, node: &str, vmid: u32) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/qemu/{vmid}/agent/suspend-disk"))?
            .send()
            .await?;
        Self::handle_error(response, &format!("VM {vmid} agent suspend-disk")).await?;
        Ok(())
    }

    /// Performs hybrid suspend on the guest.
    pub async fn agent_suspend_hybrid(&self, node: &str, vmid: u32) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/qemu/{vmid}/agent/suspend-hybrid"))?
            .send()
            .await?;
        Self::handle_error(response, &format!("VM {vmid} agent suspend-hybrid")).await?;
        Ok(())
    }

    /// Suspends the guest to RAM.
    pub async fn agent_suspend_ram(&self, node: &str, vmid: u32) -> Result<()> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/qemu/{vmid}/agent/suspend-ram"))?
            .send()
            .await?;
        Self::handle_error(response, &format!("VM {vmid} agent suspend-ram")).await?;
        Ok(())
    }

    /// Freezes all filesystems in the guest.
    pub async fn agent_fsfreeze_freeze(&self, node: &str, vmid: u32) -> Result<Value> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/qemu/{vmid}/agent/fsfreeze-freeze"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("VM {vmid} agent fsfreeze-freeze")).await
    }

    /// Returns filesystem freeze status.
    pub async fn agent_fsfreeze_status(&self, node: &str, vmid: u32) -> Result<Value> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/qemu/{vmid}/agent/fsfreeze-status"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("VM {vmid} agent fsfreeze-status")).await
    }

    /// Thaws all frozen filesystems in the guest.
    pub async fn agent_fsfreeze_thaw(&self, node: &str, vmid: u32) -> Result<Value> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/qemu/{vmid}/agent/fsfreeze-thaw"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("VM {vmid} agent fsfreeze-thaw")).await
    }

    /// Trims filesystems in the guest.
    pub async fn agent_fstrim(&self, node: &str, vmid: u32) -> Result<Value> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let response = self
            .post(&format!("/nodes/{node}/qemu/{vmid}/agent/fstrim"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("VM {vmid} agent fstrim")).await
    }

    /// Executes an arbitrary guest agent command by name.
    ///
    /// `POST /nodes/{node}/qemu/{vmid}/agent`
    pub async fn run_agent_command(&self, node: &str, vmid: u32, command: &str) -> Result<Value> {
        validate_node_name(node)?;
        validate_vmid(vmid)?;
        let params = serde_json::json!({ "command": command });
        self.post_parsed(
            &format!("/nodes/{node}/qemu/{vmid}/agent"),
            &params,
            &format!("VM {vmid} agent command"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_exec_status_serde_roundtrip() {
        let json = r#"{
            "exited": true,
            "exitcode": 0,
            "out-data": "Hello, World!\n",
            "err-data": ""
        }"#;
        let status: AgentExecStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status.exited, Some(true));
        assert_eq!(status.exitcode, Some(0));
        assert_eq!(status.out_data.as_deref(), Some("Hello, World!\n"));

        let serialized = serde_json::to_string(&status).unwrap();
        let deserialized: AgentExecStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(status, deserialized);
    }

    #[test]
    fn agent_exec_params_serialize() {
        let params = AgentExecParams::new("ls -la");
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["command"], "ls -la");
        assert!(json.get("input-data").is_none());
    }
}
