use serde::Serialize;
use serde_json::Value;
use urlencoding::encode;

use crate::client::ProxmoxClient;
use crate::error::Result;

/// Parameters for verifying a VNC ticket.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct VncTicketParams {
    /// Path to verify.
    pub path: String,

    /// VNC ticket to verify.
    pub vncticket: String,
}

impl VncTicketParams {
    /// Creates a new `VncTicketParams` with the required fields.
    pub fn new(path: impl Into<String>, vncticket: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            vncticket: vncticket.into(),
            ..Default::default()
        }
    }
}

impl ProxmoxClient {
    /// Gets effective permissions for a user.
    ///
    /// `GET /access/permissions`
    pub async fn get_permissions(&self, userid: Option<&str>, path: Option<&str>) -> Result<Value> {
        let userid_enc = userid.map(|u| format!("{}", encode(u)));
        let path_enc = path.map(|p| format!("{}", encode(p)));
        let url = Self::build_query_path(
            "/access/permissions",
            &[
                ("userid", userid_enc.as_deref()),
                ("path", path_enc.as_deref()),
            ],
        );
        self.get_parsed(&url, "permissions").await
    }

    /// Verifies a VNC ticket.
    ///
    /// `POST /access/vncticket`
    pub async fn verify_vnc_ticket(&self, params: &VncTicketParams) -> Result<Value> {
        self.post_parsed("/access/vncticket", params, "VNC ticket verification")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vnc_ticket_params_serialization() {
        let params = VncTicketParams {
            path: "/nodes/pve1/qemu/100".to_string(),
            vncticket: "PVE:tkt:abc123".to_string(),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["path"], "/nodes/pve1/qemu/100");
        assert_eq!(json["vncticket"], "PVE:tkt:abc123");
    }
}
