use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_node_name;

/// Certificate information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CertificateInfo {
    /// Filename.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// Fingerprint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// Issuer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,

    /// Subject.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    /// Not-before date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notbefore: Option<u64>,

    /// Not-after date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notafter: Option<u64>,

    /// Subject Alternative Names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub san: Option<Vec<String>>,

    /// Whether this is a public key only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_type: Option<String>,

    /// Key bits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_bits: Option<u32>,

    /// PEM data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pem: Option<String>,
}

/// Parameters for uploading a custom certificate.
#[derive(Debug, Clone, Serialize)]
pub struct CustomCertUpload {
    /// PEM-encoded certificate.
    pub certificates: String,

    /// PEM-encoded private key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// Force overwrite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,

    /// Restart pveproxy after upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<bool>,
}

impl CustomCertUpload {
    /// Creates a new `CustomCertUpload` with the required fields.
    pub fn new(certificates: impl Into<String>) -> Self {
        Self {
            certificates: certificates.into(),
            key: None,
            force: None,
            restart: None,
        }
    }
}

impl ProxmoxClient {
    /// Lists certificates on a node.
    pub async fn list_certificates(&self, node: &str) -> Result<Vec<CertificateInfo>> {
        validate_node_name(node)?;
        self.get_parsed(
            &format!("/nodes/{node}/certificates/info"),
            &format!("node {node} certificates"),
        )
        .await
    }

    /// Uploads a custom certificate.
    pub async fn upload_custom_certificate(
        &self,
        node: &str,
        params: &CustomCertUpload,
    ) -> Result<CertificateInfo> {
        validate_node_name(node)?;
        self.post_parsed(
            &format!("/nodes/{node}/certificates/custom"),
            params,
            &format!("node {node} custom certificate"),
        )
        .await
    }

    /// Deletes a custom certificate.
    pub async fn delete_custom_certificate(&self, node: &str) -> Result<()> {
        validate_node_name(node)?;
        self.delete_void(
            &format!("/nodes/{node}/certificates/custom"),
            &format!("node {node} custom certificate"),
        )
        .await
    }

    /// Orders an ACME certificate.
    pub async fn order_acme_certificate(&self, node: &str) -> Result<String> {
        validate_node_name(node)?;
        let response = self
            .post(&format!("/nodes/{node}/certificates/acme/certificate"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("node {node} ACME order")).await
    }

    /// Renews an ACME certificate.
    pub async fn renew_acme_certificate(&self, node: &str) -> Result<String> {
        validate_node_name(node)?;
        let response = self
            .put(&format!("/nodes/{node}/certificates/acme/certificate"))?
            .send()
            .await?;
        Self::parse_response(response, &format!("node {node} ACME renew")).await
    }

    /// Revokes an ACME certificate.
    pub async fn revoke_acme_certificate(&self, node: &str) -> Result<()> {
        validate_node_name(node)?;
        self.delete_void(
            &format!("/nodes/{node}/certificates/acme/certificate"),
            &format!("node {node} ACME revoke"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn certificate_info_serde_roundtrip() {
        let json = r#"{
            "filename": "pveproxy-ssl.pem",
            "fingerprint": "AB:CD:EF:12:34:56",
            "issuer": "CN=Proxmox Virtual Environment",
            "subject": "CN=pve.example.com",
            "notbefore": 1700000000,
            "notafter": 1731536000,
            "san": ["pve.example.com", "10.0.0.1"]
        }"#;
        let cert: CertificateInfo = serde_json::from_str(json).unwrap();
        assert_eq!(cert.filename.as_deref(), Some("pveproxy-ssl.pem"));
        assert_eq!(cert.san.as_ref().map(|v| v.len()), Some(2));

        let serialized = serde_json::to_string(&cert).unwrap();
        let deserialized: CertificateInfo = serde_json::from_str(&serialized).unwrap();
        assert_eq!(cert, deserialized);
    }
}
