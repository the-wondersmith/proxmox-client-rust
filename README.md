# proxmox-client

Rust client library for the [Proxmox VE REST API](https://pve.proxmox.com/pve-docs/api-viewer/).

> **Warning**
> This is an experimental version of the library. The API is subject to change and may contain bugs. Use at your own risk in production environments.

## Features

- **Async** — built on [reqwest](https://docs.rs/reqwest) and [Tokio](https://tokio.rs)
- **Type-safe** — strongly typed request/response models with [serde](https://serde.rs)
- **Input validation** — node names, VMIDs, user IDs, storage IDs, and more are validated before any network call
- **Security-first** — auth tokens and passwords redacted from `Debug`, `#![forbid(unsafe_code)]`, HTTPS enforced by default
- **Ergonomic errors** — typed `Error` enum with per-status-code variants (`NotFound`, `Unauthorized`, `Forbidden`, `RateLimited`)
- **Builder pattern** — configurable client with self-signed cert support, timeouts, and HTTP override
- **Dual auth** — API token (stateless) and ticket-based (session) authentication

## Installation

```toml
[dependencies]
proxmox-client = "0.9.2"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Quick start

### API token authentication (preferred for automation)

```rust
let client = proxmox_client::ProxmoxClient::with_api_token(
    "https://pve:8006",
    "root@pam!mytoken",
    "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
)?;

let version = client.version().await?;
println!("Proxmox VE {}", version.version.unwrap_or_default());
```

### Ticket-based login with self-signed cert support

```rust
let client = proxmox_client::ProxmoxClient::builder("https://pve:8006")
    .accept_invalid_certs(true)
    .build()?;
client.login("root@pam", "password").await?;

let version = client.version().await?;
println!("Proxmox VE {}", version.version.unwrap_or_default());
```

## API overview

### Authentication

```rust
// Ticket-based login
client.login("root@pam", "password").await?;

// Two-factor authentication
// let resp = client.login("root@pam", "password").await; // returns TFA challenge
// client.login_tfa("root@pam", &challenge_ticket, "123456").await?;

// Change a user's password
client.change_password("user@pam", "new-password").await?;
```

### Nodes

```rust
// List all cluster nodes
let nodes = client.list_nodes().await?;
for node in &nodes {
    println!("{}: {}", node.node.as_deref().unwrap_or("?"), node.status.as_deref().unwrap_or("?"));
}

// Node details
let status = client.get_node_status("pve1").await?;
let dns = client.get_node_dns("pve1").await?;
let ver = client.version().await?;
```

### QEMU virtual machines

```rust
use proxmox_client::nodes::qemu::{VmCreateParams, VmCloneParams, VmResizeParams};

// List VMs on a node
let vms = client.list_vms("pve1").await?;

// Create a VM
let mut params = VmCreateParams::new(100);
params.name = Some("my-vm".into());
params.memory = Some(2048);
params.cores = Some(4);
let upid = client.create_vm("pve1", &params).await?;

// Power management
client.start_vm("pve1", 100, None).await?;
client.shutdown_vm("pve1", 100, None).await?;
client.stop_vm("pve1", 100, None).await?;

// Status
let status = client.get_vm_status("pve1", 100).await?;

// Clone a VM
let clone = VmCloneParams::new(101);
client.clone_vm("pve1", 100, &clone).await?;

// Resize a disk
let resize = VmResizeParams::new("scsi0", "+10G");
client.resize_vm_disk("pve1", 100, &resize).await?;

// Delete a VM
client.delete_vm("pve1", 100, None).await?;
```

Snapshots and configuration:

```rust
use proxmox_client::nodes::qemu::SnapshotCreateParams;

// Snapshots
let snaps = client.list_vm_snapshots("pve1", 100).await?;
client.create_vm_snapshot("pve1", 100, &SnapshotCreateParams {
    snapname: "before-upgrade".into(),
    description: None,
    vmstate: None,
}).await?;
client.rollback_vm_snapshot("pve1", 100, "before-upgrade").await?;
client.delete_vm_snapshot("pve1", 100, "before-upgrade").await?;

// Configuration
let config = client.get_vm_config("pve1", 100).await?;
```

### LXC containers

```rust
use proxmox_client::nodes::lxc::{ContainerCreateParams, ContainerResizeParams};

// List containers
let cts = client.list_containers("pve1").await?;

// Create a container
let mut params = ContainerCreateParams::new(200, "local:vztmpl/ubuntu-22.04-standard_22.04-1_amd64.tar.zst");
params.hostname = Some("my-ct".into());
params.memory = Some(1024);
params.cores = Some(2);
params.unprivileged = Some(true);
client.create_container("pve1", &params).await?;

// Power management
client.start_container("pve1", 200, None).await?;
client.shutdown_container("pve1", 200, None).await?;
client.stop_container("pve1", 200, None).await?;

// Status
let status = client.get_container_status("pve1", 200).await?;

// Resize rootfs
let resize = ContainerResizeParams::new("rootfs", "+5G");
client.resize_container_disk("pve1", 200, &resize).await?;

// Delete
client.delete_container("pve1", 200, None).await?;
```

### Storage

```rust
use proxmox_client::{StorageCreateParams, StorageUpdateParams};

// List datacenter-level storage
let storage = client.list_storage().await?;

// Create an NFS storage
let mut params = StorageCreateParams::new("nfs-backup", "nfs");
params.server = Some("192.168.1.10".into());
params.export = Some("/mnt/backup".into());
params.content = Some("backup,images".into());
client.create_storage(&params).await?;

// Get / update / delete
let config = client.get_storage("nfs-backup").await?;
let mut update = StorageUpdateParams::default();
update.content = Some("backup".into());
client.update_storage("nfs-backup", &update).await?;
client.delete_storage("nfs-backup").await?;
```

### Pools

```rust
use proxmox_client::{PoolCreateParams, PoolUpdateParams};

let pools = client.list_pools().await?;
let mut pool_params = PoolCreateParams::new("dev-pool");
pool_params.comment = Some("Development VMs".into());
client.create_pool(&pool_params).await?;
let pool = client.get_pool("dev-pool").await?;
client.delete_pool("dev-pool").await?;
```

### Access control

```rust
use proxmox_client::access::users::UserCreateParams;
use proxmox_client::access::groups::GroupCreateParams;
use proxmox_client::access::roles::RoleCreateParams;
use proxmox_client::access::acl::AclUpdateParams;

// Users
let users = client.list_users().await?;
let mut user = UserCreateParams::default();
user.userid = "dev@pam".into();
client.create_user(&user).await?;

// Groups
let groups = client.list_groups().await?;
let mut group = GroupCreateParams::default();
group.groupid = "developers".into();
client.create_group(&group).await?;

// Roles
let roles = client.list_roles().await?;
let mut role = RoleCreateParams::default();
role.roleid = "VMOperator".into();
client.create_role(&role).await?;

// ACLs
let acls = client.get_acl().await?;

// API tokens
let tokens = client.list_user_tokens("dev@pam").await?;

// TFA
let tfa = client.list_tfa().await?;

// Realms / authentication domains
let realms = client.list_realms().await?;
```

### Cluster

```rust
// Resources and status
let resources = client.list_cluster_resources().await?;
let status = client.get_cluster_status().await?;

// HA
let ha_status = client.list_ha_resources().await?;
let ha_groups = client.list_ha_groups().await?;

// Backup jobs
let jobs = client.list_backup_jobs().await?;

// Firewall
let rules = client.list_cluster_firewall_rules().await?;

// ACME
let accounts = client.list_acme_accounts().await?;

// SDN
let vnets = client.list_sdn_vnets().await?;
let zones = client.list_sdn_zones().await?;
```

### Node operations

```rust
// Tasks
let tasks = client.list_tasks("pve1", None).await?;
let task = client.get_task_status("pve1", "UPID:pve1:...").await?;

// Network interfaces
let interfaces = client.list_network_interfaces("pve1", None).await?;

// Disks
let disks = client.list_disks("pve1", None).await?;

// Services
let services = client.list_services("pve1").await?;

// APT
let updates = client.list_apt_updates("pve1").await?;

// Certificates
let certs = client.list_certificates("pve1").await?;
```

## Error handling

All methods return `proxmox_client::Result<T>`, which uses a typed `Error` enum:

```rust
use proxmox_client::Error;

match client.get_vm_status("pve1", 999).await {
    Ok(status) => println!("VM status: {:?}", status.status),
    Err(Error::NotFound { resource, .. }) => println!("{resource} does not exist"),
    Err(Error::Unauthorized { .. }) => println!("bad credentials"),
    Err(Error::Forbidden { .. }) => println!("insufficient permissions"),
    Err(Error::RateLimited { retry_after }) => {
        println!("slow down — retry after {retry_after:?}");
    }
    Err(Error::InvalidVmid(id)) => println!("VMID {id} is out of range"),
    Err(e) => eprintln!("unexpected error: {e}"),
}
```

## Security

- **`#![forbid(unsafe_code)]`** — no unsafe code anywhere in the crate
- **Input validation** — node names, VMIDs, user IDs, storage IDs, and other identifiers are validated before network calls to prevent path-traversal and injection attacks
- **Debug redaction** — auth tokens, passwords, and secrets are replaced with `<redacted>` in `Debug` output
- **HTTPS enforced** — HTTP URLs are rejected by default; opt in with `.allow_insecure_http(true)` for local development
- **Timeouts** — 30-second request timeout and 10-second connect timeout by default
- **Response truncation** — error response bodies are truncated to 512 bytes to prevent unbounded memory allocation

## License

[MIT](LICENSE)
