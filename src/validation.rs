use crate::error::{Error, Result};

/// Validates a Proxmox node name.
///
/// Node names must be non-empty, start with a letter, and contain only
/// alphanumeric characters, hyphens, and dots (valid DNS hostname).
pub(crate) fn validate_node_name(name: &str) -> Result<()> {
    if name.is_empty()
        || !name.as_bytes()[0].is_ascii_alphabetic()
        || !name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'.')
        || name.contains("..")
    {
        Err(Error::InvalidNodeName(name.to_owned()))
    } else {
        Ok(())
    }
}

/// Validates a VMID (virtual machine / container ID).
///
/// VMIDs must be integers in the range 100–999999999.
pub(crate) fn validate_vmid(vmid: u32) -> Result<()> {
    if (100..=999_999_999).contains(&vmid) {
        Ok(())
    } else {
        Err(Error::InvalidVmid(vmid.to_string()))
    }
}

/// Validates an identifier using common rules: non-empty, alphanumeric plus
/// hyphens/underscores/dots, no `..` sequences.
fn validate_id(id: &str, make_error: impl FnOnce(String) -> Error) -> Result<()> {
    if id.is_empty()
        || !id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.')
        || id.contains("..")
    {
        Err(make_error(id.to_owned()))
    } else {
        Ok(())
    }
}

/// Validates a Proxmox storage ID.
///
/// Storage IDs must be non-empty and contain only alphanumeric characters,
/// hyphens, underscores, and dots (no `..` sequences).
pub(crate) fn validate_storage_id(id: &str) -> Result<()> {
    validate_id(id, Error::InvalidStorageId)
}

/// Validates a Proxmox pool ID.
///
/// Pool IDs follow the same rules as storage IDs.
pub(crate) fn validate_pool_id(id: &str) -> Result<()> {
    validate_id(id, Error::InvalidPoolId)
}

/// Validates a Proxmox user ID (`user@realm` format).
pub(crate) fn validate_user_id(user_id: &str) -> Result<()> {
    let Some((username, realm)) = user_id.split_once('@') else {
        return Err(Error::InvalidUserId(user_id.to_owned()));
    };
    if username.is_empty() || realm.is_empty() {
        return Err(Error::InvalidUserId(user_id.to_owned()));
    }
    // Username part: alphanumeric + hyphen + underscore + dot
    if !username
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.')
    {
        return Err(Error::InvalidUserId(user_id.to_owned()));
    }
    // Realm part: alphanumeric + hyphen + underscore
    if !realm
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
    {
        return Err(Error::InvalidUserId(user_id.to_owned()));
    }
    Ok(())
}

/// Validates a Proxmox realm/domain ID.
pub(crate) fn validate_realm_id(id: &str) -> Result<()> {
    validate_id(id, Error::InvalidRealmId)
}

/// Validates a Proxmox group ID.
pub(crate) fn validate_group_id(id: &str) -> Result<()> {
    validate_id(id, Error::InvalidGroupId)
}

/// Validates a Proxmox API token ID.
///
/// Token IDs must be non-empty and contain only alphanumeric characters,
/// hyphens, underscores, and dots (no `..` sequences).
pub(crate) fn validate_token_id(id: &str) -> Result<()> {
    validate_id(id, Error::InvalidTokenId)
}

/// Validates a Proxmox role ID.
pub(crate) fn validate_role_id(id: &str) -> Result<()> {
    validate_id(id, Error::InvalidRoleId)
}

/// Validates a generic Proxmox resource ID (e.g., backup job, notification endpoint,
/// SDN resource, mapping, etc.).
pub(crate) fn validate_resource_id(id: &str) -> Result<()> {
    validate_id(id, Error::InvalidResourceId)
}

/// Validates a Proxmox HA service ID (`type:vmid` format, e.g., `vm:100`, `ct:200`).
pub(crate) fn validate_ha_sid(sid: &str) -> Result<()> {
    let Some((res_type, vmid)) = sid.split_once(':') else {
        return Err(Error::InvalidHaSid(sid.to_owned()));
    };
    if res_type.is_empty()
        || !res_type.bytes().all(|b| b.is_ascii_alphanumeric())
        || vmid.is_empty()
        || !vmid.bytes().all(|b| b.is_ascii_digit())
    {
        return Err(Error::InvalidHaSid(sid.to_owned()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Node name tests
    #[test]
    fn valid_node_names() {
        assert!(validate_node_name("pve1").is_ok());
        assert!(validate_node_name("node-01").is_ok());
        assert!(validate_node_name("my.node.local").is_ok());
        assert!(validate_node_name("PVE").is_ok());
    }

    #[test]
    fn invalid_node_name_empty() {
        assert!(matches!(
            validate_node_name(""),
            Err(Error::InvalidNodeName(_))
        ));
    }

    #[test]
    fn invalid_node_name_starts_with_number() {
        assert!(matches!(
            validate_node_name("1node"),
            Err(Error::InvalidNodeName(_))
        ));
    }

    #[test]
    fn invalid_node_name_starts_with_hyphen() {
        assert!(matches!(
            validate_node_name("-node"),
            Err(Error::InvalidNodeName(_))
        ));
    }

    #[test]
    fn invalid_node_name_special_chars() {
        assert!(matches!(
            validate_node_name("node/bad"),
            Err(Error::InvalidNodeName(_))
        ));
        assert!(matches!(
            validate_node_name("node@bad"),
            Err(Error::InvalidNodeName(_))
        ));
    }

    #[test]
    fn invalid_node_name_path_traversal() {
        assert!(matches!(
            validate_node_name("node..bad"),
            Err(Error::InvalidNodeName(_))
        ));
    }

    // VMID tests
    #[test]
    fn valid_vmids() {
        assert!(validate_vmid(100).is_ok());
        assert!(validate_vmid(200).is_ok());
        assert!(validate_vmid(999_999_999).is_ok());
    }

    #[test]
    fn invalid_vmid_too_low() {
        assert!(matches!(validate_vmid(0), Err(Error::InvalidVmid(_))));
        assert!(matches!(validate_vmid(99), Err(Error::InvalidVmid(_))));
    }

    #[test]
    fn invalid_vmid_too_high() {
        assert!(matches!(
            validate_vmid(1_000_000_000),
            Err(Error::InvalidVmid(_))
        ));
    }

    // Storage ID tests
    #[test]
    fn valid_storage_ids() {
        assert!(validate_storage_id("local").is_ok());
        assert!(validate_storage_id("local-lvm").is_ok());
        assert!(validate_storage_id("nfs_share").is_ok());
        assert!(validate_storage_id("ceph.pool1").is_ok());
    }

    #[test]
    fn invalid_storage_id_empty() {
        assert!(matches!(
            validate_storage_id(""),
            Err(Error::InvalidStorageId(_))
        ));
    }

    #[test]
    fn invalid_storage_id_special_chars() {
        assert!(matches!(
            validate_storage_id("bad/storage"),
            Err(Error::InvalidStorageId(_))
        ));
        assert!(matches!(
            validate_storage_id("bad@storage"),
            Err(Error::InvalidStorageId(_))
        ));
    }

    #[test]
    fn invalid_storage_id_path_traversal() {
        assert!(matches!(
            validate_storage_id(".."),
            Err(Error::InvalidStorageId(_))
        ));
    }

    // Pool ID tests
    #[test]
    fn valid_pool_ids() {
        assert!(validate_pool_id("production").is_ok());
        assert!(validate_pool_id("dev-pool").is_ok());
        assert!(validate_pool_id("pool_1").is_ok());
    }

    #[test]
    fn invalid_pool_id_empty() {
        assert!(matches!(validate_pool_id(""), Err(Error::InvalidPoolId(_))));
    }

    // User ID tests
    #[test]
    fn valid_user_ids() {
        assert!(validate_user_id("root@pam").is_ok());
        assert!(validate_user_id("admin@pve").is_ok());
        assert!(validate_user_id("user.name@ldap").is_ok());
        assert!(validate_user_id("test-user@my_realm").is_ok());
    }

    #[test]
    fn invalid_user_id_no_realm() {
        assert!(matches!(
            validate_user_id("root"),
            Err(Error::InvalidUserId(_))
        ));
    }

    #[test]
    fn invalid_user_id_empty_parts() {
        assert!(matches!(
            validate_user_id("@pam"),
            Err(Error::InvalidUserId(_))
        ));
        assert!(matches!(
            validate_user_id("root@"),
            Err(Error::InvalidUserId(_))
        ));
    }

    #[test]
    fn invalid_user_id_special_chars() {
        assert!(matches!(
            validate_user_id("root/admin@pam"),
            Err(Error::InvalidUserId(_))
        ));
    }

    // Realm ID tests
    #[test]
    fn valid_realm_ids() {
        assert!(validate_realm_id("pam").is_ok());
        assert!(validate_realm_id("pve").is_ok());
        assert!(validate_realm_id("ldap-server").is_ok());
    }

    #[test]
    fn invalid_realm_id_empty() {
        assert!(matches!(
            validate_realm_id(""),
            Err(Error::InvalidRealmId(_))
        ));
    }

    // Group ID tests
    #[test]
    fn valid_group_ids() {
        assert!(validate_group_id("admins").is_ok());
        assert!(validate_group_id("dev-team").is_ok());
    }

    #[test]
    fn invalid_group_id_empty() {
        assert!(matches!(
            validate_group_id(""),
            Err(Error::InvalidGroupId(_))
        ));
    }

    // Token ID tests
    #[test]
    fn valid_token_ids() {
        assert!(validate_token_id("mytoken").is_ok());
        assert!(validate_token_id("ci-token").is_ok());
        assert!(validate_token_id("token_1").is_ok());
        assert!(validate_token_id("my.token").is_ok());
    }

    #[test]
    fn invalid_token_id_empty() {
        assert!(matches!(
            validate_token_id(""),
            Err(Error::InvalidTokenId(_))
        ));
    }

    #[test]
    fn invalid_token_id_special_chars() {
        assert!(matches!(
            validate_token_id("bad/token"),
            Err(Error::InvalidTokenId(_))
        ));
        assert!(matches!(
            validate_token_id("../etc/passwd"),
            Err(Error::InvalidTokenId(_))
        ));
    }

    #[test]
    fn invalid_token_id_path_traversal() {
        assert!(matches!(
            validate_token_id(".."),
            Err(Error::InvalidTokenId(_))
        ));
    }

    // Role ID tests
    #[test]
    fn valid_role_ids() {
        assert!(validate_role_id("PVEAdmin").is_ok());
        assert!(validate_role_id("custom-role").is_ok());
    }

    #[test]
    fn invalid_role_id_empty() {
        assert!(matches!(validate_role_id(""), Err(Error::InvalidRoleId(_))));
    }

    #[test]
    fn invalid_role_id_special_chars() {
        assert!(matches!(
            validate_role_id("bad/role"),
            Err(Error::InvalidRoleId(_))
        ));
        assert!(matches!(
            validate_role_id("bad@role"),
            Err(Error::InvalidRoleId(_))
        ));
    }

    #[test]
    fn invalid_role_id_path_traversal() {
        assert!(matches!(
            validate_role_id(".."),
            Err(Error::InvalidRoleId(_))
        ));
    }

    #[test]
    fn invalid_group_id_special_chars() {
        assert!(matches!(
            validate_group_id("bad/group"),
            Err(Error::InvalidGroupId(_))
        ));
        assert!(matches!(
            validate_group_id("bad@group"),
            Err(Error::InvalidGroupId(_))
        ));
    }

    #[test]
    fn invalid_group_id_path_traversal() {
        assert!(matches!(
            validate_group_id(".."),
            Err(Error::InvalidGroupId(_))
        ));
    }

    #[test]
    fn invalid_realm_id_special_chars() {
        assert!(matches!(
            validate_realm_id("bad/realm"),
            Err(Error::InvalidRealmId(_))
        ));
        assert!(matches!(
            validate_realm_id("bad@realm"),
            Err(Error::InvalidRealmId(_))
        ));
    }

    #[test]
    fn invalid_realm_id_path_traversal() {
        assert!(matches!(
            validate_realm_id(".."),
            Err(Error::InvalidRealmId(_))
        ));
    }

    #[test]
    fn invalid_pool_id_special_chars() {
        assert!(matches!(
            validate_pool_id("bad/pool"),
            Err(Error::InvalidPoolId(_))
        ));
        assert!(matches!(
            validate_pool_id("bad@pool"),
            Err(Error::InvalidPoolId(_))
        ));
    }

    #[test]
    fn invalid_pool_id_path_traversal() {
        assert!(matches!(
            validate_pool_id(".."),
            Err(Error::InvalidPoolId(_))
        ));
    }

    // Resource ID tests
    #[test]
    fn valid_resource_ids() {
        assert!(validate_resource_id("backup-abc123").is_ok());
        assert!(validate_resource_id("mail1").is_ok());
        assert!(validate_resource_id("my.resource_1").is_ok());
    }

    #[test]
    fn invalid_resource_id_empty() {
        assert!(matches!(
            validate_resource_id(""),
            Err(Error::InvalidResourceId(_))
        ));
    }

    #[test]
    fn invalid_resource_id_special_chars() {
        assert!(matches!(
            validate_resource_id("bad/id"),
            Err(Error::InvalidResourceId(_))
        ));
        assert!(matches!(
            validate_resource_id("bad@id"),
            Err(Error::InvalidResourceId(_))
        ));
    }

    #[test]
    fn invalid_resource_id_path_traversal() {
        assert!(matches!(
            validate_resource_id(".."),
            Err(Error::InvalidResourceId(_))
        ));
    }

    // HA SID tests
    #[test]
    fn valid_ha_sids() {
        assert!(validate_ha_sid("vm:100").is_ok());
        assert!(validate_ha_sid("ct:200").is_ok());
        assert!(validate_ha_sid("vm:999999999").is_ok());
    }

    #[test]
    fn invalid_ha_sid_no_colon() {
        assert!(matches!(
            validate_ha_sid("vm100"),
            Err(Error::InvalidHaSid(_))
        ));
    }

    #[test]
    fn invalid_ha_sid_empty_parts() {
        assert!(matches!(
            validate_ha_sid(":100"),
            Err(Error::InvalidHaSid(_))
        ));
        assert!(matches!(
            validate_ha_sid("vm:"),
            Err(Error::InvalidHaSid(_))
        ));
    }

    #[test]
    fn invalid_ha_sid_non_digit_vmid() {
        assert!(matches!(
            validate_ha_sid("vm:abc"),
            Err(Error::InvalidHaSid(_))
        ));
    }

    #[test]
    fn invalid_ha_sid_special_chars_in_type() {
        assert!(matches!(
            validate_ha_sid("v/m:100"),
            Err(Error::InvalidHaSid(_))
        ));
    }

    /// `validate_user_id` does not reject `..` within the username part
    /// (e.g. `"user..name@pam"` is valid). This is safe because the `@`
    /// character in the URL path prevents path traversal attacks.
    #[test]
    fn user_id_allows_double_dots_in_username() {
        assert!(validate_user_id("user..name@pam").is_ok());
    }
}
