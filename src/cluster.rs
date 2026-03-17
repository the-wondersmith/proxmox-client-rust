//! Cluster-wide configuration and management.
//!
//! This module covers cluster config, resources, options, firewall, HA,
//! backup, ACME, Ceph, SDN, notifications, metrics, replication, jobs,
//! device mappings, and bulk actions.

mod acme;
mod backup;
mod bulk_action;
mod ceph;
mod config;
mod firewall;
mod ha;
mod jobs;
mod mapping;
mod metrics;
mod notifications;
mod options;
mod replication;
mod resources;
mod sdn;
mod sdn_fabrics;

pub use acme::{
    AcmeAccount, AcmeAccountCreateParams, AcmeDirectory, AcmePlugin, AcmePluginCreateParams,
};
pub use backup::{BackupIncludedVolume, BackupJob, BackupJobCreateParams, NotBackedUpGuest};
pub use bulk_action::{BulkMigrateParams, BulkShutdownParams, BulkStartParams, BulkSuspendParams};
pub use ceph::{CephFlags, ClusterCephMetadata, ClusterCephStatus};
pub use config::{
    ClusterConfig, ClusterCreateParams, ClusterJoinInfo, ClusterJoinParams, ClusterNode,
    TotemConfig,
};
pub use firewall::{
    ClusterFirewallAlias, ClusterFirewallIpSet, ClusterFirewallIpSetEntry, ClusterFirewallOptions,
    ClusterFirewallRule, ClusterFirewallRuleCreateParams, FirewallMacro, FirewallRef,
    SecurityGroup, SecurityGroupCreateParams,
};
pub use ha::{
    HaGroup, HaGroupCreateParams, HaManagerStatus, HaResource, HaResourceCreateParams, HaRule,
    HaRuleCreateParams, HaStatus,
};
pub use jobs::{RealmSyncJob, RealmSyncJobCreateParams, ScheduleAnalysis};
pub use mapping::{
    DirMapping, DirMappingCreateParams, PciMapping, PciMappingCreateParams, UsbMapping,
    UsbMappingCreateParams,
};
pub use metrics::{MetricServer, MetricServerCreateParams};
pub use notifications::{
    GotifyEndpoint, GotifyEndpointCreateParams, NotificationMatcher,
    NotificationMatcherCreateParams, NotificationTarget, SendmailEndpoint,
    SendmailEndpointCreateParams, SmtpEndpoint, SmtpEndpointCreateParams, WebhookEndpoint,
    WebhookEndpointCreateParams,
};
pub use options::ClusterOptions;
pub use replication::{ReplicationJob, ReplicationJobCreateParams};
pub use resources::{ClusterLogEntry, ClusterResource, ClusterStatus};
pub use sdn::{
    SdnController, SdnControllerCreateParams, SdnDns, SdnDnsCreateParams, SdnIpMapping, SdnIpam,
    SdnIpamCreateParams, SdnSubnet, SdnSubnetCreateParams, SdnVnet, SdnVnetCreateParams, SdnZone,
    SdnZoneCreateParams,
};
pub use sdn_fabrics::{SdnFabric, SdnFabricCreateParams, SdnFabricNode, SdnFabricNodeCreateParams};
