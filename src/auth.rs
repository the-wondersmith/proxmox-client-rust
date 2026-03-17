use std::fmt;

use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::{Error, Result};

/// Request body for `POST /access/ticket`.
#[derive(Serialize)]
struct TicketRequest<'a> {
    username: &'a str,
    password: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    otp: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    realm: Option<&'a str>,
    #[serde(rename = "tfa-challenge", skip_serializing_if = "Option::is_none")]
    tfa_challenge: Option<&'a str>,
}

impl fmt::Debug for TicketRequest<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TicketRequest")
            .field("username", &self.username)
            .field("password", &"<redacted>")
            .field("otp", &self.otp)
            .field("realm", &self.realm)
            .field("tfa_challenge", &self.tfa_challenge)
            .finish()
    }
}

/// Response from `POST /access/ticket`.
#[derive(Deserialize)]
pub struct TicketResponse {
    /// The authentication ticket (used as cookie value).
    pub ticket: Option<String>,

    /// CSRF prevention token for write operations.
    #[serde(rename = "CSRFPreventionToken")]
    pub csrf_prevention_token: Option<String>,

    /// Username that was authenticated.
    pub username: Option<String>,

    /// Cluster name, if the node is part of a cluster.
    pub clustername: Option<String>,

    /// TFA challenge data (when two-factor is required).
    ///
    /// If present, the initial login returned a challenge instead of a ticket.
    /// You must call [`login_tfa()`](ProxmoxClient::login_tfa) with the challenge
    /// and the user's TFA response.
    #[serde(rename = "new-format", default)]
    pub new_format: Option<serde_json::Value>,
}

impl fmt::Debug for TicketResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TicketResponse")
            .field("ticket", &self.ticket.as_ref().map(|_| "<redacted>"))
            .field(
                "csrf_prevention_token",
                &self.csrf_prevention_token.as_ref().map(|_| "<redacted>"),
            )
            .field("username", &self.username)
            .field("clustername", &self.clustername)
            .field("new_format", &self.new_format)
            .finish()
    }
}

impl TicketResponse {
    /// Returns `true` if this response contains a TFA challenge
    /// instead of a usable ticket.
    #[must_use]
    pub fn is_tfa_challenge(&self) -> bool {
        // When TFA is required, ticket starts with "PVE:tfa-challenge:"
        self.ticket
            .as_deref()
            .is_some_and(|t| t.starts_with("PVE:tfa-challenge:"))
    }
}

/// Password change request body.
#[derive(Serialize)]
struct PasswordChangeRequest<'a> {
    userid: &'a str,
    password: &'a str,
}

impl fmt::Debug for PasswordChangeRequest<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PasswordChangeRequest")
            .field("userid", &self.userid)
            .field("password", &"<redacted>")
            .finish()
    }
}

impl ProxmoxClient {
    /// Authenticates with username and password, storing the ticket and CSRF token.
    ///
    /// After a successful login, subsequent API calls will use ticket-based auth.
    ///
    /// # Errors
    ///
    /// Returns [`Error::LoginFailed`] if the server does not return a valid ticket,
    /// or if TFA is required (use [`login_tfa()`](Self::login_tfa) for that flow).
    pub async fn login(&self, username: &str, password: &str) -> Result<TicketResponse> {
        let req = TicketRequest {
            username,
            password,
            otp: None,
            realm: None,
            tfa_challenge: None,
        };

        let url = self.url("/access/ticket");
        let response = self.http.post(&url).json(&req).send().await?;
        let ticket_resp: TicketResponse = Self::parse_response(response, "login").await?;

        if ticket_resp.is_tfa_challenge() {
            return Err(Error::LoginFailed(
                "TFA challenge required — use login_tfa()".to_owned(),
            ));
        }

        match (&ticket_resp.ticket, &ticket_resp.csrf_prevention_token) {
            (Some(ticket), Some(csrf)) => {
                self.set_ticket_auth(ticket, csrf);
                Ok(ticket_resp)
            }
            _ => Err(Error::LoginFailed(
                "server did not return ticket or CSRF token".to_owned(),
            )),
        }
    }

    /// Completes a two-factor authentication challenge.
    ///
    /// `challenge_ticket` is the ticket from the initial login response
    /// (starting with `PVE:tfa-challenge:`).
    /// `tfa_response` is the user's TOTP code, U2F response, etc.
    pub async fn login_tfa(
        &self,
        username: &str,
        challenge_ticket: &str,
        tfa_response: &str,
    ) -> Result<TicketResponse> {
        let req = TicketRequest {
            username,
            password: challenge_ticket,
            otp: None,
            realm: None,
            tfa_challenge: Some(tfa_response),
        };

        let url = self.url("/access/ticket");
        let response = self.http.post(&url).json(&req).send().await?;
        let ticket_resp: TicketResponse = Self::parse_response(response, "login_tfa").await?;

        match (&ticket_resp.ticket, &ticket_resp.csrf_prevention_token) {
            (Some(ticket), Some(csrf)) => {
                self.set_ticket_auth(ticket, csrf);
                Ok(ticket_resp)
            }
            _ => Err(Error::LoginFailed(
                "TFA verification did not return a valid ticket".to_owned(),
            )),
        }
    }

    /// Changes a user's password.
    pub async fn change_password(&self, userid: &str, password: &str) -> Result<()> {
        crate::validation::validate_user_id(userid)?;
        let req = PasswordChangeRequest { userid, password };
        let response = self.put("/access/password")?.json(&req).send().await?;
        Self::handle_error(response, "password change").await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ticket_response_deserialize() {
        let json = r#"{
            "ticket": "PVE:root@pam:ABCDEF123456::...",
            "CSRFPreventionToken": "6A47B28E:csrf-token-value",
            "username": "root@pam",
            "clustername": "mycluster"
        }"#;
        let resp: TicketResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.username.as_deref(), Some("root@pam"));
        assert!(resp.ticket.is_some());
        assert!(resp.csrf_prevention_token.is_some());
        assert!(!resp.is_tfa_challenge());
    }

    #[test]
    fn ticket_response_tfa_challenge() {
        let json = r#"{
            "ticket": "PVE:tfa-challenge:abcdef123456::...",
            "CSRFPreventionToken": null,
            "username": "root@pam"
        }"#;
        let resp: TicketResponse = serde_json::from_str(json).unwrap();
        assert!(resp.is_tfa_challenge());
    }

    #[test]
    fn ticket_response_no_ticket_not_tfa() {
        let json = r#"{}"#;
        let resp: TicketResponse = serde_json::from_str(json).unwrap();
        assert!(!resp.is_tfa_challenge());
    }

    #[test]
    fn ticket_request_serialization() {
        let req = TicketRequest {
            username: "root@pam",
            password: "secret",
            otp: None,
            realm: None,
            tfa_challenge: None,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["username"], "root@pam");
        assert_eq!(json["password"], "secret");
        assert!(!json.as_object().unwrap().contains_key("otp"));
        assert!(!json.as_object().unwrap().contains_key("realm"));
    }

    #[test]
    fn ticket_request_debug_redacts_password() {
        let req = TicketRequest {
            username: "root@pam",
            password: "super-secret-password",
            otp: None,
            realm: None,
            tfa_challenge: None,
        };
        let debug = format!("{req:?}");
        assert!(
            !debug.contains("super-secret-password"),
            "password must be redacted in Debug output"
        );
        assert!(debug.contains("<redacted>"));
    }

    #[test]
    fn password_change_request_debug_redacts_password() {
        let req = PasswordChangeRequest {
            userid: "root@pam",
            password: "new-secret-password",
        };
        let debug = format!("{req:?}");
        assert!(
            !debug.contains("new-secret-password"),
            "password must be redacted in Debug output"
        );
        assert!(debug.contains("<redacted>"));
    }
}
