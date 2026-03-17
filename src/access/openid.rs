use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::{Error, Result};

/// Request parameters for obtaining an OpenID Connect authorization URL.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct OpenIdAuthUrlRequest {
    /// The realm name (required).
    pub realm: String,

    /// The redirect URL after authentication (required).
    #[serde(rename = "redirect-url")]
    pub redirect_url: String,
}

impl OpenIdAuthUrlRequest {
    /// Creates a new `OpenIdAuthUrlRequest` with the required fields.
    pub fn new(realm: impl Into<String>, redirect_url: impl Into<String>) -> Self {
        Self {
            realm: realm.into(),
            redirect_url: redirect_url.into(),
            ..Default::default()
        }
    }
}

/// Response containing the OpenID Connect authorization URL.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenIdAuthUrl {
    /// The authorization URL to redirect the user to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Parameters for the OpenID Connect login callback.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct OpenIdLoginParams {
    /// The authorization code from the OpenID provider callback (required).
    pub code: String,

    /// The redirect URL used in the initial auth request (required).
    #[serde(rename = "redirect-url")]
    pub redirect_url: String,

    /// The state parameter from the callback (required).
    pub state: String,
}

impl OpenIdLoginParams {
    /// Creates a new `OpenIdLoginParams` with the required fields.
    pub fn new(
        code: impl Into<String>,
        redirect_url: impl Into<String>,
        state: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            redirect_url: redirect_url.into(),
            state: state.into(),
            ..Default::default()
        }
    }
}

/// Response from a successful OpenID Connect login.
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenIdLoginResponse {
    /// The authentication ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,

    /// The CSRF prevention token.
    #[serde(
        rename = "CSRFPreventionToken",
        skip_serializing_if = "Option::is_none"
    )]
    pub csrf_prevention_token: Option<String>,

    /// The authenticated username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Cluster name, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clustername: Option<String>,

    /// Capabilities bitmask.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap: Option<serde_json::Value>,
}

impl std::fmt::Debug for OpenIdLoginResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenIdLoginResponse")
            .field("ticket", &self.ticket.as_ref().map(|_| "<redacted>"))
            .field(
                "csrf_prevention_token",
                &self.csrf_prevention_token.as_ref().map(|_| "<redacted>"),
            )
            .field("username", &self.username)
            .field("clustername", &self.clustername)
            .field("cap", &self.cap)
            .finish()
    }
}

impl ProxmoxClient {
    /// Gets the OpenID Connect authorization URL for a realm.
    ///
    /// `POST /access/openid/auth-url`
    ///
    /// This endpoint does not require prior authentication.
    pub async fn openid_auth_url(&self, params: &OpenIdAuthUrlRequest) -> Result<OpenIdAuthUrl> {
        let url = self.url("/access/openid/auth-url");
        let response = self.http.post(&url).json(params).send().await?;
        Self::parse_response(response, "openid auth-url").await
    }

    /// Completes an OpenID Connect login with the callback parameters.
    ///
    /// `POST /access/openid/login`
    ///
    /// This endpoint does not require prior authentication.
    /// On success, the client is automatically authenticated with the returned ticket.
    pub async fn openid_login(&self, params: &OpenIdLoginParams) -> Result<OpenIdLoginResponse> {
        let url = self.url("/access/openid/login");
        let response = self.http.post(&url).json(params).send().await?;
        let resp: OpenIdLoginResponse = Self::parse_response(response, "openid login").await?;

        // Store the ticket for subsequent API calls, same as regular login.
        match (&resp.ticket, &resp.csrf_prevention_token) {
            (Some(ticket), Some(csrf)) => {
                self.set_ticket_auth(ticket, csrf);
                Ok(resp)
            }
            _ => Err(Error::LoginFailed(
                "OpenID login did not return ticket or CSRF token".to_owned(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn openid_auth_url_request_serialization() {
        let req = OpenIdAuthUrlRequest {
            realm: "myoidc".to_string(),
            redirect_url: "https://pve:8006".to_string(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["realm"], "myoidc");
        assert_eq!(json["redirect-url"], "https://pve:8006");
        assert!(
            json.get("redirect_url").is_none(),
            "must serialize as redirect-url, not redirect_url"
        );
    }

    #[test]
    fn openid_auth_url_serde_roundtrip() {
        let url = OpenIdAuthUrl {
            url: Some("https://auth.example.com/authorize?client_id=...".to_string()),
        };

        let json = serde_json::to_string(&url).unwrap();
        let deserialized: OpenIdAuthUrl = serde_json::from_str(&json).unwrap();
        assert_eq!(url, deserialized);
    }

    #[test]
    fn openid_auth_url_skip_none() {
        let url = OpenIdAuthUrl::default();
        let json = serde_json::to_value(&url).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default OpenIdAuthUrl should serialize to {{}}"
        );
    }

    #[test]
    fn openid_login_params_serialization() {
        let params = OpenIdLoginParams {
            code: "auth-code-123".to_string(),
            redirect_url: "https://pve:8006".to_string(),
            state: "state-abc".to_string(),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["code"], "auth-code-123");
        assert_eq!(json["redirect-url"], "https://pve:8006");
        assert_eq!(json["state"], "state-abc");
        assert!(
            json.get("redirect_url").is_none(),
            "must serialize as redirect-url, not redirect_url"
        );
    }

    #[test]
    fn openid_login_response_serde_roundtrip() {
        let resp = OpenIdLoginResponse {
            ticket: Some("PVE:root@myoidc:TICKET...".to_string()),
            csrf_prevention_token: Some("6A47B28E:csrf-value".to_string()),
            username: Some("user@myoidc".to_string()),
            clustername: Some("mycluster".to_string()),
            cap: None,
        };

        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: OpenIdLoginResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn openid_login_response_csrf_rename() {
        let resp = OpenIdLoginResponse {
            csrf_prevention_token: Some("token-value".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_value(&resp).unwrap();
        assert!(
            json.get("CSRFPreventionToken").is_some(),
            "csrf_prevention_token must serialize as CSRFPreventionToken"
        );
        assert!(
            json.get("csrf_prevention_token").is_none(),
            "must not serialize as csrf_prevention_token"
        );
    }

    #[test]
    fn openid_login_response_deserialize_from_api() {
        let json = r#"{
            "ticket": "PVE:user@oidcrealm:ABCDEF...",
            "CSRFPreventionToken": "6A47B28E:csrf-token",
            "username": "user@oidcrealm",
            "clustername": "testcluster"
        }"#;
        let resp: OpenIdLoginResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.ticket.as_deref(), Some("PVE:user@oidcrealm:ABCDEF..."));
        assert_eq!(
            resp.csrf_prevention_token.as_deref(),
            Some("6A47B28E:csrf-token")
        );
        assert_eq!(resp.username.as_deref(), Some("user@oidcrealm"));
        assert_eq!(resp.clustername.as_deref(), Some("testcluster"));
    }

    #[test]
    fn openid_login_response_skip_none() {
        let resp = OpenIdLoginResponse::default();
        let json = serde_json::to_value(&resp).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default OpenIdLoginResponse should serialize to {{}}"
        );
    }

    #[test]
    fn openid_login_response_unknown_fields_ignored() {
        let json = r#"{"ticket": "PVE:test:...", "newField": true}"#;
        let resp: OpenIdLoginResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.ticket.as_deref(), Some("PVE:test:..."));
    }
}
