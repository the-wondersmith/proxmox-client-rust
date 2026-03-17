use serde::{Deserialize, Serialize};
use serde_json::Value;

use urlencoding::encode;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::nodes::qemu::VncProxyData;
use crate::validation::validate_node_name;

/// Summary status of a Proxmox node as returned by `GET /nodes`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NodeStatus {
    /// Node name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,

    /// Node status (`online`, `offline`, `unknown`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// CPU usage (0.0 - 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f64>,

    /// Number of CPUs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxcpu: Option<u32>,

    /// Used memory in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem: Option<u64>,

    /// Total memory in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxmem: Option<u64>,

    /// Used disk space in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<u64>,

    /// Total disk space in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxdisk: Option<u64>,

    /// Uptime in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime: Option<u64>,

    /// Node level (e.g., support level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,

    /// Node ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Node type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,

    /// SSL fingerprint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_fingerprint: Option<String>,
}

/// DNS settings for a node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NodeDns {
    /// Search domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

    /// Primary DNS server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns1: Option<String>,

    /// Secondary DNS server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns2: Option<String>,

    /// Tertiary DNS server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns3: Option<String>,
}

/// Parameters for updating DNS settings.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct NodeDnsUpdateParams {
    /// Search domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

    /// Primary DNS server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns1: Option<String>,

    /// Secondary DNS server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns2: Option<String>,

    /// Tertiary DNS server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns3: Option<String>,
}

/// Time settings for a node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NodeTime {
    /// Current timezone (e.g., `Europe/Berlin`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,

    /// Current time (epoch seconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,

    /// Local time as string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localtime: Option<u64>,
}

/// Parameters for setting the timezone.
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub struct NodeTimeUpdateParams {
    /// Timezone to set (e.g., `Europe/Berlin`).
    pub timezone: String,
}

impl NodeTimeUpdateParams {
    /// Creates new params with the required timezone.
    pub fn new(timezone: impl Into<String>) -> Self {
        Self {
            timezone: timezone.into(),
        }
    }
}

/// Version information for a node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NodeVersion {
    /// Proxmox VE version string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Release version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,

    /// Repository ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repoid: Option<String>,
}

/// RRD (Round Robin Database) data point.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RrdData {
    /// Timestamp (epoch seconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,

    /// CPU usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f64>,

    /// IO wait.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iowait: Option<f64>,

    /// Load average.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loadavg: Option<f64>,

    /// Used memory in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memused: Option<f64>,

    /// Total memory in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memtotal: Option<f64>,

    /// Used swap in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swapused: Option<f64>,

    /// Total swap in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swaptotal: Option<f64>,

    /// Used root filesystem in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rootused: Option<f64>,

    /// Total root filesystem in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roottotal: Option<f64>,

    /// Network in (bytes/s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netin: Option<f64>,

    /// Network out (bytes/s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netout: Option<f64>,

    /// Maximum CPU count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxcpu: Option<f64>,
}

/// Appliance template information.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ApplianceInfo {
    /// Template identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,

    /// Package name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,

    /// Short description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headline: Option<String>,

    /// OS type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,

    /// Section/category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,

    /// Full description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Version string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// SHA512 checksum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha512sum: Option<String>,

    /// Source URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Architecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,

    /// Info page URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infopage: Option<String>,

    /// Type (lxc, qemu).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub appliance_type: Option<String>,
}

/// Parameters for downloading an appliance template.
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub struct ApplianceDownloadParams {
    /// Storage to download to (required).
    pub storage: String,

    /// Template name (required).
    pub template: String,
}

impl ApplianceDownloadParams {
    /// Creates new params with the required storage and template.
    pub fn new(storage: impl Into<String>, template: impl Into<String>) -> Self {
        Self {
            storage: storage.into(),
            template: template.into(),
        }
    }
}

/// Hosts file data.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HostsFile {
    /// Contents of the hosts file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// Configuration digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

/// Parameters for writing the hosts file.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct HostsWriteParams {
    /// New hosts file content (required).
    pub data: String,

    /// Prevent changes if digest differs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

impl HostsWriteParams {
    /// Creates new params with the required hosts file data.
    pub fn new(data: impl Into<String>) -> Self {
        Self {
            data: data.into(),
            ..Default::default()
        }
    }
}

/// Parameters for migrating all VMs/CTs.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct MigrateAllParams {
    /// Target node (required).
    pub target: String,

    /// Maximum number of parallel workers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxworkers: Option<i64>,

    /// Only migrate these VMs (comma-separated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vms: Option<String>,

    /// Migrate VMs with local disks.
    #[serde(rename = "with-local-disks", skip_serializing_if = "Option::is_none")]
    pub with_local_disks: Option<i32>,
}

impl MigrateAllParams {
    /// Creates new params with the required target node.
    pub fn new(target: impl Into<String>) -> Self {
        Self {
            target: target.into(),
            ..Default::default()
        }
    }
}

/// URL metadata information.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UrlMetadata {
    /// Filename from URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// Content size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,

    /// MIME type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<String>,
}

/// Parameters for suspending all VMs/CTs.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct SuspendAllParams {
    /// Only suspend these VMs (comma-separated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vms: Option<String>,
}

/// Parameters for node power control.
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub struct NodeControlParams {
    /// Command to execute: `reboot`, `shutdown`.
    pub command: String,
}

impl NodeControlParams {
    /// Creates new params with the required command.
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
        }
    }
}

/// Parameters for start-all / stop-all.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct StartStopAllParams {
    /// Only consider VMs with these IDs (comma-separated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vms: Option<String>,

    /// Force stop (only for stop-all).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,

    /// Timeout in seconds for each VM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
}

impl ProxmoxClient {
    /// Lists all nodes in the cluster.
    pub async fn list_nodes(&self) -> Result<Vec<NodeStatus>> {
        self.get_parsed("/nodes", "nodes").await
    }

    /// Returns the status of a specific node.
    pub async fn get_node_status(&self, node: &str) -> Result<NodeStatus> {
        validate_node_name(node)?;
        self.get_parsed(&format!("/nodes/{node}/status"), &format!("node {node}"))
            .await
    }

    /// Controls a node (reboot or shutdown).
    pub async fn control_node(&self, node: &str, command: &str) -> Result<String> {
        validate_node_name(node)?;
        let params = NodeControlParams {
            command: command.to_owned(),
        };
        self.post_parsed(
            &format!("/nodes/{node}/status"),
            &params,
            &format!("node {node} control"),
        )
        .await
    }

    /// Returns the DNS settings for a node.
    pub async fn get_node_dns(&self, node: &str) -> Result<NodeDns> {
        validate_node_name(node)?;
        self.get_parsed(&format!("/nodes/{node}/dns"), &format!("node {node} DNS"))
            .await
    }

    /// Updates the DNS settings for a node.
    pub async fn update_node_dns(&self, node: &str, params: &NodeDnsUpdateParams) -> Result<()> {
        validate_node_name(node)?;
        let response = self
            .put(&format!("/nodes/{node}/dns"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("node {node} DNS")).await?;
        Ok(())
    }

    /// Returns the time configuration for a node.
    pub async fn get_node_time(&self, node: &str) -> Result<NodeTime> {
        validate_node_name(node)?;
        self.get_parsed(&format!("/nodes/{node}/time"), &format!("node {node} time"))
            .await
    }

    /// Sets the timezone for a node.
    pub async fn set_node_timezone(&self, node: &str, timezone: &str) -> Result<()> {
        validate_node_name(node)?;
        let params = NodeTimeUpdateParams {
            timezone: timezone.to_owned(),
        };
        let response = self
            .put(&format!("/nodes/{node}/time"))?
            .json(&params)
            .send()
            .await?;
        Self::handle_error(response, &format!("node {node} time")).await?;
        Ok(())
    }

    /// Returns version information for a node.
    pub async fn get_node_version(&self, node: &str) -> Result<NodeVersion> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/version"),
            &format!("node {node} version"),
        )
        .await
    }

    /// Sends a Wake-on-LAN magic packet.
    pub async fn wake_on_lan(&self, node: &str) -> Result<String> {
        validate_node_name(node)?;
        let response = self
            .post(&format!("/nodes/{node}/wakeonlan"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("node {node} WoL")).await
    }

    /// Starts all VMs/CTs on a node.
    pub async fn start_all(&self, node: &str, params: &StartStopAllParams) -> Result<String> {
        validate_node_name(node)?;
        self.post_parsed(
            &format!("/nodes/{node}/startall"),
            params,
            &format!("node {node} start all"),
        )
        .await
    }

    /// Stops all VMs/CTs on a node.
    pub async fn stop_all(&self, node: &str, params: &StartStopAllParams) -> Result<String> {
        validate_node_name(node)?;
        self.post_parsed(
            &format!("/nodes/{node}/stopall"),
            params,
            &format!("node {node} stop all"),
        )
        .await
    }

    /// Returns RRD data for a node.
    pub async fn get_node_rrddata(
        &self,
        node: &str,
        timeframe: &str,
        cf: Option<&str>,
    ) -> Result<Vec<RrdData>> {
        validate_node_name(node)?;
        let mut path = format!("/nodes/{node}/rrddata?timeframe={}", encode(timeframe));
        if let Some(cf) = cf {
            path.push_str(&format!("&cf={}", encode(cf)));
        }
        self.get_parsed(&path, &format!("node {node} rrddata"))
            .await
    }

    /// Returns a system report for a node.
    pub async fn get_node_report(&self, node: &str) -> Result<String> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/report"),
            &format!("node {node} report"),
        )
        .await
    }

    /// Lists available appliance templates.
    ///
    /// `GET /nodes/{node}/aplinfo`
    pub async fn list_appliances(&self, node: &str) -> Result<Vec<ApplianceInfo>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/aplinfo"),
            &format!("node {node} appliances"),
        )
        .await
    }

    /// Downloads an appliance template.
    ///
    /// `POST /nodes/{node}/aplinfo`
    pub async fn download_appliance(
        &self,
        node: &str,
        params: &ApplianceDownloadParams,
    ) -> Result<String> {
        validate_node_name(node)?;
        self.post_parsed(
            &format!("/nodes/{node}/aplinfo"),
            params,
            &format!("node {node} appliance download"),
        )
        .await
    }

    /// Gets the hosts file content.
    ///
    /// `GET /nodes/{node}/hosts`
    pub async fn get_hosts_file(&self, node: &str) -> Result<HostsFile> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/hosts"),
            &format!("node {node} hosts"),
        )
        .await
    }

    /// Writes the hosts file.
    ///
    /// `POST /nodes/{node}/hosts`
    pub async fn write_hosts_file(&self, node: &str, params: &HostsWriteParams) -> Result<()> {
        validate_node_name(node)?;
        let response = self
            .post(&format!("/nodes/{node}/hosts"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("node {node} hosts")).await?;
        Ok(())
    }

    /// Gets the systemd journal.
    ///
    /// `GET /nodes/{node}/journal`
    pub async fn get_journal(
        &self,
        node: &str,
        start: Option<u64>,
        limit: Option<u64>,
        since: Option<&str>,
        until: Option<&str>,
    ) -> Result<Vec<String>> {
        validate_node_name(node)?;
        let start_s = start.map(|v| v.to_string());
        let limit_s = limit.map(|v| v.to_string());
        let path = Self::build_query_path(
            &format!("/nodes/{node}/journal"),
            &[
                ("start", start_s.as_deref()),
                ("limit", limit_s.as_deref()),
                ("since", since),
                ("until", until),
            ],
        );
        self.get_parsed(&path, &format!("node {node} journal"))
            .await
    }

    /// Migrates all VMs/CTs to another node.
    ///
    /// `POST /nodes/{node}/migrateall`
    pub async fn migrate_all(&self, node: &str, params: &MigrateAllParams) -> Result<String> {
        validate_node_name(node)?;
        self.post_parsed(
            &format!("/nodes/{node}/migrateall"),
            params,
            &format!("node {node} migrate all"),
        )
        .await
    }

    /// Gets network statistics for a node.
    ///
    /// `GET /nodes/{node}/netstat`
    pub async fn get_netstat(&self, node: &str) -> Result<Vec<Value>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/netstat"),
            &format!("node {node} netstat"),
        )
        .await
    }

    /// Queries URL metadata.
    ///
    /// `GET /nodes/{node}/query-url-metadata`
    pub async fn query_url_metadata(
        &self,
        node: &str,
        url: &str,
        verify_certs: Option<bool>,
    ) -> Result<UrlMetadata> {
        validate_node_name(node)?;
        let mut path = format!("/nodes/{node}/query-url-metadata?url={}", encode(url));
        if let Some(v) = verify_certs {
            path.push_str(&format!("&verify-certificates={}", if v { 1 } else { 0 }));
        }
        self.get_parsed(&path, &format!("node {node} URL metadata"))
            .await
    }

    /// Queries OCI repository tags.
    ///
    /// `GET /nodes/{node}/query-oci-repo-tags`
    pub async fn query_oci_repo_tags(&self, node: &str, repository: &str) -> Result<Vec<Value>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!(
                "/nodes/{node}/query-oci-repo-tags?repository={}",
                encode(repository)
            ),
            &format!("node {node} OCI repo tags"),
        )
        .await
    }

    /// Creates a SPICE shell connection.
    ///
    /// `POST /nodes/{node}/spiceshell`
    pub async fn create_spice_shell(&self, node: &str) -> Result<Value> {
        validate_node_name(node)?;
        let response = self
            .post(&format!("/nodes/{node}/spiceshell"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("node {node} SPICE shell")).await
    }

    /// Suspends all VMs/CTs on a node.
    ///
    /// `POST /nodes/{node}/suspendall`
    pub async fn suspend_all(&self, node: &str, params: &SuspendAllParams) -> Result<String> {
        validate_node_name(node)?;
        self.post_parsed(
            &format!("/nodes/{node}/suspendall"),
            params,
            &format!("node {node} suspend all"),
        )
        .await
    }

    /// Gets syslog entries.
    ///
    /// `GET /nodes/{node}/syslog`
    pub async fn get_syslog(
        &self,
        node: &str,
        start: Option<u64>,
        limit: Option<u64>,
        service: Option<&str>,
        since: Option<&str>,
        until: Option<&str>,
    ) -> Result<Vec<Value>> {
        validate_node_name(node)?;
        let start_s = start.map(|v| v.to_string());
        let limit_s = limit.map(|v| v.to_string());
        let path = Self::build_query_path(
            &format!("/nodes/{node}/syslog"),
            &[
                ("start", start_s.as_deref()),
                ("limit", limit_s.as_deref()),
                ("service", service),
                ("since", since),
                ("until", until),
            ],
        );
        self.get_parsed(&path, &format!("node {node} syslog")).await
    }

    /// Creates a terminal proxy for a node.
    ///
    /// `POST /nodes/{node}/termproxy`
    pub async fn create_node_term_proxy(&self, node: &str) -> Result<VncProxyData> {
        validate_node_name(node)?;
        let response = self
            .post(&format!("/nodes/{node}/termproxy"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("node {node} terminal proxy")).await
    }

    /// Creates a VNC shell for a node.
    ///
    /// `POST /nodes/{node}/vncshell`
    pub async fn create_node_vnc_shell(&self, node: &str) -> Result<VncProxyData> {
        validate_node_name(node)?;
        let response = self
            .post(&format!("/nodes/{node}/vncshell"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("node {node} VNC shell")).await
    }

    /// Gets the VNC websocket connection for a node.
    ///
    /// `GET /nodes/{node}/vncwebsocket`
    pub async fn get_node_vnc_websocket(
        &self,
        node: &str,
        port: &str,
        vncticket: &str,
    ) -> Result<Value> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!(
                "/nodes/{node}/vncwebsocket?port={}&vncticket={}",
                encode(port),
                encode(vncticket)
            ),
            &format!("node {node} VNC websocket"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_status_serde_roundtrip() {
        let json = r#"{
            "node": "pve1",
            "status": "online",
            "cpu": 0.15,
            "maxcpu": 8,
            "mem": 4294967296,
            "maxmem": 17179869184,
            "disk": 10737418240,
            "maxdisk": 107374182400,
            "uptime": 86400,
            "type": "node"
        }"#;
        let status: NodeStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status.node.as_deref(), Some("pve1"));
        assert_eq!(status.status.as_deref(), Some("online"));
        assert_eq!(status.maxcpu, Some(8));
        assert_eq!(status.node_type.as_deref(), Some("node"));

        let serialized = serde_json::to_string(&status).unwrap();
        let deserialized: NodeStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(status, deserialized);
    }

    #[test]
    fn node_dns_serde_roundtrip() {
        let json = r#"{
            "search": "example.com",
            "dns1": "8.8.8.8",
            "dns2": "8.8.4.4"
        }"#;
        let dns: NodeDns = serde_json::from_str(json).unwrap();
        assert_eq!(dns.search.as_deref(), Some("example.com"));
        assert_eq!(dns.dns1.as_deref(), Some("8.8.8.8"));
        assert!(dns.dns3.is_none());

        let serialized = serde_json::to_string(&dns).unwrap();
        let deserialized: NodeDns = serde_json::from_str(&serialized).unwrap();
        assert_eq!(dns, deserialized);
    }

    #[test]
    fn node_time_serde_roundtrip() {
        let json = r#"{
            "timezone": "Europe/Berlin",
            "time": 1700000000,
            "localtime": 1700003600
        }"#;
        let time: NodeTime = serde_json::from_str(json).unwrap();
        assert_eq!(time.timezone.as_deref(), Some("Europe/Berlin"));

        let serialized = serde_json::to_string(&time).unwrap();
        let deserialized: NodeTime = serde_json::from_str(&serialized).unwrap();
        assert_eq!(time, deserialized);
    }

    #[test]
    fn rrd_data_serde_roundtrip() {
        let json = r#"{
            "time": 1700000000.0,
            "cpu": 0.25,
            "memused": 4294967296.0,
            "memtotal": 17179869184.0,
            "netin": 1048576.0,
            "netout": 524288.0
        }"#;
        let rrd: RrdData = serde_json::from_str(json).unwrap();
        assert_eq!(rrd.cpu, Some(0.25));

        let serialized = serde_json::to_string(&rrd).unwrap();
        let deserialized: RrdData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(rrd, deserialized);
    }

    #[test]
    fn node_version_serde_roundtrip() {
        let json = r#"{
            "version": "8.1.3",
            "release": "8.1",
            "repoid": "abc123"
        }"#;
        let version: NodeVersion = serde_json::from_str(json).unwrap();
        assert_eq!(version.version.as_deref(), Some("8.1.3"));

        let serialized = serde_json::to_string(&version).unwrap();
        let deserialized: NodeVersion = serde_json::from_str(&serialized).unwrap();
        assert_eq!(version, deserialized);
    }
}
