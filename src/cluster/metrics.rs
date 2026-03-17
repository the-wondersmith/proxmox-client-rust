use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_resource_id;

/// A metric server configuration.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MetricServer {
    /// Metric server ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Server type (e.g., `influxdb`, `graphite`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,

    /// Server hostname or IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,

    /// Server port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

    /// Whether the server is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,

    /// MTU for UDP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i64>,

    /// Timeout in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,

    /// Protocol (udp, tcp, https).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proto: Option<String>,

    /// Path prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Organization (for InfluxDB v2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,

    /// Bucket (for InfluxDB v2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,

    /// Token (for InfluxDB v2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    /// InfluxDB protocol version.
    #[serde(rename = "influxdbproto", skip_serializing_if = "Option::is_none")]
    pub influxdb_proto: Option<String>,

    /// Max body size.
    #[serde(rename = "max-body-size", skip_serializing_if = "Option::is_none")]
    pub max_body_size: Option<i64>,

    /// Whether to verify the server certificate.
    #[serde(default, rename = "verify-certificate", skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub verify_certificate: Option<bool>,
}

/// Parameters for creating a metric server.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct MetricServerCreateParams {
    /// Metric server ID (required).
    pub id: String,

    /// Server type (required, e.g., `influxdb`, `graphite`).
    #[serde(rename = "type")]
    pub server_type: String,

    /// Server hostname or IP (required).
    pub server: String,

    /// Server port (required).
    pub port: i64,

    /// Whether the server is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,

    /// MTU for UDP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i64>,

    /// Timeout in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,

    /// Protocol (udp, tcp, https).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proto: Option<String>,

    /// Path prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Organization (for InfluxDB v2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,

    /// Bucket (for InfluxDB v2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,

    /// Token (for InfluxDB v2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    /// InfluxDB protocol version.
    #[serde(rename = "influxdbproto", skip_serializing_if = "Option::is_none")]
    pub influxdb_proto: Option<String>,

    /// Max body size.
    #[serde(rename = "max-body-size", skip_serializing_if = "Option::is_none")]
    pub max_body_size: Option<i64>,

    /// Whether to verify the server certificate.
    #[serde(default, rename = "verify-certificate", skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub verify_certificate: Option<bool>,
}

impl MetricServerCreateParams {
    /// Creates a new `MetricServerCreateParams` with the required fields.
    pub fn new(
        id: impl Into<String>,
        server_type: impl Into<String>,
        server: impl Into<String>,
        port: i64,
    ) -> Self {
        Self {
            id: id.into(),
            server_type: server_type.into(),
            server: server.into(),
            port,
            ..Default::default()
        }
    }
}

impl ProxmoxClient {
    /// Lists all metric servers.
    ///
    /// `GET /cluster/metrics/server`
    pub async fn list_metric_servers(&self) -> Result<Vec<MetricServer>> {
        self.get_parsed("/cluster/metrics/server", "metric servers")
            .await
    }

    /// Creates a new metric server.
    ///
    /// `POST /cluster/metrics/server/{id}`
    pub async fn create_metric_server(&self, params: &MetricServerCreateParams) -> Result<()> {
        validate_resource_id(&params.id)?;
        let response = self
            .post(&format!("/cluster/metrics/server/{}", params.id))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, "metric server creation").await?;
        Ok(())
    }

    /// Gets a specific metric server.
    ///
    /// `GET /cluster/metrics/server/{id}`
    pub async fn get_metric_server(&self, id: &str) -> Result<MetricServer> {
        validate_resource_id(id)?;
        self.get_parsed(
            &format!("/cluster/metrics/server/{id}"),
            &format!("metric server {id}"),
        )
        .await
    }

    /// Updates a metric server.
    ///
    /// `PUT /cluster/metrics/server/{id}`
    pub async fn update_metric_server(&self, id: &str, params: &serde_json::Value) -> Result<()> {
        validate_resource_id(id)?;
        let response = self
            .put(&format!("/cluster/metrics/server/{id}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("metric server {id}")).await?;
        Ok(())
    }

    /// Deletes a metric server.
    ///
    /// `DELETE /cluster/metrics/server/{id}`
    pub async fn delete_metric_server(&self, id: &str) -> Result<()> {
        validate_resource_id(id)?;
        self.delete_void(
            &format!("/cluster/metrics/server/{id}"),
            &format!("metric server {id}"),
        )
        .await
    }

    /// Exports cluster metrics.
    ///
    /// `GET /cluster/metrics/export`
    pub async fn export_cluster_metrics(&self) -> Result<Vec<serde_json::Value>> {
        self.get_parsed("/cluster/metrics/export", "cluster metrics export")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metric_server_serde_roundtrip() {
        let server = MetricServer {
            id: Some("influx1".to_string()),
            server_type: Some("influxdb".to_string()),
            server: Some("10.0.0.50".to_string()),
            port: Some(8086),
            disable: Some(false),
            mtu: None,
            timeout: Some(5),
            proto: Some("https".to_string()),
            path: None,
            organization: Some("myorg".to_string()),
            bucket: Some("proxmox".to_string()),
            token: Some("my-token".to_string()),
            influxdb_proto: Some("v2".to_string()),
            max_body_size: None,
            verify_certificate: Some(true),
        };

        let json = serde_json::to_string(&server).unwrap();
        let deserialized: MetricServer = serde_json::from_str(&json).unwrap();
        assert_eq!(server, deserialized);
    }

    #[test]
    fn metric_server_skip_serializing_none() {
        let server = MetricServer::default();
        let json = serde_json::to_value(&server).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default MetricServer should serialize to {{}}"
        );
    }

    #[test]
    fn metric_server_type_rename() {
        let json = r#"{"type": "influxdb", "id": "influx1"}"#;
        let server: MetricServer = serde_json::from_str(json).unwrap();
        assert_eq!(server.server_type.as_deref(), Some("influxdb"));

        let serialized = serde_json::to_value(&server).unwrap();
        assert!(serialized.get("type").is_some());
        assert!(serialized.get("server_type").is_none());
    }

    #[test]
    fn metric_server_create_params_serialization() {
        let mut params = MetricServerCreateParams::new("influx1", "influxdb", "10.0.0.50", 8086);
        params.proto = Some("https".to_string());
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["id"], "influx1");
        assert_eq!(json["type"], "influxdb");
        assert_eq!(json["server"], "10.0.0.50");
        assert_eq!(json["port"], 8086);
        assert_eq!(json["proto"], "https");
        assert!(!json.as_object().unwrap().contains_key("bucket"));
    }

    #[test]
    fn metric_server_unknown_fields_ignored() {
        let json = r#"{"id": "test", "unknownField": true}"#;
        let server: MetricServer = serde_json::from_str(json).unwrap();
        assert_eq!(server.id.as_deref(), Some("test"));
    }

    #[test]
    fn metric_server_hyphenated_fields() {
        let json = r#"{"id": "test", "max-body-size": 25000000, "verify-certificate": 1}"#;
        let server: MetricServer = serde_json::from_str(json).unwrap();
        assert_eq!(server.max_body_size, Some(25000000));
        assert_eq!(server.verify_certificate, Some(true));
    }
}
