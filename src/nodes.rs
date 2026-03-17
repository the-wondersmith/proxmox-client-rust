//! Node-specific operations and monitoring.
//!
//! This module covers status, tasks, QEMU VMs, LXC containers, storage,
//! networking, services, disks, APT, certificates, scanning, Ceph, SDN, and vzdump.

mod apt;
mod capabilities;
mod ceph;
mod certificates;
mod config;
mod disks;
mod firewall;
mod hardware;
mod network;
mod replication;
mod scan;
mod sdn;
mod services;
mod status;
mod storage;
mod subscription;
mod tasks;
mod vzdump;

/// LXC container management.
pub mod lxc;
/// QEMU virtual machine management.
pub mod qemu;

pub use apt::{AptPackageVersion, AptRepository, AptUpdate};
pub use capabilities::{CpuModel, MachineType};
pub use ceph::{
    CephCmdSafety, CephFs, CephInitParams, CephMds, CephMgr, CephMon, CephOsd, CephOsdCreateParams,
    CephPool, CephPoolCreateParams, CephPoolUpdateParams, CephRule, CephStatus,
};
pub use certificates::{CertificateInfo, CustomCertUpload};
pub use config::{NodeConfig, NodeConfigUpdateParams};
pub use disks::{
    DirectoryCreateParams, Disk, DiskDirectory, LvmCreateParams, LvmThinCreateParams, LvmThinPool,
    LvmVolume, SmartData, WipeDiskParams, ZfsCreateParams, ZfsPool,
};
pub use firewall::NodeFirewallOptions;
pub use hardware::{MediatedDevice, PciDevice, UsbDevice};
pub use network::{NetworkInterface, NetworkInterfaceCreateParams, NetworkInterfaceUpdateParams};
pub use replication::NodeReplicationStatus;
pub use scan::{
    CifsShare, IscsiTarget, LvmThinScanResult, LvmVolumeGroup, NfsShare, PbsNamespace, ZfsDataset,
};
pub use sdn::SdnZoneContent;
pub use services::{NodeService, ServiceState};
pub use status::{
    ApplianceDownloadParams, ApplianceInfo, HostsFile, HostsWriteParams, MigrateAllParams,
    NodeControlParams, NodeDns, NodeDnsUpdateParams, NodeStatus, NodeTime, NodeTimeUpdateParams,
    NodeVersion, RrdData, StartStopAllParams, SuspendAllParams, UrlMetadata,
};
pub use storage::{
    DownloadUrlParams, FileRestoreEntry, NodeStorage, NodeStorageStatus, OciRegistryPullParams,
    StorageContent, StorageVolume,
};
pub use subscription::{SubscriptionInfo, SubscriptionKeyParams};
pub use tasks::{TaskListItem, TaskLogLine};
pub use vzdump::{VzdumpDefaults, VzdumpParams};
