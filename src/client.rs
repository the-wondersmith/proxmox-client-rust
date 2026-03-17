use std::fmt;
use std::sync::RwLock;
use std::time::Duration;

use reqwest::{Client, StatusCode, header};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::error::{Error, Result};
use crate::types::ProxmoxResponse;

/// Authentication state for the Proxmox client.
#[derive(Clone)]
pub(crate) enum AuthState {
    /// No authentication configured.
    None,
    /// API token authentication (`PVEAPIToken=USER@REALM!TOKENID=UUID`).
    ApiToken {
        /// The full authorization header value.
        header_value: String,
    },
    /// Ticket-based authentication (session cookie + CSRF token).
    Ticket {
        /// The authentication ticket (cookie value).
        ticket: String,
        /// The CSRF prevention token for write operations.
        csrf_token: String,
    },
}

/// Builder for constructing a [`ProxmoxClient`] with custom configuration.
pub struct ProxmoxClientBuilder {
    base_url: String,
    auth: AuthState,
    accept_invalid_certs: bool,
    allow_insecure_http: bool,
    timeout: Duration,
    connect_timeout: Duration,
}

impl ProxmoxClientBuilder {
    /// Sets API token authentication.
    ///
    /// `token_id` should be in the format `user@realm!tokenname`.
    /// `secret` is the UUID token value.
    #[must_use]
    pub fn api_token(mut self, token_id: &str, secret: &str) -> Self {
        self.auth = AuthState::ApiToken {
            header_value: format!("PVEAPIToken={token_id}={secret}"),
        };
        self
    }

    /// Accept invalid TLS certificates (common with self-signed Proxmox certs).
    #[must_use]
    pub fn accept_invalid_certs(mut self, accept: bool) -> Self {
        self.accept_invalid_certs = accept;
        self
    }

    /// Sets the request timeout (default: 30 seconds).
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the connection timeout (default: 10 seconds).
    #[must_use]
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// Allows insecure HTTP connections (default: `false`).
    ///
    /// By default, only HTTPS URLs are accepted to prevent accidental
    /// transmission of credentials in plaintext. Call this with `true`
    /// if you intentionally need to use HTTP (e.g., local development).
    #[must_use]
    pub fn allow_insecure_http(mut self, allow: bool) -> Self {
        self.allow_insecure_http = allow;
        self
    }

    /// Builds the [`ProxmoxClient`].
    pub fn build(self) -> Result<ProxmoxClient> {
        let base_url = self.base_url.trim_end_matches('/').to_owned();

        if base_url.is_empty() {
            return Err(Error::InvalidBaseUrl("empty URL".to_owned()));
        }

        if self.allow_insecure_http {
            if !base_url.starts_with("https://") && !base_url.starts_with("http://") {
                return Err(Error::InvalidBaseUrl(
                    "URL must start with https:// or http://".to_owned(),
                ));
            }
        } else if !base_url.starts_with("https://") {
            return Err(Error::InvalidBaseUrl(
                "URL must start with https:// — use .allow_insecure_http(true) for HTTP".to_owned(),
            ));
        }

        let http = Client::builder()
            .danger_accept_invalid_certs(self.accept_invalid_certs)
            .cookie_store(true)
            .timeout(self.timeout)
            .connect_timeout(self.connect_timeout)
            .build()?;

        Ok(ProxmoxClient {
            http,
            base_url,
            auth: RwLock::new(self.auth),
        })
    }
}

/// Client for the Proxmox VE API.
///
/// Supports two authentication modes:
/// - **API token**: Stateless, preferred for automation
/// - **Ticket**: Session-based, obtained via [`login()`](ProxmoxClient::login)
pub struct ProxmoxClient {
    pub(crate) http: Client,
    pub(crate) base_url: String,
    pub(crate) auth: RwLock<AuthState>,
}

impl fmt::Debug for ProxmoxClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProxmoxClient")
            .field("base_url", &self.base_url)
            .field("http", &"<redacted>")
            .field("auth", &"<redacted>")
            .finish()
    }
}

impl ProxmoxClient {
    /// Creates a new unauthenticated client.
    ///
    /// You must call [`login()`](ProxmoxClient::login) or use [`builder()`](ProxmoxClient::builder)
    /// with `.api_token()` before making API calls.
    pub fn new(base_url: &str) -> Result<Self> {
        Self::builder(base_url).build()
    }

    /// Creates a new client with API token authentication.
    ///
    /// `token_id` should be in the format `user@realm!tokenname`.
    /// `secret` is the UUID token value.
    pub fn with_api_token(base_url: &str, token_id: &str, secret: &str) -> Result<Self> {
        Self::builder(base_url).api_token(token_id, secret).build()
    }

    /// Returns a builder for constructing a client with custom configuration.
    #[must_use]
    pub fn builder(base_url: &str) -> ProxmoxClientBuilder {
        ProxmoxClientBuilder {
            base_url: base_url.to_owned(),
            auth: AuthState::None,
            accept_invalid_certs: false,
            allow_insecure_http: false,
            timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(10),
        }
    }

    /// Builds a full API URL from a path segment.
    ///
    /// Prepends the base URL and `/api2/json` prefix.
    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}/api2/json{path}", self.base_url)
    }

    /// Acquires a read lock on the authentication state, recovering from poison.
    fn read_auth(&self) -> std::sync::RwLockReadGuard<'_, AuthState> {
        self.auth.read().unwrap_or_else(|e| e.into_inner())
    }

    /// Acquires a write lock on the authentication state, recovering from poison.
    fn write_auth(&self) -> std::sync::RwLockWriteGuard<'_, AuthState> {
        self.auth.write().unwrap_or_else(|e| e.into_inner())
    }

    /// Stores ticket authentication state after a successful login.
    pub(crate) fn set_ticket_auth(&self, ticket: &str, csrf_token: &str) {
        *self.write_auth() = AuthState::Ticket {
            ticket: ticket.to_owned(),
            csrf_token: csrf_token.to_owned(),
        };
    }

    /// Returns `true` if the client has authentication configured.
    #[must_use]
    pub fn is_authenticated(&self) -> bool {
        !matches!(*self.read_auth(), AuthState::None)
    }

    /// Prepares a GET request with authentication headers.
    pub(crate) fn get(&self, path: &str) -> Result<reqwest::RequestBuilder> {
        let url = self.url(path);
        let auth = self.read_auth();
        match &*auth {
            AuthState::None => Err(Error::NotAuthenticated),
            AuthState::ApiToken { header_value } => Ok(self
                .http
                .get(&url)
                .header(header::AUTHORIZATION, header_value)),
            AuthState::Ticket { ticket, .. } => Ok(self
                .http
                .get(&url)
                .header(header::COOKIE, format!("PVEAuthCookie={ticket}"))),
        }
    }

    /// Prepares a POST request with authentication and CSRF headers.
    pub(crate) fn post(&self, path: &str) -> Result<reqwest::RequestBuilder> {
        self.mutating_request(|url| self.http.post(url), path)
    }

    /// Prepares a PUT request with authentication and CSRF headers.
    pub(crate) fn put(&self, path: &str) -> Result<reqwest::RequestBuilder> {
        self.mutating_request(|url| self.http.put(url), path)
    }

    /// Prepares a DELETE request with authentication and CSRF headers.
    pub(crate) fn delete(&self, path: &str) -> Result<reqwest::RequestBuilder> {
        self.mutating_request(|url| self.http.delete(url), path)
    }

    /// Common logic for POST/PUT/DELETE — attaches auth + CSRF token.
    fn mutating_request(
        &self,
        method: impl FnOnce(&str) -> reqwest::RequestBuilder,
        path: &str,
    ) -> Result<reqwest::RequestBuilder> {
        let url = self.url(path);
        let auth = self.read_auth();
        match &*auth {
            AuthState::None => Err(Error::NotAuthenticated),
            AuthState::ApiToken { header_value } => {
                Ok(method(&url).header(header::AUTHORIZATION, header_value))
            }
            AuthState::Ticket { ticket, csrf_token } => Ok(method(&url)
                .header(header::COOKIE, format!("PVEAuthCookie={ticket}"))
                .header("CSRFPreventionToken", csrf_token)),
        }
    }

    /// Checks for HTTP errors and deserializes the response body,
    /// unwrapping the Proxmox `{"data": T}` envelope.
    pub(crate) async fn parse_response<T: DeserializeOwned>(
        response: reqwest::Response,
        resource: &str,
    ) -> Result<T> {
        let response = Self::handle_error(response, resource).await?;
        let text = response.text().await?;
        let envelope: ProxmoxResponse<T> = serde_json::from_str(&text)
            .map_err(|e| Error::Deserialization(format!("{e}: {}", truncate(&text, 256))))?;
        if let Some(ref errors) = envelope.errors
            && !errors.is_empty()
        {
            return Err(Error::ApiValidationErrors {
                resource: resource.to_owned(),
                errors: errors.clone(),
            });
        }
        Ok(envelope.data)
    }

    /// Sends a GET request and parses the response, unwrapping the envelope.
    pub(crate) async fn get_parsed<T: DeserializeOwned>(
        &self,
        path: &str,
        resource: &str,
    ) -> Result<T> {
        let response = self.get(path)?.send().await?;
        Self::parse_response(response, resource).await
    }

    /// Sends a POST request with a JSON body and parses the response.
    pub(crate) async fn post_parsed<T: DeserializeOwned, B: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &B,
        resource: &str,
    ) -> Result<T> {
        let response = self.post(path)?.json(body).send().await?;
        Self::parse_response(response, resource).await
    }

    /// Sends a PUT request with a JSON body and parses the response.
    pub(crate) async fn put_parsed<T: DeserializeOwned, B: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &B,
        resource: &str,
    ) -> Result<T> {
        let response = self.put(path)?.json(body).send().await?;
        Self::parse_response(response, resource).await
    }

    /// Sends a POST request with an optional JSON body and parses the response.
    ///
    /// When `body` is `None`, sends an empty POST.
    pub(crate) async fn post_with_optional_body<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: Option<&B>,
        resource: &str,
    ) -> Result<T> {
        match body {
            Some(b) => self.post_parsed(path, b, resource).await,
            None => {
                let response = self.post(path)?.send().await?;
                Self::parse_response(response, resource).await
            }
        }
    }

    /// Sends a DELETE request and checks for errors.
    pub(crate) async fn delete_void(&self, path: &str, resource: &str) -> Result<()> {
        let response = self.delete(path)?.send().await?;
        Self::handle_error(response, resource).await?;
        Ok(())
    }

    /// Sends a GET request and returns the raw response bytes.
    pub(crate) async fn get_bytes(&self, path: &str, resource: &str) -> Result<Vec<u8>> {
        let response = self.get(path)?.send().await?;
        let response = Self::handle_error(response, resource).await?;
        Ok(response.bytes().await?.to_vec())
    }

    /// Builds a URL path with optional query parameters.
    ///
    /// Filters out `None` values and joins remaining key-value pairs with `&`.
    pub(crate) fn build_query_path(base: &str, params: &[(&str, Option<&str>)]) -> String {
        let query: Vec<String> = params
            .iter()
            .filter_map(|(k, v)| v.map(|val| format!("{k}={}", urlencoding::encode(val))))
            .collect();
        if query.is_empty() {
            base.to_string()
        } else {
            format!("{base}?{}", query.join("&"))
        }
    }

    /// Serializes a `Serialize` struct into a URL query string.
    ///
    /// Converts the struct to a JSON object and formats each field as `key=value`.
    /// Booleans are serialized as `0`/`1`. `None` fields (serialized as null) are skipped.
    pub(crate) fn serialize_as_query_string<T: Serialize>(params: &T) -> String {
        let obj =
            serde_json::to_value(params).expect("Serialize impl is infallible for derived types");
        let Some(map) = obj.as_object() else {
            return String::new();
        };
        map.iter()
            .filter_map(|(k, v)| match v {
                serde_json::Value::Bool(b) => Some(format!("{k}={}", *b as u8)),
                serde_json::Value::String(s) => Some(format!("{k}={}", urlencoding::encode(s))),
                serde_json::Value::Number(n) => Some(format!("{k}={n}")),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("&")
    }

    /// Maps HTTP error status codes to typed errors.
    pub(crate) async fn handle_error(
        response: reqwest::Response,
        resource: &str,
    ) -> Result<reqwest::Response> {
        let status = response.status();

        if status.is_success() {
            return Ok(response);
        }

        let retry_after = response
            .headers()
            .get("retry-after")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok())
            .map(Duration::from_secs);

        let body = truncate_owned(response.text().await.unwrap_or_default(), 512);

        match status {
            StatusCode::UNAUTHORIZED => Err(Error::Unauthorized { body }),
            StatusCode::FORBIDDEN => Err(Error::Forbidden { body }),
            StatusCode::NOT_FOUND => Err(Error::NotFound {
                resource: resource.to_owned(),
                body,
            }),
            StatusCode::TOO_MANY_REQUESTS => Err(Error::RateLimited { retry_after }),
            StatusCode::INTERNAL_SERVER_ERROR => Err(Error::InternalServerError { body }),
            _ => Err(Error::Api { status, body }),
        }
    }
}

/// Truncates a string to `max_len` bytes at a char boundary.
fn truncate(s: &str, max_len: usize) -> &str {
    if s.len() <= max_len {
        s
    } else {
        &s[..s.floor_char_boundary(max_len)]
    }
}

/// Truncates an owned string to `max_len` bytes at a char boundary, appending
/// `...<truncated>` if the string was shortened.
fn truncate_owned(mut s: String, max_len: usize) -> String {
    if s.len() > max_len {
        s.truncate(s.floor_char_boundary(max_len));
        s.push_str("...<truncated>");
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<ProxmoxClient>();
    }

    #[test]
    fn debug_does_not_leak_auth() {
        let client =
            ProxmoxClient::with_api_token("https://pve:8006", "root@pam!test", "secret-uuid")
                .unwrap();
        let debug_output = format!("{client:?}");
        assert!(
            !debug_output.contains("secret-uuid"),
            "Debug output must not contain the API token"
        );
        assert!(debug_output.contains("<redacted>"));
    }

    #[test]
    fn url_building() {
        let client = ProxmoxClient::new("https://pve:8006").unwrap();
        assert_eq!(
            client.url("/nodes/pve1/status"),
            "https://pve:8006/api2/json/nodes/pve1/status"
        );
    }

    #[test]
    fn url_building_trailing_slash() {
        let client = ProxmoxClient::new("https://pve:8006/").unwrap();
        assert_eq!(client.url("/version"), "https://pve:8006/api2/json/version");
    }

    #[test]
    fn builder_default_config() {
        let client = ProxmoxClient::builder("https://pve:8006")
            .api_token("root@pam!test", "uuid-value")
            .accept_invalid_certs(true)
            .build()
            .unwrap();
        assert_eq!(client.base_url, "https://pve:8006");
        assert!(client.is_authenticated());
    }

    #[test]
    fn new_creates_unauthenticated_client() {
        let client = ProxmoxClient::new("https://pve:8006").unwrap();
        assert!(!client.is_authenticated());
    }

    #[test]
    fn with_api_token_creates_authenticated_client() {
        let client =
            ProxmoxClient::with_api_token("https://pve:8006", "root@pam!test", "uuid").unwrap();
        assert!(client.is_authenticated());
    }

    #[test]
    fn empty_base_url_rejected() {
        assert!(matches!(
            ProxmoxClient::new(""),
            Err(Error::InvalidBaseUrl(_))
        ));
    }

    #[test]
    fn unauthenticated_get_returns_error() {
        let client = ProxmoxClient::new("https://pve:8006").unwrap();
        assert!(matches!(
            client.get("/version"),
            Err(Error::NotAuthenticated)
        ));
    }

    #[test]
    fn unauthenticated_post_returns_error() {
        let client = ProxmoxClient::new("https://pve:8006").unwrap();
        assert!(matches!(
            client.post("/nodes/pve1/qemu"),
            Err(Error::NotAuthenticated)
        ));
    }

    #[test]
    fn set_ticket_auth_makes_authenticated() {
        let client = ProxmoxClient::new("https://pve:8006").unwrap();
        assert!(!client.is_authenticated());
        client.set_ticket_auth("ticket-value", "csrf-value");
        assert!(client.is_authenticated());
    }

    #[test]
    fn truncate_short_string() {
        assert_eq!(truncate("hello", 10), "hello");
    }

    #[test]
    fn truncate_long_string() {
        let s = "a".repeat(300);
        let result = truncate(&s, 256);
        assert_eq!(result.len(), 256);
    }

    #[test]
    fn http_url_rejected_by_default() {
        assert!(matches!(
            ProxmoxClient::new("http://pve:8006"),
            Err(Error::InvalidBaseUrl(_))
        ));
    }

    #[test]
    fn http_url_accepted_when_allowed() {
        let client = ProxmoxClient::builder("http://pve:8006")
            .allow_insecure_http(true)
            .build()
            .unwrap();
        assert_eq!(client.base_url, "http://pve:8006");
    }

    #[test]
    fn unauthenticated_put_returns_error() {
        let client = ProxmoxClient::new("https://pve:8006").unwrap();
        assert!(matches!(
            client.put("/nodes/pve1/qemu/100/config"),
            Err(Error::NotAuthenticated)
        ));
    }

    #[test]
    fn unauthenticated_delete_returns_error() {
        let client = ProxmoxClient::new("https://pve:8006").unwrap();
        assert!(matches!(
            client.delete("/nodes/pve1/qemu/100"),
            Err(Error::NotAuthenticated)
        ));
    }

    #[test]
    fn truncate_owned_short_string() {
        let s = "hello".to_string();
        let result = truncate_owned(s, 100);
        assert_eq!(result, "hello");
    }

    #[test]
    fn truncate_owned_long_string() {
        let s = "a".repeat(600);
        let result = truncate_owned(s, 512);
        assert_eq!(result.len(), 512 + "...<truncated>".len());
        assert!(result.ends_with("...<truncated>"));
    }

    #[test]
    fn truncate_owned_exact_length() {
        let s = "a".repeat(512);
        let result = truncate_owned(s, 512);
        assert_eq!(result.len(), 512);
        assert!(!result.contains("truncated"));
    }

    #[test]
    fn truncate_multibyte_does_not_split_codepoint() {
        // '🦀' is 4 bytes; cutting at byte 2 must round down to 0
        let s = "🦀hello";
        let result = truncate(s, 2);
        assert!(result.is_empty(), "must not split a multi-byte codepoint");
    }

    #[test]
    fn builder_with_custom_timeouts() {
        let client = ProxmoxClient::builder("https://pve:8006")
            .timeout(Duration::from_secs(60))
            .connect_timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        assert_eq!(client.base_url, "https://pve:8006");
    }

    #[test]
    fn non_http_scheme_rejected() {
        assert!(matches!(
            ProxmoxClient::new("ftp://pve:8006"),
            Err(Error::InvalidBaseUrl(_))
        ));
    }

    #[test]
    fn non_http_scheme_rejected_even_with_insecure_http() {
        assert!(matches!(
            ProxmoxClient::builder("ftp://pve:8006")
                .allow_insecure_http(true)
                .build(),
            Err(Error::InvalidBaseUrl(_))
        ));
    }

    #[test]
    fn build_query_path_encodes_values() {
        let path = ProxmoxClient::build_query_path(
            "/nodes/pve/journal",
            &[("since", Some("2024-01-01&malicious=true"))],
        );
        assert_eq!(
            path,
            "/nodes/pve/journal?since=2024-01-01%26malicious%3Dtrue"
        );
    }

    #[test]
    fn build_query_path_skips_none() {
        let path = ProxmoxClient::build_query_path(
            "/nodes/pve/journal",
            &[("since", None), ("until", Some("2024-12-31"))],
        );
        assert_eq!(path, "/nodes/pve/journal?until=2024-12-31");
    }

    #[test]
    fn build_query_path_empty_params() {
        let path = ProxmoxClient::build_query_path("/nodes/pve/journal", &[]);
        assert_eq!(path, "/nodes/pve/journal");
    }
}
