use serde::{Deserialize, Serialize};
use serde_json::Value;

use urlencoding::encode;

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::{validate_node_name, validate_resource_id};

/// Ceph cluster status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CephStatus {
    /// Health status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<Value>,

    /// Monitor map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monmap: Option<Value>,

    /// OSD map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osdmap: Option<Value>,

    /// PG map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pgmap: Option<Value>,

    /// MDS map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdsmap: Option<Value>,

    /// MGR map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgrmap: Option<Value>,

    /// FS map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fsmap: Option<Value>,

    /// Quorum names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quorum_names: Option<Vec<String>>,
}

/// Ceph OSD information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CephOsd {
    /// OSD ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// OSD name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Device path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_path: Option<String>,

    /// OSD type (e.g., `bluestore`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osd_type: Option<String>,

    /// Status string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Host.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// Crush weight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crush_weight: Option<f64>,

    /// Device class (e.g., `ssd`, `hdd`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_class: Option<String>,

    /// Children (nested tree entries).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Value>>,
}

/// Ceph Monitor information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CephMon {
    /// Monitor name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Host.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// Address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,

    /// Rank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<u32>,

    /// Whether this is the quorum leader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quorum: Option<bool>,

    /// Ceph version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ceph_version: Option<String>,

    /// Whether this monitor is running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<bool>,
}

/// Ceph MDS (Metadata Server) information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CephMds {
    /// MDS name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Host.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// Address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,

    /// State.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Rank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,

    /// Whether standby.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standby: Option<bool>,

    /// Ceph version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ceph_version: Option<String>,
}

/// Ceph MGR (Manager) information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CephMgr {
    /// MGR name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Host.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// Address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,

    /// State.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Ceph version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ceph_version: Option<String>,
}

/// Ceph pool information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CephPool {
    /// Pool name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_name: Option<String>,

    /// Pool number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<u32>,

    /// Size (replicas).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,

    /// Minimum size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<u32>,

    /// Placement group count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pg_num: Option<u32>,

    /// Placement group auto-scale mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pg_autoscale_mode: Option<String>,

    /// CRUSH rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crush_rule: Option<u32>,

    /// CRUSH rule name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crush_rule_name: Option<String>,

    /// Bytes used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_used: Option<u64>,

    /// Percent used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_used: Option<f64>,

    /// Application metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_metadata: Option<Value>,

    /// Target size ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_size_ratio: Option<f64>,
}

/// Ceph filesystem information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CephFs {
    /// Filesystem name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Data pool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_pool: Option<String>,

    /// Metadata pool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_pool: Option<String>,
}

/// Ceph CRUSH rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CephRule {
    /// Rule name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Rule ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<u32>,

    /// Rule type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<u32>,

    /// Minimum size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<u32>,

    /// Maximum size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<u32>,

    /// Device class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
}

/// Ceph command safety check result.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CephCmdSafety {
    /// Whether the command is safe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe: Option<bool>,

    /// Safety status message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Parameters for initializing Ceph.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CephInitParams {
    /// Ceph network (e.g., `10.0.0.0/24`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,

    /// Cluster network.
    #[serde(rename = "cluster-network", skip_serializing_if = "Option::is_none")]
    pub cluster_network: Option<String>,

    /// Number of replicas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,

    /// Minimum number of replicas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<u32>,

    /// Disable CephX authentication.
    #[serde(rename = "disable_cephx", skip_serializing_if = "Option::is_none")]
    pub disable_cephx: Option<bool>,
}

/// Parameters for creating a Ceph OSD.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CephOsdCreateParams {
    /// Device path.
    pub dev: String,

    /// DB device (WAL/journal).
    #[serde(rename = "db_dev", skip_serializing_if = "Option::is_none")]
    pub db_dev: Option<String>,

    /// DB device size.
    #[serde(rename = "db_dev_size", skip_serializing_if = "Option::is_none")]
    pub db_dev_size: Option<f64>,

    /// WAL device.
    #[serde(rename = "wal_dev", skip_serializing_if = "Option::is_none")]
    pub wal_dev: Option<String>,

    /// WAL device size.
    #[serde(rename = "wal_dev_size", skip_serializing_if = "Option::is_none")]
    pub wal_dev_size: Option<f64>,

    /// Encrypted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,

    /// CRUSH device class.
    #[serde(rename = "crush-device-class", skip_serializing_if = "Option::is_none")]
    pub crush_device_class: Option<String>,
}

impl CephOsdCreateParams {
    /// Creates a new `CephOsdCreateParams` with the required fields.
    pub fn new(dev: impl Into<String>) -> Self {
        Self {
            dev: dev.into(),
            ..Default::default()
        }
    }
}

/// Parameters for creating a Ceph pool.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CephPoolCreateParams {
    /// Pool name.
    pub name: String,

    /// Replicas (pool size).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,

    /// Minimum replicas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<u32>,

    /// Number of placement groups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pg_num: Option<u32>,

    /// PG auto-scale mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pg_autoscale_mode: Option<String>,

    /// CRUSH rule name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crush_rule: Option<String>,

    /// Application (e.g., `rbd`, `cephfs`, `rgw`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,

    /// Add as Proxmox storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_storages: Option<bool>,

    /// Erasure coding profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub erasure_coding: Option<String>,

    /// Target size ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_size_ratio: Option<f64>,
}

impl CephPoolCreateParams {
    /// Creates a new `CephPoolCreateParams` with the required fields.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
}

/// Parameters for updating Ceph pool options.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CephPoolUpdateParams {
    /// Replicas (pool size).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,

    /// Minimum replicas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<u32>,

    /// Number of placement groups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pg_num: Option<u32>,

    /// PG auto-scale mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pg_autoscale_mode: Option<String>,

    /// CRUSH rule name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crush_rule: Option<String>,

    /// Application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,

    /// Target size ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_size_ratio: Option<f64>,
}

impl ProxmoxClient {
    // ── Ceph cluster ───────────────────────────────────────────────

    /// Returns Ceph cluster status.
    pub async fn get_ceph_status(&self, node: &str) -> Result<CephStatus> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/ceph/status"),
            &format!("node {node} Ceph status"),
        )
        .await
    }

    /// Initializes Ceph on a node.
    pub async fn init_ceph(&self, node: &str, params: &CephInitParams) -> Result<()> {
        validate_node_name(node)?;
        let response = self
            .post(&format!("/nodes/{node}/ceph/init"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("node {node} Ceph init")).await?;
        Ok(())
    }

    /// Starts Ceph services on a node.
    pub async fn start_ceph(&self, node: &str, service: Option<&str>) -> Result<String> {
        validate_node_name(node)?;
        let mut path = format!("/nodes/{node}/ceph/start");
        if let Some(s) = service {
            path.push_str(&format!("?service={}", encode(s)));
        }
        let response = self.post(&path)?.send().await?;
        Self::parse_response(response, &format!("node {node} Ceph start")).await
    }

    /// Stops Ceph services on a node.
    pub async fn stop_ceph(&self, node: &str, service: Option<&str>) -> Result<String> {
        validate_node_name(node)?;
        let mut path = format!("/nodes/{node}/ceph/stop");
        if let Some(s) = service {
            path.push_str(&format!("?service={}", encode(s)));
        }
        let response = self.post(&path)?.send().await?;
        Self::parse_response(response, &format!("node {node} Ceph stop")).await
    }

    /// Restarts Ceph services on a node.
    pub async fn restart_ceph(&self, node: &str, service: Option<&str>) -> Result<String> {
        validate_node_name(node)?;
        let mut path = format!("/nodes/{node}/ceph/restart");
        if let Some(s) = service {
            path.push_str(&format!("?service={}", encode(s)));
        }
        let response = self.post(&path)?.send().await?;
        Self::parse_response(response, &format!("node {node} Ceph restart")).await
    }

    // ── Ceph OSDs ──────────────────────────────────────────────────

    /// Lists Ceph OSDs.
    pub async fn list_ceph_osds(&self, node: &str) -> Result<Value> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/ceph/osd"),
            &format!("node {node} Ceph OSDs"),
        )
        .await
    }

    /// Creates a Ceph OSD.
    pub async fn create_ceph_osd(
        &self,
        node: &str,
        params: &CephOsdCreateParams,
    ) -> Result<String> {
        validate_node_name(node)?;
        self.post_parsed(
            &format!("/nodes/{node}/ceph/osd"),
            params,
            &format!("node {node} Ceph OSD create"),
        )
        .await
    }

    /// Destroys a Ceph OSD.
    pub async fn destroy_ceph_osd(&self, node: &str, osdid: u32) -> Result<String> {
        validate_node_name(node)?;
        let response = self
            .delete(&format!("/nodes/{node}/ceph/osd/{osdid}"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("Ceph OSD {osdid}")).await
    }

    /// Marks a Ceph OSD as `in`.
    pub async fn ceph_osd_in(&self, node: &str, osdid: u32) -> Result<()> {
        validate_node_name(node)?;
        let response = self
            .post(&format!("/nodes/{node}/ceph/osd/{osdid}/in"))?
            .send()
            .await?;
        Self::handle_error(response, &format!("Ceph OSD {osdid} in")).await?;
        Ok(())
    }

    /// Marks a Ceph OSD as `out`.
    pub async fn ceph_osd_out(&self, node: &str, osdid: u32) -> Result<()> {
        validate_node_name(node)?;
        let response = self
            .post(&format!("/nodes/{node}/ceph/osd/{osdid}/out"))?
            .send()
            .await?;
        Self::handle_error(response, &format!("Ceph OSD {osdid} out")).await?;
        Ok(())
    }

    /// Scrubs a Ceph OSD.
    pub async fn scrub_ceph_osd(&self, node: &str, osdid: u32, deep: Option<bool>) -> Result<()> {
        validate_node_name(node)?;
        let path = format!("/nodes/{node}/ceph/osd/{osdid}/scrub");
        if let Some(true) = deep {
            let params = serde_json::json!({ "deep": true });
            let response = self.post(&path)?.json(&params).send().await?;
            Self::handle_error(response, &format!("Ceph OSD {osdid} scrub")).await?;
        } else {
            let response = self.post(&path)?.send().await?;
            Self::handle_error(response, &format!("Ceph OSD {osdid} scrub")).await?;
        }
        Ok(())
    }

    // ── Ceph Monitors ──────────────────────────────────────────────

    /// Lists Ceph monitors.
    pub async fn list_ceph_mons(&self, node: &str) -> Result<Vec<CephMon>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/ceph/mon"),
            &format!("node {node} Ceph MONs"),
        )
        .await
    }

    /// Creates a Ceph monitor.
    pub async fn create_ceph_mon(&self, node: &str, monid: Option<&str>) -> Result<String> {
        validate_node_name(node)?;
        let monid = monid.unwrap_or(node);
        validate_resource_id(monid)?;
        let response = self
            .post(&format!("/nodes/{node}/ceph/mon/{monid}"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("Ceph MON {monid}")).await
    }

    /// Destroys a Ceph monitor.
    pub async fn destroy_ceph_mon(&self, node: &str, monid: &str) -> Result<String> {
        validate_node_name(node)?;
        validate_resource_id(monid)?;
        let response = self
            .delete(&format!("/nodes/{node}/ceph/mon/{monid}"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("Ceph MON {monid}")).await
    }

    // ── Ceph MDS ───────────────────────────────────────────────────

    /// Lists Ceph MDS daemons.
    pub async fn list_ceph_mds(&self, node: &str) -> Result<Vec<CephMds>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/ceph/mds"),
            &format!("node {node} Ceph MDS"),
        )
        .await
    }

    /// Creates a Ceph MDS daemon.
    pub async fn create_ceph_mds(&self, node: &str, name: Option<&str>) -> Result<String> {
        validate_node_name(node)?;
        let name = name.unwrap_or(node);
        validate_resource_id(name)?;
        let response = self
            .post(&format!("/nodes/{node}/ceph/mds/{name}"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("Ceph MDS {name}")).await
    }

    /// Destroys a Ceph MDS daemon.
    pub async fn destroy_ceph_mds(&self, node: &str, name: &str) -> Result<String> {
        validate_node_name(node)?;
        validate_resource_id(name)?;
        let response = self
            .delete(&format!("/nodes/{node}/ceph/mds/{name}"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("Ceph MDS {name}")).await
    }

    // ── Ceph MGR ───────────────────────────────────────────────────

    /// Lists Ceph MGR daemons.
    pub async fn list_ceph_mgrs(&self, node: &str) -> Result<Vec<CephMgr>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/ceph/mgr"),
            &format!("node {node} Ceph MGRs"),
        )
        .await
    }

    /// Creates a Ceph MGR daemon.
    pub async fn create_ceph_mgr(&self, node: &str, id: Option<&str>) -> Result<String> {
        validate_node_name(node)?;
        let id = id.unwrap_or(node);
        validate_resource_id(id)?;
        let response = self
            .post(&format!("/nodes/{node}/ceph/mgr/{id}"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("Ceph MGR {id}")).await
    }

    /// Destroys a Ceph MGR daemon.
    pub async fn destroy_ceph_mgr(&self, node: &str, id: &str) -> Result<String> {
        validate_node_name(node)?;
        validate_resource_id(id)?;
        let response = self
            .delete(&format!("/nodes/{node}/ceph/mgr/{id}"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("Ceph MGR {id}")).await
    }

    // ── Ceph Pools ─────────────────────────────────────────────────

    /// Lists Ceph pools.
    pub async fn list_ceph_pools(&self, node: &str) -> Result<Vec<CephPool>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/ceph/pool"),
            &format!("node {node} Ceph pools"),
        )
        .await
    }

    /// Creates a Ceph pool.
    pub async fn create_ceph_pool(
        &self,
        node: &str,
        params: &CephPoolCreateParams,
    ) -> Result<String> {
        validate_node_name(node)?;
        self.post_parsed(
            &format!("/nodes/{node}/ceph/pool"),
            params,
            &format!("Ceph pool {}", params.name),
        )
        .await
    }

    /// Returns information about a Ceph pool.
    pub async fn get_ceph_pool(&self, node: &str, name: &str) -> Result<CephPool> {
        validate_node_name(node)?;
        validate_resource_id(name)?;
        self.get_parsed(
            &format!("/nodes/{node}/ceph/pool/{name}"),
            &format!("Ceph pool {name}"),
        )
        .await
    }

    /// Updates Ceph pool options.
    pub async fn update_ceph_pool(
        &self,
        node: &str,
        name: &str,
        params: &CephPoolUpdateParams,
    ) -> Result<()> {
        validate_node_name(node)?;
        validate_resource_id(name)?;
        let response = self
            .put(&format!("/nodes/{node}/ceph/pool/{name}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("Ceph pool {name}")).await?;
        Ok(())
    }

    /// Destroys a Ceph pool.
    pub async fn destroy_ceph_pool(&self, node: &str, name: &str) -> Result<String> {
        validate_node_name(node)?;
        validate_resource_id(name)?;
        let response = self
            .delete(&format!("/nodes/{node}/ceph/pool/{name}"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("Ceph pool {name}")).await
    }

    // ── Ceph Filesystems ───────────────────────────────────────────

    /// Lists Ceph filesystems.
    pub async fn list_ceph_fs(&self, node: &str) -> Result<Vec<CephFs>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/ceph/fs"),
            &format!("node {node} CephFS"),
        )
        .await
    }

    /// Creates a Ceph filesystem.
    pub async fn create_ceph_fs(
        &self,
        node: &str,
        name: &str,
        pg_num: Option<u32>,
        add_storage: Option<bool>,
    ) -> Result<String> {
        validate_node_name(node)?;
        validate_resource_id(name)?;
        let mut params = serde_json::json!({});
        if let Some(p) = pg_num {
            params["pg_num"] = Value::Number(p.into());
        }
        if let Some(a) = add_storage {
            params["add-storage"] = Value::Bool(a);
        }
        let response = self
            .post(&format!("/nodes/{node}/ceph/fs/{name}"))?
            .json(&params)
            .send()
            .await?;
        Self::parse_response(response, &format!("CephFS {name}")).await
    }

    // ── Ceph config/info ───────────────────────────────────────────

    /// Lists Ceph CRUSH rules.
    pub async fn list_ceph_rules(&self, node: &str) -> Result<Vec<CephRule>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/ceph/rules"),
            &format!("node {node} Ceph rules"),
        )
        .await
    }

    /// Returns the Ceph configuration.
    pub async fn get_ceph_config(&self, node: &str) -> Result<String> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/ceph/cfg/raw"),
            &format!("node {node} Ceph config"),
        )
        .await
    }

    /// Returns the Ceph configuration database.
    pub async fn get_ceph_configdb(&self, node: &str) -> Result<Vec<Value>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/ceph/cfg/db"),
            &format!("node {node} Ceph configdb"),
        )
        .await
    }

    /// Gets Ceph configuration values.
    ///
    /// `GET /nodes/{node}/ceph/cfg/value`
    pub async fn get_ceph_config_value(&self, node: &str) -> Result<Vec<Value>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/ceph/cfg/value"),
            &format!("node {node} Ceph config value"),
        )
        .await
    }

    /// Checks if a Ceph command is safe to execute.
    ///
    /// `GET /nodes/{node}/ceph/cmd-safety`
    pub async fn check_ceph_cmd_safety(
        &self,
        node: &str,
        id: &str,
        action: &str,
    ) -> Result<CephCmdSafety> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!(
                "/nodes/{node}/ceph/cmd-safety?id={}&action={}",
                encode(id),
                encode(action)
            ),
            &format!("node {node} Ceph cmd safety"),
        )
        .await
    }

    /// Gets LV info for a Ceph OSD.
    ///
    /// `GET /nodes/{node}/ceph/osd/{osdid}/lv-info`
    pub async fn get_ceph_osd_lv_info(&self, node: &str, osdid: u32) -> Result<Value> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/ceph/osd/{osdid}/lv-info"),
            &format!("Ceph OSD {osdid} LV info"),
        )
        .await
    }

    /// Gets metadata for a Ceph OSD.
    ///
    /// `GET /nodes/{node}/ceph/osd/{osdid}/metadata`
    pub async fn get_ceph_osd_metadata(&self, node: &str, osdid: u32) -> Result<Value> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/ceph/osd/{osdid}/metadata"),
            &format!("Ceph OSD {osdid} metadata"),
        )
        .await
    }

    /// Returns the status of a specific Ceph pool.
    ///
    /// `GET /nodes/{node}/ceph/pool/{name}/status`
    pub async fn get_ceph_pool_status(&self, node: &str, name: &str) -> Result<Value> {
        validate_node_name(node)?;
        validate_resource_id(name)?;
        self.get_parsed(
            &format!("/nodes/{node}/ceph/pool/{name}/status"),
            &format!("Ceph pool {name} status"),
        )
        .await
    }

    /// Returns the CRUSH map.
    pub async fn get_ceph_crush(&self, node: &str) -> Result<String> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/ceph/crush"),
            &format!("node {node} Ceph CRUSH"),
        )
        .await
    }

    /// Returns the Ceph log.
    pub async fn get_ceph_log(
        &self,
        node: &str,
        start: Option<u64>,
        limit: Option<u64>,
    ) -> Result<Vec<Value>> {
        validate_node_name(node)?;
        let start_s = start.map(|v| v.to_string());
        let limit_s = limit.map(|v| v.to_string());
        let path = Self::build_query_path(
            &format!("/nodes/{node}/ceph/log"),
            &[("start", start_s.as_deref()), ("limit", limit_s.as_deref())],
        );
        self.get_parsed(&path, &format!("node {node} Ceph log"))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ceph_pool_serde_roundtrip() {
        let json = r#"{
            "pool_name": "rbd",
            "pool": 1,
            "size": 3,
            "min_size": 2,
            "pg_num": 128,
            "pg_autoscale_mode": "on",
            "crush_rule": 0,
            "crush_rule_name": "replicated_rule",
            "bytes_used": 1073741824,
            "percent_used": 0.01
        }"#;
        let pool: CephPool = serde_json::from_str(json).unwrap();
        assert_eq!(pool.pool_name.as_deref(), Some("rbd"));
        assert_eq!(pool.size, Some(3));
        assert_eq!(pool.pg_num, Some(128));

        let serialized = serde_json::to_string(&pool).unwrap();
        let deserialized: CephPool = serde_json::from_str(&serialized).unwrap();
        assert_eq!(pool, deserialized);
    }

    #[test]
    fn ceph_mon_serde_roundtrip() {
        let json = r#"{
            "name": "pve1",
            "host": "pve1",
            "addr": "10.0.0.1:6789/0",
            "rank": 0,
            "quorum": true,
            "running": true
        }"#;
        let mon: CephMon = serde_json::from_str(json).unwrap();
        assert_eq!(mon.name.as_deref(), Some("pve1"));
        assert_eq!(mon.quorum, Some(true));

        let serialized = serde_json::to_string(&mon).unwrap();
        let deserialized: CephMon = serde_json::from_str(&serialized).unwrap();
        assert_eq!(mon, deserialized);
    }

    #[test]
    fn ceph_pool_create_params_serialize() {
        let mut params = CephPoolCreateParams::new("mypool");
        params.size = Some(3);
        params.min_size = Some(2);
        params.pg_num = Some(128);
        params.application = Some("rbd".to_string());
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["name"], "mypool");
        assert_eq!(json["size"], 3);
        assert_eq!(json["application"], "rbd");
    }
}
