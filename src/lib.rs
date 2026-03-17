#![forbid(unsafe_code)]
#![warn(missing_docs)]
//! Rust client for the [Proxmox VE API](https://pve.proxmox.com/pve-docs/api-viewer/).
//!
//! # Quick start
//!
//! ```no_run
//! # async fn example() -> proxmox_client::Result<()> {
//! // API token authentication (preferred for automation)
//! let client = proxmox_client::ProxmoxClient::with_api_token(
//!     "https://pve:8006",
//!     "root@pam!mytoken",
//!     "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
//! )?;
//!
//! let version = client.version().await?;
//! println!("Proxmox VE {}", version.version.unwrap_or_default());
//! # Ok(())
//! # }
//! ```
//!
//! ```no_run
//! # async fn example() -> proxmox_client::Result<()> {
//! // Ticket authentication with self-signed cert support
//! let client = proxmox_client::ProxmoxClient::builder("https://pve:8006")
//!     .accept_invalid_certs(true)
//!     .build()?;
//! client.login("root@pam", "password").await?;
//!
//! let version = client.version().await?;
//! println!("Proxmox VE {}", version.version.unwrap_or_default());
//! # Ok(())
//! # }
//! ```

mod client;
/// Error types for the Proxmox client.
pub mod error;
mod validation;

mod auth;
/// Unified firewall types shared across cluster, node, VM, and container endpoints.
pub mod firewall_types;
mod serde_helpers;
mod types;
mod version;

/// Access control (users, groups, roles, ACLs, domains, TFA, OpenID).
pub mod access;
/// Cluster-wide configuration and management.
pub mod cluster;
/// Node-specific operations and monitoring.
pub mod nodes;
mod pools;
mod storage;

pub use auth::TicketResponse;
pub use client::{ProxmoxClient, ProxmoxClientBuilder};
pub use error::{Error, Result};
pub use pools::{Pool, PoolCreateParams, PoolMember, PoolUpdateParams};
pub use storage::{StorageConfig, StorageContentSummary, StorageCreateParams, StorageUpdateParams};
pub use types::{TaskStatus, Upid};
pub use version::Version;
