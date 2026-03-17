use std::time::Duration;

use reqwest::StatusCode;

/// Errors returned by the Proxmox client.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// An underlying HTTP request failed.
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// A header value contained invalid characters.
    #[error("invalid header value: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),

    /// The base URL is not valid.
    #[error("invalid base URL: {0}")]
    InvalidBaseUrl(String),

    /// The supplied node name is not valid.
    #[error("invalid node name: {0}")]
    InvalidNodeName(String),

    /// The supplied VMID is not valid (must be 100–999999999).
    #[error("invalid VMID: {0}")]
    InvalidVmid(String),

    /// The supplied storage ID is not valid.
    #[error("invalid storage ID: {0}")]
    InvalidStorageId(String),

    /// The supplied pool ID is not valid.
    #[error("invalid pool ID: {0}")]
    InvalidPoolId(String),

    /// The supplied user ID is not valid (must be `user@realm`).
    #[error("invalid user ID: {0}")]
    InvalidUserId(String),

    /// The supplied realm/domain ID is not valid.
    #[error("invalid realm ID: {0}")]
    InvalidRealmId(String),

    /// The supplied group ID is not valid.
    #[error("invalid group ID: {0}")]
    InvalidGroupId(String),

    /// The supplied role ID is not valid.
    #[error("invalid role ID: {0}")]
    InvalidRoleId(String),

    /// The supplied token ID is not valid.
    #[error("invalid token ID: {0}")]
    InvalidTokenId(String),

    /// The supplied resource ID is not valid.
    #[error("invalid resource ID: {0}")]
    InvalidResourceId(String),

    /// The supplied HA service ID is not valid (must be `type:vmid`).
    #[error("invalid HA service ID: {0}")]
    InvalidHaSid(String),

    /// The client is not authenticated.
    #[error("not authenticated — call login() or use with_api_token()")]
    NotAuthenticated,

    /// Login failed (bad credentials or TFA required).
    #[error("login failed: {0}")]
    LoginFailed(String),

    /// The API returned `401 Unauthorized`.
    #[error("authentication failed (401)")]
    Unauthorized {
        /// Response body from the server.
        body: String,
    },

    /// The API returned `403 Forbidden`.
    #[error("access denied (403)")]
    Forbidden {
        /// Response body from the server.
        body: String,
    },

    /// The requested resource was not found (`404`).
    #[error("resource not found (404): {resource}")]
    NotFound {
        /// Name of the resource that was not found.
        resource: String,
        /// Response body from the server.
        body: String,
    },

    /// The API returned `429 Too Many Requests`.
    #[error("rate limited (429)")]
    RateLimited {
        /// Value of the `Retry-After` header, if present.
        retry_after: Option<Duration>,
    },

    /// The API returned `500 Internal Server Error`.
    #[error("internal server error (500): {body}")]
    InternalServerError {
        /// Response body from the server.
        body: String,
    },

    /// Any other non-success HTTP status code.
    #[error("API error (HTTP {status}): {body}")]
    Api {
        /// HTTP status code.
        status: StatusCode,
        /// Response body from the server.
        body: String,
    },

    /// The API returned validation errors.
    #[error("API validation errors for {resource}: {errors:?}")]
    ApiValidationErrors {
        /// Name of the resource.
        resource: String,
        /// Field-level error messages from the server.
        errors: std::collections::HashMap<String, String>,
    },

    /// JSON deserialization failed.
    #[error("failed to parse response: {0}")]
    Deserialization(String),
}

/// A specialized [`Result`](std::result::Result) type for Proxmox client operations.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Error>();
    }

    #[test]
    fn error_implements_std_error() {
        fn assert_std_error<T: std::error::Error>() {}
        assert_std_error::<Error>();
    }

    #[test]
    fn display_invalid_node_name() {
        let err = Error::InvalidNodeName("bad/node".to_string());
        let msg = err.to_string();
        assert!(msg.contains("invalid node name"), "got: {msg}");
        assert!(msg.contains("bad/node"), "got: {msg}");
    }

    #[test]
    fn display_invalid_vmid() {
        let err = Error::InvalidVmid("99".to_string());
        let msg = err.to_string();
        assert!(msg.contains("invalid VMID"), "got: {msg}");
        assert!(msg.contains("99"), "got: {msg}");
    }

    #[test]
    fn display_not_authenticated() {
        let err = Error::NotAuthenticated;
        let msg = err.to_string();
        assert!(msg.contains("not authenticated"), "got: {msg}");
    }

    #[test]
    fn display_unauthorized() {
        let err = Error::Unauthorized {
            body: "nope".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("401"), "got: {msg}");
    }

    #[test]
    fn display_forbidden() {
        let err = Error::Forbidden {
            body: "nope".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("403"), "got: {msg}");
    }

    #[test]
    fn display_not_found() {
        let err = Error::NotFound {
            resource: "node pve1".to_string(),
            body: "not here".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("404"), "got: {msg}");
        assert!(msg.contains("node pve1"), "got: {msg}");
    }

    #[test]
    fn display_api_error() {
        let err = Error::Api {
            status: StatusCode::BAD_GATEWAY,
            body: "upstream error".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("502"), "got: {msg}");
        assert!(msg.contains("upstream error"), "got: {msg}");
    }
}
