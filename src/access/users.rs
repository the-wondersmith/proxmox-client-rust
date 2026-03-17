use std::fmt;

use serde::{Deserialize, Serialize};

use crate::client::ProxmoxClient;
use crate::error::Result;
use crate::validation::{validate_token_id, validate_user_id};

/// A Proxmox VE user account.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct User {
    /// User ID in `user@realm` format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userid: Option<String>,

    /// Comment / description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Whether the account is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub enable: Option<bool>,

    /// Account expiration date (Unix epoch, 0 = never).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire: Option<i64>,

    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,

    /// Group memberships (comma-separated list).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<String>,

    /// SSH/TLS keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<String>,

    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,

    /// API tokens associated with this user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<serde_json::Value>,
}

/// Parameters for creating a new user.
#[derive(Clone, Default, Serialize)]
#[non_exhaustive]
pub struct UserCreateParams {
    /// User ID in `user@realm` format (required).
    pub userid: String,

    /// Comment / description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Whether the account is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub enable: Option<bool>,

    /// Account expiration date (Unix epoch, 0 = never).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire: Option<i64>,

    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,

    /// Group memberships (comma-separated list).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<String>,

    /// SSH/TLS keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<String>,

    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,

    /// Initial password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl fmt::Debug for UserCreateParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UserCreateParams")
            .field("userid", &self.userid)
            .field("comment", &self.comment)
            .field("email", &self.email)
            .field("enable", &self.enable)
            .field("expire", &self.expire)
            .field("firstname", &self.firstname)
            .field("groups", &self.groups)
            .field("keys", &self.keys)
            .field("lastname", &self.lastname)
            .field("password", &self.password.as_ref().map(|_| "<redacted>"))
            .finish()
    }
}

impl UserCreateParams {
    /// Creates a new `UserCreateParams` with the required fields.
    pub fn new(userid: impl Into<String>) -> Self {
        Self {
            userid: userid.into(),
            ..Default::default()
        }
    }
}

/// Parameters for updating an existing user.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct UserUpdateParams {
    /// Comment / description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Whether the account is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub enable: Option<bool>,

    /// Account expiration date (Unix epoch, 0 = never).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire: Option<i64>,

    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,

    /// Group memberships (comma-separated list).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<String>,

    /// SSH/TLS keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<String>,

    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
}

/// A user API token.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserToken {
    /// The token ID (e.g., `mytoken`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokenid: Option<String>,

    /// Comment / description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Expiration date (Unix epoch, 0 = never).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire: Option<i64>,

    /// Whether the token has separate privileges or inherits user privileges.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub privsep: Option<bool>,
}

/// Parameters for creating a new API token.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct UserTokenCreateParams {
    /// Comment / description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Expiration date (Unix epoch, 0 = never).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire: Option<i64>,

    /// Whether the token has separate privileges or inherits user privileges.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub privsep: Option<bool>,
}

/// Parameters for updating an existing API token.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct UserTokenUpdateParams {
    /// Comment / description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Expiration date (Unix epoch, 0 = never).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire: Option<i64>,

    /// Whether the token has separate privileges or inherits user privileges.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::option_bool_as_int")]
    pub privsep: Option<bool>,
}

/// Response when creating an API token, includes the secret value.
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TokenCreateResponse {
    /// Full token ID in `user@realm!tokenid` format.
    #[serde(rename = "full-tokenid", skip_serializing_if = "Option::is_none")]
    pub full_tokenid: Option<String>,

    /// Token metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<UserToken>,

    /// The API token secret value. Store this securely; it cannot be retrieved again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl fmt::Debug for TokenCreateResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TokenCreateResponse")
            .field("full_tokenid", &self.full_tokenid)
            .field("info", &self.info)
            .field("value", &self.value.as_ref().map(|_| "<redacted>"))
            .finish()
    }
}

impl ProxmoxClient {
    /// Lists all users.
    ///
    /// `GET /access/users`
    pub async fn list_users(&self) -> Result<Vec<User>> {
        self.get_parsed("/access/users", "users").await
    }

    /// Creates a new user.
    ///
    /// `POST /access/users`
    pub async fn create_user(&self, params: &UserCreateParams) -> Result<()> {
        validate_user_id(&params.userid)?;
        let response = self.post("/access/users")?.json(params).send().await?;
        Self::handle_error(response, "user creation").await?;
        Ok(())
    }

    /// Gets a single user by user ID (`user@realm` format).
    ///
    /// `GET /access/users/{userid}`
    pub async fn get_user(&self, userid: &str) -> Result<User> {
        validate_user_id(userid)?;
        self.get_parsed(
            &format!("/access/users/{userid}"),
            &format!("user {userid}"),
        )
        .await
    }

    /// Updates an existing user.
    ///
    /// `PUT /access/users/{userid}`
    pub async fn update_user(&self, userid: &str, params: &UserUpdateParams) -> Result<()> {
        validate_user_id(userid)?;
        let response = self
            .put(&format!("/access/users/{userid}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("user {userid}")).await?;
        Ok(())
    }

    /// Deletes a user.
    ///
    /// `DELETE /access/users/{userid}`
    pub async fn delete_user(&self, userid: &str) -> Result<()> {
        validate_user_id(userid)?;
        self.delete_void(
            &format!("/access/users/{userid}"),
            &format!("user {userid}"),
        )
        .await
    }

    /// Lists API tokens for a user.
    ///
    /// `GET /access/users/{userid}/token`
    pub async fn list_user_tokens(&self, userid: &str) -> Result<Vec<UserToken>> {
        validate_user_id(userid)?;
        self.get_parsed(
            &format!("/access/users/{userid}/token"),
            &format!("user {userid} tokens"),
        )
        .await
    }

    /// Creates a new API token for a user.
    ///
    /// `POST /access/users/{userid}/token/{tokenid}`
    ///
    /// Returns the token info including the secret value. The secret is only
    /// returned at creation time and cannot be retrieved later.
    pub async fn create_user_token(
        &self,
        userid: &str,
        tokenid: &str,
        params: &UserTokenCreateParams,
    ) -> Result<TokenCreateResponse> {
        validate_user_id(userid)?;
        validate_token_id(tokenid)?;
        self.post_parsed(
            &format!("/access/users/{userid}/token/{tokenid}"),
            params,
            &format!("user {userid} token {tokenid}"),
        )
        .await
    }

    /// Gets information about a specific API token.
    ///
    /// `GET /access/users/{userid}/token/{tokenid}`
    pub async fn get_user_token(&self, userid: &str, tokenid: &str) -> Result<UserToken> {
        validate_user_id(userid)?;
        validate_token_id(tokenid)?;
        self.get_parsed(
            &format!("/access/users/{userid}/token/{tokenid}"),
            &format!("user {userid} token {tokenid}"),
        )
        .await
    }

    /// Updates an existing API token.
    ///
    /// `PUT /access/users/{userid}/token/{tokenid}`
    pub async fn update_user_token(
        &self,
        userid: &str,
        tokenid: &str,
        params: &UserTokenUpdateParams,
    ) -> Result<()> {
        validate_user_id(userid)?;
        validate_token_id(tokenid)?;
        let response = self
            .put(&format!("/access/users/{userid}/token/{tokenid}"))?
            .json(params)
            .send()
            .await?;
        Self::handle_error(response, &format!("user {userid} token {tokenid}")).await?;
        Ok(())
    }

    /// Deletes an API token.
    ///
    /// `DELETE /access/users/{userid}/token/{tokenid}`
    pub async fn delete_user_token(&self, userid: &str, tokenid: &str) -> Result<()> {
        validate_user_id(userid)?;
        validate_token_id(tokenid)?;
        self.delete_void(
            &format!("/access/users/{userid}/token/{tokenid}"),
            &format!("user {userid} token {tokenid}"),
        )
        .await
    }

    /// Gets TFA types available for a user.
    ///
    /// `GET /access/users/{userid}/tfa`
    pub async fn get_user_tfa_types(&self, userid: &str) -> Result<serde_json::Value> {
        validate_user_id(userid)?;
        self.get_parsed(
            &format!("/access/users/{userid}/tfa"),
            &format!("user {userid} TFA types"),
        )
        .await
    }

    /// Unlocks a user's TFA authentication.
    ///
    /// `PUT /access/users/{userid}/unlock-tfa`
    pub async fn unlock_user_tfa(&self, userid: &str) -> Result<()> {
        validate_user_id(userid)?;
        let response = self
            .put(&format!("/access/users/{userid}/unlock-tfa"))?
            .send()
            .await?;
        Self::handle_error(response, &format!("user {userid} unlock TFA")).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_serde_roundtrip() {
        let user = User {
            userid: Some("admin@pve".to_string()),
            comment: Some("Administrator".to_string()),
            email: Some("admin@example.com".to_string()),
            enable: Some(true),
            expire: Some(0),
            firstname: Some("Admin".to_string()),
            groups: Some("admins,operators".to_string()),
            keys: None,
            lastname: Some("User".to_string()),
            tokens: None,
        };

        let json = serde_json::to_string(&user).unwrap();
        let deserialized: User = serde_json::from_str(&json).unwrap();
        assert_eq!(user, deserialized);
    }

    #[test]
    fn user_skip_serializing_none() {
        let user = User::default();
        let json = serde_json::to_value(&user).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty(), "default User should serialize to {{}}");
    }

    #[test]
    fn user_deserialize_from_api() {
        let json = r#"{
            "userid": "root@pam",
            "comment": "Root user",
            "email": "root@example.com",
            "enable": 1,
            "expire": 0,
            "firstname": "Root",
            "groups": "",
            "keys": "",
            "lastname": "Admin"
        }"#;
        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.userid.as_deref(), Some("root@pam"));
        assert_eq!(user.enable, Some(true));
        assert_eq!(user.expire, Some(0));
        assert_eq!(user.firstname.as_deref(), Some("Root"));
    }

    #[test]
    fn user_unknown_fields_ignored() {
        let json = r#"{"userid": "test@pam", "unknownField": 42}"#;
        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.userid.as_deref(), Some("test@pam"));
    }

    #[test]
    fn user_create_params_serialization() {
        let mut params = UserCreateParams::new("newuser@pve");
        params.comment = Some("New user".to_string());
        params.email = Some("new@example.com".to_string());
        params.enable = Some(true);
        params.password = Some("secret123".to_string());
        let params = params;
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["userid"], "newuser@pve");
        assert_eq!(json["comment"], "New user");
        assert_eq!(json["email"], "new@example.com");
        assert_eq!(json["enable"], 1);
        assert_eq!(json["password"], "secret123");
        assert!(!json.as_object().unwrap().contains_key("expire"));
        assert!(!json.as_object().unwrap().contains_key("firstname"));
    }

    #[test]
    fn user_update_params_serialization() {
        let params = UserUpdateParams {
            comment: Some("Updated comment".to_string()),
            enable: Some(false),
            ..Default::default()
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["comment"], "Updated comment");
        assert_eq!(json["enable"], 0);
        assert!(!json.as_object().unwrap().contains_key("email"));
    }

    #[test]
    fn user_token_serde_roundtrip() {
        let token = UserToken {
            tokenid: Some("mytoken".to_string()),
            comment: Some("CI token".to_string()),
            expire: Some(0),
            privsep: Some(true),
        };

        let json = serde_json::to_string(&token).unwrap();
        let deserialized: UserToken = serde_json::from_str(&json).unwrap();
        assert_eq!(token, deserialized);
    }

    #[test]
    fn user_token_skip_serializing_none() {
        let token = UserToken::default();
        let json = serde_json::to_value(&token).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty(), "default UserToken should serialize to {{}}");
    }

    #[test]
    fn token_create_response_serde_roundtrip() {
        let resp = TokenCreateResponse {
            full_tokenid: Some("root@pam!mytoken".to_string()),
            info: Some(UserToken {
                tokenid: Some("mytoken".to_string()),
                comment: Some("test".to_string()),
                expire: Some(0),
                privsep: Some(true),
            }),
            value: Some("xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx".to_string()),
        };

        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: TokenCreateResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn token_create_response_full_tokenid_rename() {
        let resp = TokenCreateResponse {
            full_tokenid: Some("root@pam!mytoken".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_value(&resp).unwrap();
        assert!(
            json.get("full-tokenid").is_some(),
            "full_tokenid must serialize as full-tokenid"
        );
        assert!(
            json.get("full_tokenid").is_none(),
            "must not serialize as full_tokenid"
        );
    }

    #[test]
    fn token_create_response_deserialize_from_api() {
        let json = r#"{
            "full-tokenid": "root@pam!automation",
            "info": {
                "tokenid": "automation",
                "comment": "Automation token",
                "expire": 0,
                "privsep": 1
            },
            "value": "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee"
        }"#;
        let resp: TokenCreateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.full_tokenid.as_deref(), Some("root@pam!automation"));
        assert_eq!(
            resp.value.as_deref(),
            Some("aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee")
        );
        let info = resp.info.unwrap();
        assert_eq!(info.tokenid.as_deref(), Some("automation"));
        assert_eq!(info.privsep, Some(true));
    }

    #[test]
    fn token_create_response_debug_redacts_value() {
        let resp = TokenCreateResponse {
            full_tokenid: Some("root@pam!mytoken".to_string()),
            info: None,
            value: Some("super-secret-token-value-12345".to_string()),
        };
        let debug = format!("{resp:?}");
        assert!(
            !debug.contains("super-secret-token-value-12345"),
            "value must be redacted in Debug output"
        );
        assert!(debug.contains("<redacted>"));
        assert!(
            debug.contains("root@pam!mytoken"),
            "full_tokenid should remain visible"
        );
    }

    #[test]
    fn user_token_create_params_serialization() {
        let params = UserTokenCreateParams {
            comment: Some("New token".to_string()),
            expire: Some(0),
            privsep: Some(false),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["comment"], "New token");
        assert_eq!(json["expire"], 0);
        assert_eq!(json["privsep"], 0);
    }

    #[test]
    fn user_create_params_debug_redacts_password() {
        let mut params = UserCreateParams::new("newuser@pve");
        params.password = Some("user-secret-pass".to_string());
        let params = params;
        let debug = format!("{params:?}");
        assert!(
            !debug.contains("user-secret-pass"),
            "password must be redacted in Debug output"
        );
        assert!(debug.contains("<redacted>"));
    }
}
