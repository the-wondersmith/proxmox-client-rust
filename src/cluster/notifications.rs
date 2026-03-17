use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::validate_resource_id;

/// A sendmail notification endpoint.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SendmailEndpoint {
    /// Endpoint name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Recipient email addresses (comma-separated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailto: Option<String>,

    /// Recipient user IDs (comma-separated).
    #[serde(rename = "mailto-user", skip_serializing_if = "Option::is_none")]
    pub mailto_user: Option<String>,

    /// Sender email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,

    /// Author name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Whether the endpoint is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,

    /// Origin (where the config comes from).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,

    /// Digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

/// Parameters for creating a sendmail endpoint.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct SendmailEndpointCreateParams {
    /// Endpoint name (required).
    pub name: String,

    /// Recipient email addresses (comma-separated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailto: Option<String>,

    /// Recipient user IDs (comma-separated).
    #[serde(rename = "mailto-user", skip_serializing_if = "Option::is_none")]
    pub mailto_user: Option<String>,

    /// Sender email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,

    /// Author name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Whether the endpoint is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,
}

impl SendmailEndpointCreateParams {
    /// Creates new parameters with required fields.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
}

/// Parameters for updating a sendmail endpoint.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct SendmailEndpointUpdateParams {
    /// Recipient email addresses (comma-separated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailto: Option<String>,

    /// Recipient user IDs (comma-separated).
    #[serde(rename = "mailto-user", skip_serializing_if = "Option::is_none")]
    pub mailto_user: Option<String>,

    /// Sender email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,

    /// Author name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Whether the endpoint is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,

    /// Comma-separated list of properties to delete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<String>,

    /// Digest for change detection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

/// A Gotify notification endpoint.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GotifyEndpoint {
    /// Endpoint name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Gotify server URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Whether the endpoint is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,

    /// Origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,

    /// Digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

/// Parameters for creating a Gotify endpoint.
#[derive(Clone, Default, Serialize)]
#[non_exhaustive]
pub struct GotifyEndpointCreateParams {
    /// Endpoint name (required).
    pub name: String,

    /// Gotify server URL (required).
    pub server: String,

    /// Gotify application token (required).
    pub token: String,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Whether the endpoint is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,
}

impl GotifyEndpointCreateParams {
    /// Creates new parameters with required fields.
    pub fn new(
        name: impl Into<String>,
        server: impl Into<String>,
        token: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            server: server.into(),
            token: token.into(),
            ..Default::default()
        }
    }
}

impl std::fmt::Debug for GotifyEndpointCreateParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GotifyEndpointCreateParams")
            .field("name", &self.name)
            .field("server", &self.server)
            .field("token", &"<redacted>")
            .field("comment", &self.comment)
            .field("disable", &self.disable)
            .finish()
    }
}

/// Parameters for updating a Gotify endpoint.
#[derive(Clone, Default, Serialize)]
#[non_exhaustive]
pub struct GotifyEndpointUpdateParams {
    /// Gotify server URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,

    /// Gotify application token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Whether the endpoint is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,

    /// Comma-separated list of properties to delete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<String>,

    /// Digest for change detection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

impl std::fmt::Debug for GotifyEndpointUpdateParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GotifyEndpointUpdateParams")
            .field("server", &self.server)
            .field("token", &"<redacted>")
            .field("comment", &self.comment)
            .field("disable", &self.disable)
            .field("delete", &self.delete)
            .field("digest", &self.digest)
            .finish()
    }
}

/// An SMTP notification endpoint.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SmtpEndpoint {
    /// Endpoint name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// SMTP server hostname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,

    /// SMTP server port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

    /// SMTP mode (insecure, starttls, tls).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// SMTP username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Sender email address.
    #[serde(rename = "from-address", skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,

    /// Recipient email addresses (comma-separated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailto: Option<String>,

    /// Recipient user IDs (comma-separated).
    #[serde(rename = "mailto-user", skip_serializing_if = "Option::is_none")]
    pub mailto_user: Option<String>,

    /// Author name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Whether the endpoint is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,

    /// Origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,

    /// Digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

/// Parameters for creating an SMTP endpoint.
#[derive(Clone, Default, Serialize)]
#[non_exhaustive]
pub struct SmtpEndpointCreateParams {
    /// Endpoint name (required).
    pub name: String,

    /// SMTP server hostname (required).
    pub server: String,

    /// Sender email address (required).
    #[serde(rename = "from-address")]
    pub from_address: String,

    /// SMTP server port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

    /// SMTP mode (insecure, starttls, tls).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// SMTP username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// SMTP password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// Recipient email addresses (comma-separated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailto: Option<String>,

    /// Recipient user IDs (comma-separated).
    #[serde(rename = "mailto-user", skip_serializing_if = "Option::is_none")]
    pub mailto_user: Option<String>,

    /// Author name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Whether the endpoint is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,
}

impl SmtpEndpointCreateParams {
    /// Creates new parameters with required fields.
    pub fn new(
        name: impl Into<String>,
        server: impl Into<String>,
        from_address: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            server: server.into(),
            from_address: from_address.into(),
            ..Default::default()
        }
    }
}

impl std::fmt::Debug for SmtpEndpointCreateParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SmtpEndpointCreateParams")
            .field("name", &self.name)
            .field("server", &self.server)
            .field("from_address", &self.from_address)
            .field("port", &self.port)
            .field("mode", &self.mode)
            .field("username", &self.username)
            .field("password", &"<redacted>")
            .field("mailto", &self.mailto)
            .field("mailto_user", &self.mailto_user)
            .field("author", &self.author)
            .field("comment", &self.comment)
            .field("disable", &self.disable)
            .finish()
    }
}

/// Parameters for updating an SMTP endpoint.
#[derive(Clone, Default, Serialize)]
#[non_exhaustive]
pub struct SmtpEndpointUpdateParams {
    /// SMTP server hostname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,

    /// SMTP server port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

    /// SMTP mode (insecure, starttls, tls).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// SMTP username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// SMTP password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// Sender email address.
    #[serde(rename = "from-address", skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,

    /// Recipient email addresses (comma-separated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailto: Option<String>,

    /// Recipient user IDs (comma-separated).
    #[serde(rename = "mailto-user", skip_serializing_if = "Option::is_none")]
    pub mailto_user: Option<String>,

    /// Author name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Whether the endpoint is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,

    /// Comma-separated list of properties to delete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<String>,

    /// Digest for change detection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

impl std::fmt::Debug for SmtpEndpointUpdateParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SmtpEndpointUpdateParams")
            .field("server", &self.server)
            .field("port", &self.port)
            .field("mode", &self.mode)
            .field("username", &self.username)
            .field("password", &"<redacted>")
            .field("from_address", &self.from_address)
            .field("mailto", &self.mailto)
            .field("mailto_user", &self.mailto_user)
            .field("author", &self.author)
            .field("comment", &self.comment)
            .field("disable", &self.disable)
            .field("delete", &self.delete)
            .field("digest", &self.digest)
            .finish()
    }
}

/// A webhook notification endpoint.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WebhookEndpoint {
    /// Endpoint name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Webhook URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// HTTP method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,

    /// Request headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,

    /// Request body template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Whether the endpoint is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,

    /// Origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,

    /// Digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

/// Parameters for creating a webhook endpoint.
#[derive(Clone, Default, Serialize)]
#[non_exhaustive]
pub struct WebhookEndpointCreateParams {
    /// Endpoint name (required).
    pub name: String,

    /// Webhook URL (required).
    pub url: String,

    /// HTTP method (required).
    pub method: String,

    /// Request headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,

    /// Request body template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    /// Secret value for template use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Whether the endpoint is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,
}

impl WebhookEndpointCreateParams {
    /// Creates new parameters with required fields.
    pub fn new(name: impl Into<String>, url: impl Into<String>, method: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
            method: method.into(),
            ..Default::default()
        }
    }
}

impl std::fmt::Debug for WebhookEndpointCreateParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebhookEndpointCreateParams")
            .field("name", &self.name)
            .field("url", &self.url)
            .field("method", &self.method)
            .field("header", &self.header)
            .field("body", &self.body)
            .field("secret", &"<redacted>")
            .field("comment", &self.comment)
            .field("disable", &self.disable)
            .finish()
    }
}

/// Parameters for updating a webhook endpoint.
#[derive(Clone, Default, Serialize)]
#[non_exhaustive]
pub struct WebhookEndpointUpdateParams {
    /// Webhook URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// HTTP method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,

    /// Request headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,

    /// Request body template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    /// Secret value for template use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Whether the endpoint is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,

    /// Comma-separated list of properties to delete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<String>,

    /// Digest for change detection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

impl std::fmt::Debug for WebhookEndpointUpdateParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebhookEndpointUpdateParams")
            .field("url", &self.url)
            .field("method", &self.method)
            .field("header", &self.header)
            .field("body", &self.body)
            .field("secret", &"<redacted>")
            .field("comment", &self.comment)
            .field("disable", &self.disable)
            .field("delete", &self.delete)
            .field("digest", &self.digest)
            .finish()
    }
}

/// A notification matcher.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NotificationMatcher {
    /// Matcher name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Target endpoints (comma-separated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Match severity levels.
    #[serde(rename = "match-severity", skip_serializing_if = "Option::is_none")]
    pub match_severity: Option<String>,

    /// Match calendar events.
    #[serde(rename = "match-calendar", skip_serializing_if = "Option::is_none")]
    pub match_calendar: Option<String>,

    /// Match fields.
    #[serde(rename = "match-field", skip_serializing_if = "Option::is_none")]
    pub match_field: Option<String>,

    /// Whether to invert the match result.
    #[serde(default, rename = "invert-match", skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub invert_match: Option<bool>,

    /// Mode (all, any).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Whether the matcher is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,

    /// Origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,

    /// Digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

/// Parameters for creating a notification matcher.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct NotificationMatcherCreateParams {
    /// Matcher name (required).
    pub name: String,

    /// Target endpoints (comma-separated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Match severity levels.
    #[serde(rename = "match-severity", skip_serializing_if = "Option::is_none")]
    pub match_severity: Option<String>,

    /// Match calendar events.
    #[serde(rename = "match-calendar", skip_serializing_if = "Option::is_none")]
    pub match_calendar: Option<String>,

    /// Match fields.
    #[serde(rename = "match-field", skip_serializing_if = "Option::is_none")]
    pub match_field: Option<String>,

    /// Whether to invert the match result.
    #[serde(default, rename = "invert-match", skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub invert_match: Option<bool>,

    /// Mode (all, any).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Whether the matcher is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,
}

impl NotificationMatcherCreateParams {
    /// Creates new parameters with required fields.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
}

/// Parameters for updating a notification matcher.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct NotificationMatcherUpdateParams {
    /// Target endpoints (comma-separated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Match severity levels.
    #[serde(rename = "match-severity", skip_serializing_if = "Option::is_none")]
    pub match_severity: Option<String>,

    /// Match calendar events.
    #[serde(rename = "match-calendar", skip_serializing_if = "Option::is_none")]
    pub match_calendar: Option<String>,

    /// Match fields.
    #[serde(rename = "match-field", skip_serializing_if = "Option::is_none")]
    pub match_field: Option<String>,

    /// Whether to invert the match result.
    #[serde(default, rename = "invert-match", skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub invert_match: Option<bool>,

    /// Mode (all, any).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Whether the matcher is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,

    /// Comma-separated list of properties to delete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<String>,

    /// Digest for change detection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

/// A notification target summary.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NotificationTarget {
    /// Target name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Target type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,

    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Whether the target is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub disable: Option<bool>,

    /// Origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}

impl ProxmoxClient {
    // --- Sendmail Endpoints ---

    /// Lists sendmail notification endpoints.
    ///
    /// `GET /cluster/notifications/endpoints/sendmail`
    pub async fn list_sendmail_endpoints(&self) -> Result<Vec<SendmailEndpoint>> {
        self.get_parsed(
            "/cluster/notifications/endpoints/sendmail",
            "sendmail endpoints",
        )
        .await
    }

    /// Creates a sendmail notification endpoint.
    ///
    /// `POST /cluster/notifications/endpoints/sendmail`
    pub async fn create_sendmail_endpoint(
        &self,
        params: &SendmailEndpointCreateParams,
    ) -> Result<()> {
        let response = self
            .post("/cluster/notifications/endpoints/sendmail")?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, "sendmail endpoint creation").await?;
        Ok(())
    }

    /// Gets a specific sendmail endpoint.
    ///
    /// `GET /cluster/notifications/endpoints/sendmail/{name}`
    pub async fn get_sendmail_endpoint(&self, name: &str) -> Result<SendmailEndpoint> {
        validate_resource_id(name)?;
        self.get_parsed(
            &format!("/cluster/notifications/endpoints/sendmail/{name}"),
            &format!("sendmail endpoint {name}"),
        )
        .await
    }

    /// Updates a sendmail endpoint.
    ///
    /// `PUT /cluster/notifications/endpoints/sendmail/{name}`
    pub async fn update_sendmail_endpoint(
        &self,
        name: &str,
        params: &SendmailEndpointUpdateParams,
    ) -> Result<()> {
        validate_resource_id(name)?;
        let response = self
            .put(&format!("/cluster/notifications/endpoints/sendmail/{name}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("sendmail endpoint {name}")).await?;
        Ok(())
    }

    /// Deletes a sendmail endpoint.
    ///
    /// `DELETE /cluster/notifications/endpoints/sendmail/{name}`
    pub async fn delete_sendmail_endpoint(&self, name: &str) -> Result<()> {
        validate_resource_id(name)?;
        self.delete_void(
            &format!("/cluster/notifications/endpoints/sendmail/{name}"),
            &format!("sendmail endpoint {name}"),
        )
        .await
    }

    // --- Gotify Endpoints ---

    /// Lists Gotify notification endpoints.
    ///
    /// `GET /cluster/notifications/endpoints/gotify`
    pub async fn list_gotify_endpoints(&self) -> Result<Vec<GotifyEndpoint>> {
        self.get_parsed(
            "/cluster/notifications/endpoints/gotify",
            "Gotify endpoints",
        )
        .await
    }

    /// Creates a Gotify notification endpoint.
    ///
    /// `POST /cluster/notifications/endpoints/gotify`
    pub async fn create_gotify_endpoint(&self, params: &GotifyEndpointCreateParams) -> Result<()> {
        let response = self
            .post("/cluster/notifications/endpoints/gotify")?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, "Gotify endpoint creation").await?;
        Ok(())
    }

    /// Gets a specific Gotify endpoint.
    ///
    /// `GET /cluster/notifications/endpoints/gotify/{name}`
    pub async fn get_gotify_endpoint(&self, name: &str) -> Result<GotifyEndpoint> {
        validate_resource_id(name)?;
        self.get_parsed(
            &format!("/cluster/notifications/endpoints/gotify/{name}"),
            &format!("Gotify endpoint {name}"),
        )
        .await
    }

    /// Updates a Gotify endpoint.
    ///
    /// `PUT /cluster/notifications/endpoints/gotify/{name}`
    pub async fn update_gotify_endpoint(
        &self,
        name: &str,
        params: &GotifyEndpointUpdateParams,
    ) -> Result<()> {
        validate_resource_id(name)?;
        let response = self
            .put(&format!("/cluster/notifications/endpoints/gotify/{name}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("Gotify endpoint {name}")).await?;
        Ok(())
    }

    /// Deletes a Gotify endpoint.
    ///
    /// `DELETE /cluster/notifications/endpoints/gotify/{name}`
    pub async fn delete_gotify_endpoint(&self, name: &str) -> Result<()> {
        validate_resource_id(name)?;
        self.delete_void(
            &format!("/cluster/notifications/endpoints/gotify/{name}"),
            &format!("Gotify endpoint {name}"),
        )
        .await
    }

    // --- SMTP Endpoints ---

    /// Lists SMTP notification endpoints.
    ///
    /// `GET /cluster/notifications/endpoints/smtp`
    pub async fn list_smtp_endpoints(&self) -> Result<Vec<SmtpEndpoint>> {
        self.get_parsed("/cluster/notifications/endpoints/smtp", "SMTP endpoints")
            .await
    }

    /// Creates an SMTP notification endpoint.
    ///
    /// `POST /cluster/notifications/endpoints/smtp`
    pub async fn create_smtp_endpoint(&self, params: &SmtpEndpointCreateParams) -> Result<()> {
        let response = self
            .post("/cluster/notifications/endpoints/smtp")?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, "SMTP endpoint creation").await?;
        Ok(())
    }

    /// Gets a specific SMTP endpoint.
    ///
    /// `GET /cluster/notifications/endpoints/smtp/{name}`
    pub async fn get_smtp_endpoint(&self, name: &str) -> Result<SmtpEndpoint> {
        validate_resource_id(name)?;
        self.get_parsed(
            &format!("/cluster/notifications/endpoints/smtp/{name}"),
            &format!("SMTP endpoint {name}"),
        )
        .await
    }

    /// Updates an SMTP endpoint.
    ///
    /// `PUT /cluster/notifications/endpoints/smtp/{name}`
    pub async fn update_smtp_endpoint(&self, name: &str, params: &SmtpEndpointUpdateParams) -> Result<()> {
        validate_resource_id(name)?;
        let response = self
            .put(&format!("/cluster/notifications/endpoints/smtp/{name}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("SMTP endpoint {name}")).await?;
        Ok(())
    }

    /// Deletes an SMTP endpoint.
    ///
    /// `DELETE /cluster/notifications/endpoints/smtp/{name}`
    pub async fn delete_smtp_endpoint(&self, name: &str) -> Result<()> {
        validate_resource_id(name)?;
        self.delete_void(
            &format!("/cluster/notifications/endpoints/smtp/{name}"),
            &format!("SMTP endpoint {name}"),
        )
        .await
    }

    // --- Webhook Endpoints ---

    /// Lists webhook notification endpoints.
    ///
    /// `GET /cluster/notifications/endpoints/webhook`
    pub async fn list_webhook_endpoints(&self) -> Result<Vec<WebhookEndpoint>> {
        self.get_parsed(
            "/cluster/notifications/endpoints/webhook",
            "webhook endpoints",
        )
        .await
    }

    /// Creates a webhook notification endpoint.
    ///
    /// `POST /cluster/notifications/endpoints/webhook`
    pub async fn create_webhook_endpoint(
        &self,
        params: &WebhookEndpointCreateParams,
    ) -> Result<()> {
        let response = self
            .post("/cluster/notifications/endpoints/webhook")?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, "webhook endpoint creation").await?;
        Ok(())
    }

    /// Gets a specific webhook endpoint.
    ///
    /// `GET /cluster/notifications/endpoints/webhook/{name}`
    pub async fn get_webhook_endpoint(&self, name: &str) -> Result<WebhookEndpoint> {
        validate_resource_id(name)?;
        self.get_parsed(
            &format!("/cluster/notifications/endpoints/webhook/{name}"),
            &format!("webhook endpoint {name}"),
        )
        .await
    }

    /// Updates a webhook endpoint.
    ///
    /// `PUT /cluster/notifications/endpoints/webhook/{name}`
    pub async fn update_webhook_endpoint(
        &self,
        name: &str,
        params: &WebhookEndpointUpdateParams,
    ) -> Result<()> {
        validate_resource_id(name)?;
        let response = self
            .put(&format!("/cluster/notifications/endpoints/webhook/{name}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("webhook endpoint {name}")).await?;
        Ok(())
    }

    /// Deletes a webhook endpoint.
    ///
    /// `DELETE /cluster/notifications/endpoints/webhook/{name}`
    pub async fn delete_webhook_endpoint(&self, name: &str) -> Result<()> {
        validate_resource_id(name)?;
        self.delete_void(
            &format!("/cluster/notifications/endpoints/webhook/{name}"),
            &format!("webhook endpoint {name}"),
        )
        .await
    }

    // --- Notification Matchers ---

    /// Lists notification matchers.
    ///
    /// `GET /cluster/notifications/matchers`
    pub async fn list_notification_matchers(&self) -> Result<Vec<NotificationMatcher>> {
        self.get_parsed("/cluster/notifications/matchers", "notification matchers")
            .await
    }

    /// Creates a notification matcher.
    ///
    /// `POST /cluster/notifications/matchers`
    pub async fn create_notification_matcher(
        &self,
        params: &NotificationMatcherCreateParams,
    ) -> Result<()> {
        let response = self
            .post("/cluster/notifications/matchers")?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, "notification matcher creation").await?;
        Ok(())
    }

    /// Gets a specific notification matcher.
    ///
    /// `GET /cluster/notifications/matchers/{name}`
    pub async fn get_notification_matcher(&self, name: &str) -> Result<NotificationMatcher> {
        validate_resource_id(name)?;
        self.get_parsed(
            &format!("/cluster/notifications/matchers/{name}"),
            &format!("notification matcher {name}"),
        )
        .await
    }

    /// Updates a notification matcher.
    ///
    /// `PUT /cluster/notifications/matchers/{name}`
    pub async fn update_notification_matcher(
        &self,
        name: &str,
        params: &NotificationMatcherUpdateParams,
    ) -> Result<()> {
        validate_resource_id(name)?;
        let response = self
            .put(&format!("/cluster/notifications/matchers/{name}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("notification matcher {name}")).await?;
        Ok(())
    }

    /// Deletes a notification matcher.
    ///
    /// `DELETE /cluster/notifications/matchers/{name}`
    pub async fn delete_notification_matcher(&self, name: &str) -> Result<()> {
        validate_resource_id(name)?;
        self.delete_void(
            &format!("/cluster/notifications/matchers/{name}"),
            &format!("notification matcher {name}"),
        )
        .await
    }

    // --- Notification Targets ---

    /// Lists notification targets (aggregated view of all endpoints).
    ///
    /// `GET /cluster/notifications/targets`
    pub async fn list_notification_targets(&self) -> Result<Vec<NotificationTarget>> {
        self.get_parsed("/cluster/notifications/targets", "notification targets")
            .await
    }

    /// Sends a test notification to a target.
    ///
    /// `POST /cluster/notifications/targets/{name}/test`
    pub async fn test_notification_target(&self, name: &str) -> Result<()> {
        validate_resource_id(name)?;
        let response = self
            .post(&format!("/cluster/notifications/targets/{name}/test"))?
            .send()
            .await?;
        Self::handle_error(response, &format!("notification target {name} test")).await?;
        Ok(())
    }

    /// Lists available notification matcher fields.
    ///
    /// `GET /cluster/notifications/matcher-fields`
    pub async fn list_notification_matcher_fields(&self) -> Result<Vec<serde_json::Value>> {
        self.get_parsed(
            "/cluster/notifications/matcher-fields",
            "notification matcher fields",
        )
        .await
    }

    /// Lists available values for notification matcher fields.
    ///
    /// `GET /cluster/notifications/matcher-field-values`
    pub async fn list_notification_matcher_field_values(&self) -> Result<Vec<serde_json::Value>> {
        self.get_parsed(
            "/cluster/notifications/matcher-field-values",
            "notification matcher field values",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sendmail_endpoint_serde_roundtrip() {
        let endpoint = SendmailEndpoint {
            name: Some("mail1".to_string()),
            mailto: Some("admin@example.com".to_string()),
            mailto_user: Some("root@pam".to_string()),
            from_address: Some("pve@example.com".to_string()),
            author: Some("Proxmox VE".to_string()),
            comment: Some("Default mail".to_string()),
            disable: Some(false),
            origin: None,
            digest: None,
        };

        let json = serde_json::to_string(&endpoint).unwrap();
        let deserialized: SendmailEndpoint = serde_json::from_str(&json).unwrap();
        assert_eq!(endpoint, deserialized);
    }

    #[test]
    fn sendmail_endpoint_skip_serializing_none() {
        let endpoint = SendmailEndpoint::default();
        let json = serde_json::to_value(&endpoint).unwrap();
        let obj = json.as_object().unwrap();
        assert!(
            obj.is_empty(),
            "default SendmailEndpoint should serialize to {{}}"
        );
    }

    #[test]
    fn sendmail_endpoint_mailto_user_rename() {
        let json = r#"{"name": "test", "mailto-user": "root@pam"}"#;
        let endpoint: SendmailEndpoint = serde_json::from_str(json).unwrap();
        assert_eq!(endpoint.mailto_user.as_deref(), Some("root@pam"));

        let serialized = serde_json::to_value(&endpoint).unwrap();
        assert!(serialized.get("mailto-user").is_some());
        assert!(serialized.get("mailto_user").is_none());
    }

    #[test]
    fn sendmail_create_params_serialization() {
        let mut params = SendmailEndpointCreateParams::new("mail1");
        params.mailto = Some("admin@example.com".to_string());
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["name"], "mail1");
        assert_eq!(json["mailto"], "admin@example.com");
        assert!(!json.as_object().unwrap().contains_key("comment"));
    }

    #[test]
    fn gotify_endpoint_serde_roundtrip() {
        let endpoint = GotifyEndpoint {
            name: Some("gotify1".to_string()),
            server: Some("https://gotify.example.com".to_string()),
            comment: Some("Gotify notifications".to_string()),
            disable: Some(false),
            origin: None,
            digest: None,
        };

        let json = serde_json::to_string(&endpoint).unwrap();
        let deserialized: GotifyEndpoint = serde_json::from_str(&json).unwrap();
        assert_eq!(endpoint, deserialized);
    }

    #[test]
    fn gotify_create_params_serialization() {
        let params = GotifyEndpointCreateParams {
            name: "gotify1".to_string(),
            server: "https://gotify.example.com".to_string(),
            token: "mytoken".to_string(),
            comment: None,
            disable: None,
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["name"], "gotify1");
        assert_eq!(json["server"], "https://gotify.example.com");
        assert_eq!(json["token"], "mytoken");
    }

    #[test]
    fn smtp_endpoint_serde_roundtrip() {
        let endpoint = SmtpEndpoint {
            name: Some("smtp1".to_string()),
            server: Some("smtp.example.com".to_string()),
            port: Some(587),
            mode: Some("starttls".to_string()),
            username: Some("user@example.com".to_string()),
            from_address: Some("pve@example.com".to_string()),
            mailto: Some("admin@example.com".to_string()),
            mailto_user: None,
            author: None,
            comment: None,
            disable: Some(false),
            origin: None,
            digest: None,
        };

        let json = serde_json::to_string(&endpoint).unwrap();
        let deserialized: SmtpEndpoint = serde_json::from_str(&json).unwrap();
        assert_eq!(endpoint, deserialized);
    }

    #[test]
    fn smtp_endpoint_from_address_rename() {
        let json = r#"{"name": "test", "from-address": "pve@example.com"}"#;
        let endpoint: SmtpEndpoint = serde_json::from_str(json).unwrap();
        assert_eq!(endpoint.from_address.as_deref(), Some("pve@example.com"));

        let serialized = serde_json::to_value(&endpoint).unwrap();
        assert!(serialized.get("from-address").is_some());
    }

    #[test]
    fn smtp_create_params_serialization() {
        let mut params =
            SmtpEndpointCreateParams::new("smtp1", "smtp.example.com", "pve@example.com");
        params.port = Some(587);
        params.mode = Some("starttls".to_string());
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["name"], "smtp1");
        assert_eq!(json["from-address"], "pve@example.com");
        assert_eq!(json["port"], 587);
    }

    #[test]
    fn webhook_endpoint_serde_roundtrip() {
        let endpoint = WebhookEndpoint {
            name: Some("slack1".to_string()),
            url: Some("https://hooks.slack.com/services/xxx".to_string()),
            method: Some("POST".to_string()),
            header: None,
            body: Some(r#"{"text": "{{message}}"}"#.to_string()),
            comment: Some("Slack webhook".to_string()),
            disable: Some(false),
            origin: None,
            digest: None,
        };

        let json = serde_json::to_string(&endpoint).unwrap();
        let deserialized: WebhookEndpoint = serde_json::from_str(&json).unwrap();
        assert_eq!(endpoint, deserialized);
    }

    #[test]
    fn webhook_create_params_serialization() {
        let mut params = WebhookEndpointCreateParams::new(
            "slack1",
            "https://hooks.slack.com/services/xxx",
            "POST",
        );
        params.body = Some(r#"{"text": "test"}"#.to_string());
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["name"], "slack1");
        assert_eq!(json["url"], "https://hooks.slack.com/services/xxx");
        assert_eq!(json["method"], "POST");
        assert!(!json.as_object().unwrap().contains_key("secret"));
    }

    #[test]
    fn notification_matcher_serde_roundtrip() {
        let matcher = NotificationMatcher {
            name: Some("default-matcher".to_string()),
            target: Some("mail1".to_string()),
            match_severity: Some("error,warning".to_string()),
            match_calendar: None,
            match_field: None,
            invert_match: Some(false),
            mode: Some("all".to_string()),
            comment: Some("Match errors and warnings".to_string()),
            disable: Some(false),
            origin: None,
            digest: None,
        };

        let json = serde_json::to_string(&matcher).unwrap();
        let deserialized: NotificationMatcher = serde_json::from_str(&json).unwrap();
        assert_eq!(matcher, deserialized);
    }

    #[test]
    fn notification_matcher_hyphenated_fields() {
        let json = r#"{"name": "test", "match-severity": "error", "match-field": "type=vzdump", "invert-match": 0}"#;
        let matcher: NotificationMatcher = serde_json::from_str(json).unwrap();
        assert_eq!(matcher.match_severity.as_deref(), Some("error"));
        assert_eq!(matcher.match_field.as_deref(), Some("type=vzdump"));
        assert_eq!(matcher.invert_match, Some(false));

        let serialized = serde_json::to_value(&matcher).unwrap();
        assert!(serialized.get("match-severity").is_some());
        assert!(serialized.get("match-field").is_some());
        assert!(serialized.get("invert-match").is_some());
    }

    #[test]
    fn notification_matcher_create_params_serialization() {
        let mut params = NotificationMatcherCreateParams::new("matcher1");
        params.target = Some("mail1".to_string());
        params.match_severity = Some("error".to_string());
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["name"], "matcher1");
        assert_eq!(json["target"], "mail1");
        assert_eq!(json["match-severity"], "error");
        assert!(!json.as_object().unwrap().contains_key("mode"));
    }

    #[test]
    fn notification_target_serde_roundtrip() {
        let target = NotificationTarget {
            name: Some("mail-to-root".to_string()),
            target_type: Some("sendmail".to_string()),
            comment: Some("Send to root".to_string()),
            disable: Some(false),
            origin: Some("builtin".to_string()),
        };

        let json = serde_json::to_string(&target).unwrap();
        let deserialized: NotificationTarget = serde_json::from_str(&json).unwrap();
        assert_eq!(target, deserialized);
    }

    #[test]
    fn notification_target_type_rename() {
        let json = r#"{"type": "sendmail", "name": "mail1"}"#;
        let target: NotificationTarget = serde_json::from_str(json).unwrap();
        assert_eq!(target.target_type.as_deref(), Some("sendmail"));

        let serialized = serde_json::to_value(&target).unwrap();
        assert!(serialized.get("type").is_some());
        assert!(serialized.get("target_type").is_none());
    }

    #[test]
    fn sendmail_endpoint_unknown_fields_ignored() {
        let json = r#"{"name": "test", "unknownField": true}"#;
        let endpoint: SendmailEndpoint = serde_json::from_str(json).unwrap();
        assert_eq!(endpoint.name.as_deref(), Some("test"));
    }

    #[test]
    fn gotify_create_params_debug_redacts_token() {
        let params = GotifyEndpointCreateParams::new(
            "gotify1",
            "https://gotify.example.com",
            "super-secret-token",
        );
        let debug = format!("{params:?}");
        assert!(
            !debug.contains("super-secret-token"),
            "token must be redacted in Debug output"
        );
        assert!(debug.contains("<redacted>"));
    }

    #[test]
    fn smtp_create_params_debug_redacts_password() {
        let mut params =
            SmtpEndpointCreateParams::new("smtp1", "smtp.example.com", "pve@example.com");
        params.password = Some("smtp-password".to_string());
        let debug = format!("{params:?}");
        assert!(
            !debug.contains("smtp-password"),
            "password must be redacted in Debug output"
        );
        assert!(debug.contains("<redacted>"));
    }

    #[test]
    fn webhook_create_params_debug_redacts_secret() {
        let mut params =
            WebhookEndpointCreateParams::new("hook1", "https://example.com/hook", "POST");
        params.secret = Some("webhook-secret-value".to_string());
        let debug = format!("{params:?}");
        assert!(
            !debug.contains("webhook-secret-value"),
            "secret must be redacted in Debug output"
        );
        assert!(debug.contains("<redacted>"));
    }
}
