#![allow(unused_qualifications)]

use validator::Validate;

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

/// Account represents a user account within the workspace service. It serves as the top-level container for all user-specific workspaces and settings.  Key features: - Unique identification via Auth0 - Organization and tenant context - Audit timestamps - Associated scraping jobs  Database considerations: - Uses GORM for ORM mapping - Includes indexes for efficient querying - Supports soft deletes via deleted_at  Usage example: ```go account := &Account{     AuthPlatformUserId: \"auth0|123\",     OrgId: \"org_123\",     TenantId: \"tenant_456\", } ```
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Account {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "authPlatformUserId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auth_platform_user_id: Option<String>,

    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub org_id: Option<String>,

    #[serde(rename = "tenantId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tenant_id: Option<String>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "accountStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub account_status: Option<models::AccountStatus>,

    #[serde(rename = "roles")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub roles: Option<Vec<String>>,

    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<Vec<String>>,

    #[serde(rename = "mfaEnabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mfa_enabled: Option<bool>,

    #[serde(rename = "lastLoginAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_login_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "timezone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timezone: Option<String>,

    #[serde(rename = "totalJobsRun")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_jobs_run: Option<i32>,

    #[serde(rename = "monthlyJobLimit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub monthly_job_limit: Option<i32>,

    #[serde(rename = "concurrentJobLimit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub concurrent_job_limit: Option<i32>,

    #[serde(rename = "workspaces")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workspaces: Option<Vec<models::Workspace>>,

    #[serde(rename = "settings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub settings: Option<models::AccountSettings>,

}


impl Account {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Account {
        Account {
            id: None,
            auth_platform_user_id: None,
            org_id: None,
            tenant_id: None,
            email: None,
            deleted_at: None,
            created_at: None,
            account_status: None,
            roles: None,
            permissions: None,
            mfa_enabled: None,
            last_login_at: None,
            timezone: None,
            total_jobs_run: None,
            monthly_job_limit: None,
            concurrent_job_limit: None,
            workspaces: None,
            settings: None,
        }
    }
}

/// Converts the Account value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Account {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.id.as_ref().map(|id| {
                [
                    "id".to_string(),
                    id.to_string(),
                ].join(",")
            }),


            self.auth_platform_user_id.as_ref().map(|auth_platform_user_id| {
                [
                    "authPlatformUserId".to_string(),
                    auth_platform_user_id.to_string(),
                ].join(",")
            }),


            self.org_id.as_ref().map(|org_id| {
                [
                    "orgId".to_string(),
                    org_id.to_string(),
                ].join(",")
            }),


            self.tenant_id.as_ref().map(|tenant_id| {
                [
                    "tenantId".to_string(),
                    tenant_id.to_string(),
                ].join(",")
            }),


            self.email.as_ref().map(|email| {
                [
                    "email".to_string(),
                    email.to_string(),
                ].join(",")
            }),

            // Skipping deletedAt in query parameter serialization

            // Skipping createdAt in query parameter serialization

            // Skipping accountStatus in query parameter serialization


            self.roles.as_ref().map(|roles| {
                [
                    "roles".to_string(),
                    roles.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.permissions.as_ref().map(|permissions| {
                [
                    "permissions".to_string(),
                    permissions.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.mfa_enabled.as_ref().map(|mfa_enabled| {
                [
                    "mfaEnabled".to_string(),
                    mfa_enabled.to_string(),
                ].join(",")
            }),

            // Skipping lastLoginAt in query parameter serialization


            self.timezone.as_ref().map(|timezone| {
                [
                    "timezone".to_string(),
                    timezone.to_string(),
                ].join(",")
            }),


            self.total_jobs_run.as_ref().map(|total_jobs_run| {
                [
                    "totalJobsRun".to_string(),
                    total_jobs_run.to_string(),
                ].join(",")
            }),


            self.monthly_job_limit.as_ref().map(|monthly_job_limit| {
                [
                    "monthlyJobLimit".to_string(),
                    monthly_job_limit.to_string(),
                ].join(",")
            }),


            self.concurrent_job_limit.as_ref().map(|concurrent_job_limit| {
                [
                    "concurrentJobLimit".to_string(),
                    concurrent_job_limit.to_string(),
                ].join(",")
            }),

            // Skipping workspaces in query parameter serialization

            // Skipping settings in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Account value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Account {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub auth_platform_user_id: Vec<String>,
            pub org_id: Vec<String>,
            pub tenant_id: Vec<String>,
            pub email: Vec<String>,
            pub deleted_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub account_status: Vec<models::AccountStatus>,
            pub roles: Vec<Vec<String>>,
            pub permissions: Vec<Vec<String>>,
            pub mfa_enabled: Vec<bool>,
            pub last_login_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub timezone: Vec<String>,
            pub total_jobs_run: Vec<i32>,
            pub monthly_job_limit: Vec<i32>,
            pub concurrent_job_limit: Vec<i32>,
            pub workspaces: Vec<Vec<models::Workspace>>,
            pub settings: Vec<models::AccountSettings>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Account".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "authPlatformUserId" => intermediate_rep.auth_platform_user_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "orgId" => intermediate_rep.org_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "tenantId" => intermediate_rep.tenant_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "email" => intermediate_rep.email.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "deletedAt" => intermediate_rep.deleted_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "createdAt" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "accountStatus" => intermediate_rep.account_status.push(<models::AccountStatus as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "roles" => return std::result::Result::Err("Parsing a container in this style is not supported in Account".to_string()),
                    "permissions" => return std::result::Result::Err("Parsing a container in this style is not supported in Account".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "mfaEnabled" => intermediate_rep.mfa_enabled.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "lastLoginAt" => intermediate_rep.last_login_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "timezone" => intermediate_rep.timezone.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "totalJobsRun" => intermediate_rep.total_jobs_run.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "monthlyJobLimit" => intermediate_rep.monthly_job_limit.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "concurrentJobLimit" => intermediate_rep.concurrent_job_limit.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "workspaces" => return std::result::Result::Err("Parsing a container in this style is not supported in Account".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "settings" => intermediate_rep.settings.push(<models::AccountSettings as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Account".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Account {
            id: intermediate_rep.id.into_iter().next(),
            auth_platform_user_id: intermediate_rep.auth_platform_user_id.into_iter().next(),
            org_id: intermediate_rep.org_id.into_iter().next(),
            tenant_id: intermediate_rep.tenant_id.into_iter().next(),
            email: intermediate_rep.email.into_iter().next(),
            deleted_at: intermediate_rep.deleted_at.into_iter().next(),
            created_at: intermediate_rep.created_at.into_iter().next(),
            account_status: intermediate_rep.account_status.into_iter().next(),
            roles: intermediate_rep.roles.into_iter().next(),
            permissions: intermediate_rep.permissions.into_iter().next(),
            mfa_enabled: intermediate_rep.mfa_enabled.into_iter().next(),
            last_login_at: intermediate_rep.last_login_at.into_iter().next(),
            timezone: intermediate_rep.timezone.into_iter().next(),
            total_jobs_run: intermediate_rep.total_jobs_run.into_iter().next(),
            monthly_job_limit: intermediate_rep.monthly_job_limit.into_iter().next(),
            concurrent_job_limit: intermediate_rep.concurrent_job_limit.into_iter().next(),
            workspaces: intermediate_rep.workspaces.into_iter().next(),
            settings: intermediate_rep.settings.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Account> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Account>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Account>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Account - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Account> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Account as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Account - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AccountSettings {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "emailNotifications")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email_notifications: Option<bool>,

    #[serde(rename = "slackNotifications")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub slack_notifications: Option<bool>,

    #[serde(rename = "defaultDataRetention")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_data_retention: Option<String>,

    #[serde(rename = "autoPurgeEnabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_purge_enabled: Option<bool>,

    #[serde(rename = "require2fa")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub require2fa: Option<bool>,

    #[serde(rename = "sessionTimeout")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub session_timeout: Option<String>,

    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_at: Option<chrono::DateTime::<chrono::Utc>>,

}


impl AccountSettings {
    #[allow(clippy::new_without_default)]
    pub fn new() -> AccountSettings {
        AccountSettings {
            id: None,
            email_notifications: None,
            slack_notifications: None,
            default_data_retention: None,
            auto_purge_enabled: None,
            require2fa: None,
            session_timeout: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }
}

/// Converts the AccountSettings value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AccountSettings {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.id.as_ref().map(|id| {
                [
                    "id".to_string(),
                    id.to_string(),
                ].join(",")
            }),


            self.email_notifications.as_ref().map(|email_notifications| {
                [
                    "emailNotifications".to_string(),
                    email_notifications.to_string(),
                ].join(",")
            }),


            self.slack_notifications.as_ref().map(|slack_notifications| {
                [
                    "slackNotifications".to_string(),
                    slack_notifications.to_string(),
                ].join(",")
            }),


            self.default_data_retention.as_ref().map(|default_data_retention| {
                [
                    "defaultDataRetention".to_string(),
                    default_data_retention.to_string(),
                ].join(",")
            }),


            self.auto_purge_enabled.as_ref().map(|auto_purge_enabled| {
                [
                    "autoPurgeEnabled".to_string(),
                    auto_purge_enabled.to_string(),
                ].join(",")
            }),


            self.require2fa.as_ref().map(|require2fa| {
                [
                    "require2fa".to_string(),
                    require2fa.to_string(),
                ].join(",")
            }),


            self.session_timeout.as_ref().map(|session_timeout| {
                [
                    "sessionTimeout".to_string(),
                    session_timeout.to_string(),
                ].join(",")
            }),

            // Skipping createdAt in query parameter serialization

            // Skipping updatedAt in query parameter serialization

            // Skipping deletedAt in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AccountSettings value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AccountSettings {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub email_notifications: Vec<bool>,
            pub slack_notifications: Vec<bool>,
            pub default_data_retention: Vec<String>,
            pub auto_purge_enabled: Vec<bool>,
            pub require2fa: Vec<bool>,
            pub session_timeout: Vec<String>,
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub updated_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub deleted_at: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AccountSettings".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "emailNotifications" => intermediate_rep.email_notifications.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "slackNotifications" => intermediate_rep.slack_notifications.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "defaultDataRetention" => intermediate_rep.default_data_retention.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "autoPurgeEnabled" => intermediate_rep.auto_purge_enabled.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "require2fa" => intermediate_rep.require2fa.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "sessionTimeout" => intermediate_rep.session_timeout.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "createdAt" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "updatedAt" => intermediate_rep.updated_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "deletedAt" => intermediate_rep.deleted_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AccountSettings".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AccountSettings {
            id: intermediate_rep.id.into_iter().next(),
            email_notifications: intermediate_rep.email_notifications.into_iter().next(),
            slack_notifications: intermediate_rep.slack_notifications.into_iter().next(),
            default_data_retention: intermediate_rep.default_data_retention.into_iter().next(),
            auto_purge_enabled: intermediate_rep.auto_purge_enabled.into_iter().next(),
            require2fa: intermediate_rep.require2fa.into_iter().next(),
            session_timeout: intermediate_rep.session_timeout.into_iter().next(),
            created_at: intermediate_rep.created_at.into_iter().next(),
            updated_at: intermediate_rep.updated_at.into_iter().next(),
            deleted_at: intermediate_rep.deleted_at.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AccountSettings> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AccountSettings>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AccountSettings>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AccountSettings - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AccountSettings> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AccountSettings as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AccountSettings - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum AccountStatus {
    #[serde(rename = "ACCOUNT_STATUS_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "ACCOUNT_STATUS_ACTIVE")]
    Active,
    #[serde(rename = "ACCOUNT_STATUS_SUSPENDED")]
    Suspended,
    #[serde(rename = "ACCOUNT_STATUS_PENDING_VERIFICATION")]
    PendingVerification,
}

impl std::fmt::Display for AccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            AccountStatus::Unspecified => write!(f, "ACCOUNT_STATUS_UNSPECIFIED"),
            AccountStatus::Active => write!(f, "ACCOUNT_STATUS_ACTIVE"),
            AccountStatus::Suspended => write!(f, "ACCOUNT_STATUS_SUSPENDED"),
            AccountStatus::PendingVerification => write!(f, "ACCOUNT_STATUS_PENDING_VERIFICATION"),
        }
    }
}

impl std::str::FromStr for AccountStatus {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "ACCOUNT_STATUS_UNSPECIFIED" => std::result::Result::Ok(AccountStatus::Unspecified),
            "ACCOUNT_STATUS_ACTIVE" => std::result::Result::Ok(AccountStatus::Active),
            "ACCOUNT_STATUS_SUSPENDED" => std::result::Result::Ok(AccountStatus::Suspended),
            "ACCOUNT_STATUS_PENDING_VERIFICATION" => std::result::Result::Ok(AccountStatus::PendingVerification),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// `Any` contains an arbitrary serialized protocol buffer message along with a URL that describes the type of the serialized message.  Protobuf library provides support to pack/unpack Any values in the form of utility functions or additional generated methods of the Any type.  Example 1: Pack and unpack a message in C++.      Foo foo = ...;     Any any;     any.PackFrom(foo);     ...     if (any.UnpackTo(&foo)) {       ...     }  Example 2: Pack and unpack a message in Java.      Foo foo = ...;     Any any = Any.pack(foo);     ...     if (any.is(Foo.class)) {       foo = any.unpack(Foo.class);     }  Example 3: Pack and unpack a message in Python.      foo = Foo(...)     any = Any()     any.Pack(foo)     ...     if any.Is(Foo.DESCRIPTOR):       any.Unpack(foo)       ...  Example 4: Pack and unpack a message in Go       foo := &pb.Foo{...}      any, err := anypb.New(foo)      if err != nil {        ...      }      ...      foo := &pb.Foo{}      if err := any.UnmarshalTo(foo); err != nil {        ...      }  The pack methods provided by protobuf library will by default use 'type.googleapis.com/full.type.name' as the type URL and the unpack methods only use the fully qualified type name after the last '/' in the type URL, for example \"foo.bar.com/x/y.z\" will yield type name \"y.z\".   JSON  The JSON representation of an `Any` value uses the regular representation of the deserialized, embedded message, with an additional field `@type` which contains the type URL. Example:      package google.profile;     message Person {       string first_name = 1;       string last_name = 2;     }      {       \"@type\": \"type.googleapis.com/google.profile.Person\",       \"firstName\": <string>,       \"lastName\": <string>     }  If the embedded message type is well-known and has a custom JSON representation, that representation will be embedded adding a field `value` which holds the custom JSON in addition to the `@type` field. Example (for message [google.protobuf.Duration][]):      {       \"@type\": \"type.googleapis.com/google.protobuf.Duration\",       \"value\": \"1.212s\"     }
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Any {
    /// A URL/resource name that uniquely identifies the type of the serialized protocol buffer message. This string must contain at least one \"/\" character. The last segment of the URL's path must represent the fully qualified name of the type (as in `path/google.protobuf.Duration`). The name should be in a canonical form (e.g., leading \".\" is not accepted).  In practice, teams usually precompile into the binary all types that they expect it to use in the context of Any. However, for URLs which use the scheme `http`, `https`, or no scheme, one can optionally set up a type server that maps type URLs to message definitions as follows:  * If no scheme is provided, `https` is assumed. * An HTTP GET on the URL must yield a [google.protobuf.Type][]   value in binary format, or produce an error. * Applications are allowed to cache lookup results based on the   URL, or have them precompiled into a binary to avoid any   lookup. Therefore, binary compatibility needs to be preserved   on changes to types. (Use versioned type names to manage   breaking changes.)  Note: this functionality is not currently available in the official protobuf release, and it is not used for type URLs beginning with type.googleapis.com.  Schemes other than `http`, `https` (or the empty scheme) might be used with implementation specific semantics.
    #[serde(rename = "@type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub at_type: Option<String>,

}


impl Any {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Any {
        Any {
            at_type: None,
        }
    }
}

/// Converts the Any value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Any {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.at_type.as_ref().map(|at_type| {
                [
                    "@type".to_string(),
                    at_type.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Any value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Any {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub at_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Any".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "@type" => intermediate_rep.at_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Any".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Any {
            at_type: intermediate_rep.at_type.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Any> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Any>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Any>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Any - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Any> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Any as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Any - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApiInfo {
    #[serde(rename = "version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<String>,

    #[serde(rename = "supportedVersions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub supported_versions: Option<Vec<String>>,

    #[serde(rename = "isDeprecated")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_deprecated: Option<bool>,

    #[serde(rename = "sunsetDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sunset_date: Option<chrono::DateTime::<chrono::Utc>>,

}


impl ApiInfo {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ApiInfo {
        ApiInfo {
            version: None,
            supported_versions: None,
            is_deprecated: None,
            sunset_date: None,
        }
    }
}

/// Converts the ApiInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ApiInfo {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.version.as_ref().map(|version| {
                [
                    "version".to_string(),
                    version.to_string(),
                ].join(",")
            }),


            self.supported_versions.as_ref().map(|supported_versions| {
                [
                    "supportedVersions".to_string(),
                    supported_versions.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.is_deprecated.as_ref().map(|is_deprecated| {
                [
                    "isDeprecated".to_string(),
                    is_deprecated.to_string(),
                ].join(",")
            }),

            // Skipping sunsetDate in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ApiInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ApiInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub version: Vec<String>,
            pub supported_versions: Vec<Vec<String>>,
            pub is_deprecated: Vec<bool>,
            pub sunset_date: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ApiInfo".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "version" => intermediate_rep.version.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "supportedVersions" => return std::result::Result::Err("Parsing a container in this style is not supported in ApiInfo".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "isDeprecated" => intermediate_rep.is_deprecated.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "sunsetDate" => intermediate_rep.sunset_date.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ApiInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ApiInfo {
            version: intermediate_rep.version.into_iter().next(),
            supported_versions: intermediate_rep.supported_versions.into_iter().next(),
            is_deprecated: intermediate_rep.is_deprecated.into_iter().next(),
            sunset_date: intermediate_rep.sunset_date.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ApiInfo> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ApiInfo>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ApiInfo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ApiInfo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ApiInfo> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ApiInfo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ApiInfo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AuthContext {
    #[serde(rename = "requiredScopes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_scopes: Option<Vec<String>>,

    #[serde(rename = "providedScopes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provided_scopes: Option<Vec<String>>,

    #[serde(rename = "missingPermissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub missing_permissions: Option<Vec<String>>,

    #[serde(rename = "tenantId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tenant_id: Option<String>,

}


impl AuthContext {
    #[allow(clippy::new_without_default)]
    pub fn new() -> AuthContext {
        AuthContext {
            required_scopes: None,
            provided_scopes: None,
            missing_permissions: None,
            tenant_id: None,
        }
    }
}

/// Converts the AuthContext value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AuthContext {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.required_scopes.as_ref().map(|required_scopes| {
                [
                    "requiredScopes".to_string(),
                    required_scopes.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.provided_scopes.as_ref().map(|provided_scopes| {
                [
                    "providedScopes".to_string(),
                    provided_scopes.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.missing_permissions.as_ref().map(|missing_permissions| {
                [
                    "missingPermissions".to_string(),
                    missing_permissions.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.tenant_id.as_ref().map(|tenant_id| {
                [
                    "tenantId".to_string(),
                    tenant_id.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AuthContext value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AuthContext {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub required_scopes: Vec<Vec<String>>,
            pub provided_scopes: Vec<Vec<String>>,
            pub missing_permissions: Vec<Vec<String>>,
            pub tenant_id: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AuthContext".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "requiredScopes" => return std::result::Result::Err("Parsing a container in this style is not supported in AuthContext".to_string()),
                    "providedScopes" => return std::result::Result::Err("Parsing a container in this style is not supported in AuthContext".to_string()),
                    "missingPermissions" => return std::result::Result::Err("Parsing a container in this style is not supported in AuthContext".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "tenantId" => intermediate_rep.tenant_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AuthContext".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AuthContext {
            required_scopes: intermediate_rep.required_scopes.into_iter().next(),
            provided_scopes: intermediate_rep.provided_scopes.into_iter().next(),
            missing_permissions: intermediate_rep.missing_permissions.into_iter().next(),
            tenant_id: intermediate_rep.tenant_id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AuthContext> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AuthContext>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AuthContext>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AuthContext - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AuthContext> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AuthContext as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AuthContext - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// - AUTH_FAILED_INVALID_BEARER_TOKEN: Authentication errors  - SESSION_EXPIRED: Session errors  - UNAUTHENTICATED: Other authentication errors  - ACCOUNT_LOCKED: New authentication error codes  Account is locked due to too many failed login attempts  - ACCOUNT_DISABLED: Account has been disabled by admin  - PASSWORD_EXPIRED: Password has expired and must be changed  - PASSWORD_RESET_REQUIRED: Password reset is required  - UNRECOGNIZED_DEVICE: Login attempt from an unrecognized device
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum AuthErrorCode {
    #[serde(rename = "NO_AUTH_ERROR")]
    NoAuthError,
    #[serde(rename = "AUTH_FAILED_INVALID_BEARER_TOKEN")]
    AuthFailedInvalidBearerToken,
    #[serde(rename = "AUTH_FAILED_INVALID_SUBJECT")]
    AuthFailedInvalidSubject,
    #[serde(rename = "AUTH_FAILED_INVALID_AUDIENCE")]
    AuthFailedInvalidAudience,
    #[serde(rename = "AUTH_FAILED_INVALID_ISSUER")]
    AuthFailedInvalidIssuer,
    #[serde(rename = "BEARER_TOKEN_MISSING")]
    BearerTokenMissing,
    #[serde(rename = "TOKEN_EXPIRED")]
    TokenExpired,
    #[serde(rename = "TOKEN_NOT_ACTIVE")]
    TokenNotActive,
    #[serde(rename = "TOKEN_REVOKED")]
    TokenRevoked,
    #[serde(rename = "INVALID_CLAIMS")]
    InvalidClaims,
    #[serde(rename = "MISSING_REQUIRED_CLAIMS")]
    MissingRequiredClaims,
    #[serde(rename = "INVALID_SCOPE")]
    InvalidScope,
    #[serde(rename = "INVALID_PERMISSIONS")]
    InvalidPermissions,
    #[serde(rename = "SESSION_EXPIRED")]
    SessionExpired,
    #[serde(rename = "SESSION_INVALID")]
    SessionInvalid,
    #[serde(rename = "SESSION_REVOKED")]
    SessionRevoked,
    #[serde(rename = "UNAUTHENTICATED")]
    Unauthenticated,
    #[serde(rename = "MULTI_FACTOR_REQUIRED")]
    MultiFactorRequired,
    #[serde(rename = "MULTI_FACTOR_FAILED")]
    MultiFactorFailed,
    #[serde(rename = "ACCOUNT_LOCKED")]
    AccountLocked,
    #[serde(rename = "ACCOUNT_DISABLED")]
    AccountDisabled,
    #[serde(rename = "PASSWORD_EXPIRED")]
    PasswordExpired,
    #[serde(rename = "PASSWORD_RESET_REQUIRED")]
    PasswordResetRequired,
    #[serde(rename = "UNRECOGNIZED_DEVICE")]
    UnrecognizedDevice,
}

impl std::fmt::Display for AuthErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            AuthErrorCode::NoAuthError => write!(f, "NO_AUTH_ERROR"),
            AuthErrorCode::AuthFailedInvalidBearerToken => write!(f, "AUTH_FAILED_INVALID_BEARER_TOKEN"),
            AuthErrorCode::AuthFailedInvalidSubject => write!(f, "AUTH_FAILED_INVALID_SUBJECT"),
            AuthErrorCode::AuthFailedInvalidAudience => write!(f, "AUTH_FAILED_INVALID_AUDIENCE"),
            AuthErrorCode::AuthFailedInvalidIssuer => write!(f, "AUTH_FAILED_INVALID_ISSUER"),
            AuthErrorCode::BearerTokenMissing => write!(f, "BEARER_TOKEN_MISSING"),
            AuthErrorCode::TokenExpired => write!(f, "TOKEN_EXPIRED"),
            AuthErrorCode::TokenNotActive => write!(f, "TOKEN_NOT_ACTIVE"),
            AuthErrorCode::TokenRevoked => write!(f, "TOKEN_REVOKED"),
            AuthErrorCode::InvalidClaims => write!(f, "INVALID_CLAIMS"),
            AuthErrorCode::MissingRequiredClaims => write!(f, "MISSING_REQUIRED_CLAIMS"),
            AuthErrorCode::InvalidScope => write!(f, "INVALID_SCOPE"),
            AuthErrorCode::InvalidPermissions => write!(f, "INVALID_PERMISSIONS"),
            AuthErrorCode::SessionExpired => write!(f, "SESSION_EXPIRED"),
            AuthErrorCode::SessionInvalid => write!(f, "SESSION_INVALID"),
            AuthErrorCode::SessionRevoked => write!(f, "SESSION_REVOKED"),
            AuthErrorCode::Unauthenticated => write!(f, "UNAUTHENTICATED"),
            AuthErrorCode::MultiFactorRequired => write!(f, "MULTI_FACTOR_REQUIRED"),
            AuthErrorCode::MultiFactorFailed => write!(f, "MULTI_FACTOR_FAILED"),
            AuthErrorCode::AccountLocked => write!(f, "ACCOUNT_LOCKED"),
            AuthErrorCode::AccountDisabled => write!(f, "ACCOUNT_DISABLED"),
            AuthErrorCode::PasswordExpired => write!(f, "PASSWORD_EXPIRED"),
            AuthErrorCode::PasswordResetRequired => write!(f, "PASSWORD_RESET_REQUIRED"),
            AuthErrorCode::UnrecognizedDevice => write!(f, "UNRECOGNIZED_DEVICE"),
        }
    }
}

impl std::str::FromStr for AuthErrorCode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "NO_AUTH_ERROR" => std::result::Result::Ok(AuthErrorCode::NoAuthError),
            "AUTH_FAILED_INVALID_BEARER_TOKEN" => std::result::Result::Ok(AuthErrorCode::AuthFailedInvalidBearerToken),
            "AUTH_FAILED_INVALID_SUBJECT" => std::result::Result::Ok(AuthErrorCode::AuthFailedInvalidSubject),
            "AUTH_FAILED_INVALID_AUDIENCE" => std::result::Result::Ok(AuthErrorCode::AuthFailedInvalidAudience),
            "AUTH_FAILED_INVALID_ISSUER" => std::result::Result::Ok(AuthErrorCode::AuthFailedInvalidIssuer),
            "BEARER_TOKEN_MISSING" => std::result::Result::Ok(AuthErrorCode::BearerTokenMissing),
            "TOKEN_EXPIRED" => std::result::Result::Ok(AuthErrorCode::TokenExpired),
            "TOKEN_NOT_ACTIVE" => std::result::Result::Ok(AuthErrorCode::TokenNotActive),
            "TOKEN_REVOKED" => std::result::Result::Ok(AuthErrorCode::TokenRevoked),
            "INVALID_CLAIMS" => std::result::Result::Ok(AuthErrorCode::InvalidClaims),
            "MISSING_REQUIRED_CLAIMS" => std::result::Result::Ok(AuthErrorCode::MissingRequiredClaims),
            "INVALID_SCOPE" => std::result::Result::Ok(AuthErrorCode::InvalidScope),
            "INVALID_PERMISSIONS" => std::result::Result::Ok(AuthErrorCode::InvalidPermissions),
            "SESSION_EXPIRED" => std::result::Result::Ok(AuthErrorCode::SessionExpired),
            "SESSION_INVALID" => std::result::Result::Ok(AuthErrorCode::SessionInvalid),
            "SESSION_REVOKED" => std::result::Result::Ok(AuthErrorCode::SessionRevoked),
            "UNAUTHENTICATED" => std::result::Result::Ok(AuthErrorCode::Unauthenticated),
            "MULTI_FACTOR_REQUIRED" => std::result::Result::Ok(AuthErrorCode::MultiFactorRequired),
            "MULTI_FACTOR_FAILED" => std::result::Result::Ok(AuthErrorCode::MultiFactorFailed),
            "ACCOUNT_LOCKED" => std::result::Result::Ok(AuthErrorCode::AccountLocked),
            "ACCOUNT_DISABLED" => std::result::Result::Ok(AuthErrorCode::AccountDisabled),
            "PASSWORD_EXPIRED" => std::result::Result::Ok(AuthErrorCode::PasswordExpired),
            "PASSWORD_RESET_REQUIRED" => std::result::Result::Ok(AuthErrorCode::PasswordResetRequired),
            "UNRECOGNIZED_DEVICE" => std::result::Result::Ok(AuthErrorCode::UnrecognizedDevice),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// Represents authentication and authorization failures
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AuthenticationErrorMessageResponse {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<models::AuthErrorCode>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "tokenInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token_info: Option<models::TokenInfo>,

    #[serde(rename = "authContext")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auth_context: Option<models::AuthContext>,

    #[serde(rename = "sessionInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub session_info: Option<models::SessionInfo>,

    #[serde(rename = "mfaInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mfa_info: Option<models::MfaInfo>,

    #[serde(rename = "errorResponse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_response: Option<models::ErrorResponse>,

}


impl AuthenticationErrorMessageResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> AuthenticationErrorMessageResponse {
        AuthenticationErrorMessageResponse {
            code: None,
            message: None,
            token_info: None,
            auth_context: None,
            session_info: None,
            mfa_info: None,
            error_response: None,
        }
    }
}

/// Converts the AuthenticationErrorMessageResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AuthenticationErrorMessageResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping code in query parameter serialization


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),

            // Skipping tokenInfo in query parameter serialization

            // Skipping authContext in query parameter serialization

            // Skipping sessionInfo in query parameter serialization

            // Skipping mfaInfo in query parameter serialization

            // Skipping errorResponse in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AuthenticationErrorMessageResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AuthenticationErrorMessageResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<models::AuthErrorCode>,
            pub message: Vec<String>,
            pub token_info: Vec<models::TokenInfo>,
            pub auth_context: Vec<models::AuthContext>,
            pub session_info: Vec<models::SessionInfo>,
            pub mfa_info: Vec<models::MfaInfo>,
            pub error_response: Vec<models::ErrorResponse>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AuthenticationErrorMessageResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(<models::AuthErrorCode as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "tokenInfo" => intermediate_rep.token_info.push(<models::TokenInfo as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "authContext" => intermediate_rep.auth_context.push(<models::AuthContext as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "sessionInfo" => intermediate_rep.session_info.push(<models::SessionInfo as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "mfaInfo" => intermediate_rep.mfa_info.push(<models::MfaInfo as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "errorResponse" => intermediate_rep.error_response.push(<models::ErrorResponse as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AuthenticationErrorMessageResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AuthenticationErrorMessageResponse {
            code: intermediate_rep.code.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
            token_info: intermediate_rep.token_info.into_iter().next(),
            auth_context: intermediate_rep.auth_context.into_iter().next(),
            session_info: intermediate_rep.session_info.into_iter().next(),
            mfa_info: intermediate_rep.mfa_info.into_iter().next(),
            error_response: intermediate_rep.error_response.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AuthenticationErrorMessageResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AuthenticationErrorMessageResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AuthenticationErrorMessageResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AuthenticationErrorMessageResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AuthenticationErrorMessageResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AuthenticationErrorMessageResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AuthenticationErrorMessageResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AvailabilityInfo {
    #[serde(rename = "retryAfter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub retry_after: Option<i32>,

    #[serde(rename = "maintenanceWindow")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maintenance_window: Option<String>,

    #[serde(rename = "affectedServices")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub affected_services: Option<Vec<String>>,

    #[serde(rename = "statusPageUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_page_url: Option<String>,

}


impl AvailabilityInfo {
    #[allow(clippy::new_without_default)]
    pub fn new() -> AvailabilityInfo {
        AvailabilityInfo {
            retry_after: None,
            maintenance_window: None,
            affected_services: None,
            status_page_url: None,
        }
    }
}

/// Converts the AvailabilityInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AvailabilityInfo {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.retry_after.as_ref().map(|retry_after| {
                [
                    "retryAfter".to_string(),
                    retry_after.to_string(),
                ].join(",")
            }),


            self.maintenance_window.as_ref().map(|maintenance_window| {
                [
                    "maintenanceWindow".to_string(),
                    maintenance_window.to_string(),
                ].join(",")
            }),


            self.affected_services.as_ref().map(|affected_services| {
                [
                    "affectedServices".to_string(),
                    affected_services.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.status_page_url.as_ref().map(|status_page_url| {
                [
                    "statusPageUrl".to_string(),
                    status_page_url.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AvailabilityInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AvailabilityInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub retry_after: Vec<i32>,
            pub maintenance_window: Vec<String>,
            pub affected_services: Vec<Vec<String>>,
            pub status_page_url: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AvailabilityInfo".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "retryAfter" => intermediate_rep.retry_after.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "maintenanceWindow" => intermediate_rep.maintenance_window.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "affectedServices" => return std::result::Result::Err("Parsing a container in this style is not supported in AvailabilityInfo".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "statusPageUrl" => intermediate_rep.status_page_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AvailabilityInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AvailabilityInfo {
            retry_after: intermediate_rep.retry_after.into_iter().next(),
            maintenance_window: intermediate_rep.maintenance_window.into_iter().next(),
            affected_services: intermediate_rep.affected_services.into_iter().next(),
            status_page_url: intermediate_rep.status_page_url.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AvailabilityInfo> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AvailabilityInfo>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AvailabilityInfo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AvailabilityInfo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AvailabilityInfo> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AvailabilityInfo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AvailabilityInfo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// BackgroundJobStatus represents the possible states of a background job. This enum is used to track the lifecycle of asynchronous tasks like scraping jobs.  State transitions: 1. QUEUED -> IN_PROGRESS 2. IN_PROGRESS -> COMPLETED/FAILED/CANCELLED/TIMED_OUT  Usage example: ```go job := &ScrapingJob{     Status: BackgroundJobStatus_BACKGROUND_JOB_STATUS_IN_PROGRESS, } ```   - BACKGROUND_JOB_STATUS_UNSPECIFIED: Default state, should not be used explicitly  - BACKGROUND_JOB_STATUS_QUEUED: Job is queued and waiting to be processed  - BACKGROUND_JOB_STATUS_IN_PROGRESS: Job is currently being processed  - BACKGROUND_JOB_STATUS_COMPLETED: Job has completed successfully  - BACKGROUND_JOB_STATUS_FAILED: Job encountered an error and failed  - BACKGROUND_JOB_STATUS_CANCELLED: Job was manually cancelled by user  - BACKGROUND_JOB_STATUS_TIMED_OUT: Job exceeded its maximum execution time
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum BackgroundJobStatus {
    #[serde(rename = "BACKGROUND_JOB_STATUS_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "BACKGROUND_JOB_STATUS_QUEUED")]
    Queued,
    #[serde(rename = "BACKGROUND_JOB_STATUS_IN_PROGRESS")]
    InProgress,
    #[serde(rename = "BACKGROUND_JOB_STATUS_COMPLETED")]
    Completed,
    #[serde(rename = "BACKGROUND_JOB_STATUS_FAILED")]
    Failed,
    #[serde(rename = "BACKGROUND_JOB_STATUS_CANCELLED")]
    Cancelled,
    #[serde(rename = "BACKGROUND_JOB_STATUS_TIMED_OUT")]
    TimedOut,
}

impl std::fmt::Display for BackgroundJobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            BackgroundJobStatus::Unspecified => write!(f, "BACKGROUND_JOB_STATUS_UNSPECIFIED"),
            BackgroundJobStatus::Queued => write!(f, "BACKGROUND_JOB_STATUS_QUEUED"),
            BackgroundJobStatus::InProgress => write!(f, "BACKGROUND_JOB_STATUS_IN_PROGRESS"),
            BackgroundJobStatus::Completed => write!(f, "BACKGROUND_JOB_STATUS_COMPLETED"),
            BackgroundJobStatus::Failed => write!(f, "BACKGROUND_JOB_STATUS_FAILED"),
            BackgroundJobStatus::Cancelled => write!(f, "BACKGROUND_JOB_STATUS_CANCELLED"),
            BackgroundJobStatus::TimedOut => write!(f, "BACKGROUND_JOB_STATUS_TIMED_OUT"),
        }
    }
}

impl std::str::FromStr for BackgroundJobStatus {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "BACKGROUND_JOB_STATUS_UNSPECIFIED" => std::result::Result::Ok(BackgroundJobStatus::Unspecified),
            "BACKGROUND_JOB_STATUS_QUEUED" => std::result::Result::Ok(BackgroundJobStatus::Queued),
            "BACKGROUND_JOB_STATUS_IN_PROGRESS" => std::result::Result::Ok(BackgroundJobStatus::InProgress),
            "BACKGROUND_JOB_STATUS_COMPLETED" => std::result::Result::Ok(BackgroundJobStatus::Completed),
            "BACKGROUND_JOB_STATUS_FAILED" => std::result::Result::Ok(BackgroundJobStatus::Failed),
            "BACKGROUND_JOB_STATUS_CANCELLED" => std::result::Result::Ok(BackgroundJobStatus::Cancelled),
            "BACKGROUND_JOB_STATUS_TIMED_OUT" => std::result::Result::Ok(BackgroundJobStatus::TimedOut),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// Represents errors when the server received an invalid response from an upstream server
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct BadGatewayErrorMessageResponse {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<i32>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "upstreamService")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub upstream_service: Option<String>,

    #[serde(rename = "errorResponse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_response: Option<models::ErrorResponse>,

}


impl BadGatewayErrorMessageResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> BadGatewayErrorMessageResponse {
        BadGatewayErrorMessageResponse {
            code: None,
            message: None,
            upstream_service: None,
            error_response: None,
        }
    }
}

/// Converts the BadGatewayErrorMessageResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for BadGatewayErrorMessageResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.code.as_ref().map(|code| {
                [
                    "code".to_string(),
                    code.to_string(),
                ].join(",")
            }),


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),


            self.upstream_service.as_ref().map(|upstream_service| {
                [
                    "upstreamService".to_string(),
                    upstream_service.to_string(),
                ].join(",")
            }),

            // Skipping errorResponse in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a BadGatewayErrorMessageResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for BadGatewayErrorMessageResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<i32>,
            pub message: Vec<String>,
            pub upstream_service: Vec<String>,
            pub error_response: Vec<models::ErrorResponse>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing BadGatewayErrorMessageResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "upstreamService" => intermediate_rep.upstream_service.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "errorResponse" => intermediate_rep.error_response.push(<models::ErrorResponse as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing BadGatewayErrorMessageResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(BadGatewayErrorMessageResponse {
            code: intermediate_rep.code.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
            upstream_service: intermediate_rep.upstream_service.into_iter().next(),
            error_response: intermediate_rep.error_response.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<BadGatewayErrorMessageResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<BadGatewayErrorMessageResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<BadGatewayErrorMessageResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for BadGatewayErrorMessageResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<BadGatewayErrorMessageResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <BadGatewayErrorMessageResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into BadGatewayErrorMessageResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct BusinessHours {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "day")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub day: Option<models::DayOfWeek>,

    #[serde(rename = "openTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub open_time: Option<String>,

    #[serde(rename = "closeTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub close_time: Option<String>,

    #[serde(rename = "closed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub closed: Option<bool>,

    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_at: Option<chrono::DateTime::<chrono::Utc>>,

}


impl BusinessHours {
    #[allow(clippy::new_without_default)]
    pub fn new() -> BusinessHours {
        BusinessHours {
            id: None,
            day: None,
            open_time: None,
            close_time: None,
            closed: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }
}

/// Converts the BusinessHours value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for BusinessHours {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.id.as_ref().map(|id| {
                [
                    "id".to_string(),
                    id.to_string(),
                ].join(",")
            }),

            // Skipping day in query parameter serialization


            self.open_time.as_ref().map(|open_time| {
                [
                    "openTime".to_string(),
                    open_time.to_string(),
                ].join(",")
            }),


            self.close_time.as_ref().map(|close_time| {
                [
                    "closeTime".to_string(),
                    close_time.to_string(),
                ].join(",")
            }),


            self.closed.as_ref().map(|closed| {
                [
                    "closed".to_string(),
                    closed.to_string(),
                ].join(",")
            }),

            // Skipping createdAt in query parameter serialization

            // Skipping updatedAt in query parameter serialization

            // Skipping deletedAt in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a BusinessHours value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for BusinessHours {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub day: Vec<models::DayOfWeek>,
            pub open_time: Vec<String>,
            pub close_time: Vec<String>,
            pub closed: Vec<bool>,
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub updated_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub deleted_at: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing BusinessHours".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "day" => intermediate_rep.day.push(<models::DayOfWeek as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "openTime" => intermediate_rep.open_time.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "closeTime" => intermediate_rep.close_time.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "closed" => intermediate_rep.closed.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "createdAt" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "updatedAt" => intermediate_rep.updated_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "deletedAt" => intermediate_rep.deleted_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing BusinessHours".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(BusinessHours {
            id: intermediate_rep.id.into_iter().next(),
            day: intermediate_rep.day.into_iter().next(),
            open_time: intermediate_rep.open_time.into_iter().next(),
            close_time: intermediate_rep.close_time.into_iter().next(),
            closed: intermediate_rep.closed.into_iter().next(),
            created_at: intermediate_rep.created_at.into_iter().next(),
            updated_at: intermediate_rep.updated_at.into_iter().next(),
            deleted_at: intermediate_rep.deleted_at.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<BusinessHours> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<BusinessHours>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<BusinessHours>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for BusinessHours - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<BusinessHours> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <BusinessHours as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into BusinessHours - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Represents resource conflict errors
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ConflictErrorMessageResponse {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<models::ValidationErrorCode>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "conflictInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub conflict_info: Option<models::ConflictInfo>,

    #[serde(rename = "errorResponse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_response: Option<models::ErrorResponse>,

}


impl ConflictErrorMessageResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ConflictErrorMessageResponse {
        ConflictErrorMessageResponse {
            code: None,
            message: None,
            conflict_info: None,
            error_response: None,
        }
    }
}

/// Converts the ConflictErrorMessageResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ConflictErrorMessageResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping code in query parameter serialization


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),

            // Skipping conflictInfo in query parameter serialization

            // Skipping errorResponse in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ConflictErrorMessageResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ConflictErrorMessageResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<models::ValidationErrorCode>,
            pub message: Vec<String>,
            pub conflict_info: Vec<models::ConflictInfo>,
            pub error_response: Vec<models::ErrorResponse>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ConflictErrorMessageResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(<models::ValidationErrorCode as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "conflictInfo" => intermediate_rep.conflict_info.push(<models::ConflictInfo as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "errorResponse" => intermediate_rep.error_response.push(<models::ErrorResponse as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ConflictErrorMessageResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ConflictErrorMessageResponse {
            code: intermediate_rep.code.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
            conflict_info: intermediate_rep.conflict_info.into_iter().next(),
            error_response: intermediate_rep.error_response.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ConflictErrorMessageResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ConflictErrorMessageResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ConflictErrorMessageResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ConflictErrorMessageResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ConflictErrorMessageResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ConflictErrorMessageResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ConflictErrorMessageResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ConflictInfo {
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_type: Option<String>,

    #[serde(rename = "identifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub identifier: Option<String>,

    #[serde(rename = "conflictReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub conflict_reason: Option<String>,

    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_by: Option<String>,

}


impl ConflictInfo {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ConflictInfo {
        ConflictInfo {
            resource_type: None,
            identifier: None,
            conflict_reason: None,
            created_at: None,
            created_by: None,
        }
    }
}

/// Converts the ConflictInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ConflictInfo {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.resource_type.as_ref().map(|resource_type| {
                [
                    "resourceType".to_string(),
                    resource_type.to_string(),
                ].join(",")
            }),


            self.identifier.as_ref().map(|identifier| {
                [
                    "identifier".to_string(),
                    identifier.to_string(),
                ].join(",")
            }),


            self.conflict_reason.as_ref().map(|conflict_reason| {
                [
                    "conflictReason".to_string(),
                    conflict_reason.to_string(),
                ].join(",")
            }),

            // Skipping createdAt in query parameter serialization


            self.created_by.as_ref().map(|created_by| {
                [
                    "createdBy".to_string(),
                    created_by.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ConflictInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ConflictInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub resource_type: Vec<String>,
            pub identifier: Vec<String>,
            pub conflict_reason: Vec<String>,
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub created_by: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ConflictInfo".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "resourceType" => intermediate_rep.resource_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "identifier" => intermediate_rep.identifier.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "conflictReason" => intermediate_rep.conflict_reason.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "createdAt" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "createdBy" => intermediate_rep.created_by.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ConflictInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ConflictInfo {
            resource_type: intermediate_rep.resource_type.into_iter().next(),
            identifier: intermediate_rep.identifier.into_iter().next(),
            conflict_reason: intermediate_rep.conflict_reason.into_iter().next(),
            created_at: intermediate_rep.created_at.into_iter().next(),
            created_by: intermediate_rep.created_by.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ConflictInfo> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ConflictInfo>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ConflictInfo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ConflictInfo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ConflictInfo> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ConflictInfo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ConflictInfo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateAccountRequest {
    #[serde(rename = "account")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub account: Option<models::Account>,

    #[serde(rename = "initialWorkspaceName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub initial_workspace_name: Option<String>,

}


impl CreateAccountRequest {
    #[allow(clippy::new_without_default)]
    pub fn new() -> CreateAccountRequest {
        CreateAccountRequest {
            account: None,
            initial_workspace_name: None,
        }
    }
}

/// Converts the CreateAccountRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateAccountRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping account in query parameter serialization


            self.initial_workspace_name.as_ref().map(|initial_workspace_name| {
                [
                    "initialWorkspaceName".to_string(),
                    initial_workspace_name.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateAccountRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateAccountRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub account: Vec<models::Account>,
            pub initial_workspace_name: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateAccountRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "account" => intermediate_rep.account.push(<models::Account as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "initialWorkspaceName" => intermediate_rep.initial_workspace_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateAccountRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateAccountRequest {
            account: intermediate_rep.account.into_iter().next(),
            initial_workspace_name: intermediate_rep.initial_workspace_name.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateAccountRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateAccountRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateAccountRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateAccountRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateAccountRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateAccountRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateAccountRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateAccountResponse {
    #[serde(rename = "account")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub account: Option<models::Account>,

    #[serde(rename = "initialWorkspaceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub initial_workspace_id: Option<String>,

}


impl CreateAccountResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> CreateAccountResponse {
        CreateAccountResponse {
            account: None,
            initial_workspace_id: None,
        }
    }
}

/// Converts the CreateAccountResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateAccountResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping account in query parameter serialization


            self.initial_workspace_id.as_ref().map(|initial_workspace_id| {
                [
                    "initialWorkspaceId".to_string(),
                    initial_workspace_id.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateAccountResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateAccountResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub account: Vec<models::Account>,
            pub initial_workspace_id: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateAccountResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "account" => intermediate_rep.account.push(<models::Account as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "initialWorkspaceId" => intermediate_rep.initial_workspace_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateAccountResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateAccountResponse {
            account: intermediate_rep.account.into_iter().next(),
            initial_workspace_id: intermediate_rep.initial_workspace_id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateAccountResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateAccountResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateAccountResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateAccountResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateAccountResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateAccountResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateAccountResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateScrapingJobRequest {
    #[serde(rename = "userId")]
    pub user_id: String,

    #[serde(rename = "orgId")]
    pub org_id: String,

    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "keywords")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub keywords: Option<Vec<String>>,

    #[serde(rename = "lang")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lang: Option<String>,

    #[serde(rename = "zoom")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub zoom: Option<i32>,

    #[serde(rename = "lat")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lat: Option<String>,

    #[serde(rename = "lon")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lon: Option<String>,

    #[serde(rename = "fastMode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fast_mode: Option<bool>,

    #[serde(rename = "radius")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub radius: Option<i32>,

    #[serde(rename = "depth")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub depth: Option<i32>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<bool>,

    #[serde(rename = "maxTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_time: Option<i32>,

    #[serde(rename = "proxies")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub proxies: Option<Vec<String>>,

}


impl CreateScrapingJobRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(user_id: String, org_id: String, tenant_id: String, ) -> CreateScrapingJobRequest {
        CreateScrapingJobRequest {
            user_id,
            org_id,
            tenant_id,
            name: None,
            keywords: None,
            lang: None,
            zoom: None,
            lat: None,
            lon: None,
            fast_mode: None,
            radius: None,
            depth: None,
            email: None,
            max_time: None,
            proxies: None,
        }
    }
}

/// Converts the CreateScrapingJobRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateScrapingJobRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("userId".to_string()),
            Some(self.user_id.to_string()),


            Some("orgId".to_string()),
            Some(self.org_id.to_string()),


            Some("tenantId".to_string()),
            Some(self.tenant_id.to_string()),


            self.name.as_ref().map(|name| {
                [
                    "name".to_string(),
                    name.to_string(),
                ].join(",")
            }),


            self.keywords.as_ref().map(|keywords| {
                [
                    "keywords".to_string(),
                    keywords.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.lang.as_ref().map(|lang| {
                [
                    "lang".to_string(),
                    lang.to_string(),
                ].join(",")
            }),


            self.zoom.as_ref().map(|zoom| {
                [
                    "zoom".to_string(),
                    zoom.to_string(),
                ].join(",")
            }),


            self.lat.as_ref().map(|lat| {
                [
                    "lat".to_string(),
                    lat.to_string(),
                ].join(",")
            }),


            self.lon.as_ref().map(|lon| {
                [
                    "lon".to_string(),
                    lon.to_string(),
                ].join(",")
            }),


            self.fast_mode.as_ref().map(|fast_mode| {
                [
                    "fastMode".to_string(),
                    fast_mode.to_string(),
                ].join(",")
            }),


            self.radius.as_ref().map(|radius| {
                [
                    "radius".to_string(),
                    radius.to_string(),
                ].join(",")
            }),


            self.depth.as_ref().map(|depth| {
                [
                    "depth".to_string(),
                    depth.to_string(),
                ].join(",")
            }),


            self.email.as_ref().map(|email| {
                [
                    "email".to_string(),
                    email.to_string(),
                ].join(",")
            }),


            self.max_time.as_ref().map(|max_time| {
                [
                    "maxTime".to_string(),
                    max_time.to_string(),
                ].join(",")
            }),


            self.proxies.as_ref().map(|proxies| {
                [
                    "proxies".to_string(),
                    proxies.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateScrapingJobRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateScrapingJobRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub user_id: Vec<String>,
            pub org_id: Vec<String>,
            pub tenant_id: Vec<String>,
            pub name: Vec<String>,
            pub keywords: Vec<Vec<String>>,
            pub lang: Vec<String>,
            pub zoom: Vec<i32>,
            pub lat: Vec<String>,
            pub lon: Vec<String>,
            pub fast_mode: Vec<bool>,
            pub radius: Vec<i32>,
            pub depth: Vec<i32>,
            pub email: Vec<bool>,
            pub max_time: Vec<i32>,
            pub proxies: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateScrapingJobRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "userId" => intermediate_rep.user_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "orgId" => intermediate_rep.org_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "tenantId" => intermediate_rep.tenant_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "keywords" => return std::result::Result::Err("Parsing a container in this style is not supported in CreateScrapingJobRequest".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "lang" => intermediate_rep.lang.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "zoom" => intermediate_rep.zoom.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "lat" => intermediate_rep.lat.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "lon" => intermediate_rep.lon.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "fastMode" => intermediate_rep.fast_mode.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "radius" => intermediate_rep.radius.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "depth" => intermediate_rep.depth.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "email" => intermediate_rep.email.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "maxTime" => intermediate_rep.max_time.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "proxies" => return std::result::Result::Err("Parsing a container in this style is not supported in CreateScrapingJobRequest".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateScrapingJobRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateScrapingJobRequest {
            user_id: intermediate_rep.user_id.into_iter().next().ok_or_else(|| "userId missing in CreateScrapingJobRequest".to_string())?,
            org_id: intermediate_rep.org_id.into_iter().next().ok_or_else(|| "orgId missing in CreateScrapingJobRequest".to_string())?,
            tenant_id: intermediate_rep.tenant_id.into_iter().next().ok_or_else(|| "tenantId missing in CreateScrapingJobRequest".to_string())?,
            name: intermediate_rep.name.into_iter().next(),
            keywords: intermediate_rep.keywords.into_iter().next(),
            lang: intermediate_rep.lang.into_iter().next(),
            zoom: intermediate_rep.zoom.into_iter().next(),
            lat: intermediate_rep.lat.into_iter().next(),
            lon: intermediate_rep.lon.into_iter().next(),
            fast_mode: intermediate_rep.fast_mode.into_iter().next(),
            radius: intermediate_rep.radius.into_iter().next(),
            depth: intermediate_rep.depth.into_iter().next(),
            email: intermediate_rep.email.into_iter().next(),
            max_time: intermediate_rep.max_time.into_iter().next(),
            proxies: intermediate_rep.proxies.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateScrapingJobRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateScrapingJobRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateScrapingJobRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateScrapingJobRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateScrapingJobRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateScrapingJobRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateScrapingJobRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateScrapingJobResponse {
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_id: Option<String>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<models::BackgroundJobStatus>,

}


impl CreateScrapingJobResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> CreateScrapingJobResponse {
        CreateScrapingJobResponse {
            job_id: None,
            status: None,
        }
    }
}

/// Converts the CreateScrapingJobResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateScrapingJobResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.job_id.as_ref().map(|job_id| {
                [
                    "jobId".to_string(),
                    job_id.to_string(),
                ].join(",")
            }),

            // Skipping status in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateScrapingJobResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateScrapingJobResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub job_id: Vec<String>,
            pub status: Vec<models::BackgroundJobStatus>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateScrapingJobResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "jobId" => intermediate_rep.job_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<models::BackgroundJobStatus as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateScrapingJobResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateScrapingJobResponse {
            job_id: intermediate_rep.job_id.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateScrapingJobResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateScrapingJobResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateScrapingJobResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateScrapingJobResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateScrapingJobResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateScrapingJobResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateScrapingJobResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateWorkflowBody {
    #[serde(rename = "workflow")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workflow: Option<models::ScrapingWorkflow>,

}


impl CreateWorkflowBody {
    #[allow(clippy::new_without_default)]
    pub fn new() -> CreateWorkflowBody {
        CreateWorkflowBody {
            workflow: None,
        }
    }
}

/// Converts the CreateWorkflowBody value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateWorkflowBody {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping workflow in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateWorkflowBody value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateWorkflowBody {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub workflow: Vec<models::ScrapingWorkflow>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateWorkflowBody".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "workflow" => intermediate_rep.workflow.push(<models::ScrapingWorkflow as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateWorkflowBody".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateWorkflowBody {
            workflow: intermediate_rep.workflow.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateWorkflowBody> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateWorkflowBody>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateWorkflowBody>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateWorkflowBody - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateWorkflowBody> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateWorkflowBody as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateWorkflowBody - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateWorkflowResponse {
    #[serde(rename = "workflow")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workflow: Option<models::ScrapingWorkflow>,

}


impl CreateWorkflowResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> CreateWorkflowResponse {
        CreateWorkflowResponse {
            workflow: None,
        }
    }
}

/// Converts the CreateWorkflowResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateWorkflowResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping workflow in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateWorkflowResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateWorkflowResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub workflow: Vec<models::ScrapingWorkflow>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateWorkflowResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "workflow" => intermediate_rep.workflow.push(<models::ScrapingWorkflow as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateWorkflowResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateWorkflowResponse {
            workflow: intermediate_rep.workflow.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateWorkflowResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateWorkflowResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateWorkflowResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateWorkflowResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateWorkflowResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateWorkflowResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateWorkflowResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateWorkspaceRequest {
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workspace: Option<models::Workspace>,

    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub account_id: Option<String>,

}


impl CreateWorkspaceRequest {
    #[allow(clippy::new_without_default)]
    pub fn new() -> CreateWorkspaceRequest {
        CreateWorkspaceRequest {
            workspace: None,
            account_id: None,
        }
    }
}

/// Converts the CreateWorkspaceRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateWorkspaceRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping workspace in query parameter serialization


            self.account_id.as_ref().map(|account_id| {
                [
                    "accountId".to_string(),
                    account_id.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateWorkspaceRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateWorkspaceRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub workspace: Vec<models::Workspace>,
            pub account_id: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateWorkspaceRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "workspace" => intermediate_rep.workspace.push(<models::Workspace as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "accountId" => intermediate_rep.account_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateWorkspaceRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateWorkspaceRequest {
            workspace: intermediate_rep.workspace.into_iter().next(),
            account_id: intermediate_rep.account_id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateWorkspaceRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateWorkspaceRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateWorkspaceRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateWorkspaceRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateWorkspaceRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateWorkspaceRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateWorkspaceRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateWorkspaceResponse {
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workspace: Option<models::Workspace>,

}


impl CreateWorkspaceResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> CreateWorkspaceResponse {
        CreateWorkspaceResponse {
            workspace: None,
        }
    }
}

/// Converts the CreateWorkspaceResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateWorkspaceResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping workspace in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateWorkspaceResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateWorkspaceResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub workspace: Vec<models::Workspace>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateWorkspaceResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "workspace" => intermediate_rep.workspace.push(<models::Workspace as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateWorkspaceResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateWorkspaceResponse {
            workspace: intermediate_rep.workspace.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateWorkspaceResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateWorkspaceResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateWorkspaceResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateWorkspaceResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateWorkspaceResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateWorkspaceResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateWorkspaceResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum DayOfWeek {
    #[serde(rename = "DAY_OF_WEEK_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "DAY_OF_WEEK_MONDAY")]
    Monday,
    #[serde(rename = "DAY_OF_WEEK_TUESDAY")]
    Tuesday,
    #[serde(rename = "DAY_OF_WEEK_WEDNESDAY")]
    Wednesday,
    #[serde(rename = "DAY_OF_WEEK_THURSDAY")]
    Thursday,
    #[serde(rename = "DAY_OF_WEEK_FRIDAY")]
    Friday,
    #[serde(rename = "DAY_OF_WEEK_SATURDAY")]
    Saturday,
    #[serde(rename = "DAY_OF_WEEK_SUNDAY")]
    Sunday,
}

impl std::fmt::Display for DayOfWeek {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DayOfWeek::Unspecified => write!(f, "DAY_OF_WEEK_UNSPECIFIED"),
            DayOfWeek::Monday => write!(f, "DAY_OF_WEEK_MONDAY"),
            DayOfWeek::Tuesday => write!(f, "DAY_OF_WEEK_TUESDAY"),
            DayOfWeek::Wednesday => write!(f, "DAY_OF_WEEK_WEDNESDAY"),
            DayOfWeek::Thursday => write!(f, "DAY_OF_WEEK_THURSDAY"),
            DayOfWeek::Friday => write!(f, "DAY_OF_WEEK_FRIDAY"),
            DayOfWeek::Saturday => write!(f, "DAY_OF_WEEK_SATURDAY"),
            DayOfWeek::Sunday => write!(f, "DAY_OF_WEEK_SUNDAY"),
        }
    }
}

impl std::str::FromStr for DayOfWeek {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "DAY_OF_WEEK_UNSPECIFIED" => std::result::Result::Ok(DayOfWeek::Unspecified),
            "DAY_OF_WEEK_MONDAY" => std::result::Result::Ok(DayOfWeek::Monday),
            "DAY_OF_WEEK_TUESDAY" => std::result::Result::Ok(DayOfWeek::Tuesday),
            "DAY_OF_WEEK_WEDNESDAY" => std::result::Result::Ok(DayOfWeek::Wednesday),
            "DAY_OF_WEEK_THURSDAY" => std::result::Result::Ok(DayOfWeek::Thursday),
            "DAY_OF_WEEK_FRIDAY" => std::result::Result::Ok(DayOfWeek::Friday),
            "DAY_OF_WEEK_SATURDAY" => std::result::Result::Ok(DayOfWeek::Saturday),
            "DAY_OF_WEEK_SUNDAY" => std::result::Result::Ok(DayOfWeek::Sunday),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteAccountResponse {
    #[serde(rename = "success")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub success: Option<bool>,

}


impl DeleteAccountResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> DeleteAccountResponse {
        DeleteAccountResponse {
            success: None,
        }
    }
}

/// Converts the DeleteAccountResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DeleteAccountResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.success.as_ref().map(|success| {
                [
                    "success".to_string(),
                    success.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DeleteAccountResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DeleteAccountResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub success: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing DeleteAccountResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "success" => intermediate_rep.success.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing DeleteAccountResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DeleteAccountResponse {
            success: intermediate_rep.success.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DeleteAccountResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DeleteAccountResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<DeleteAccountResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for DeleteAccountResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<DeleteAccountResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <DeleteAccountResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into DeleteAccountResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteScrapingJobResponse {
    #[serde(rename = "success")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub success: Option<bool>,

}


impl DeleteScrapingJobResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> DeleteScrapingJobResponse {
        DeleteScrapingJobResponse {
            success: None,
        }
    }
}

/// Converts the DeleteScrapingJobResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DeleteScrapingJobResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.success.as_ref().map(|success| {
                [
                    "success".to_string(),
                    success.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DeleteScrapingJobResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DeleteScrapingJobResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub success: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing DeleteScrapingJobResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "success" => intermediate_rep.success.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing DeleteScrapingJobResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DeleteScrapingJobResponse {
            success: intermediate_rep.success.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DeleteScrapingJobResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DeleteScrapingJobResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<DeleteScrapingJobResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for DeleteScrapingJobResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<DeleteScrapingJobResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <DeleteScrapingJobResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into DeleteScrapingJobResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteWorkspaceResponse {
    #[serde(rename = "success")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub success: Option<bool>,

}


impl DeleteWorkspaceResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> DeleteWorkspaceResponse {
        DeleteWorkspaceResponse {
            success: None,
        }
    }
}

/// Converts the DeleteWorkspaceResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DeleteWorkspaceResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.success.as_ref().map(|success| {
                [
                    "success".to_string(),
                    success.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DeleteWorkspaceResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DeleteWorkspaceResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub success: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing DeleteWorkspaceResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "success" => intermediate_rep.success.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing DeleteWorkspaceResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DeleteWorkspaceResponse {
            success: intermediate_rep.success.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DeleteWorkspaceResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DeleteWorkspaceResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<DeleteWorkspaceResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for DeleteWorkspaceResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<DeleteWorkspaceResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <DeleteWorkspaceResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into DeleteWorkspaceResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Dependency {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "error")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<String>,

    #[serde(rename = "latency")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub latency: Option<i32>,

}


impl Dependency {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Dependency {
        Dependency {
            name: None,
            status: None,
            error: None,
            latency: None,
        }
    }
}

/// Converts the Dependency value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Dependency {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.name.as_ref().map(|name| {
                [
                    "name".to_string(),
                    name.to_string(),
                ].join(",")
            }),


            self.status.as_ref().map(|status| {
                [
                    "status".to_string(),
                    status.to_string(),
                ].join(",")
            }),


            self.error.as_ref().map(|error| {
                [
                    "error".to_string(),
                    error.to_string(),
                ].join(",")
            }),


            self.latency.as_ref().map(|latency| {
                [
                    "latency".to_string(),
                    latency.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Dependency value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Dependency {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub status: Vec<String>,
            pub error: Vec<String>,
            pub latency: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Dependency".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "error" => intermediate_rep.error.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "latency" => intermediate_rep.latency.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Dependency".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Dependency {
            name: intermediate_rep.name.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            error: intermediate_rep.error.into_iter().next(),
            latency: intermediate_rep.latency.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Dependency> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Dependency>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Dependency>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Dependency - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Dependency> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Dependency as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Dependency - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DownloadScrapingResultsResponse {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<swagger::ByteArray>,

    #[serde(rename = "filename")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filename: Option<String>,

    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<String>,

}


impl DownloadScrapingResultsResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> DownloadScrapingResultsResponse {
        DownloadScrapingResultsResponse {
            content: None,
            filename: None,
            content_type: None,
        }
    }
}

/// Converts the DownloadScrapingResultsResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DownloadScrapingResultsResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping content in query parameter serialization
            // Skipping content in query parameter serialization


            self.filename.as_ref().map(|filename| {
                [
                    "filename".to_string(),
                    filename.to_string(),
                ].join(",")
            }),


            self.content_type.as_ref().map(|content_type| {
                [
                    "contentType".to_string(),
                    content_type.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DownloadScrapingResultsResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DownloadScrapingResultsResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub content: Vec<swagger::ByteArray>,
            pub filename: Vec<String>,
            pub content_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing DownloadScrapingResultsResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "content" => return std::result::Result::Err("Parsing binary data in this style is not supported in DownloadScrapingResultsResponse".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "filename" => intermediate_rep.filename.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "contentType" => intermediate_rep.content_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing DownloadScrapingResultsResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DownloadScrapingResultsResponse {
            content: intermediate_rep.content.into_iter().next(),
            filename: intermediate_rep.filename.into_iter().next(),
            content_type: intermediate_rep.content_type.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DownloadScrapingResultsResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DownloadScrapingResultsResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<DownloadScrapingResultsResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for DownloadScrapingResultsResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<DownloadScrapingResultsResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <DownloadScrapingResultsResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into DownloadScrapingResultsResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum EmployeeBenefit {
    #[serde(rename = "EMPLOYEE_BENEFIT_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "EMPLOYEE_BENEFIT_HEALTH_INSURANCE")]
    HealthInsurance,
    #[serde(rename = "EMPLOYEE_BENEFIT_RETIREMENT_PLAN")]
    RetirementPlan,
    #[serde(rename = "EMPLOYEE_BENEFIT_PAID_TIME_OFF")]
    PaidTimeOff,
    #[serde(rename = "EMPLOYEE_BENEFIT_REMOTE_WORK")]
    RemoteWork,
}

impl std::fmt::Display for EmployeeBenefit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            EmployeeBenefit::Unspecified => write!(f, "EMPLOYEE_BENEFIT_UNSPECIFIED"),
            EmployeeBenefit::HealthInsurance => write!(f, "EMPLOYEE_BENEFIT_HEALTH_INSURANCE"),
            EmployeeBenefit::RetirementPlan => write!(f, "EMPLOYEE_BENEFIT_RETIREMENT_PLAN"),
            EmployeeBenefit::PaidTimeOff => write!(f, "EMPLOYEE_BENEFIT_PAID_TIME_OFF"),
            EmployeeBenefit::RemoteWork => write!(f, "EMPLOYEE_BENEFIT_REMOTE_WORK"),
        }
    }
}

impl std::str::FromStr for EmployeeBenefit {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "EMPLOYEE_BENEFIT_UNSPECIFIED" => std::result::Result::Ok(EmployeeBenefit::Unspecified),
            "EMPLOYEE_BENEFIT_HEALTH_INSURANCE" => std::result::Result::Ok(EmployeeBenefit::HealthInsurance),
            "EMPLOYEE_BENEFIT_RETIREMENT_PLAN" => std::result::Result::Ok(EmployeeBenefit::RetirementPlan),
            "EMPLOYEE_BENEFIT_PAID_TIME_OFF" => std::result::Result::Ok(EmployeeBenefit::PaidTimeOff),
            "EMPLOYEE_BENEFIT_REMOTE_WORK" => std::result::Result::Ok(EmployeeBenefit::RemoteWork),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ErrorResponse {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<models::RpcPeriodStatus>,

}


impl ErrorResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ErrorResponse {
        ErrorResponse {
            status: None,
        }
    }
}

/// Converts the ErrorResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ErrorResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping status in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ErrorResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ErrorResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub status: Vec<models::RpcPeriodStatus>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ErrorResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<models::RpcPeriodStatus as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ErrorResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ErrorResponse {
            status: intermediate_rep.status.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ErrorResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ErrorResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ErrorResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ErrorResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ErrorResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ErrorResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ErrorResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct FieldError {
    #[serde(rename = "field")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub field: Option<String>,

    #[serde(rename = "error")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<String>,

}


impl FieldError {
    #[allow(clippy::new_without_default)]
    pub fn new() -> FieldError {
        FieldError {
            field: None,
            error: None,
        }
    }
}

/// Converts the FieldError value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for FieldError {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.field.as_ref().map(|field| {
                [
                    "field".to_string(),
                    field.to_string(),
                ].join(",")
            }),


            self.error.as_ref().map(|error| {
                [
                    "error".to_string(),
                    error.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a FieldError value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for FieldError {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub field: Vec<String>,
            pub error: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing FieldError".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "field" => intermediate_rep.field.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "error" => intermediate_rep.error.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing FieldError".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(FieldError {
            field: intermediate_rep.field.into_iter().next(),
            error: intermediate_rep.error.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<FieldError> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<FieldError>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<FieldError>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for FieldError - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<FieldError> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <FieldError as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into FieldError - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct FieldViolation {
    #[serde(rename = "field")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub field: Option<String>,

    #[serde(rename = "validation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub validation: Option<String>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "expected")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expected: Option<String>,

    #[serde(rename = "actual")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub actual: Option<String>,

}


impl FieldViolation {
    #[allow(clippy::new_without_default)]
    pub fn new() -> FieldViolation {
        FieldViolation {
            field: None,
            validation: None,
            message: None,
            expected: None,
            actual: None,
        }
    }
}

/// Converts the FieldViolation value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for FieldViolation {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.field.as_ref().map(|field| {
                [
                    "field".to_string(),
                    field.to_string(),
                ].join(",")
            }),


            self.validation.as_ref().map(|validation| {
                [
                    "validation".to_string(),
                    validation.to_string(),
                ].join(",")
            }),


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),


            self.expected.as_ref().map(|expected| {
                [
                    "expected".to_string(),
                    expected.to_string(),
                ].join(",")
            }),


            self.actual.as_ref().map(|actual| {
                [
                    "actual".to_string(),
                    actual.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a FieldViolation value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for FieldViolation {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub field: Vec<String>,
            pub validation: Vec<String>,
            pub message: Vec<String>,
            pub expected: Vec<String>,
            pub actual: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing FieldViolation".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "field" => intermediate_rep.field.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "validation" => intermediate_rep.validation.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "expected" => intermediate_rep.expected.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "actual" => intermediate_rep.actual.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing FieldViolation".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(FieldViolation {
            field: intermediate_rep.field.into_iter().next(),
            validation: intermediate_rep.validation.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
            expected: intermediate_rep.expected.into_iter().next(),
            actual: intermediate_rep.actual.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<FieldViolation> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<FieldViolation>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<FieldViolation>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for FieldViolation - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<FieldViolation> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <FieldViolation as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into FieldViolation - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Indicates that the server understood the request but refuses to authorize it
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ForbiddenErrorMessageResponse {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<i32>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reason: Option<String>,

    #[serde(rename = "requiredPermissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_permissions: Option<Vec<String>>,

    #[serde(rename = "errorResponse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_response: Option<models::ErrorResponse>,

}


impl ForbiddenErrorMessageResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ForbiddenErrorMessageResponse {
        ForbiddenErrorMessageResponse {
            code: None,
            message: None,
            reason: None,
            required_permissions: None,
            error_response: None,
        }
    }
}

/// Converts the ForbiddenErrorMessageResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ForbiddenErrorMessageResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.code.as_ref().map(|code| {
                [
                    "code".to_string(),
                    code.to_string(),
                ].join(",")
            }),


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),


            self.reason.as_ref().map(|reason| {
                [
                    "reason".to_string(),
                    reason.to_string(),
                ].join(",")
            }),


            self.required_permissions.as_ref().map(|required_permissions| {
                [
                    "requiredPermissions".to_string(),
                    required_permissions.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

            // Skipping errorResponse in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ForbiddenErrorMessageResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ForbiddenErrorMessageResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<i32>,
            pub message: Vec<String>,
            pub reason: Vec<String>,
            pub required_permissions: Vec<Vec<String>>,
            pub error_response: Vec<models::ErrorResponse>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ForbiddenErrorMessageResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "reason" => intermediate_rep.reason.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "requiredPermissions" => return std::result::Result::Err("Parsing a container in this style is not supported in ForbiddenErrorMessageResponse".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "errorResponse" => intermediate_rep.error_response.push(<models::ErrorResponse as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ForbiddenErrorMessageResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ForbiddenErrorMessageResponse {
            code: intermediate_rep.code.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
            reason: intermediate_rep.reason.into_iter().next(),
            required_permissions: intermediate_rep.required_permissions.into_iter().next(),
            error_response: intermediate_rep.error_response.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ForbiddenErrorMessageResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ForbiddenErrorMessageResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ForbiddenErrorMessageResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ForbiddenErrorMessageResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ForbiddenErrorMessageResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ForbiddenErrorMessageResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ForbiddenErrorMessageResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Represents errors when the server did not receive a timely response from an upstream server
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GatewayTimeoutErrorMessageResponse {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<i32>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "upstreamService")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub upstream_service: Option<String>,

    #[serde(rename = "errorResponse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_response: Option<models::ErrorResponse>,

}


impl GatewayTimeoutErrorMessageResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> GatewayTimeoutErrorMessageResponse {
        GatewayTimeoutErrorMessageResponse {
            code: None,
            message: None,
            upstream_service: None,
            error_response: None,
        }
    }
}

/// Converts the GatewayTimeoutErrorMessageResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GatewayTimeoutErrorMessageResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.code.as_ref().map(|code| {
                [
                    "code".to_string(),
                    code.to_string(),
                ].join(",")
            }),


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),


            self.upstream_service.as_ref().map(|upstream_service| {
                [
                    "upstreamService".to_string(),
                    upstream_service.to_string(),
                ].join(",")
            }),

            // Skipping errorResponse in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GatewayTimeoutErrorMessageResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GatewayTimeoutErrorMessageResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<i32>,
            pub message: Vec<String>,
            pub upstream_service: Vec<String>,
            pub error_response: Vec<models::ErrorResponse>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GatewayTimeoutErrorMessageResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "upstreamService" => intermediate_rep.upstream_service.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "errorResponse" => intermediate_rep.error_response.push(<models::ErrorResponse as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GatewayTimeoutErrorMessageResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GatewayTimeoutErrorMessageResponse {
            code: intermediate_rep.code.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
            upstream_service: intermediate_rep.upstream_service.into_iter().next(),
            error_response: intermediate_rep.error_response.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GatewayTimeoutErrorMessageResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GatewayTimeoutErrorMessageResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GatewayTimeoutErrorMessageResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GatewayTimeoutErrorMessageResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GatewayTimeoutErrorMessageResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GatewayTimeoutErrorMessageResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GatewayTimeoutErrorMessageResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetAccountResponse {
    #[serde(rename = "account")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub account: Option<models::Account>,

}


impl GetAccountResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> GetAccountResponse {
        GetAccountResponse {
            account: None,
        }
    }
}

/// Converts the GetAccountResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetAccountResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping account in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetAccountResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetAccountResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub account: Vec<models::Account>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetAccountResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "account" => intermediate_rep.account.push(<models::Account as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetAccountResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetAccountResponse {
            account: intermediate_rep.account.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetAccountResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetAccountResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetAccountResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetAccountResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetAccountResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetAccountResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetAccountResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetAccountUsageResponse {
    #[serde(rename = "totalJobsRun")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_jobs_run: Option<i32>,

    #[serde(rename = "monthlyJobLimit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub monthly_job_limit: Option<i32>,

    #[serde(rename = "remainingJobs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remaining_jobs: Option<i32>,

    #[serde(rename = "resetTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reset_time: Option<chrono::DateTime::<chrono::Utc>>,

}


impl GetAccountUsageResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> GetAccountUsageResponse {
        GetAccountUsageResponse {
            total_jobs_run: None,
            monthly_job_limit: None,
            remaining_jobs: None,
            reset_time: None,
        }
    }
}

/// Converts the GetAccountUsageResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetAccountUsageResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.total_jobs_run.as_ref().map(|total_jobs_run| {
                [
                    "totalJobsRun".to_string(),
                    total_jobs_run.to_string(),
                ].join(",")
            }),


            self.monthly_job_limit.as_ref().map(|monthly_job_limit| {
                [
                    "monthlyJobLimit".to_string(),
                    monthly_job_limit.to_string(),
                ].join(",")
            }),


            self.remaining_jobs.as_ref().map(|remaining_jobs| {
                [
                    "remainingJobs".to_string(),
                    remaining_jobs.to_string(),
                ].join(",")
            }),

            // Skipping resetTime in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetAccountUsageResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetAccountUsageResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub total_jobs_run: Vec<i32>,
            pub monthly_job_limit: Vec<i32>,
            pub remaining_jobs: Vec<i32>,
            pub reset_time: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetAccountUsageResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "totalJobsRun" => intermediate_rep.total_jobs_run.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "monthlyJobLimit" => intermediate_rep.monthly_job_limit.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "remainingJobs" => intermediate_rep.remaining_jobs.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "resetTime" => intermediate_rep.reset_time.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetAccountUsageResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetAccountUsageResponse {
            total_jobs_run: intermediate_rep.total_jobs_run.into_iter().next(),
            monthly_job_limit: intermediate_rep.monthly_job_limit.into_iter().next(),
            remaining_jobs: intermediate_rep.remaining_jobs.into_iter().next(),
            reset_time: intermediate_rep.reset_time.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetAccountUsageResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetAccountUsageResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetAccountUsageResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetAccountUsageResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetAccountUsageResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetAccountUsageResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetAccountUsageResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetScrapingJobResponse {
    #[serde(rename = "job")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job: Option<models::ScrapingJob>,

}


impl GetScrapingJobResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> GetScrapingJobResponse {
        GetScrapingJobResponse {
            job: None,
        }
    }
}

/// Converts the GetScrapingJobResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetScrapingJobResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping job in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetScrapingJobResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetScrapingJobResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub job: Vec<models::ScrapingJob>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetScrapingJobResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "job" => intermediate_rep.job.push(<models::ScrapingJob as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetScrapingJobResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetScrapingJobResponse {
            job: intermediate_rep.job.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetScrapingJobResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetScrapingJobResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetScrapingJobResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetScrapingJobResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetScrapingJobResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetScrapingJobResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetScrapingJobResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetWorkflowResponse {
    #[serde(rename = "workflow")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workflow: Option<models::ScrapingWorkflow>,

}


impl GetWorkflowResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> GetWorkflowResponse {
        GetWorkflowResponse {
            workflow: None,
        }
    }
}

/// Converts the GetWorkflowResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetWorkflowResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping workflow in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetWorkflowResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetWorkflowResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub workflow: Vec<models::ScrapingWorkflow>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetWorkflowResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "workflow" => intermediate_rep.workflow.push(<models::ScrapingWorkflow as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetWorkflowResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetWorkflowResponse {
            workflow: intermediate_rep.workflow.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetWorkflowResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetWorkflowResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetWorkflowResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetWorkflowResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetWorkflowResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetWorkflowResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetWorkflowResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetWorkspaceAnalyticsResponse {
    #[serde(rename = "totalLeads")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_leads: Option<i32>,

    #[serde(rename = "activeWorkflows")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active_workflows: Option<i32>,

    #[serde(rename = "jobsLast30Days")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub jobs_last30_days: Option<i32>,

    #[serde(rename = "successRates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub success_rates: Option<Vec<models::JobSuccessRate>>,

}


impl GetWorkspaceAnalyticsResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> GetWorkspaceAnalyticsResponse {
        GetWorkspaceAnalyticsResponse {
            total_leads: None,
            active_workflows: None,
            jobs_last30_days: None,
            success_rates: None,
        }
    }
}

/// Converts the GetWorkspaceAnalyticsResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetWorkspaceAnalyticsResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.total_leads.as_ref().map(|total_leads| {
                [
                    "totalLeads".to_string(),
                    total_leads.to_string(),
                ].join(",")
            }),


            self.active_workflows.as_ref().map(|active_workflows| {
                [
                    "activeWorkflows".to_string(),
                    active_workflows.to_string(),
                ].join(",")
            }),


            self.jobs_last30_days.as_ref().map(|jobs_last30_days| {
                [
                    "jobsLast30Days".to_string(),
                    jobs_last30_days.to_string(),
                ].join(",")
            }),

            // Skipping successRates in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetWorkspaceAnalyticsResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetWorkspaceAnalyticsResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub total_leads: Vec<i32>,
            pub active_workflows: Vec<i32>,
            pub jobs_last30_days: Vec<i32>,
            pub success_rates: Vec<Vec<models::JobSuccessRate>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetWorkspaceAnalyticsResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "totalLeads" => intermediate_rep.total_leads.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "activeWorkflows" => intermediate_rep.active_workflows.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "jobsLast30Days" => intermediate_rep.jobs_last30_days.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "successRates" => return std::result::Result::Err("Parsing a container in this style is not supported in GetWorkspaceAnalyticsResponse".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetWorkspaceAnalyticsResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetWorkspaceAnalyticsResponse {
            total_leads: intermediate_rep.total_leads.into_iter().next(),
            active_workflows: intermediate_rep.active_workflows.into_iter().next(),
            jobs_last30_days: intermediate_rep.jobs_last30_days.into_iter().next(),
            success_rates: intermediate_rep.success_rates.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetWorkspaceAnalyticsResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetWorkspaceAnalyticsResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetWorkspaceAnalyticsResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetWorkspaceAnalyticsResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetWorkspaceAnalyticsResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetWorkspaceAnalyticsResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetWorkspaceAnalyticsResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetWorkspaceResponse {
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workspace: Option<models::Workspace>,

}


impl GetWorkspaceResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> GetWorkspaceResponse {
        GetWorkspaceResponse {
            workspace: None,
        }
    }
}

/// Converts the GetWorkspaceResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetWorkspaceResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping workspace in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetWorkspaceResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetWorkspaceResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub workspace: Vec<models::Workspace>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetWorkspaceResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "workspace" => intermediate_rep.workspace.push(<models::Workspace as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetWorkspaceResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetWorkspaceResponse {
            workspace: intermediate_rep.workspace.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetWorkspaceResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetWorkspaceResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetWorkspaceResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetWorkspaceResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetWorkspaceResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetWorkspaceResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetWorkspaceResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Indicates that the resource requested is no longer available and will not be available again
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GoneErrorMessageResponse {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<i32>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "resource")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource: Option<String>,

    #[serde(rename = "errorResponse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_response: Option<models::ErrorResponse>,

}


impl GoneErrorMessageResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> GoneErrorMessageResponse {
        GoneErrorMessageResponse {
            code: None,
            message: None,
            resource: None,
            error_response: None,
        }
    }
}

/// Converts the GoneErrorMessageResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GoneErrorMessageResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.code.as_ref().map(|code| {
                [
                    "code".to_string(),
                    code.to_string(),
                ].join(",")
            }),


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),


            self.resource.as_ref().map(|resource| {
                [
                    "resource".to_string(),
                    resource.to_string(),
                ].join(",")
            }),

            // Skipping errorResponse in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GoneErrorMessageResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GoneErrorMessageResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<i32>,
            pub message: Vec<String>,
            pub resource: Vec<String>,
            pub error_response: Vec<models::ErrorResponse>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GoneErrorMessageResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "resource" => intermediate_rep.resource.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "errorResponse" => intermediate_rep.error_response.push(<models::ErrorResponse as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GoneErrorMessageResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GoneErrorMessageResponse {
            code: intermediate_rep.code.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
            resource: intermediate_rep.resource.into_iter().next(),
            error_response: intermediate_rep.error_response.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GoneErrorMessageResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GoneErrorMessageResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GoneErrorMessageResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GoneErrorMessageResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GoneErrorMessageResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GoneErrorMessageResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GoneErrorMessageResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// - INTERNAL_ERROR: General internal errors  - RESOURCE_EXHAUSTED: Resource exhaustion errors  - CANCELLED: Operation errors  - UNAVAILABLE: Availability errors  - DEPENDENCY_FAILURE: New internal error codes  Failure in external dependency  - CONFIGURATION_ERROR: Service configuration error  - SECURITY_BREACH_DETECTED: Potential security breach detected
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum InternalErrorCode {
    #[serde(rename = "NO_INTERNAL_ERROR")]
    NoInternalError,
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError,
    #[serde(rename = "DATA_CORRUPTION")]
    DataCorruption,
    #[serde(rename = "DATABASE_ERROR")]
    DatabaseError,
    #[serde(rename = "CACHE_ERROR")]
    CacheError,
    #[serde(rename = "RESOURCE_EXHAUSTED")]
    ResourceExhausted,
    #[serde(rename = "QUOTA_EXCEEDED")]
    QuotaExceeded,
    #[serde(rename = "RATE_LIMIT_EXCEEDED")]
    RateLimitExceeded,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "DEADLINE_EXCEEDED")]
    DeadlineExceeded,
    #[serde(rename = "ALREADY_EXISTS")]
    AlreadyExists,
    #[serde(rename = "FAILED_PRECONDITION")]
    FailedPrecondition,
    #[serde(rename = "ABORTED")]
    Aborted,
    #[serde(rename = "OUT_OF_RANGE")]
    OutOfRange,
    #[serde(rename = "UNAVAILABLE")]
    Unavailable,
    #[serde(rename = "DATA_LOSS")]
    DataLoss,
    #[serde(rename = "SERVICE_DEGRADED")]
    ServiceDegraded,
    #[serde(rename = "DEPENDENCY_FAILURE")]
    DependencyFailure,
    #[serde(rename = "CONFIGURATION_ERROR")]
    ConfigurationError,
    #[serde(rename = "SECURITY_BREACH_DETECTED")]
    SecurityBreachDetected,
}

impl std::fmt::Display for InternalErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            InternalErrorCode::NoInternalError => write!(f, "NO_INTERNAL_ERROR"),
            InternalErrorCode::InternalError => write!(f, "INTERNAL_ERROR"),
            InternalErrorCode::DataCorruption => write!(f, "DATA_CORRUPTION"),
            InternalErrorCode::DatabaseError => write!(f, "DATABASE_ERROR"),
            InternalErrorCode::CacheError => write!(f, "CACHE_ERROR"),
            InternalErrorCode::ResourceExhausted => write!(f, "RESOURCE_EXHAUSTED"),
            InternalErrorCode::QuotaExceeded => write!(f, "QUOTA_EXCEEDED"),
            InternalErrorCode::RateLimitExceeded => write!(f, "RATE_LIMIT_EXCEEDED"),
            InternalErrorCode::Cancelled => write!(f, "CANCELLED"),
            InternalErrorCode::DeadlineExceeded => write!(f, "DEADLINE_EXCEEDED"),
            InternalErrorCode::AlreadyExists => write!(f, "ALREADY_EXISTS"),
            InternalErrorCode::FailedPrecondition => write!(f, "FAILED_PRECONDITION"),
            InternalErrorCode::Aborted => write!(f, "ABORTED"),
            InternalErrorCode::OutOfRange => write!(f, "OUT_OF_RANGE"),
            InternalErrorCode::Unavailable => write!(f, "UNAVAILABLE"),
            InternalErrorCode::DataLoss => write!(f, "DATA_LOSS"),
            InternalErrorCode::ServiceDegraded => write!(f, "SERVICE_DEGRADED"),
            InternalErrorCode::DependencyFailure => write!(f, "DEPENDENCY_FAILURE"),
            InternalErrorCode::ConfigurationError => write!(f, "CONFIGURATION_ERROR"),
            InternalErrorCode::SecurityBreachDetected => write!(f, "SECURITY_BREACH_DETECTED"),
        }
    }
}

impl std::str::FromStr for InternalErrorCode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "NO_INTERNAL_ERROR" => std::result::Result::Ok(InternalErrorCode::NoInternalError),
            "INTERNAL_ERROR" => std::result::Result::Ok(InternalErrorCode::InternalError),
            "DATA_CORRUPTION" => std::result::Result::Ok(InternalErrorCode::DataCorruption),
            "DATABASE_ERROR" => std::result::Result::Ok(InternalErrorCode::DatabaseError),
            "CACHE_ERROR" => std::result::Result::Ok(InternalErrorCode::CacheError),
            "RESOURCE_EXHAUSTED" => std::result::Result::Ok(InternalErrorCode::ResourceExhausted),
            "QUOTA_EXCEEDED" => std::result::Result::Ok(InternalErrorCode::QuotaExceeded),
            "RATE_LIMIT_EXCEEDED" => std::result::Result::Ok(InternalErrorCode::RateLimitExceeded),
            "CANCELLED" => std::result::Result::Ok(InternalErrorCode::Cancelled),
            "DEADLINE_EXCEEDED" => std::result::Result::Ok(InternalErrorCode::DeadlineExceeded),
            "ALREADY_EXISTS" => std::result::Result::Ok(InternalErrorCode::AlreadyExists),
            "FAILED_PRECONDITION" => std::result::Result::Ok(InternalErrorCode::FailedPrecondition),
            "ABORTED" => std::result::Result::Ok(InternalErrorCode::Aborted),
            "OUT_OF_RANGE" => std::result::Result::Ok(InternalErrorCode::OutOfRange),
            "UNAVAILABLE" => std::result::Result::Ok(InternalErrorCode::Unavailable),
            "DATA_LOSS" => std::result::Result::Ok(InternalErrorCode::DataLoss),
            "SERVICE_DEGRADED" => std::result::Result::Ok(InternalErrorCode::ServiceDegraded),
            "DEPENDENCY_FAILURE" => std::result::Result::Ok(InternalErrorCode::DependencyFailure),
            "CONFIGURATION_ERROR" => std::result::Result::Ok(InternalErrorCode::ConfigurationError),
            "SECURITY_BREACH_DETECTED" => std::result::Result::Ok(InternalErrorCode::SecurityBreachDetected),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// Represents internal server and system-level errors
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InternalErrorMessageResponse {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<models::InternalErrorCode>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "incidentId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub incident_id: Option<String>,

    #[serde(rename = "serviceStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_status: Option<models::ServiceStatus>,

    #[serde(rename = "resourceUtilization")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_utilization: Option<models::ResourceUtilization>,

    #[serde(rename = "operationDetails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub operation_details: Option<models::OperationDetails>,

    #[serde(rename = "errorResponse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_response: Option<models::ErrorResponse>,

}


impl InternalErrorMessageResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> InternalErrorMessageResponse {
        InternalErrorMessageResponse {
            code: None,
            message: None,
            incident_id: None,
            service_status: None,
            resource_utilization: None,
            operation_details: None,
            error_response: None,
        }
    }
}

/// Converts the InternalErrorMessageResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InternalErrorMessageResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping code in query parameter serialization


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),


            self.incident_id.as_ref().map(|incident_id| {
                [
                    "incidentId".to_string(),
                    incident_id.to_string(),
                ].join(",")
            }),

            // Skipping serviceStatus in query parameter serialization

            // Skipping resourceUtilization in query parameter serialization

            // Skipping operationDetails in query parameter serialization

            // Skipping errorResponse in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InternalErrorMessageResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InternalErrorMessageResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<models::InternalErrorCode>,
            pub message: Vec<String>,
            pub incident_id: Vec<String>,
            pub service_status: Vec<models::ServiceStatus>,
            pub resource_utilization: Vec<models::ResourceUtilization>,
            pub operation_details: Vec<models::OperationDetails>,
            pub error_response: Vec<models::ErrorResponse>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing InternalErrorMessageResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(<models::InternalErrorCode as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "incidentId" => intermediate_rep.incident_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "serviceStatus" => intermediate_rep.service_status.push(<models::ServiceStatus as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "resourceUtilization" => intermediate_rep.resource_utilization.push(<models::ResourceUtilization as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "operationDetails" => intermediate_rep.operation_details.push(<models::OperationDetails as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "errorResponse" => intermediate_rep.error_response.push(<models::ErrorResponse as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing InternalErrorMessageResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InternalErrorMessageResponse {
            code: intermediate_rep.code.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
            incident_id: intermediate_rep.incident_id.into_iter().next(),
            service_status: intermediate_rep.service_status.into_iter().next(),
            resource_utilization: intermediate_rep.resource_utilization.into_iter().next(),
            operation_details: intermediate_rep.operation_details.into_iter().next(),
            error_response: intermediate_rep.error_response.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InternalErrorMessageResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InternalErrorMessageResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<InternalErrorMessageResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for InternalErrorMessageResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<InternalErrorMessageResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InternalErrorMessageResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InternalErrorMessageResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct JobSuccessRate {
    #[serde(rename = "workflowId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workflow_id: Option<String>,

    #[serde(rename = "successRate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub success_rate: Option<f32>,

    #[serde(rename = "totalRuns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_runs: Option<i32>,

}


impl JobSuccessRate {
    #[allow(clippy::new_without_default)]
    pub fn new() -> JobSuccessRate {
        JobSuccessRate {
            workflow_id: None,
            success_rate: None,
            total_runs: None,
        }
    }
}

/// Converts the JobSuccessRate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for JobSuccessRate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.workflow_id.as_ref().map(|workflow_id| {
                [
                    "workflowId".to_string(),
                    workflow_id.to_string(),
                ].join(",")
            }),


            self.success_rate.as_ref().map(|success_rate| {
                [
                    "successRate".to_string(),
                    success_rate.to_string(),
                ].join(",")
            }),


            self.total_runs.as_ref().map(|total_runs| {
                [
                    "totalRuns".to_string(),
                    total_runs.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a JobSuccessRate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for JobSuccessRate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub workflow_id: Vec<String>,
            pub success_rate: Vec<f32>,
            pub total_runs: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing JobSuccessRate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "workflowId" => intermediate_rep.workflow_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "successRate" => intermediate_rep.success_rate.push(<f32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "totalRuns" => intermediate_rep.total_runs.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing JobSuccessRate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(JobSuccessRate {
            workflow_id: intermediate_rep.workflow_id.into_iter().next(),
            success_rate: intermediate_rep.success_rate.into_iter().next(),
            total_runs: intermediate_rep.total_runs.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<JobSuccessRate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<JobSuccessRate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<JobSuccessRate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for JobSuccessRate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<JobSuccessRate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <JobSuccessRate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into JobSuccessRate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Lead {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "website")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub website: Option<String>,

    #[serde(rename = "phone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub phone: Option<String>,

    #[serde(rename = "address")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub address: Option<String>,

    #[serde(rename = "city")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub city: Option<String>,

    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "country")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub country: Option<String>,

    #[serde(rename = "latitude")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub latitude: Option<f64>,

    #[serde(rename = "longitude")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub longitude: Option<f64>,

    #[serde(rename = "googleRating")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub google_rating: Option<f32>,

    #[serde(rename = "reviewCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub review_count: Option<i32>,

    #[serde(rename = "industry")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub industry: Option<String>,

    #[serde(rename = "employeeCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub employee_count: Option<i32>,

    #[serde(rename = "estimatedRevenue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub estimated_revenue: Option<String>,

    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub org_id: Option<String>,

    #[serde(rename = "tenantId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tenant_id: Option<String>,

    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "job")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job: Option<models::ScrapingJob>,

    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workspace: Option<models::Workspace>,

    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub place_id: Option<String>,

    #[serde(rename = "googleMapsUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub google_maps_url: Option<String>,

    #[serde(rename = "businessStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub business_status: Option<String>,

    #[serde(rename = "regularHours")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub regular_hours: Option<Vec<models::BusinessHours>>,

    #[serde(rename = "specialHours")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub special_hours: Option<Vec<models::BusinessHours>>,

    /// Google Places photo IDs
    #[serde(rename = "photoReferences")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub photo_references: Option<Vec<String>>,

    #[serde(rename = "mainPhotoUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub main_photo_url: Option<String>,

    #[serde(rename = "reviews")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reviews: Option<Vec<models::Review>>,

    /// Google Places types
    #[serde(rename = "types")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub types: Option<Vec<String>>,

    #[serde(rename = "amenities")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub amenities: Option<Vec<String>>,

    #[serde(rename = "servesVegetarianFood")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub serves_vegetarian_food: Option<bool>,

    #[serde(rename = "outdoorSeating")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub outdoor_seating: Option<bool>,

    /// visa, amex, cash, etc
    #[serde(rename = "paymentMethods")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub payment_methods: Option<Vec<String>>,

    #[serde(rename = "wheelchairAccessible")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub wheelchair_accessible: Option<bool>,

    #[serde(rename = "parkingAvailable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parking_available: Option<bool>,

    /// platform -> url
    #[serde(rename = "socialMedia")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub social_media: Option<std::collections::HashMap<String, String>>,

    #[serde(rename = "ratingCategory")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rating_category: Option<String>,

    #[serde(rename = "rating")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rating: Option<f32>,

    #[serde(rename = "count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub count: Option<i32>,

    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_updated: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "dataSourceVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub data_source_version: Option<String>,

    #[serde(rename = "scrapingSessionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scraping_session_id: Option<String>,

    #[serde(rename = "alternatePhones")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alternate_phones: Option<Vec<String>>,

    #[serde(rename = "contactPersonName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contact_person_name: Option<String>,

    #[serde(rename = "contactPersonTitle")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contact_person_title: Option<String>,

    #[serde(rename = "contactEmail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contact_email: Option<String>,

    #[serde(rename = "foundedYear")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub founded_year: Option<i32>,

    #[serde(rename = "businessType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub business_type: Option<String>,

    #[serde(rename = "certifications")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certifications: Option<Vec<String>>,

    #[serde(rename = "licenseNumber")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub license_number: Option<String>,

    #[serde(rename = "revenueRange")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub revenue_range: Option<models::RevenueRange>,

    #[serde(rename = "fundingStage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub funding_stage: Option<String>,

    #[serde(rename = "isPublicCompany")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_public_company: Option<bool>,

    /// In seconds
    #[serde(rename = "websiteLoadSpeed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub website_load_speed: Option<f32>,

    #[serde(rename = "hasSslCertificate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_ssl_certificate: Option<bool>,

    #[serde(rename = "cmsUsed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cms_used: Option<String>,

    #[serde(rename = "ecommercePlatforms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ecommerce_platforms: Option<Vec<String>>,

    #[serde(rename = "timezone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timezone: Option<String>,

    #[serde(rename = "neighborhood")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub neighborhood: Option<String>,

    #[serde(rename = "nearbyLandmarks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub nearby_landmarks: Option<Vec<String>>,

    #[serde(rename = "transportationAccess")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub transportation_access: Option<String>,

    #[serde(rename = "employeeBenefits")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub employee_benefits: Option<Vec<models::EmployeeBenefit>>,

    #[serde(rename = "parentCompany")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent_company: Option<String>,

    #[serde(rename = "subsidiaries")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subsidiaries: Option<Vec<String>>,

    #[serde(rename = "isFranchise")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_franchise: Option<bool>,

    #[serde(rename = "seoKeywords")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub seo_keywords: Option<Vec<String>>,

    #[serde(rename = "usesGoogleAds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub uses_google_ads: Option<bool>,

    #[serde(rename = "googleMyBusinessCategory")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub google_my_business_category: Option<String>,

    #[serde(rename = "naicsCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub naics_code: Option<String>,

    #[serde(rename = "sicCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sic_code: Option<String>,

    #[serde(rename = "unspscCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub unspsc_code: Option<String>,

    #[serde(rename = "isGreenCertified")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_green_certified: Option<bool>,

    #[serde(rename = "energySources")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub energy_sources: Option<Vec<String>>,

    #[serde(rename = "sustainabilityRating")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sustainability_rating: Option<String>,

    #[serde(rename = "recentAnnouncements")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub recent_announcements: Option<Vec<String>>,

    #[serde(rename = "lastProductLaunch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_product_launch: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "hasLitigationHistory")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_litigation_history: Option<bool>,

    #[serde(rename = "exportControlStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub export_control_status: Option<String>,

}


impl Lead {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Lead {
        Lead {
            id: None,
            name: None,
            website: None,
            phone: None,
            address: None,
            city: None,
            state: None,
            country: None,
            latitude: None,
            longitude: None,
            google_rating: None,
            review_count: None,
            industry: None,
            employee_count: None,
            estimated_revenue: None,
            org_id: None,
            tenant_id: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
            job: None,
            workspace: None,
            place_id: None,
            google_maps_url: None,
            business_status: None,
            regular_hours: None,
            special_hours: None,
            photo_references: None,
            main_photo_url: None,
            reviews: None,
            types: None,
            amenities: None,
            serves_vegetarian_food: None,
            outdoor_seating: None,
            payment_methods: None,
            wheelchair_accessible: None,
            parking_available: None,
            social_media: None,
            rating_category: None,
            rating: None,
            count: None,
            last_updated: None,
            data_source_version: None,
            scraping_session_id: None,
            alternate_phones: None,
            contact_person_name: None,
            contact_person_title: None,
            contact_email: None,
            founded_year: None,
            business_type: None,
            certifications: None,
            license_number: None,
            revenue_range: None,
            funding_stage: None,
            is_public_company: None,
            website_load_speed: None,
            has_ssl_certificate: None,
            cms_used: None,
            ecommerce_platforms: None,
            timezone: None,
            neighborhood: None,
            nearby_landmarks: None,
            transportation_access: None,
            employee_benefits: None,
            parent_company: None,
            subsidiaries: None,
            is_franchise: None,
            seo_keywords: None,
            uses_google_ads: None,
            google_my_business_category: None,
            naics_code: None,
            sic_code: None,
            unspsc_code: None,
            is_green_certified: None,
            energy_sources: None,
            sustainability_rating: None,
            recent_announcements: None,
            last_product_launch: None,
            has_litigation_history: None,
            export_control_status: None,
        }
    }
}

/// Converts the Lead value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Lead {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.id.as_ref().map(|id| {
                [
                    "id".to_string(),
                    id.to_string(),
                ].join(",")
            }),


            self.name.as_ref().map(|name| {
                [
                    "name".to_string(),
                    name.to_string(),
                ].join(",")
            }),


            self.website.as_ref().map(|website| {
                [
                    "website".to_string(),
                    website.to_string(),
                ].join(",")
            }),


            self.phone.as_ref().map(|phone| {
                [
                    "phone".to_string(),
                    phone.to_string(),
                ].join(",")
            }),


            self.address.as_ref().map(|address| {
                [
                    "address".to_string(),
                    address.to_string(),
                ].join(",")
            }),


            self.city.as_ref().map(|city| {
                [
                    "city".to_string(),
                    city.to_string(),
                ].join(",")
            }),


            self.state.as_ref().map(|state| {
                [
                    "state".to_string(),
                    state.to_string(),
                ].join(",")
            }),


            self.country.as_ref().map(|country| {
                [
                    "country".to_string(),
                    country.to_string(),
                ].join(",")
            }),


            self.latitude.as_ref().map(|latitude| {
                [
                    "latitude".to_string(),
                    latitude.to_string(),
                ].join(",")
            }),


            self.longitude.as_ref().map(|longitude| {
                [
                    "longitude".to_string(),
                    longitude.to_string(),
                ].join(",")
            }),


            self.google_rating.as_ref().map(|google_rating| {
                [
                    "googleRating".to_string(),
                    google_rating.to_string(),
                ].join(",")
            }),


            self.review_count.as_ref().map(|review_count| {
                [
                    "reviewCount".to_string(),
                    review_count.to_string(),
                ].join(",")
            }),


            self.industry.as_ref().map(|industry| {
                [
                    "industry".to_string(),
                    industry.to_string(),
                ].join(",")
            }),


            self.employee_count.as_ref().map(|employee_count| {
                [
                    "employeeCount".to_string(),
                    employee_count.to_string(),
                ].join(",")
            }),


            self.estimated_revenue.as_ref().map(|estimated_revenue| {
                [
                    "estimatedRevenue".to_string(),
                    estimated_revenue.to_string(),
                ].join(",")
            }),


            self.org_id.as_ref().map(|org_id| {
                [
                    "orgId".to_string(),
                    org_id.to_string(),
                ].join(",")
            }),


            self.tenant_id.as_ref().map(|tenant_id| {
                [
                    "tenantId".to_string(),
                    tenant_id.to_string(),
                ].join(",")
            }),

            // Skipping createdAt in query parameter serialization

            // Skipping updatedAt in query parameter serialization

            // Skipping deletedAt in query parameter serialization

            // Skipping job in query parameter serialization

            // Skipping workspace in query parameter serialization


            self.place_id.as_ref().map(|place_id| {
                [
                    "placeId".to_string(),
                    place_id.to_string(),
                ].join(",")
            }),


            self.google_maps_url.as_ref().map(|google_maps_url| {
                [
                    "googleMapsUrl".to_string(),
                    google_maps_url.to_string(),
                ].join(",")
            }),


            self.business_status.as_ref().map(|business_status| {
                [
                    "businessStatus".to_string(),
                    business_status.to_string(),
                ].join(",")
            }),

            // Skipping regularHours in query parameter serialization

            // Skipping specialHours in query parameter serialization


            self.photo_references.as_ref().map(|photo_references| {
                [
                    "photoReferences".to_string(),
                    photo_references.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.main_photo_url.as_ref().map(|main_photo_url| {
                [
                    "mainPhotoUrl".to_string(),
                    main_photo_url.to_string(),
                ].join(",")
            }),

            // Skipping reviews in query parameter serialization


            self.types.as_ref().map(|types| {
                [
                    "types".to_string(),
                    types.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.amenities.as_ref().map(|amenities| {
                [
                    "amenities".to_string(),
                    amenities.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.serves_vegetarian_food.as_ref().map(|serves_vegetarian_food| {
                [
                    "servesVegetarianFood".to_string(),
                    serves_vegetarian_food.to_string(),
                ].join(",")
            }),


            self.outdoor_seating.as_ref().map(|outdoor_seating| {
                [
                    "outdoorSeating".to_string(),
                    outdoor_seating.to_string(),
                ].join(",")
            }),


            self.payment_methods.as_ref().map(|payment_methods| {
                [
                    "paymentMethods".to_string(),
                    payment_methods.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.wheelchair_accessible.as_ref().map(|wheelchair_accessible| {
                [
                    "wheelchairAccessible".to_string(),
                    wheelchair_accessible.to_string(),
                ].join(",")
            }),


            self.parking_available.as_ref().map(|parking_available| {
                [
                    "parkingAvailable".to_string(),
                    parking_available.to_string(),
                ].join(",")
            }),

            // Skipping socialMedia in query parameter serialization


            self.rating_category.as_ref().map(|rating_category| {
                [
                    "ratingCategory".to_string(),
                    rating_category.to_string(),
                ].join(",")
            }),


            self.rating.as_ref().map(|rating| {
                [
                    "rating".to_string(),
                    rating.to_string(),
                ].join(",")
            }),


            self.count.as_ref().map(|count| {
                [
                    "count".to_string(),
                    count.to_string(),
                ].join(",")
            }),

            // Skipping lastUpdated in query parameter serialization


            self.data_source_version.as_ref().map(|data_source_version| {
                [
                    "dataSourceVersion".to_string(),
                    data_source_version.to_string(),
                ].join(",")
            }),


            self.scraping_session_id.as_ref().map(|scraping_session_id| {
                [
                    "scrapingSessionId".to_string(),
                    scraping_session_id.to_string(),
                ].join(",")
            }),


            self.alternate_phones.as_ref().map(|alternate_phones| {
                [
                    "alternatePhones".to_string(),
                    alternate_phones.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.contact_person_name.as_ref().map(|contact_person_name| {
                [
                    "contactPersonName".to_string(),
                    contact_person_name.to_string(),
                ].join(",")
            }),


            self.contact_person_title.as_ref().map(|contact_person_title| {
                [
                    "contactPersonTitle".to_string(),
                    contact_person_title.to_string(),
                ].join(",")
            }),


            self.contact_email.as_ref().map(|contact_email| {
                [
                    "contactEmail".to_string(),
                    contact_email.to_string(),
                ].join(",")
            }),


            self.founded_year.as_ref().map(|founded_year| {
                [
                    "foundedYear".to_string(),
                    founded_year.to_string(),
                ].join(",")
            }),


            self.business_type.as_ref().map(|business_type| {
                [
                    "businessType".to_string(),
                    business_type.to_string(),
                ].join(",")
            }),


            self.certifications.as_ref().map(|certifications| {
                [
                    "certifications".to_string(),
                    certifications.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.license_number.as_ref().map(|license_number| {
                [
                    "licenseNumber".to_string(),
                    license_number.to_string(),
                ].join(",")
            }),

            // Skipping revenueRange in query parameter serialization


            self.funding_stage.as_ref().map(|funding_stage| {
                [
                    "fundingStage".to_string(),
                    funding_stage.to_string(),
                ].join(",")
            }),


            self.is_public_company.as_ref().map(|is_public_company| {
                [
                    "isPublicCompany".to_string(),
                    is_public_company.to_string(),
                ].join(",")
            }),


            self.website_load_speed.as_ref().map(|website_load_speed| {
                [
                    "websiteLoadSpeed".to_string(),
                    website_load_speed.to_string(),
                ].join(",")
            }),


            self.has_ssl_certificate.as_ref().map(|has_ssl_certificate| {
                [
                    "hasSslCertificate".to_string(),
                    has_ssl_certificate.to_string(),
                ].join(",")
            }),


            self.cms_used.as_ref().map(|cms_used| {
                [
                    "cmsUsed".to_string(),
                    cms_used.to_string(),
                ].join(",")
            }),


            self.ecommerce_platforms.as_ref().map(|ecommerce_platforms| {
                [
                    "ecommercePlatforms".to_string(),
                    ecommerce_platforms.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.timezone.as_ref().map(|timezone| {
                [
                    "timezone".to_string(),
                    timezone.to_string(),
                ].join(",")
            }),


            self.neighborhood.as_ref().map(|neighborhood| {
                [
                    "neighborhood".to_string(),
                    neighborhood.to_string(),
                ].join(",")
            }),


            self.nearby_landmarks.as_ref().map(|nearby_landmarks| {
                [
                    "nearbyLandmarks".to_string(),
                    nearby_landmarks.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.transportation_access.as_ref().map(|transportation_access| {
                [
                    "transportationAccess".to_string(),
                    transportation_access.to_string(),
                ].join(",")
            }),

            // Skipping employeeBenefits in query parameter serialization


            self.parent_company.as_ref().map(|parent_company| {
                [
                    "parentCompany".to_string(),
                    parent_company.to_string(),
                ].join(",")
            }),


            self.subsidiaries.as_ref().map(|subsidiaries| {
                [
                    "subsidiaries".to_string(),
                    subsidiaries.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.is_franchise.as_ref().map(|is_franchise| {
                [
                    "isFranchise".to_string(),
                    is_franchise.to_string(),
                ].join(",")
            }),


            self.seo_keywords.as_ref().map(|seo_keywords| {
                [
                    "seoKeywords".to_string(),
                    seo_keywords.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.uses_google_ads.as_ref().map(|uses_google_ads| {
                [
                    "usesGoogleAds".to_string(),
                    uses_google_ads.to_string(),
                ].join(",")
            }),


            self.google_my_business_category.as_ref().map(|google_my_business_category| {
                [
                    "googleMyBusinessCategory".to_string(),
                    google_my_business_category.to_string(),
                ].join(",")
            }),


            self.naics_code.as_ref().map(|naics_code| {
                [
                    "naicsCode".to_string(),
                    naics_code.to_string(),
                ].join(",")
            }),


            self.sic_code.as_ref().map(|sic_code| {
                [
                    "sicCode".to_string(),
                    sic_code.to_string(),
                ].join(",")
            }),


            self.unspsc_code.as_ref().map(|unspsc_code| {
                [
                    "unspscCode".to_string(),
                    unspsc_code.to_string(),
                ].join(",")
            }),


            self.is_green_certified.as_ref().map(|is_green_certified| {
                [
                    "isGreenCertified".to_string(),
                    is_green_certified.to_string(),
                ].join(",")
            }),


            self.energy_sources.as_ref().map(|energy_sources| {
                [
                    "energySources".to_string(),
                    energy_sources.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.sustainability_rating.as_ref().map(|sustainability_rating| {
                [
                    "sustainabilityRating".to_string(),
                    sustainability_rating.to_string(),
                ].join(",")
            }),


            self.recent_announcements.as_ref().map(|recent_announcements| {
                [
                    "recentAnnouncements".to_string(),
                    recent_announcements.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

            // Skipping lastProductLaunch in query parameter serialization


            self.has_litigation_history.as_ref().map(|has_litigation_history| {
                [
                    "hasLitigationHistory".to_string(),
                    has_litigation_history.to_string(),
                ].join(",")
            }),


            self.export_control_status.as_ref().map(|export_control_status| {
                [
                    "exportControlStatus".to_string(),
                    export_control_status.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Lead value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Lead {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub name: Vec<String>,
            pub website: Vec<String>,
            pub phone: Vec<String>,
            pub address: Vec<String>,
            pub city: Vec<String>,
            pub state: Vec<String>,
            pub country: Vec<String>,
            pub latitude: Vec<f64>,
            pub longitude: Vec<f64>,
            pub google_rating: Vec<f32>,
            pub review_count: Vec<i32>,
            pub industry: Vec<String>,
            pub employee_count: Vec<i32>,
            pub estimated_revenue: Vec<String>,
            pub org_id: Vec<String>,
            pub tenant_id: Vec<String>,
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub updated_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub deleted_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub job: Vec<models::ScrapingJob>,
            pub workspace: Vec<models::Workspace>,
            pub place_id: Vec<String>,
            pub google_maps_url: Vec<String>,
            pub business_status: Vec<String>,
            pub regular_hours: Vec<Vec<models::BusinessHours>>,
            pub special_hours: Vec<Vec<models::BusinessHours>>,
            pub photo_references: Vec<Vec<String>>,
            pub main_photo_url: Vec<String>,
            pub reviews: Vec<Vec<models::Review>>,
            pub types: Vec<Vec<String>>,
            pub amenities: Vec<Vec<String>>,
            pub serves_vegetarian_food: Vec<bool>,
            pub outdoor_seating: Vec<bool>,
            pub payment_methods: Vec<Vec<String>>,
            pub wheelchair_accessible: Vec<bool>,
            pub parking_available: Vec<bool>,
            pub social_media: Vec<std::collections::HashMap<String, String>>,
            pub rating_category: Vec<String>,
            pub rating: Vec<f32>,
            pub count: Vec<i32>,
            pub last_updated: Vec<chrono::DateTime::<chrono::Utc>>,
            pub data_source_version: Vec<String>,
            pub scraping_session_id: Vec<String>,
            pub alternate_phones: Vec<Vec<String>>,
            pub contact_person_name: Vec<String>,
            pub contact_person_title: Vec<String>,
            pub contact_email: Vec<String>,
            pub founded_year: Vec<i32>,
            pub business_type: Vec<String>,
            pub certifications: Vec<Vec<String>>,
            pub license_number: Vec<String>,
            pub revenue_range: Vec<models::RevenueRange>,
            pub funding_stage: Vec<String>,
            pub is_public_company: Vec<bool>,
            pub website_load_speed: Vec<f32>,
            pub has_ssl_certificate: Vec<bool>,
            pub cms_used: Vec<String>,
            pub ecommerce_platforms: Vec<Vec<String>>,
            pub timezone: Vec<String>,
            pub neighborhood: Vec<String>,
            pub nearby_landmarks: Vec<Vec<String>>,
            pub transportation_access: Vec<String>,
            pub employee_benefits: Vec<Vec<models::EmployeeBenefit>>,
            pub parent_company: Vec<String>,
            pub subsidiaries: Vec<Vec<String>>,
            pub is_franchise: Vec<bool>,
            pub seo_keywords: Vec<Vec<String>>,
            pub uses_google_ads: Vec<bool>,
            pub google_my_business_category: Vec<String>,
            pub naics_code: Vec<String>,
            pub sic_code: Vec<String>,
            pub unspsc_code: Vec<String>,
            pub is_green_certified: Vec<bool>,
            pub energy_sources: Vec<Vec<String>>,
            pub sustainability_rating: Vec<String>,
            pub recent_announcements: Vec<Vec<String>>,
            pub last_product_launch: Vec<chrono::DateTime::<chrono::Utc>>,
            pub has_litigation_history: Vec<bool>,
            pub export_control_status: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Lead".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "website" => intermediate_rep.website.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "phone" => intermediate_rep.phone.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "address" => intermediate_rep.address.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "city" => intermediate_rep.city.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "state" => intermediate_rep.state.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "country" => intermediate_rep.country.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "latitude" => intermediate_rep.latitude.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "longitude" => intermediate_rep.longitude.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "googleRating" => intermediate_rep.google_rating.push(<f32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "reviewCount" => intermediate_rep.review_count.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "industry" => intermediate_rep.industry.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "employeeCount" => intermediate_rep.employee_count.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "estimatedRevenue" => intermediate_rep.estimated_revenue.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "orgId" => intermediate_rep.org_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "tenantId" => intermediate_rep.tenant_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "createdAt" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "updatedAt" => intermediate_rep.updated_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "deletedAt" => intermediate_rep.deleted_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "job" => intermediate_rep.job.push(<models::ScrapingJob as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "workspace" => intermediate_rep.workspace.push(<models::Workspace as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "placeId" => intermediate_rep.place_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "googleMapsUrl" => intermediate_rep.google_maps_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "businessStatus" => intermediate_rep.business_status.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "regularHours" => return std::result::Result::Err("Parsing a container in this style is not supported in Lead".to_string()),
                    "specialHours" => return std::result::Result::Err("Parsing a container in this style is not supported in Lead".to_string()),
                    "photoReferences" => return std::result::Result::Err("Parsing a container in this style is not supported in Lead".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "mainPhotoUrl" => intermediate_rep.main_photo_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "reviews" => return std::result::Result::Err("Parsing a container in this style is not supported in Lead".to_string()),
                    "types" => return std::result::Result::Err("Parsing a container in this style is not supported in Lead".to_string()),
                    "amenities" => return std::result::Result::Err("Parsing a container in this style is not supported in Lead".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "servesVegetarianFood" => intermediate_rep.serves_vegetarian_food.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "outdoorSeating" => intermediate_rep.outdoor_seating.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "paymentMethods" => return std::result::Result::Err("Parsing a container in this style is not supported in Lead".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "wheelchairAccessible" => intermediate_rep.wheelchair_accessible.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "parkingAvailable" => intermediate_rep.parking_available.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "socialMedia" => return std::result::Result::Err("Parsing a container in this style is not supported in Lead".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "ratingCategory" => intermediate_rep.rating_category.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "rating" => intermediate_rep.rating.push(<f32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "count" => intermediate_rep.count.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "lastUpdated" => intermediate_rep.last_updated.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "dataSourceVersion" => intermediate_rep.data_source_version.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "scrapingSessionId" => intermediate_rep.scraping_session_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "alternatePhones" => return std::result::Result::Err("Parsing a container in this style is not supported in Lead".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "contactPersonName" => intermediate_rep.contact_person_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "contactPersonTitle" => intermediate_rep.contact_person_title.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "contactEmail" => intermediate_rep.contact_email.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "foundedYear" => intermediate_rep.founded_year.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "businessType" => intermediate_rep.business_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "certifications" => return std::result::Result::Err("Parsing a container in this style is not supported in Lead".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "licenseNumber" => intermediate_rep.license_number.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "revenueRange" => intermediate_rep.revenue_range.push(<models::RevenueRange as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "fundingStage" => intermediate_rep.funding_stage.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "isPublicCompany" => intermediate_rep.is_public_company.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "websiteLoadSpeed" => intermediate_rep.website_load_speed.push(<f32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "hasSslCertificate" => intermediate_rep.has_ssl_certificate.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "cmsUsed" => intermediate_rep.cms_used.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "ecommercePlatforms" => return std::result::Result::Err("Parsing a container in this style is not supported in Lead".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "timezone" => intermediate_rep.timezone.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "neighborhood" => intermediate_rep.neighborhood.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "nearbyLandmarks" => return std::result::Result::Err("Parsing a container in this style is not supported in Lead".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "transportationAccess" => intermediate_rep.transportation_access.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "employeeBenefits" => return std::result::Result::Err("Parsing a container in this style is not supported in Lead".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "parentCompany" => intermediate_rep.parent_company.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "subsidiaries" => return std::result::Result::Err("Parsing a container in this style is not supported in Lead".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "isFranchise" => intermediate_rep.is_franchise.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "seoKeywords" => return std::result::Result::Err("Parsing a container in this style is not supported in Lead".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "usesGoogleAds" => intermediate_rep.uses_google_ads.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "googleMyBusinessCategory" => intermediate_rep.google_my_business_category.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "naicsCode" => intermediate_rep.naics_code.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "sicCode" => intermediate_rep.sic_code.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "unspscCode" => intermediate_rep.unspsc_code.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "isGreenCertified" => intermediate_rep.is_green_certified.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "energySources" => return std::result::Result::Err("Parsing a container in this style is not supported in Lead".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "sustainabilityRating" => intermediate_rep.sustainability_rating.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "recentAnnouncements" => return std::result::Result::Err("Parsing a container in this style is not supported in Lead".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "lastProductLaunch" => intermediate_rep.last_product_launch.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "hasLitigationHistory" => intermediate_rep.has_litigation_history.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "exportControlStatus" => intermediate_rep.export_control_status.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Lead".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Lead {
            id: intermediate_rep.id.into_iter().next(),
            name: intermediate_rep.name.into_iter().next(),
            website: intermediate_rep.website.into_iter().next(),
            phone: intermediate_rep.phone.into_iter().next(),
            address: intermediate_rep.address.into_iter().next(),
            city: intermediate_rep.city.into_iter().next(),
            state: intermediate_rep.state.into_iter().next(),
            country: intermediate_rep.country.into_iter().next(),
            latitude: intermediate_rep.latitude.into_iter().next(),
            longitude: intermediate_rep.longitude.into_iter().next(),
            google_rating: intermediate_rep.google_rating.into_iter().next(),
            review_count: intermediate_rep.review_count.into_iter().next(),
            industry: intermediate_rep.industry.into_iter().next(),
            employee_count: intermediate_rep.employee_count.into_iter().next(),
            estimated_revenue: intermediate_rep.estimated_revenue.into_iter().next(),
            org_id: intermediate_rep.org_id.into_iter().next(),
            tenant_id: intermediate_rep.tenant_id.into_iter().next(),
            created_at: intermediate_rep.created_at.into_iter().next(),
            updated_at: intermediate_rep.updated_at.into_iter().next(),
            deleted_at: intermediate_rep.deleted_at.into_iter().next(),
            job: intermediate_rep.job.into_iter().next(),
            workspace: intermediate_rep.workspace.into_iter().next(),
            place_id: intermediate_rep.place_id.into_iter().next(),
            google_maps_url: intermediate_rep.google_maps_url.into_iter().next(),
            business_status: intermediate_rep.business_status.into_iter().next(),
            regular_hours: intermediate_rep.regular_hours.into_iter().next(),
            special_hours: intermediate_rep.special_hours.into_iter().next(),
            photo_references: intermediate_rep.photo_references.into_iter().next(),
            main_photo_url: intermediate_rep.main_photo_url.into_iter().next(),
            reviews: intermediate_rep.reviews.into_iter().next(),
            types: intermediate_rep.types.into_iter().next(),
            amenities: intermediate_rep.amenities.into_iter().next(),
            serves_vegetarian_food: intermediate_rep.serves_vegetarian_food.into_iter().next(),
            outdoor_seating: intermediate_rep.outdoor_seating.into_iter().next(),
            payment_methods: intermediate_rep.payment_methods.into_iter().next(),
            wheelchair_accessible: intermediate_rep.wheelchair_accessible.into_iter().next(),
            parking_available: intermediate_rep.parking_available.into_iter().next(),
            social_media: intermediate_rep.social_media.into_iter().next(),
            rating_category: intermediate_rep.rating_category.into_iter().next(),
            rating: intermediate_rep.rating.into_iter().next(),
            count: intermediate_rep.count.into_iter().next(),
            last_updated: intermediate_rep.last_updated.into_iter().next(),
            data_source_version: intermediate_rep.data_source_version.into_iter().next(),
            scraping_session_id: intermediate_rep.scraping_session_id.into_iter().next(),
            alternate_phones: intermediate_rep.alternate_phones.into_iter().next(),
            contact_person_name: intermediate_rep.contact_person_name.into_iter().next(),
            contact_person_title: intermediate_rep.contact_person_title.into_iter().next(),
            contact_email: intermediate_rep.contact_email.into_iter().next(),
            founded_year: intermediate_rep.founded_year.into_iter().next(),
            business_type: intermediate_rep.business_type.into_iter().next(),
            certifications: intermediate_rep.certifications.into_iter().next(),
            license_number: intermediate_rep.license_number.into_iter().next(),
            revenue_range: intermediate_rep.revenue_range.into_iter().next(),
            funding_stage: intermediate_rep.funding_stage.into_iter().next(),
            is_public_company: intermediate_rep.is_public_company.into_iter().next(),
            website_load_speed: intermediate_rep.website_load_speed.into_iter().next(),
            has_ssl_certificate: intermediate_rep.has_ssl_certificate.into_iter().next(),
            cms_used: intermediate_rep.cms_used.into_iter().next(),
            ecommerce_platforms: intermediate_rep.ecommerce_platforms.into_iter().next(),
            timezone: intermediate_rep.timezone.into_iter().next(),
            neighborhood: intermediate_rep.neighborhood.into_iter().next(),
            nearby_landmarks: intermediate_rep.nearby_landmarks.into_iter().next(),
            transportation_access: intermediate_rep.transportation_access.into_iter().next(),
            employee_benefits: intermediate_rep.employee_benefits.into_iter().next(),
            parent_company: intermediate_rep.parent_company.into_iter().next(),
            subsidiaries: intermediate_rep.subsidiaries.into_iter().next(),
            is_franchise: intermediate_rep.is_franchise.into_iter().next(),
            seo_keywords: intermediate_rep.seo_keywords.into_iter().next(),
            uses_google_ads: intermediate_rep.uses_google_ads.into_iter().next(),
            google_my_business_category: intermediate_rep.google_my_business_category.into_iter().next(),
            naics_code: intermediate_rep.naics_code.into_iter().next(),
            sic_code: intermediate_rep.sic_code.into_iter().next(),
            unspsc_code: intermediate_rep.unspsc_code.into_iter().next(),
            is_green_certified: intermediate_rep.is_green_certified.into_iter().next(),
            energy_sources: intermediate_rep.energy_sources.into_iter().next(),
            sustainability_rating: intermediate_rep.sustainability_rating.into_iter().next(),
            recent_announcements: intermediate_rep.recent_announcements.into_iter().next(),
            last_product_launch: intermediate_rep.last_product_launch.into_iter().next(),
            has_litigation_history: intermediate_rep.has_litigation_history.into_iter().next(),
            export_control_status: intermediate_rep.export_control_status.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Lead> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Lead>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Lead>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Lead - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Lead> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Lead as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Lead - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LimitInfo {
    #[serde(rename = "current")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current: Option<i32>,

    #[serde(rename = "limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i32>,

    #[serde(rename = "resetTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reset_time: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "windowSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub window_size: Option<String>,

}


impl LimitInfo {
    #[allow(clippy::new_without_default)]
    pub fn new() -> LimitInfo {
        LimitInfo {
            current: None,
            limit: None,
            reset_time: None,
            window_size: None,
        }
    }
}

/// Converts the LimitInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for LimitInfo {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.current.as_ref().map(|current| {
                [
                    "current".to_string(),
                    current.to_string(),
                ].join(",")
            }),


            self.limit.as_ref().map(|limit| {
                [
                    "limit".to_string(),
                    limit.to_string(),
                ].join(",")
            }),

            // Skipping resetTime in query parameter serialization


            self.window_size.as_ref().map(|window_size| {
                [
                    "windowSize".to_string(),
                    window_size.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LimitInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LimitInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub current: Vec<i32>,
            pub limit: Vec<i32>,
            pub reset_time: Vec<chrono::DateTime::<chrono::Utc>>,
            pub window_size: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing LimitInfo".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "current" => intermediate_rep.current.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "limit" => intermediate_rep.limit.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "resetTime" => intermediate_rep.reset_time.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "windowSize" => intermediate_rep.window_size.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing LimitInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LimitInfo {
            current: intermediate_rep.current.into_iter().next(),
            limit: intermediate_rep.limit.into_iter().next(),
            reset_time: intermediate_rep.reset_time.into_iter().next(),
            window_size: intermediate_rep.window_size.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<LimitInfo> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<LimitInfo>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<LimitInfo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for LimitInfo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<LimitInfo> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <LimitInfo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into LimitInfo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ListAccountsResponse {
    #[serde(rename = "accounts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accounts: Option<Vec<models::Account>>,

    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_page_token: Option<String>,

}


impl ListAccountsResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ListAccountsResponse {
        ListAccountsResponse {
            accounts: None,
            next_page_token: None,
        }
    }
}

/// Converts the ListAccountsResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ListAccountsResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping accounts in query parameter serialization


            self.next_page_token.as_ref().map(|next_page_token| {
                [
                    "nextPageToken".to_string(),
                    next_page_token.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ListAccountsResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ListAccountsResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub accounts: Vec<Vec<models::Account>>,
            pub next_page_token: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ListAccountsResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "accounts" => return std::result::Result::Err("Parsing a container in this style is not supported in ListAccountsResponse".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "nextPageToken" => intermediate_rep.next_page_token.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ListAccountsResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ListAccountsResponse {
            accounts: intermediate_rep.accounts.into_iter().next(),
            next_page_token: intermediate_rep.next_page_token.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ListAccountsResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ListAccountsResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ListAccountsResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ListAccountsResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ListAccountsResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ListAccountsResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ListAccountsResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ListScrapingJobsResponse {
    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub jobs: Option<Vec<models::ScrapingJob>>,

}


impl ListScrapingJobsResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ListScrapingJobsResponse {
        ListScrapingJobsResponse {
            jobs: None,
        }
    }
}

/// Converts the ListScrapingJobsResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ListScrapingJobsResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping jobs in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ListScrapingJobsResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ListScrapingJobsResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub jobs: Vec<Vec<models::ScrapingJob>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ListScrapingJobsResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "jobs" => return std::result::Result::Err("Parsing a container in this style is not supported in ListScrapingJobsResponse".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ListScrapingJobsResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ListScrapingJobsResponse {
            jobs: intermediate_rep.jobs.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ListScrapingJobsResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ListScrapingJobsResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ListScrapingJobsResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ListScrapingJobsResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ListScrapingJobsResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ListScrapingJobsResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ListScrapingJobsResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ListWorkflowsResponse {
    #[serde(rename = "workflows")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workflows: Option<Vec<models::ScrapingWorkflow>>,

    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_page_token: Option<String>,

}


impl ListWorkflowsResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ListWorkflowsResponse {
        ListWorkflowsResponse {
            workflows: None,
            next_page_token: None,
        }
    }
}

/// Converts the ListWorkflowsResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ListWorkflowsResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping workflows in query parameter serialization


            self.next_page_token.as_ref().map(|next_page_token| {
                [
                    "nextPageToken".to_string(),
                    next_page_token.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ListWorkflowsResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ListWorkflowsResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub workflows: Vec<Vec<models::ScrapingWorkflow>>,
            pub next_page_token: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ListWorkflowsResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "workflows" => return std::result::Result::Err("Parsing a container in this style is not supported in ListWorkflowsResponse".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "nextPageToken" => intermediate_rep.next_page_token.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ListWorkflowsResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ListWorkflowsResponse {
            workflows: intermediate_rep.workflows.into_iter().next(),
            next_page_token: intermediate_rep.next_page_token.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ListWorkflowsResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ListWorkflowsResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ListWorkflowsResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ListWorkflowsResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ListWorkflowsResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ListWorkflowsResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ListWorkflowsResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ListWorkspacesResponse {
    #[serde(rename = "workspaces")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workspaces: Option<Vec<models::Workspace>>,

    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_page_token: Option<String>,

}


impl ListWorkspacesResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ListWorkspacesResponse {
        ListWorkspacesResponse {
            workspaces: None,
            next_page_token: None,
        }
    }
}

/// Converts the ListWorkspacesResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ListWorkspacesResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping workspaces in query parameter serialization


            self.next_page_token.as_ref().map(|next_page_token| {
                [
                    "nextPageToken".to_string(),
                    next_page_token.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ListWorkspacesResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ListWorkspacesResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub workspaces: Vec<Vec<models::Workspace>>,
            pub next_page_token: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ListWorkspacesResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "workspaces" => return std::result::Result::Err("Parsing a container in this style is not supported in ListWorkspacesResponse".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "nextPageToken" => intermediate_rep.next_page_token.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ListWorkspacesResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ListWorkspacesResponse {
            workspaces: intermediate_rep.workspaces.into_iter().next(),
            next_page_token: intermediate_rep.next_page_token.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ListWorkspacesResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ListWorkspacesResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ListWorkspacesResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ListWorkspacesResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ListWorkspacesResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ListWorkspacesResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ListWorkspacesResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Represents errors when HTTP method is not allowed on the endpoint
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MethodNotAllowedErrorMessageResponse {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<i32>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "allowedMethods")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_methods: Option<Vec<String>>,

    #[serde(rename = "errorResponse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_response: Option<models::ErrorResponse>,

}


impl MethodNotAllowedErrorMessageResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> MethodNotAllowedErrorMessageResponse {
        MethodNotAllowedErrorMessageResponse {
            code: None,
            message: None,
            allowed_methods: None,
            error_response: None,
        }
    }
}

/// Converts the MethodNotAllowedErrorMessageResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MethodNotAllowedErrorMessageResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.code.as_ref().map(|code| {
                [
                    "code".to_string(),
                    code.to_string(),
                ].join(",")
            }),


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),


            self.allowed_methods.as_ref().map(|allowed_methods| {
                [
                    "allowedMethods".to_string(),
                    allowed_methods.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

            // Skipping errorResponse in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MethodNotAllowedErrorMessageResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MethodNotAllowedErrorMessageResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<i32>,
            pub message: Vec<String>,
            pub allowed_methods: Vec<Vec<String>>,
            pub error_response: Vec<models::ErrorResponse>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MethodNotAllowedErrorMessageResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "allowedMethods" => return std::result::Result::Err("Parsing a container in this style is not supported in MethodNotAllowedErrorMessageResponse".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "errorResponse" => intermediate_rep.error_response.push(<models::ErrorResponse as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MethodNotAllowedErrorMessageResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MethodNotAllowedErrorMessageResponse {
            code: intermediate_rep.code.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
            allowed_methods: intermediate_rep.allowed_methods.into_iter().next(),
            error_response: intermediate_rep.error_response.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MethodNotAllowedErrorMessageResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MethodNotAllowedErrorMessageResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MethodNotAllowedErrorMessageResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MethodNotAllowedErrorMessageResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MethodNotAllowedErrorMessageResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MethodNotAllowedErrorMessageResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MethodNotAllowedErrorMessageResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MfaInfo {
    #[serde(rename = "mfaRequired")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mfa_required: Option<bool>,

    #[serde(rename = "allowedMethods")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_methods: Option<Vec<String>>,

    #[serde(rename = "failedReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failed_reason: Option<String>,

}


impl MfaInfo {
    #[allow(clippy::new_without_default)]
    pub fn new() -> MfaInfo {
        MfaInfo {
            mfa_required: None,
            allowed_methods: None,
            failed_reason: None,
        }
    }
}

/// Converts the MfaInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MfaInfo {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.mfa_required.as_ref().map(|mfa_required| {
                [
                    "mfaRequired".to_string(),
                    mfa_required.to_string(),
                ].join(",")
            }),


            self.allowed_methods.as_ref().map(|allowed_methods| {
                [
                    "allowedMethods".to_string(),
                    allowed_methods.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.failed_reason.as_ref().map(|failed_reason| {
                [
                    "failedReason".to_string(),
                    failed_reason.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MfaInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MfaInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub mfa_required: Vec<bool>,
            pub allowed_methods: Vec<Vec<String>>,
            pub failed_reason: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MfaInfo".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "mfaRequired" => intermediate_rep.mfa_required.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "allowedMethods" => return std::result::Result::Err("Parsing a container in this style is not supported in MfaInfo".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "failedReason" => intermediate_rep.failed_reason.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MfaInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MfaInfo {
            mfa_required: intermediate_rep.mfa_required.into_iter().next(),
            allowed_methods: intermediate_rep.allowed_methods.into_iter().next(),
            failed_reason: intermediate_rep.failed_reason.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MfaInfo> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MfaInfo>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MfaInfo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MfaInfo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MfaInfo> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MfaInfo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MfaInfo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// - NO_NOT_FOUND_ERROR: Default value as per proto3 convention  - UNDEFINED_ENDPOINT: Endpoint errors  Requested endpoint doesn't exist  - UNIMPLEMENTED: Endpoint not implemented  - STORE_ID_NOT_FOUND: Resource errors  Requested store ID doesn't exist  - USER_NOT_FOUND: Requested user doesn't exist  - RESOURCE_NOT_FOUND: Generic resource not found  - TENANT_NOT_FOUND: Requested tenant doesn't exist
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum NotFoundErrorCode {
    #[serde(rename = "NO_NOT_FOUND_ERROR")]
    NoNotFoundError,
    #[serde(rename = "UNDEFINED_ENDPOINT")]
    UndefinedEndpoint,
    #[serde(rename = "UNIMPLEMENTED")]
    Unimplemented,
    #[serde(rename = "STORE_ID_NOT_FOUND")]
    StoreIdNotFound,
    #[serde(rename = "USER_NOT_FOUND")]
    UserNotFound,
    #[serde(rename = "RESOURCE_NOT_FOUND")]
    ResourceNotFound,
    #[serde(rename = "TENANT_NOT_FOUND")]
    TenantNotFound,
}

impl std::fmt::Display for NotFoundErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            NotFoundErrorCode::NoNotFoundError => write!(f, "NO_NOT_FOUND_ERROR"),
            NotFoundErrorCode::UndefinedEndpoint => write!(f, "UNDEFINED_ENDPOINT"),
            NotFoundErrorCode::Unimplemented => write!(f, "UNIMPLEMENTED"),
            NotFoundErrorCode::StoreIdNotFound => write!(f, "STORE_ID_NOT_FOUND"),
            NotFoundErrorCode::UserNotFound => write!(f, "USER_NOT_FOUND"),
            NotFoundErrorCode::ResourceNotFound => write!(f, "RESOURCE_NOT_FOUND"),
            NotFoundErrorCode::TenantNotFound => write!(f, "TENANT_NOT_FOUND"),
        }
    }
}

impl std::str::FromStr for NotFoundErrorCode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "NO_NOT_FOUND_ERROR" => std::result::Result::Ok(NotFoundErrorCode::NoNotFoundError),
            "UNDEFINED_ENDPOINT" => std::result::Result::Ok(NotFoundErrorCode::UndefinedEndpoint),
            "UNIMPLEMENTED" => std::result::Result::Ok(NotFoundErrorCode::Unimplemented),
            "STORE_ID_NOT_FOUND" => std::result::Result::Ok(NotFoundErrorCode::StoreIdNotFound),
            "USER_NOT_FOUND" => std::result::Result::Ok(NotFoundErrorCode::UserNotFound),
            "RESOURCE_NOT_FOUND" => std::result::Result::Ok(NotFoundErrorCode::ResourceNotFound),
            "TENANT_NOT_FOUND" => std::result::Result::Ok(NotFoundErrorCode::TenantNotFound),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// Represents resource not found errors
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NotFoundErrorMessageResponse {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<models::NotFoundErrorCode>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "resourceInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_info: Option<models::ResourceInfo>,

    #[serde(rename = "suggestions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub suggestions: Option<models::Suggestions>,

    #[serde(rename = "apiInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub api_info: Option<models::ApiInfo>,

    #[serde(rename = "errorResponse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_response: Option<models::ErrorResponse>,

}


impl NotFoundErrorMessageResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> NotFoundErrorMessageResponse {
        NotFoundErrorMessageResponse {
            code: None,
            message: None,
            resource_info: None,
            suggestions: None,
            api_info: None,
            error_response: None,
        }
    }
}

/// Converts the NotFoundErrorMessageResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for NotFoundErrorMessageResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping code in query parameter serialization


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),

            // Skipping resourceInfo in query parameter serialization

            // Skipping suggestions in query parameter serialization

            // Skipping apiInfo in query parameter serialization

            // Skipping errorResponse in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NotFoundErrorMessageResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NotFoundErrorMessageResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<models::NotFoundErrorCode>,
            pub message: Vec<String>,
            pub resource_info: Vec<models::ResourceInfo>,
            pub suggestions: Vec<models::Suggestions>,
            pub api_info: Vec<models::ApiInfo>,
            pub error_response: Vec<models::ErrorResponse>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NotFoundErrorMessageResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(<models::NotFoundErrorCode as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "resourceInfo" => intermediate_rep.resource_info.push(<models::ResourceInfo as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "suggestions" => intermediate_rep.suggestions.push(<models::Suggestions as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "apiInfo" => intermediate_rep.api_info.push(<models::ApiInfo as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "errorResponse" => intermediate_rep.error_response.push(<models::ErrorResponse as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NotFoundErrorMessageResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NotFoundErrorMessageResponse {
            code: intermediate_rep.code.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
            resource_info: intermediate_rep.resource_info.into_iter().next(),
            suggestions: intermediate_rep.suggestions.into_iter().next(),
            api_info: intermediate_rep.api_info.into_iter().next(),
            error_response: intermediate_rep.error_response.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NotFoundErrorMessageResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<NotFoundErrorMessageResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NotFoundErrorMessageResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for NotFoundErrorMessageResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<NotFoundErrorMessageResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NotFoundErrorMessageResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into NotFoundErrorMessageResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Represents errors when the requested functionality is not implemented
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NotImplementedErrorMessageResponse {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<i32>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "feature")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub feature: Option<String>,

    #[serde(rename = "errorResponse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_response: Option<models::ErrorResponse>,

}


impl NotImplementedErrorMessageResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> NotImplementedErrorMessageResponse {
        NotImplementedErrorMessageResponse {
            code: None,
            message: None,
            feature: None,
            error_response: None,
        }
    }
}

/// Converts the NotImplementedErrorMessageResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for NotImplementedErrorMessageResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.code.as_ref().map(|code| {
                [
                    "code".to_string(),
                    code.to_string(),
                ].join(",")
            }),


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),


            self.feature.as_ref().map(|feature| {
                [
                    "feature".to_string(),
                    feature.to_string(),
                ].join(",")
            }),

            // Skipping errorResponse in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NotImplementedErrorMessageResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NotImplementedErrorMessageResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<i32>,
            pub message: Vec<String>,
            pub feature: Vec<String>,
            pub error_response: Vec<models::ErrorResponse>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NotImplementedErrorMessageResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "feature" => intermediate_rep.feature.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "errorResponse" => intermediate_rep.error_response.push(<models::ErrorResponse as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NotImplementedErrorMessageResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NotImplementedErrorMessageResponse {
            code: intermediate_rep.code.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
            feature: intermediate_rep.feature.into_iter().next(),
            error_response: intermediate_rep.error_response.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NotImplementedErrorMessageResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<NotImplementedErrorMessageResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NotImplementedErrorMessageResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for NotImplementedErrorMessageResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<NotImplementedErrorMessageResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NotImplementedErrorMessageResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into NotImplementedErrorMessageResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct OperationDetails {
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub operation_id: Option<String>,

    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_time: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_time: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "stackTrace")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_trace: Option<Vec<String>>,

    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,

}


impl OperationDetails {
    #[allow(clippy::new_without_default)]
    pub fn new() -> OperationDetails {
        OperationDetails {
            operation_id: None,
            start_time: None,
            end_time: None,
            stack_trace: None,
            parameters: None,
        }
    }
}

/// Converts the OperationDetails value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for OperationDetails {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.operation_id.as_ref().map(|operation_id| {
                [
                    "operationId".to_string(),
                    operation_id.to_string(),
                ].join(",")
            }),

            // Skipping startTime in query parameter serialization

            // Skipping endTime in query parameter serialization


            self.stack_trace.as_ref().map(|stack_trace| {
                [
                    "stackTrace".to_string(),
                    stack_trace.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

            // Skipping parameters in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a OperationDetails value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for OperationDetails {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub operation_id: Vec<String>,
            pub start_time: Vec<chrono::DateTime::<chrono::Utc>>,
            pub end_time: Vec<chrono::DateTime::<chrono::Utc>>,
            pub stack_trace: Vec<Vec<String>>,
            pub parameters: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing OperationDetails".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "operationId" => intermediate_rep.operation_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "startTime" => intermediate_rep.start_time.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "endTime" => intermediate_rep.end_time.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "stackTrace" => return std::result::Result::Err("Parsing a container in this style is not supported in OperationDetails".to_string()),
                    "parameters" => return std::result::Result::Err("Parsing a container in this style is not supported in OperationDetails".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing OperationDetails".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(OperationDetails {
            operation_id: intermediate_rep.operation_id.into_iter().next(),
            start_time: intermediate_rep.start_time.into_iter().next(),
            end_time: intermediate_rep.end_time.into_iter().next(),
            stack_trace: intermediate_rep.stack_trace.into_iter().next(),
            parameters: intermediate_rep.parameters.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<OperationDetails> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<OperationDetails>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<OperationDetails>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for OperationDetails - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<OperationDetails> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <OperationDetails as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into OperationDetails - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum OutputFormat {
    #[serde(rename = "OUTPUT_FORMAT_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "OUTPUT_FORMAT_JSON")]
    Json,
    #[serde(rename = "OUTPUT_FORMAT_CSV")]
    Csv,
    #[serde(rename = "OUTPUT_FORMAT_BIGQUERY")]
    Bigquery,
    #[serde(rename = "OUTPUT_FORMAT_POSTGRES")]
    Postgres,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OutputFormat::Unspecified => write!(f, "OUTPUT_FORMAT_UNSPECIFIED"),
            OutputFormat::Json => write!(f, "OUTPUT_FORMAT_JSON"),
            OutputFormat::Csv => write!(f, "OUTPUT_FORMAT_CSV"),
            OutputFormat::Bigquery => write!(f, "OUTPUT_FORMAT_BIGQUERY"),
            OutputFormat::Postgres => write!(f, "OUTPUT_FORMAT_POSTGRES"),
        }
    }
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "OUTPUT_FORMAT_UNSPECIFIED" => std::result::Result::Ok(OutputFormat::Unspecified),
            "OUTPUT_FORMAT_JSON" => std::result::Result::Ok(OutputFormat::Json),
            "OUTPUT_FORMAT_CSV" => std::result::Result::Ok(OutputFormat::Csv),
            "OUTPUT_FORMAT_BIGQUERY" => std::result::Result::Ok(OutputFormat::Bigquery),
            "OUTPUT_FORMAT_POSTGRES" => std::result::Result::Ok(OutputFormat::Postgres),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PauseWorkflowBody {
    #[serde(rename = "pause")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pause: Option<bool>,

}


impl PauseWorkflowBody {
    #[allow(clippy::new_without_default)]
    pub fn new() -> PauseWorkflowBody {
        PauseWorkflowBody {
            pause: None,
        }
    }
}

/// Converts the PauseWorkflowBody value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PauseWorkflowBody {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.pause.as_ref().map(|pause| {
                [
                    "pause".to_string(),
                    pause.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PauseWorkflowBody value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PauseWorkflowBody {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub pause: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PauseWorkflowBody".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "pause" => intermediate_rep.pause.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PauseWorkflowBody".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PauseWorkflowBody {
            pause: intermediate_rep.pause.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PauseWorkflowBody> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PauseWorkflowBody>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PauseWorkflowBody>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PauseWorkflowBody - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PauseWorkflowBody> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PauseWorkflowBody as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PauseWorkflowBody - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PauseWorkflowResponse {
    #[serde(rename = "success")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub success: Option<bool>,

    #[serde(rename = "workflow")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workflow: Option<models::ScrapingWorkflow>,

}


impl PauseWorkflowResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> PauseWorkflowResponse {
        PauseWorkflowResponse {
            success: None,
            workflow: None,
        }
    }
}

/// Converts the PauseWorkflowResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PauseWorkflowResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.success.as_ref().map(|success| {
                [
                    "success".to_string(),
                    success.to_string(),
                ].join(",")
            }),

            // Skipping workflow in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PauseWorkflowResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PauseWorkflowResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub success: Vec<bool>,
            pub workflow: Vec<models::ScrapingWorkflow>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PauseWorkflowResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "success" => intermediate_rep.success.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "workflow" => intermediate_rep.workflow.push(<models::ScrapingWorkflow as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PauseWorkflowResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PauseWorkflowResponse {
            success: intermediate_rep.success.into_iter().next(),
            workflow: intermediate_rep.workflow.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PauseWorkflowResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PauseWorkflowResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PauseWorkflowResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PauseWorkflowResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PauseWorkflowResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PauseWorkflowResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PauseWorkflowResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PaymentInfo {
    #[serde(rename = "amountDue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub amount_due: Option<f64>,

    #[serde(rename = "currency")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub currency: Option<String>,

    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub due_date: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "paymentUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub payment_url: Option<String>,

}


impl PaymentInfo {
    #[allow(clippy::new_without_default)]
    pub fn new() -> PaymentInfo {
        PaymentInfo {
            amount_due: None,
            currency: None,
            due_date: None,
            payment_url: None,
        }
    }
}

/// Converts the PaymentInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PaymentInfo {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.amount_due.as_ref().map(|amount_due| {
                [
                    "amountDue".to_string(),
                    amount_due.to_string(),
                ].join(",")
            }),


            self.currency.as_ref().map(|currency| {
                [
                    "currency".to_string(),
                    currency.to_string(),
                ].join(",")
            }),

            // Skipping dueDate in query parameter serialization


            self.payment_url.as_ref().map(|payment_url| {
                [
                    "paymentUrl".to_string(),
                    payment_url.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PaymentInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PaymentInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub amount_due: Vec<f64>,
            pub currency: Vec<String>,
            pub due_date: Vec<chrono::DateTime::<chrono::Utc>>,
            pub payment_url: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PaymentInfo".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "amountDue" => intermediate_rep.amount_due.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "currency" => intermediate_rep.currency.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "dueDate" => intermediate_rep.due_date.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "paymentUrl" => intermediate_rep.payment_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PaymentInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PaymentInfo {
            amount_due: intermediate_rep.amount_due.into_iter().next(),
            currency: intermediate_rep.currency.into_iter().next(),
            due_date: intermediate_rep.due_date.into_iter().next(),
            payment_url: intermediate_rep.payment_url.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PaymentInfo> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PaymentInfo>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PaymentInfo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PaymentInfo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PaymentInfo> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PaymentInfo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PaymentInfo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Indicates that payment is required to proceed
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PaymentRequiredErrorMessageResponse {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<i32>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "paymentInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub payment_info: Option<models::PaymentInfo>,

    #[serde(rename = "errorResponse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_response: Option<models::ErrorResponse>,

}


impl PaymentRequiredErrorMessageResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> PaymentRequiredErrorMessageResponse {
        PaymentRequiredErrorMessageResponse {
            code: None,
            message: None,
            payment_info: None,
            error_response: None,
        }
    }
}

/// Converts the PaymentRequiredErrorMessageResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PaymentRequiredErrorMessageResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.code.as_ref().map(|code| {
                [
                    "code".to_string(),
                    code.to_string(),
                ].join(",")
            }),


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),

            // Skipping paymentInfo in query parameter serialization

            // Skipping errorResponse in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PaymentRequiredErrorMessageResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PaymentRequiredErrorMessageResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<i32>,
            pub message: Vec<String>,
            pub payment_info: Vec<models::PaymentInfo>,
            pub error_response: Vec<models::ErrorResponse>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PaymentRequiredErrorMessageResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "paymentInfo" => intermediate_rep.payment_info.push(<models::PaymentInfo as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "errorResponse" => intermediate_rep.error_response.push(<models::ErrorResponse as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PaymentRequiredErrorMessageResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PaymentRequiredErrorMessageResponse {
            code: intermediate_rep.code.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
            payment_info: intermediate_rep.payment_info.into_iter().next(),
            error_response: intermediate_rep.error_response.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PaymentRequiredErrorMessageResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PaymentRequiredErrorMessageResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PaymentRequiredErrorMessageResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PaymentRequiredErrorMessageResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PaymentRequiredErrorMessageResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PaymentRequiredErrorMessageResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PaymentRequiredErrorMessageResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Represents errors when a precondition provided in the request fails
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PreconditionFailedErrorMessageResponse {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<i32>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "failedConditions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failed_conditions: Option<Vec<String>>,

    #[serde(rename = "errorResponse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_response: Option<models::ErrorResponse>,

}


impl PreconditionFailedErrorMessageResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> PreconditionFailedErrorMessageResponse {
        PreconditionFailedErrorMessageResponse {
            code: None,
            message: None,
            failed_conditions: None,
            error_response: None,
        }
    }
}

/// Converts the PreconditionFailedErrorMessageResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PreconditionFailedErrorMessageResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.code.as_ref().map(|code| {
                [
                    "code".to_string(),
                    code.to_string(),
                ].join(",")
            }),


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),


            self.failed_conditions.as_ref().map(|failed_conditions| {
                [
                    "failedConditions".to_string(),
                    failed_conditions.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

            // Skipping errorResponse in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PreconditionFailedErrorMessageResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PreconditionFailedErrorMessageResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<i32>,
            pub message: Vec<String>,
            pub failed_conditions: Vec<Vec<String>>,
            pub error_response: Vec<models::ErrorResponse>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PreconditionFailedErrorMessageResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "failedConditions" => return std::result::Result::Err("Parsing a container in this style is not supported in PreconditionFailedErrorMessageResponse".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "errorResponse" => intermediate_rep.error_response.push(<models::ErrorResponse as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PreconditionFailedErrorMessageResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PreconditionFailedErrorMessageResponse {
            code: intermediate_rep.code.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
            failed_conditions: intermediate_rep.failed_conditions.into_iter().next(),
            error_response: intermediate_rep.error_response.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PreconditionFailedErrorMessageResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PreconditionFailedErrorMessageResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PreconditionFailedErrorMessageResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PreconditionFailedErrorMessageResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PreconditionFailedErrorMessageResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PreconditionFailedErrorMessageResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PreconditionFailedErrorMessageResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct QuotaInfo {
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource: Option<String>,

    #[serde(rename = "used")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub used: Option<String>,

    #[serde(rename = "limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<String>,

    #[serde(rename = "resetTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reset_time: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "resourceUsage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_usage: Option<std::collections::HashMap<String, String>>,

}


impl QuotaInfo {
    #[allow(clippy::new_without_default)]
    pub fn new() -> QuotaInfo {
        QuotaInfo {
            resource: None,
            used: None,
            limit: None,
            reset_time: None,
            resource_usage: None,
        }
    }
}

/// Converts the QuotaInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for QuotaInfo {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.resource.as_ref().map(|resource| {
                [
                    "resource".to_string(),
                    resource.to_string(),
                ].join(",")
            }),


            self.used.as_ref().map(|used| {
                [
                    "used".to_string(),
                    used.to_string(),
                ].join(",")
            }),


            self.limit.as_ref().map(|limit| {
                [
                    "limit".to_string(),
                    limit.to_string(),
                ].join(",")
            }),

            // Skipping resetTime in query parameter serialization

            // Skipping resourceUsage in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a QuotaInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for QuotaInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub resource: Vec<String>,
            pub used: Vec<String>,
            pub limit: Vec<String>,
            pub reset_time: Vec<chrono::DateTime::<chrono::Utc>>,
            pub resource_usage: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing QuotaInfo".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "resource" => intermediate_rep.resource.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "used" => intermediate_rep.used.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "limit" => intermediate_rep.limit.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "resetTime" => intermediate_rep.reset_time.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "resourceUsage" => return std::result::Result::Err("Parsing a container in this style is not supported in QuotaInfo".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing QuotaInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(QuotaInfo {
            resource: intermediate_rep.resource.into_iter().next(),
            used: intermediate_rep.used.into_iter().next(),
            limit: intermediate_rep.limit.into_iter().next(),
            reset_time: intermediate_rep.reset_time.into_iter().next(),
            resource_usage: intermediate_rep.resource_usage.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<QuotaInfo> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<QuotaInfo>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<QuotaInfo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for QuotaInfo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<QuotaInfo> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <QuotaInfo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into QuotaInfo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RateLimitContext {
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_id: Option<String>,

    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ip_address: Option<String>,

    #[serde(rename = "affectedEndpoints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub affected_endpoints: Option<Vec<String>>,

    #[serde(rename = "headers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,

}


impl RateLimitContext {
    #[allow(clippy::new_without_default)]
    pub fn new() -> RateLimitContext {
        RateLimitContext {
            user_id: None,
            ip_address: None,
            affected_endpoints: None,
            headers: None,
        }
    }
}

/// Converts the RateLimitContext value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RateLimitContext {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.user_id.as_ref().map(|user_id| {
                [
                    "userId".to_string(),
                    user_id.to_string(),
                ].join(",")
            }),


            self.ip_address.as_ref().map(|ip_address| {
                [
                    "ipAddress".to_string(),
                    ip_address.to_string(),
                ].join(",")
            }),


            self.affected_endpoints.as_ref().map(|affected_endpoints| {
                [
                    "affectedEndpoints".to_string(),
                    affected_endpoints.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

            // Skipping headers in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RateLimitContext value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RateLimitContext {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub user_id: Vec<String>,
            pub ip_address: Vec<String>,
            pub affected_endpoints: Vec<Vec<String>>,
            pub headers: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RateLimitContext".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "userId" => intermediate_rep.user_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ipAddress" => intermediate_rep.ip_address.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "affectedEndpoints" => return std::result::Result::Err("Parsing a container in this style is not supported in RateLimitContext".to_string()),
                    "headers" => return std::result::Result::Err("Parsing a container in this style is not supported in RateLimitContext".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing RateLimitContext".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RateLimitContext {
            user_id: intermediate_rep.user_id.into_iter().next(),
            ip_address: intermediate_rep.ip_address.into_iter().next(),
            affected_endpoints: intermediate_rep.affected_endpoints.into_iter().next(),
            headers: intermediate_rep.headers.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RateLimitContext> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RateLimitContext>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RateLimitContext>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RateLimitContext - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<RateLimitContext> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RateLimitContext as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RateLimitContext - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Represents rate limiting and quota exceeded errors
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RateLimitErrorMessageResponse {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<models::InternalErrorCode>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "limitInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit_info: Option<models::LimitInfo>,

    #[serde(rename = "quotaInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub quota_info: Option<models::QuotaInfo>,

    #[serde(rename = "context")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub context: Option<models::RateLimitContext>,

    #[serde(rename = "errorResponse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_response: Option<models::ErrorResponse>,

}


impl RateLimitErrorMessageResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> RateLimitErrorMessageResponse {
        RateLimitErrorMessageResponse {
            code: None,
            message: None,
            limit_info: None,
            quota_info: None,
            context: None,
            error_response: None,
        }
    }
}

/// Converts the RateLimitErrorMessageResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RateLimitErrorMessageResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping code in query parameter serialization


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),

            // Skipping limitInfo in query parameter serialization

            // Skipping quotaInfo in query parameter serialization

            // Skipping context in query parameter serialization

            // Skipping errorResponse in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RateLimitErrorMessageResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RateLimitErrorMessageResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<models::InternalErrorCode>,
            pub message: Vec<String>,
            pub limit_info: Vec<models::LimitInfo>,
            pub quota_info: Vec<models::QuotaInfo>,
            pub context: Vec<models::RateLimitContext>,
            pub error_response: Vec<models::ErrorResponse>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RateLimitErrorMessageResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(<models::InternalErrorCode as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "limitInfo" => intermediate_rep.limit_info.push(<models::LimitInfo as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "quotaInfo" => intermediate_rep.quota_info.push(<models::QuotaInfo as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "context" => intermediate_rep.context.push(<models::RateLimitContext as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "errorResponse" => intermediate_rep.error_response.push(<models::ErrorResponse as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing RateLimitErrorMessageResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RateLimitErrorMessageResponse {
            code: intermediate_rep.code.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
            limit_info: intermediate_rep.limit_info.into_iter().next(),
            quota_info: intermediate_rep.quota_info.into_iter().next(),
            context: intermediate_rep.context.into_iter().next(),
            error_response: intermediate_rep.error_response.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RateLimitErrorMessageResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RateLimitErrorMessageResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RateLimitErrorMessageResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RateLimitErrorMessageResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<RateLimitErrorMessageResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RateLimitErrorMessageResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RateLimitErrorMessageResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ResourceInfo {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub r#type: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "tenantId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tenant_id: Option<String>,

    #[serde(rename = "scopes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scopes: Option<Vec<String>>,

}


impl ResourceInfo {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ResourceInfo {
        ResourceInfo {
            r#type: None,
            id: None,
            path: None,
            tenant_id: None,
            scopes: None,
        }
    }
}

/// Converts the ResourceInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ResourceInfo {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.r#type.as_ref().map(|r#type| {
                [
                    "type".to_string(),
                    r#type.to_string(),
                ].join(",")
            }),


            self.id.as_ref().map(|id| {
                [
                    "id".to_string(),
                    id.to_string(),
                ].join(",")
            }),


            self.path.as_ref().map(|path| {
                [
                    "path".to_string(),
                    path.to_string(),
                ].join(",")
            }),


            self.tenant_id.as_ref().map(|tenant_id| {
                [
                    "tenantId".to_string(),
                    tenant_id.to_string(),
                ].join(",")
            }),


            self.scopes.as_ref().map(|scopes| {
                [
                    "scopes".to_string(),
                    scopes.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ResourceInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ResourceInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub id: Vec<String>,
            pub path: Vec<String>,
            pub tenant_id: Vec<String>,
            pub scopes: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ResourceInfo".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "path" => intermediate_rep.path.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "tenantId" => intermediate_rep.tenant_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "scopes" => return std::result::Result::Err("Parsing a container in this style is not supported in ResourceInfo".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ResourceInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ResourceInfo {
            r#type: intermediate_rep.r#type.into_iter().next(),
            id: intermediate_rep.id.into_iter().next(),
            path: intermediate_rep.path.into_iter().next(),
            tenant_id: intermediate_rep.tenant_id.into_iter().next(),
            scopes: intermediate_rep.scopes.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ResourceInfo> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ResourceInfo>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ResourceInfo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ResourceInfo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ResourceInfo> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ResourceInfo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ResourceInfo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ResourceUtilization {
    #[serde(rename = "cpuUsage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cpu_usage: Option<f64>,

    #[serde(rename = "memoryUsage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub memory_usage: Option<f64>,

    #[serde(rename = "activeConnections")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active_connections: Option<i32>,

    #[serde(rename = "quotas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub quotas: Option<std::collections::HashMap<String, f64>>,

}


impl ResourceUtilization {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ResourceUtilization {
        ResourceUtilization {
            cpu_usage: None,
            memory_usage: None,
            active_connections: None,
            quotas: None,
        }
    }
}

/// Converts the ResourceUtilization value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ResourceUtilization {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.cpu_usage.as_ref().map(|cpu_usage| {
                [
                    "cpuUsage".to_string(),
                    cpu_usage.to_string(),
                ].join(",")
            }),


            self.memory_usage.as_ref().map(|memory_usage| {
                [
                    "memoryUsage".to_string(),
                    memory_usage.to_string(),
                ].join(",")
            }),


            self.active_connections.as_ref().map(|active_connections| {
                [
                    "activeConnections".to_string(),
                    active_connections.to_string(),
                ].join(",")
            }),

            // Skipping quotas in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ResourceUtilization value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ResourceUtilization {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub cpu_usage: Vec<f64>,
            pub memory_usage: Vec<f64>,
            pub active_connections: Vec<i32>,
            pub quotas: Vec<std::collections::HashMap<String, f64>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ResourceUtilization".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cpuUsage" => intermediate_rep.cpu_usage.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "memoryUsage" => intermediate_rep.memory_usage.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "activeConnections" => intermediate_rep.active_connections.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "quotas" => return std::result::Result::Err("Parsing a container in this style is not supported in ResourceUtilization".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ResourceUtilization".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ResourceUtilization {
            cpu_usage: intermediate_rep.cpu_usage.into_iter().next(),
            memory_usage: intermediate_rep.memory_usage.into_iter().next(),
            active_connections: intermediate_rep.active_connections.into_iter().next(),
            quotas: intermediate_rep.quotas.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ResourceUtilization> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ResourceUtilization>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ResourceUtilization>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ResourceUtilization - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ResourceUtilization> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ResourceUtilization as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ResourceUtilization - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ResourceValidation {
    #[serde(rename = "missingResources")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub missing_resources: Option<Vec<String>>,

    #[serde(rename = "invalidReferences")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub invalid_references: Option<Vec<String>>,

    #[serde(rename = "idViolations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id_violations: Option<std::collections::HashMap<String, String>>,

}


impl ResourceValidation {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ResourceValidation {
        ResourceValidation {
            missing_resources: None,
            invalid_references: None,
            id_violations: None,
        }
    }
}

/// Converts the ResourceValidation value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ResourceValidation {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.missing_resources.as_ref().map(|missing_resources| {
                [
                    "missingResources".to_string(),
                    missing_resources.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.invalid_references.as_ref().map(|invalid_references| {
                [
                    "invalidReferences".to_string(),
                    invalid_references.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

            // Skipping idViolations in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ResourceValidation value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ResourceValidation {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub missing_resources: Vec<Vec<String>>,
            pub invalid_references: Vec<Vec<String>>,
            pub id_violations: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ResourceValidation".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "missingResources" => return std::result::Result::Err("Parsing a container in this style is not supported in ResourceValidation".to_string()),
                    "invalidReferences" => return std::result::Result::Err("Parsing a container in this style is not supported in ResourceValidation".to_string()),
                    "idViolations" => return std::result::Result::Err("Parsing a container in this style is not supported in ResourceValidation".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ResourceValidation".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ResourceValidation {
            missing_resources: intermediate_rep.missing_resources.into_iter().next(),
            invalid_references: intermediate_rep.invalid_references.into_iter().next(),
            id_violations: intermediate_rep.id_violations.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ResourceValidation> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ResourceValidation>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ResourceValidation>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ResourceValidation - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ResourceValidation> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ResourceValidation as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ResourceValidation - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum RevenueRange {
    #[serde(rename = "REVENUE_RANGE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "REVENUE_RANGE_UNDER_100K")]
    Under100K,
    #[serde(rename = "REVENUE_RANGE_100K_TO_1M")]
    Variant100Kto1M,
    #[serde(rename = "REVENUE_RANGE_1M_TO_10M")]
    Variant1Mto10M,
    #[serde(rename = "REVENUE_RANGE_10M_TO_50M")]
    Variant10Mto50M,
    #[serde(rename = "REVENUE_RANGE_OVER_50M")]
    Over50M,
}

impl std::fmt::Display for RevenueRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RevenueRange::Unspecified => write!(f, "REVENUE_RANGE_UNSPECIFIED"),
            RevenueRange::Under100K => write!(f, "REVENUE_RANGE_UNDER_100K"),
            RevenueRange::Variant100Kto1M => write!(f, "REVENUE_RANGE_100K_TO_1M"),
            RevenueRange::Variant1Mto10M => write!(f, "REVENUE_RANGE_1M_TO_10M"),
            RevenueRange::Variant10Mto50M => write!(f, "REVENUE_RANGE_10M_TO_50M"),
            RevenueRange::Over50M => write!(f, "REVENUE_RANGE_OVER_50M"),
        }
    }
}

impl std::str::FromStr for RevenueRange {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "REVENUE_RANGE_UNSPECIFIED" => std::result::Result::Ok(RevenueRange::Unspecified),
            "REVENUE_RANGE_UNDER_100K" => std::result::Result::Ok(RevenueRange::Under100K),
            "REVENUE_RANGE_100K_TO_1M" => std::result::Result::Ok(RevenueRange::Variant100Kto1M),
            "REVENUE_RANGE_1M_TO_10M" => std::result::Result::Ok(RevenueRange::Variant1Mto10M),
            "REVENUE_RANGE_10M_TO_50M" => std::result::Result::Ok(RevenueRange::Variant10Mto50M),
            "REVENUE_RANGE_OVER_50M" => std::result::Result::Ok(RevenueRange::Over50M),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Review {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "author")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author: Option<String>,

    #[serde(rename = "rating")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rating: Option<f32>,

    #[serde(rename = "text")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub text: Option<String>,

    #[serde(rename = "time")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub time: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "language")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub language: Option<String>,

    #[serde(rename = "profilePhotoUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub profile_photo_url: Option<String>,

    #[serde(rename = "reviewCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub review_count: Option<i32>,

    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_at: Option<chrono::DateTime::<chrono::Utc>>,

}


impl Review {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Review {
        Review {
            id: None,
            author: None,
            rating: None,
            text: None,
            time: None,
            language: None,
            profile_photo_url: None,
            review_count: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }
}

/// Converts the Review value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Review {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.id.as_ref().map(|id| {
                [
                    "id".to_string(),
                    id.to_string(),
                ].join(",")
            }),


            self.author.as_ref().map(|author| {
                [
                    "author".to_string(),
                    author.to_string(),
                ].join(",")
            }),


            self.rating.as_ref().map(|rating| {
                [
                    "rating".to_string(),
                    rating.to_string(),
                ].join(",")
            }),


            self.text.as_ref().map(|text| {
                [
                    "text".to_string(),
                    text.to_string(),
                ].join(",")
            }),

            // Skipping time in query parameter serialization


            self.language.as_ref().map(|language| {
                [
                    "language".to_string(),
                    language.to_string(),
                ].join(",")
            }),


            self.profile_photo_url.as_ref().map(|profile_photo_url| {
                [
                    "profilePhotoUrl".to_string(),
                    profile_photo_url.to_string(),
                ].join(",")
            }),


            self.review_count.as_ref().map(|review_count| {
                [
                    "reviewCount".to_string(),
                    review_count.to_string(),
                ].join(",")
            }),

            // Skipping createdAt in query parameter serialization

            // Skipping updatedAt in query parameter serialization

            // Skipping deletedAt in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Review value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Review {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub author: Vec<String>,
            pub rating: Vec<f32>,
            pub text: Vec<String>,
            pub time: Vec<chrono::DateTime::<chrono::Utc>>,
            pub language: Vec<String>,
            pub profile_photo_url: Vec<String>,
            pub review_count: Vec<i32>,
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub updated_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub deleted_at: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Review".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "author" => intermediate_rep.author.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "rating" => intermediate_rep.rating.push(<f32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "text" => intermediate_rep.text.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "time" => intermediate_rep.time.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "language" => intermediate_rep.language.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "profilePhotoUrl" => intermediate_rep.profile_photo_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "reviewCount" => intermediate_rep.review_count.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "createdAt" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "updatedAt" => intermediate_rep.updated_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "deletedAt" => intermediate_rep.deleted_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Review".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Review {
            id: intermediate_rep.id.into_iter().next(),
            author: intermediate_rep.author.into_iter().next(),
            rating: intermediate_rep.rating.into_iter().next(),
            text: intermediate_rep.text.into_iter().next(),
            time: intermediate_rep.time.into_iter().next(),
            language: intermediate_rep.language.into_iter().next(),
            profile_photo_url: intermediate_rep.profile_photo_url.into_iter().next(),
            review_count: intermediate_rep.review_count.into_iter().next(),
            created_at: intermediate_rep.created_at.into_iter().next(),
            updated_at: intermediate_rep.updated_at.into_iter().next(),
            deleted_at: intermediate_rep.deleted_at.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Review> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Review>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Review>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Review - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Review> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Review as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Review - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details.  You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RpcPeriodStatus {
    /// The status code, which should be an enum value of [google.rpc.Code][google.rpc.Code].
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<i32>,

    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the [google.rpc.Status.details][google.rpc.Status.details] field, or localized by the client.
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    /// A list of messages that carry the error details.  There is a common set of message types for APIs to use.
    #[serde(rename = "details")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub details: Option<Vec<models::Any>>,

}


impl RpcPeriodStatus {
    #[allow(clippy::new_without_default)]
    pub fn new() -> RpcPeriodStatus {
        RpcPeriodStatus {
            code: None,
            message: None,
            details: None,
        }
    }
}

/// Converts the RpcPeriodStatus value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RpcPeriodStatus {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.code.as_ref().map(|code| {
                [
                    "code".to_string(),
                    code.to_string(),
                ].join(",")
            }),


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),

            // Skipping details in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RpcPeriodStatus value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RpcPeriodStatus {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<i32>,
            pub message: Vec<String>,
            pub details: Vec<Vec<models::Any>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RpcPeriodStatus".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "details" => return std::result::Result::Err("Parsing a container in this style is not supported in RpcPeriodStatus".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing RpcPeriodStatus".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RpcPeriodStatus {
            code: intermediate_rep.code.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
            details: intermediate_rep.details.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RpcPeriodStatus> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RpcPeriodStatus>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RpcPeriodStatus>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RpcPeriodStatus - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<RpcPeriodStatus> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RpcPeriodStatus as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RpcPeriodStatus - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SchemaValidation {
    #[serde(rename = "schemaVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_version: Option<String>,

    #[serde(rename = "invalidPatterns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub invalid_patterns: Option<Vec<String>>,

    #[serde(rename = "typeMismatches")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_mismatches: Option<Vec<String>>,

}


impl SchemaValidation {
    #[allow(clippy::new_without_default)]
    pub fn new() -> SchemaValidation {
        SchemaValidation {
            schema_version: None,
            invalid_patterns: None,
            type_mismatches: None,
        }
    }
}

/// Converts the SchemaValidation value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SchemaValidation {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.schema_version.as_ref().map(|schema_version| {
                [
                    "schemaVersion".to_string(),
                    schema_version.to_string(),
                ].join(",")
            }),


            self.invalid_patterns.as_ref().map(|invalid_patterns| {
                [
                    "invalidPatterns".to_string(),
                    invalid_patterns.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.type_mismatches.as_ref().map(|type_mismatches| {
                [
                    "typeMismatches".to_string(),
                    type_mismatches.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SchemaValidation value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SchemaValidation {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub schema_version: Vec<String>,
            pub invalid_patterns: Vec<Vec<String>>,
            pub type_mismatches: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SchemaValidation".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "schemaVersion" => intermediate_rep.schema_version.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "invalidPatterns" => return std::result::Result::Err("Parsing a container in this style is not supported in SchemaValidation".to_string()),
                    "typeMismatches" => return std::result::Result::Err("Parsing a container in this style is not supported in SchemaValidation".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing SchemaValidation".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SchemaValidation {
            schema_version: intermediate_rep.schema_version.into_iter().next(),
            invalid_patterns: intermediate_rep.invalid_patterns.into_iter().next(),
            type_mismatches: intermediate_rep.type_mismatches.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SchemaValidation> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SchemaValidation>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SchemaValidation>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SchemaValidation - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SchemaValidation> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SchemaValidation as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SchemaValidation - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// ScrapingJob represents a Google Maps scraping task. This message defines both the configuration and current state of a scraping operation.  Key components: - Basic metadata (id, name, timestamps) - Job status tracking - Search configuration parameters - Geographic settings - Performance options - Multi-tenant context  Database considerations: - Stored in \"gmaps_jobs\" table - Uses GORM for ORM mapping - Includes foreign key to Account - Supports soft deletes  Usage example: ```go job := &ScrapingJob{     Name: \"Athens Cafes\",     Status: BackgroundJobStatus_BACKGROUND_JOB_STATUS_QUEUED,     Keywords: []string{\"cafe\", \"coffee\"},     Lang: \"el\",     Zoom: 15,     FastMode: true,     MaxTime: 3600, } ```
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ScrapingJob {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "priority")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub priority: Option<i32>,

    #[serde(rename = "payloadType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub payload_type: Option<String>,

    #[serde(rename = "payload")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub payload: Option<swagger::ByteArray>,

    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<models::BackgroundJobStatus>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "keywords")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub keywords: Option<Vec<String>>,

    #[serde(rename = "lang")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lang: Option<String>,

    #[serde(rename = "zoom")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub zoom: Option<i32>,

    #[serde(rename = "lat")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lat: Option<String>,

    #[serde(rename = "lon")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lon: Option<String>,

    #[serde(rename = "fastMode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fast_mode: Option<bool>,

    #[serde(rename = "radius")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub radius: Option<i32>,

    #[serde(rename = "depth")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub depth: Option<i32>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<bool>,

    #[serde(rename = "maxTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_time: Option<i32>,

    #[serde(rename = "proxies")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub proxies: Option<Vec<String>>,

    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "leads")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub leads: Option<Vec<models::Lead>>,

}


impl ScrapingJob {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ScrapingJob {
        ScrapingJob {
            id: None,
            priority: None,
            payload_type: None,
            payload: None,
            created_at: None,
            status: None,
            name: None,
            keywords: None,
            lang: None,
            zoom: None,
            lat: None,
            lon: None,
            fast_mode: None,
            radius: None,
            depth: None,
            email: None,
            max_time: None,
            proxies: None,
            updated_at: None,
            deleted_at: None,
            leads: None,
        }
    }
}

/// Converts the ScrapingJob value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ScrapingJob {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.id.as_ref().map(|id| {
                [
                    "id".to_string(),
                    id.to_string(),
                ].join(",")
            }),


            self.priority.as_ref().map(|priority| {
                [
                    "priority".to_string(),
                    priority.to_string(),
                ].join(",")
            }),


            self.payload_type.as_ref().map(|payload_type| {
                [
                    "payloadType".to_string(),
                    payload_type.to_string(),
                ].join(",")
            }),

            // Skipping payload in query parameter serialization
            // Skipping payload in query parameter serialization

            // Skipping createdAt in query parameter serialization

            // Skipping status in query parameter serialization


            self.name.as_ref().map(|name| {
                [
                    "name".to_string(),
                    name.to_string(),
                ].join(",")
            }),


            self.keywords.as_ref().map(|keywords| {
                [
                    "keywords".to_string(),
                    keywords.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.lang.as_ref().map(|lang| {
                [
                    "lang".to_string(),
                    lang.to_string(),
                ].join(",")
            }),


            self.zoom.as_ref().map(|zoom| {
                [
                    "zoom".to_string(),
                    zoom.to_string(),
                ].join(",")
            }),


            self.lat.as_ref().map(|lat| {
                [
                    "lat".to_string(),
                    lat.to_string(),
                ].join(",")
            }),


            self.lon.as_ref().map(|lon| {
                [
                    "lon".to_string(),
                    lon.to_string(),
                ].join(",")
            }),


            self.fast_mode.as_ref().map(|fast_mode| {
                [
                    "fastMode".to_string(),
                    fast_mode.to_string(),
                ].join(",")
            }),


            self.radius.as_ref().map(|radius| {
                [
                    "radius".to_string(),
                    radius.to_string(),
                ].join(",")
            }),


            self.depth.as_ref().map(|depth| {
                [
                    "depth".to_string(),
                    depth.to_string(),
                ].join(",")
            }),


            self.email.as_ref().map(|email| {
                [
                    "email".to_string(),
                    email.to_string(),
                ].join(",")
            }),


            self.max_time.as_ref().map(|max_time| {
                [
                    "maxTime".to_string(),
                    max_time.to_string(),
                ].join(",")
            }),


            self.proxies.as_ref().map(|proxies| {
                [
                    "proxies".to_string(),
                    proxies.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

            // Skipping updatedAt in query parameter serialization

            // Skipping deletedAt in query parameter serialization

            // Skipping leads in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ScrapingJob value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ScrapingJob {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub priority: Vec<i32>,
            pub payload_type: Vec<String>,
            pub payload: Vec<swagger::ByteArray>,
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub status: Vec<models::BackgroundJobStatus>,
            pub name: Vec<String>,
            pub keywords: Vec<Vec<String>>,
            pub lang: Vec<String>,
            pub zoom: Vec<i32>,
            pub lat: Vec<String>,
            pub lon: Vec<String>,
            pub fast_mode: Vec<bool>,
            pub radius: Vec<i32>,
            pub depth: Vec<i32>,
            pub email: Vec<bool>,
            pub max_time: Vec<i32>,
            pub proxies: Vec<Vec<String>>,
            pub updated_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub deleted_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub leads: Vec<Vec<models::Lead>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ScrapingJob".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "priority" => intermediate_rep.priority.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "payloadType" => intermediate_rep.payload_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "payload" => return std::result::Result::Err("Parsing binary data in this style is not supported in ScrapingJob".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "createdAt" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<models::BackgroundJobStatus as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "keywords" => return std::result::Result::Err("Parsing a container in this style is not supported in ScrapingJob".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "lang" => intermediate_rep.lang.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "zoom" => intermediate_rep.zoom.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "lat" => intermediate_rep.lat.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "lon" => intermediate_rep.lon.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "fastMode" => intermediate_rep.fast_mode.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "radius" => intermediate_rep.radius.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "depth" => intermediate_rep.depth.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "email" => intermediate_rep.email.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "maxTime" => intermediate_rep.max_time.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "proxies" => return std::result::Result::Err("Parsing a container in this style is not supported in ScrapingJob".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "updatedAt" => intermediate_rep.updated_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "deletedAt" => intermediate_rep.deleted_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "leads" => return std::result::Result::Err("Parsing a container in this style is not supported in ScrapingJob".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ScrapingJob".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ScrapingJob {
            id: intermediate_rep.id.into_iter().next(),
            priority: intermediate_rep.priority.into_iter().next(),
            payload_type: intermediate_rep.payload_type.into_iter().next(),
            payload: intermediate_rep.payload.into_iter().next(),
            created_at: intermediate_rep.created_at.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            name: intermediate_rep.name.into_iter().next(),
            keywords: intermediate_rep.keywords.into_iter().next(),
            lang: intermediate_rep.lang.into_iter().next(),
            zoom: intermediate_rep.zoom.into_iter().next(),
            lat: intermediate_rep.lat.into_iter().next(),
            lon: intermediate_rep.lon.into_iter().next(),
            fast_mode: intermediate_rep.fast_mode.into_iter().next(),
            radius: intermediate_rep.radius.into_iter().next(),
            depth: intermediate_rep.depth.into_iter().next(),
            email: intermediate_rep.email.into_iter().next(),
            max_time: intermediate_rep.max_time.into_iter().next(),
            proxies: intermediate_rep.proxies.into_iter().next(),
            updated_at: intermediate_rep.updated_at.into_iter().next(),
            deleted_at: intermediate_rep.deleted_at.into_iter().next(),
            leads: intermediate_rep.leads.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ScrapingJob> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ScrapingJob>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ScrapingJob>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ScrapingJob - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ScrapingJob> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ScrapingJob as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ScrapingJob - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ScrapingWorkflow {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "cronExpression")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cron_expression: Option<String>,

    #[serde(rename = "nextRunTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_run_time: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "lastRunTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_run_time: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<models::WorkflowStatus>,

    #[serde(rename = "retryCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub retry_count: Option<i32>,

    #[serde(rename = "maxRetries")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_retries: Option<i32>,

    #[serde(rename = "alertEmails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alert_emails: Option<String>,

    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub org_id: Option<String>,

    #[serde(rename = "tenantId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tenant_id: Option<String>,

    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub jobs: Option<Vec<models::ScrapingJob>>,

    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workspace: Option<models::Workspace>,

    #[serde(rename = "geoFencingRadius")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub geo_fencing_radius: Option<f32>,

    #[serde(rename = "geoFencingLat")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub geo_fencing_lat: Option<f64>,

    #[serde(rename = "geoFencingLon")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub geo_fencing_lon: Option<f64>,

    #[serde(rename = "geoFencingZoomMin")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub geo_fencing_zoom_min: Option<i32>,

    #[serde(rename = "geoFencingZoomMax")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub geo_fencing_zoom_max: Option<i32>,

    #[serde(rename = "includeReviews")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub include_reviews: Option<bool>,

    #[serde(rename = "includePhotos")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub include_photos: Option<bool>,

    #[serde(rename = "includeBusinessHours")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub include_business_hours: Option<bool>,

    #[serde(rename = "maxReviewsPerBusiness")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_reviews_per_business: Option<i32>,

    #[serde(rename = "outputFormat")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub output_format: Option<models::OutputFormat>,

    #[serde(rename = "outputDestination")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub output_destination: Option<String>,

    #[serde(rename = "dataRetention")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub data_retention: Option<String>,

    #[serde(rename = "anonymizePii")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub anonymize_pii: Option<bool>,

    #[serde(rename = "notificationWebhookUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub notification_webhook_url: Option<String>,

    #[serde(rename = "notificationSlackChannel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub notification_slack_channel: Option<String>,

    #[serde(rename = "notificationEmailGroup")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub notification_email_group: Option<String>,

    #[serde(rename = "notificationNotifyOnStart")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub notification_notify_on_start: Option<bool>,

    #[serde(rename = "notificationNotifyOnComplete")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub notification_notify_on_complete: Option<bool>,

    #[serde(rename = "notificationNotifyOnFailure")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub notification_notify_on_failure: Option<bool>,

    #[serde(rename = "contentFilterAllowedCountries")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_filter_allowed_countries: Option<Vec<String>>,

    #[serde(rename = "contentFilterExcludedTypes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_filter_excluded_types: Option<Vec<String>>,

    #[serde(rename = "contentFilterMinimumRating")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_filter_minimum_rating: Option<f32>,

    #[serde(rename = "contentFilterMinimumReviews")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_filter_minimum_reviews: Option<i32>,

    #[serde(rename = "qosMaxConcurrentRequests")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub qos_max_concurrent_requests: Option<i32>,

    #[serde(rename = "qosMaxRetries")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub qos_max_retries: Option<i32>,

    #[serde(rename = "qosRequestTimeout")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub qos_request_timeout: Option<String>,

    #[serde(rename = "qosEnableJavascript")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub qos_enable_javascript: Option<bool>,

    #[serde(rename = "respectRobotsTxt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub respect_robots_txt: Option<bool>,

    #[serde(rename = "acceptTermsOfService")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_terms_of_service: Option<bool>,

    #[serde(rename = "userAgent")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_agent: Option<String>,

}


impl ScrapingWorkflow {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ScrapingWorkflow {
        ScrapingWorkflow {
            id: None,
            cron_expression: None,
            next_run_time: None,
            last_run_time: None,
            status: None,
            retry_count: None,
            max_retries: None,
            alert_emails: None,
            org_id: None,
            tenant_id: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
            jobs: None,
            workspace: None,
            geo_fencing_radius: None,
            geo_fencing_lat: None,
            geo_fencing_lon: None,
            geo_fencing_zoom_min: None,
            geo_fencing_zoom_max: None,
            include_reviews: None,
            include_photos: None,
            include_business_hours: None,
            max_reviews_per_business: None,
            output_format: None,
            output_destination: None,
            data_retention: None,
            anonymize_pii: None,
            notification_webhook_url: None,
            notification_slack_channel: None,
            notification_email_group: None,
            notification_notify_on_start: None,
            notification_notify_on_complete: None,
            notification_notify_on_failure: None,
            content_filter_allowed_countries: None,
            content_filter_excluded_types: None,
            content_filter_minimum_rating: None,
            content_filter_minimum_reviews: None,
            qos_max_concurrent_requests: None,
            qos_max_retries: None,
            qos_request_timeout: None,
            qos_enable_javascript: None,
            respect_robots_txt: None,
            accept_terms_of_service: None,
            user_agent: None,
        }
    }
}

/// Converts the ScrapingWorkflow value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ScrapingWorkflow {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.id.as_ref().map(|id| {
                [
                    "id".to_string(),
                    id.to_string(),
                ].join(",")
            }),


            self.cron_expression.as_ref().map(|cron_expression| {
                [
                    "cronExpression".to_string(),
                    cron_expression.to_string(),
                ].join(",")
            }),

            // Skipping nextRunTime in query parameter serialization

            // Skipping lastRunTime in query parameter serialization

            // Skipping status in query parameter serialization


            self.retry_count.as_ref().map(|retry_count| {
                [
                    "retryCount".to_string(),
                    retry_count.to_string(),
                ].join(",")
            }),


            self.max_retries.as_ref().map(|max_retries| {
                [
                    "maxRetries".to_string(),
                    max_retries.to_string(),
                ].join(",")
            }),


            self.alert_emails.as_ref().map(|alert_emails| {
                [
                    "alertEmails".to_string(),
                    alert_emails.to_string(),
                ].join(",")
            }),


            self.org_id.as_ref().map(|org_id| {
                [
                    "orgId".to_string(),
                    org_id.to_string(),
                ].join(",")
            }),


            self.tenant_id.as_ref().map(|tenant_id| {
                [
                    "tenantId".to_string(),
                    tenant_id.to_string(),
                ].join(",")
            }),

            // Skipping createdAt in query parameter serialization

            // Skipping updatedAt in query parameter serialization

            // Skipping deletedAt in query parameter serialization

            // Skipping jobs in query parameter serialization

            // Skipping workspace in query parameter serialization


            self.geo_fencing_radius.as_ref().map(|geo_fencing_radius| {
                [
                    "geoFencingRadius".to_string(),
                    geo_fencing_radius.to_string(),
                ].join(",")
            }),


            self.geo_fencing_lat.as_ref().map(|geo_fencing_lat| {
                [
                    "geoFencingLat".to_string(),
                    geo_fencing_lat.to_string(),
                ].join(",")
            }),


            self.geo_fencing_lon.as_ref().map(|geo_fencing_lon| {
                [
                    "geoFencingLon".to_string(),
                    geo_fencing_lon.to_string(),
                ].join(",")
            }),


            self.geo_fencing_zoom_min.as_ref().map(|geo_fencing_zoom_min| {
                [
                    "geoFencingZoomMin".to_string(),
                    geo_fencing_zoom_min.to_string(),
                ].join(",")
            }),


            self.geo_fencing_zoom_max.as_ref().map(|geo_fencing_zoom_max| {
                [
                    "geoFencingZoomMax".to_string(),
                    geo_fencing_zoom_max.to_string(),
                ].join(",")
            }),


            self.include_reviews.as_ref().map(|include_reviews| {
                [
                    "includeReviews".to_string(),
                    include_reviews.to_string(),
                ].join(",")
            }),


            self.include_photos.as_ref().map(|include_photos| {
                [
                    "includePhotos".to_string(),
                    include_photos.to_string(),
                ].join(",")
            }),


            self.include_business_hours.as_ref().map(|include_business_hours| {
                [
                    "includeBusinessHours".to_string(),
                    include_business_hours.to_string(),
                ].join(",")
            }),


            self.max_reviews_per_business.as_ref().map(|max_reviews_per_business| {
                [
                    "maxReviewsPerBusiness".to_string(),
                    max_reviews_per_business.to_string(),
                ].join(",")
            }),

            // Skipping outputFormat in query parameter serialization


            self.output_destination.as_ref().map(|output_destination| {
                [
                    "outputDestination".to_string(),
                    output_destination.to_string(),
                ].join(",")
            }),


            self.data_retention.as_ref().map(|data_retention| {
                [
                    "dataRetention".to_string(),
                    data_retention.to_string(),
                ].join(",")
            }),


            self.anonymize_pii.as_ref().map(|anonymize_pii| {
                [
                    "anonymizePii".to_string(),
                    anonymize_pii.to_string(),
                ].join(",")
            }),


            self.notification_webhook_url.as_ref().map(|notification_webhook_url| {
                [
                    "notificationWebhookUrl".to_string(),
                    notification_webhook_url.to_string(),
                ].join(",")
            }),


            self.notification_slack_channel.as_ref().map(|notification_slack_channel| {
                [
                    "notificationSlackChannel".to_string(),
                    notification_slack_channel.to_string(),
                ].join(",")
            }),


            self.notification_email_group.as_ref().map(|notification_email_group| {
                [
                    "notificationEmailGroup".to_string(),
                    notification_email_group.to_string(),
                ].join(",")
            }),


            self.notification_notify_on_start.as_ref().map(|notification_notify_on_start| {
                [
                    "notificationNotifyOnStart".to_string(),
                    notification_notify_on_start.to_string(),
                ].join(",")
            }),


            self.notification_notify_on_complete.as_ref().map(|notification_notify_on_complete| {
                [
                    "notificationNotifyOnComplete".to_string(),
                    notification_notify_on_complete.to_string(),
                ].join(",")
            }),


            self.notification_notify_on_failure.as_ref().map(|notification_notify_on_failure| {
                [
                    "notificationNotifyOnFailure".to_string(),
                    notification_notify_on_failure.to_string(),
                ].join(",")
            }),


            self.content_filter_allowed_countries.as_ref().map(|content_filter_allowed_countries| {
                [
                    "contentFilterAllowedCountries".to_string(),
                    content_filter_allowed_countries.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.content_filter_excluded_types.as_ref().map(|content_filter_excluded_types| {
                [
                    "contentFilterExcludedTypes".to_string(),
                    content_filter_excluded_types.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.content_filter_minimum_rating.as_ref().map(|content_filter_minimum_rating| {
                [
                    "contentFilterMinimumRating".to_string(),
                    content_filter_minimum_rating.to_string(),
                ].join(",")
            }),


            self.content_filter_minimum_reviews.as_ref().map(|content_filter_minimum_reviews| {
                [
                    "contentFilterMinimumReviews".to_string(),
                    content_filter_minimum_reviews.to_string(),
                ].join(",")
            }),


            self.qos_max_concurrent_requests.as_ref().map(|qos_max_concurrent_requests| {
                [
                    "qosMaxConcurrentRequests".to_string(),
                    qos_max_concurrent_requests.to_string(),
                ].join(",")
            }),


            self.qos_max_retries.as_ref().map(|qos_max_retries| {
                [
                    "qosMaxRetries".to_string(),
                    qos_max_retries.to_string(),
                ].join(",")
            }),


            self.qos_request_timeout.as_ref().map(|qos_request_timeout| {
                [
                    "qosRequestTimeout".to_string(),
                    qos_request_timeout.to_string(),
                ].join(",")
            }),


            self.qos_enable_javascript.as_ref().map(|qos_enable_javascript| {
                [
                    "qosEnableJavascript".to_string(),
                    qos_enable_javascript.to_string(),
                ].join(",")
            }),


            self.respect_robots_txt.as_ref().map(|respect_robots_txt| {
                [
                    "respectRobotsTxt".to_string(),
                    respect_robots_txt.to_string(),
                ].join(",")
            }),


            self.accept_terms_of_service.as_ref().map(|accept_terms_of_service| {
                [
                    "acceptTermsOfService".to_string(),
                    accept_terms_of_service.to_string(),
                ].join(",")
            }),


            self.user_agent.as_ref().map(|user_agent| {
                [
                    "userAgent".to_string(),
                    user_agent.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ScrapingWorkflow value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ScrapingWorkflow {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub cron_expression: Vec<String>,
            pub next_run_time: Vec<chrono::DateTime::<chrono::Utc>>,
            pub last_run_time: Vec<chrono::DateTime::<chrono::Utc>>,
            pub status: Vec<models::WorkflowStatus>,
            pub retry_count: Vec<i32>,
            pub max_retries: Vec<i32>,
            pub alert_emails: Vec<String>,
            pub org_id: Vec<String>,
            pub tenant_id: Vec<String>,
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub updated_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub deleted_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub jobs: Vec<Vec<models::ScrapingJob>>,
            pub workspace: Vec<models::Workspace>,
            pub geo_fencing_radius: Vec<f32>,
            pub geo_fencing_lat: Vec<f64>,
            pub geo_fencing_lon: Vec<f64>,
            pub geo_fencing_zoom_min: Vec<i32>,
            pub geo_fencing_zoom_max: Vec<i32>,
            pub include_reviews: Vec<bool>,
            pub include_photos: Vec<bool>,
            pub include_business_hours: Vec<bool>,
            pub max_reviews_per_business: Vec<i32>,
            pub output_format: Vec<models::OutputFormat>,
            pub output_destination: Vec<String>,
            pub data_retention: Vec<String>,
            pub anonymize_pii: Vec<bool>,
            pub notification_webhook_url: Vec<String>,
            pub notification_slack_channel: Vec<String>,
            pub notification_email_group: Vec<String>,
            pub notification_notify_on_start: Vec<bool>,
            pub notification_notify_on_complete: Vec<bool>,
            pub notification_notify_on_failure: Vec<bool>,
            pub content_filter_allowed_countries: Vec<Vec<String>>,
            pub content_filter_excluded_types: Vec<Vec<String>>,
            pub content_filter_minimum_rating: Vec<f32>,
            pub content_filter_minimum_reviews: Vec<i32>,
            pub qos_max_concurrent_requests: Vec<i32>,
            pub qos_max_retries: Vec<i32>,
            pub qos_request_timeout: Vec<String>,
            pub qos_enable_javascript: Vec<bool>,
            pub respect_robots_txt: Vec<bool>,
            pub accept_terms_of_service: Vec<bool>,
            pub user_agent: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ScrapingWorkflow".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "cronExpression" => intermediate_rep.cron_expression.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "nextRunTime" => intermediate_rep.next_run_time.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "lastRunTime" => intermediate_rep.last_run_time.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<models::WorkflowStatus as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "retryCount" => intermediate_rep.retry_count.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "maxRetries" => intermediate_rep.max_retries.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "alertEmails" => intermediate_rep.alert_emails.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "orgId" => intermediate_rep.org_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "tenantId" => intermediate_rep.tenant_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "createdAt" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "updatedAt" => intermediate_rep.updated_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "deletedAt" => intermediate_rep.deleted_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "jobs" => return std::result::Result::Err("Parsing a container in this style is not supported in ScrapingWorkflow".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "workspace" => intermediate_rep.workspace.push(<models::Workspace as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "geoFencingRadius" => intermediate_rep.geo_fencing_radius.push(<f32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "geoFencingLat" => intermediate_rep.geo_fencing_lat.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "geoFencingLon" => intermediate_rep.geo_fencing_lon.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "geoFencingZoomMin" => intermediate_rep.geo_fencing_zoom_min.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "geoFencingZoomMax" => intermediate_rep.geo_fencing_zoom_max.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "includeReviews" => intermediate_rep.include_reviews.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "includePhotos" => intermediate_rep.include_photos.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "includeBusinessHours" => intermediate_rep.include_business_hours.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "maxReviewsPerBusiness" => intermediate_rep.max_reviews_per_business.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "outputFormat" => intermediate_rep.output_format.push(<models::OutputFormat as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "outputDestination" => intermediate_rep.output_destination.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "dataRetention" => intermediate_rep.data_retention.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "anonymizePii" => intermediate_rep.anonymize_pii.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "notificationWebhookUrl" => intermediate_rep.notification_webhook_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "notificationSlackChannel" => intermediate_rep.notification_slack_channel.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "notificationEmailGroup" => intermediate_rep.notification_email_group.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "notificationNotifyOnStart" => intermediate_rep.notification_notify_on_start.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "notificationNotifyOnComplete" => intermediate_rep.notification_notify_on_complete.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "notificationNotifyOnFailure" => intermediate_rep.notification_notify_on_failure.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "contentFilterAllowedCountries" => return std::result::Result::Err("Parsing a container in this style is not supported in ScrapingWorkflow".to_string()),
                    "contentFilterExcludedTypes" => return std::result::Result::Err("Parsing a container in this style is not supported in ScrapingWorkflow".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "contentFilterMinimumRating" => intermediate_rep.content_filter_minimum_rating.push(<f32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "contentFilterMinimumReviews" => intermediate_rep.content_filter_minimum_reviews.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "qosMaxConcurrentRequests" => intermediate_rep.qos_max_concurrent_requests.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "qosMaxRetries" => intermediate_rep.qos_max_retries.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "qosRequestTimeout" => intermediate_rep.qos_request_timeout.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "qosEnableJavascript" => intermediate_rep.qos_enable_javascript.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "respectRobotsTxt" => intermediate_rep.respect_robots_txt.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "acceptTermsOfService" => intermediate_rep.accept_terms_of_service.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "userAgent" => intermediate_rep.user_agent.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ScrapingWorkflow".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ScrapingWorkflow {
            id: intermediate_rep.id.into_iter().next(),
            cron_expression: intermediate_rep.cron_expression.into_iter().next(),
            next_run_time: intermediate_rep.next_run_time.into_iter().next(),
            last_run_time: intermediate_rep.last_run_time.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            retry_count: intermediate_rep.retry_count.into_iter().next(),
            max_retries: intermediate_rep.max_retries.into_iter().next(),
            alert_emails: intermediate_rep.alert_emails.into_iter().next(),
            org_id: intermediate_rep.org_id.into_iter().next(),
            tenant_id: intermediate_rep.tenant_id.into_iter().next(),
            created_at: intermediate_rep.created_at.into_iter().next(),
            updated_at: intermediate_rep.updated_at.into_iter().next(),
            deleted_at: intermediate_rep.deleted_at.into_iter().next(),
            jobs: intermediate_rep.jobs.into_iter().next(),
            workspace: intermediate_rep.workspace.into_iter().next(),
            geo_fencing_radius: intermediate_rep.geo_fencing_radius.into_iter().next(),
            geo_fencing_lat: intermediate_rep.geo_fencing_lat.into_iter().next(),
            geo_fencing_lon: intermediate_rep.geo_fencing_lon.into_iter().next(),
            geo_fencing_zoom_min: intermediate_rep.geo_fencing_zoom_min.into_iter().next(),
            geo_fencing_zoom_max: intermediate_rep.geo_fencing_zoom_max.into_iter().next(),
            include_reviews: intermediate_rep.include_reviews.into_iter().next(),
            include_photos: intermediate_rep.include_photos.into_iter().next(),
            include_business_hours: intermediate_rep.include_business_hours.into_iter().next(),
            max_reviews_per_business: intermediate_rep.max_reviews_per_business.into_iter().next(),
            output_format: intermediate_rep.output_format.into_iter().next(),
            output_destination: intermediate_rep.output_destination.into_iter().next(),
            data_retention: intermediate_rep.data_retention.into_iter().next(),
            anonymize_pii: intermediate_rep.anonymize_pii.into_iter().next(),
            notification_webhook_url: intermediate_rep.notification_webhook_url.into_iter().next(),
            notification_slack_channel: intermediate_rep.notification_slack_channel.into_iter().next(),
            notification_email_group: intermediate_rep.notification_email_group.into_iter().next(),
            notification_notify_on_start: intermediate_rep.notification_notify_on_start.into_iter().next(),
            notification_notify_on_complete: intermediate_rep.notification_notify_on_complete.into_iter().next(),
            notification_notify_on_failure: intermediate_rep.notification_notify_on_failure.into_iter().next(),
            content_filter_allowed_countries: intermediate_rep.content_filter_allowed_countries.into_iter().next(),
            content_filter_excluded_types: intermediate_rep.content_filter_excluded_types.into_iter().next(),
            content_filter_minimum_rating: intermediate_rep.content_filter_minimum_rating.into_iter().next(),
            content_filter_minimum_reviews: intermediate_rep.content_filter_minimum_reviews.into_iter().next(),
            qos_max_concurrent_requests: intermediate_rep.qos_max_concurrent_requests.into_iter().next(),
            qos_max_retries: intermediate_rep.qos_max_retries.into_iter().next(),
            qos_request_timeout: intermediate_rep.qos_request_timeout.into_iter().next(),
            qos_enable_javascript: intermediate_rep.qos_enable_javascript.into_iter().next(),
            respect_robots_txt: intermediate_rep.respect_robots_txt.into_iter().next(),
            accept_terms_of_service: intermediate_rep.accept_terms_of_service.into_iter().next(),
            user_agent: intermediate_rep.user_agent.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ScrapingWorkflow> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ScrapingWorkflow>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ScrapingWorkflow>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ScrapingWorkflow - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ScrapingWorkflow> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ScrapingWorkflow as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ScrapingWorkflow - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServiceStatus {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metrics: Option<std::collections::HashMap<String, String>>,

    #[serde(rename = "dependencies")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dependencies: Option<Vec<models::Dependency>>,

}


impl ServiceStatus {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ServiceStatus {
        ServiceStatus {
            name: None,
            status: None,
            metrics: None,
            dependencies: None,
        }
    }
}

/// Converts the ServiceStatus value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServiceStatus {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.name.as_ref().map(|name| {
                [
                    "name".to_string(),
                    name.to_string(),
                ].join(",")
            }),


            self.status.as_ref().map(|status| {
                [
                    "status".to_string(),
                    status.to_string(),
                ].join(",")
            }),

            // Skipping metrics in query parameter serialization

            // Skipping dependencies in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServiceStatus value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServiceStatus {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub status: Vec<String>,
            pub metrics: Vec<std::collections::HashMap<String, String>>,
            pub dependencies: Vec<Vec<models::Dependency>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServiceStatus".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "metrics" => return std::result::Result::Err("Parsing a container in this style is not supported in ServiceStatus".to_string()),
                    "dependencies" => return std::result::Result::Err("Parsing a container in this style is not supported in ServiceStatus".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServiceStatus".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServiceStatus {
            name: intermediate_rep.name.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            metrics: intermediate_rep.metrics.into_iter().next(),
            dependencies: intermediate_rep.dependencies.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServiceStatus> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServiceStatus>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServiceStatus>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServiceStatus - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServiceStatus> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServiceStatus as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServiceStatus - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Represents service availability errors
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServiceUnavailableErrorMessageResponse {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<models::InternalErrorCode>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "availabilityInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub availability_info: Option<models::AvailabilityInfo>,

    #[serde(rename = "errorResponse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_response: Option<models::ErrorResponse>,

}


impl ServiceUnavailableErrorMessageResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ServiceUnavailableErrorMessageResponse {
        ServiceUnavailableErrorMessageResponse {
            code: None,
            message: None,
            availability_info: None,
            error_response: None,
        }
    }
}

/// Converts the ServiceUnavailableErrorMessageResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServiceUnavailableErrorMessageResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping code in query parameter serialization


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),

            // Skipping availabilityInfo in query parameter serialization

            // Skipping errorResponse in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServiceUnavailableErrorMessageResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServiceUnavailableErrorMessageResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<models::InternalErrorCode>,
            pub message: Vec<String>,
            pub availability_info: Vec<models::AvailabilityInfo>,
            pub error_response: Vec<models::ErrorResponse>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServiceUnavailableErrorMessageResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(<models::InternalErrorCode as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "availabilityInfo" => intermediate_rep.availability_info.push(<models::AvailabilityInfo as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "errorResponse" => intermediate_rep.error_response.push(<models::ErrorResponse as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServiceUnavailableErrorMessageResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServiceUnavailableErrorMessageResponse {
            code: intermediate_rep.code.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
            availability_info: intermediate_rep.availability_info.into_iter().next(),
            error_response: intermediate_rep.error_response.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServiceUnavailableErrorMessageResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServiceUnavailableErrorMessageResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServiceUnavailableErrorMessageResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServiceUnavailableErrorMessageResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServiceUnavailableErrorMessageResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServiceUnavailableErrorMessageResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServiceUnavailableErrorMessageResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SessionInfo {
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub session_id: Option<String>,

    #[serde(rename = "lastActive")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_active: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device_id: Option<String>,

    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ip_address: Option<String>,

}


impl SessionInfo {
    #[allow(clippy::new_without_default)]
    pub fn new() -> SessionInfo {
        SessionInfo {
            session_id: None,
            last_active: None,
            device_id: None,
            ip_address: None,
        }
    }
}

/// Converts the SessionInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SessionInfo {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.session_id.as_ref().map(|session_id| {
                [
                    "sessionId".to_string(),
                    session_id.to_string(),
                ].join(",")
            }),

            // Skipping lastActive in query parameter serialization


            self.device_id.as_ref().map(|device_id| {
                [
                    "deviceId".to_string(),
                    device_id.to_string(),
                ].join(",")
            }),


            self.ip_address.as_ref().map(|ip_address| {
                [
                    "ipAddress".to_string(),
                    ip_address.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SessionInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SessionInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub session_id: Vec<String>,
            pub last_active: Vec<chrono::DateTime::<chrono::Utc>>,
            pub device_id: Vec<String>,
            pub ip_address: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SessionInfo".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "sessionId" => intermediate_rep.session_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "lastActive" => intermediate_rep.last_active.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "deviceId" => intermediate_rep.device_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ipAddress" => intermediate_rep.ip_address.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SessionInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SessionInfo {
            session_id: intermediate_rep.session_id.into_iter().next(),
            last_active: intermediate_rep.last_active.into_iter().next(),
            device_id: intermediate_rep.device_id.into_iter().next(),
            ip_address: intermediate_rep.ip_address.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SessionInfo> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SessionInfo>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SessionInfo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SessionInfo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SessionInfo> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SessionInfo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SessionInfo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Suggestions {
    #[serde(rename = "similarResources")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub similar_resources: Option<Vec<String>>,

    #[serde(rename = "alternativePaths")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alternative_paths: Option<Vec<String>>,

    #[serde(rename = "documentationUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(rename = "hints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hints: Option<std::collections::HashMap<String, String>>,

}


impl Suggestions {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Suggestions {
        Suggestions {
            similar_resources: None,
            alternative_paths: None,
            documentation_url: None,
            hints: None,
        }
    }
}

/// Converts the Suggestions value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Suggestions {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.similar_resources.as_ref().map(|similar_resources| {
                [
                    "similarResources".to_string(),
                    similar_resources.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.alternative_paths.as_ref().map(|alternative_paths| {
                [
                    "alternativePaths".to_string(),
                    alternative_paths.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.documentation_url.as_ref().map(|documentation_url| {
                [
                    "documentationUrl".to_string(),
                    documentation_url.to_string(),
                ].join(",")
            }),

            // Skipping hints in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Suggestions value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Suggestions {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub similar_resources: Vec<Vec<String>>,
            pub alternative_paths: Vec<Vec<String>>,
            pub documentation_url: Vec<String>,
            pub hints: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Suggestions".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "similarResources" => return std::result::Result::Err("Parsing a container in this style is not supported in Suggestions".to_string()),
                    "alternativePaths" => return std::result::Result::Err("Parsing a container in this style is not supported in Suggestions".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "documentationUrl" => intermediate_rep.documentation_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "hints" => return std::result::Result::Err("Parsing a container in this style is not supported in Suggestions".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing Suggestions".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Suggestions {
            similar_resources: intermediate_rep.similar_resources.into_iter().next(),
            alternative_paths: intermediate_rep.alternative_paths.into_iter().next(),
            documentation_url: intermediate_rep.documentation_url.into_iter().next(),
            hints: intermediate_rep.hints.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Suggestions> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Suggestions>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Suggestions>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Suggestions - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Suggestions> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Suggestions as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Suggestions - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TokenInfo {
    #[serde(rename = "tokenId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token_id: Option<String>,

    #[serde(rename = "tokenType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token_type: Option<String>,

    #[serde(rename = "expiry")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expiry: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "issuer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issuer: Option<String>,

    #[serde(rename = "audiences")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub audiences: Option<Vec<String>>,

}


impl TokenInfo {
    #[allow(clippy::new_without_default)]
    pub fn new() -> TokenInfo {
        TokenInfo {
            token_id: None,
            token_type: None,
            expiry: None,
            issuer: None,
            audiences: None,
        }
    }
}

/// Converts the TokenInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TokenInfo {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.token_id.as_ref().map(|token_id| {
                [
                    "tokenId".to_string(),
                    token_id.to_string(),
                ].join(",")
            }),


            self.token_type.as_ref().map(|token_type| {
                [
                    "tokenType".to_string(),
                    token_type.to_string(),
                ].join(",")
            }),

            // Skipping expiry in query parameter serialization


            self.issuer.as_ref().map(|issuer| {
                [
                    "issuer".to_string(),
                    issuer.to_string(),
                ].join(",")
            }),


            self.audiences.as_ref().map(|audiences| {
                [
                    "audiences".to_string(),
                    audiences.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TokenInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TokenInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub token_id: Vec<String>,
            pub token_type: Vec<String>,
            pub expiry: Vec<chrono::DateTime::<chrono::Utc>>,
            pub issuer: Vec<String>,
            pub audiences: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TokenInfo".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "tokenId" => intermediate_rep.token_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "tokenType" => intermediate_rep.token_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "expiry" => intermediate_rep.expiry.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "issuer" => intermediate_rep.issuer.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "audiences" => return std::result::Result::Err("Parsing a container in this style is not supported in TokenInfo".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing TokenInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TokenInfo {
            token_id: intermediate_rep.token_id.into_iter().next(),
            token_type: intermediate_rep.token_type.into_iter().next(),
            expiry: intermediate_rep.expiry.into_iter().next(),
            issuer: intermediate_rep.issuer.into_iter().next(),
            audiences: intermediate_rep.audiences.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TokenInfo> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TokenInfo>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TokenInfo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for TokenInfo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<TokenInfo> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TokenInfo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into TokenInfo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Indicates that the server is unwilling to risk processing a request that might be replayed
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TooEarlyErrorMessageResponse {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<i32>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "errorResponse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_response: Option<models::ErrorResponse>,

}


impl TooEarlyErrorMessageResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> TooEarlyErrorMessageResponse {
        TooEarlyErrorMessageResponse {
            code: None,
            message: None,
            error_response: None,
        }
    }
}

/// Converts the TooEarlyErrorMessageResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TooEarlyErrorMessageResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.code.as_ref().map(|code| {
                [
                    "code".to_string(),
                    code.to_string(),
                ].join(",")
            }),


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),

            // Skipping errorResponse in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TooEarlyErrorMessageResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TooEarlyErrorMessageResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<i32>,
            pub message: Vec<String>,
            pub error_response: Vec<models::ErrorResponse>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TooEarlyErrorMessageResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "errorResponse" => intermediate_rep.error_response.push(<models::ErrorResponse as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TooEarlyErrorMessageResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TooEarlyErrorMessageResponse {
            code: intermediate_rep.code.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
            error_response: intermediate_rep.error_response.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TooEarlyErrorMessageResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TooEarlyErrorMessageResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TooEarlyErrorMessageResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for TooEarlyErrorMessageResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<TooEarlyErrorMessageResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TooEarlyErrorMessageResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into TooEarlyErrorMessageResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TriggerWorkflowBody {
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,

}


impl TriggerWorkflowBody {
    #[allow(clippy::new_without_default)]
    pub fn new() -> TriggerWorkflowBody {
        TriggerWorkflowBody {
            parameters: None,
        }
    }
}

/// Converts the TriggerWorkflowBody value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TriggerWorkflowBody {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping parameters in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TriggerWorkflowBody value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TriggerWorkflowBody {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub parameters: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TriggerWorkflowBody".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "parameters" => return std::result::Result::Err("Parsing a container in this style is not supported in TriggerWorkflowBody".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing TriggerWorkflowBody".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TriggerWorkflowBody {
            parameters: intermediate_rep.parameters.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TriggerWorkflowBody> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TriggerWorkflowBody>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TriggerWorkflowBody>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for TriggerWorkflowBody - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<TriggerWorkflowBody> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TriggerWorkflowBody as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into TriggerWorkflowBody - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TriggerWorkflowResponse {
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_id: Option<String>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<models::BackgroundJobStatus>,

}


impl TriggerWorkflowResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> TriggerWorkflowResponse {
        TriggerWorkflowResponse {
            job_id: None,
            status: None,
        }
    }
}

/// Converts the TriggerWorkflowResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TriggerWorkflowResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.job_id.as_ref().map(|job_id| {
                [
                    "jobId".to_string(),
                    job_id.to_string(),
                ].join(",")
            }),

            // Skipping status in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TriggerWorkflowResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TriggerWorkflowResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub job_id: Vec<String>,
            pub status: Vec<models::BackgroundJobStatus>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TriggerWorkflowResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "jobId" => intermediate_rep.job_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<models::BackgroundJobStatus as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TriggerWorkflowResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TriggerWorkflowResponse {
            job_id: intermediate_rep.job_id.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TriggerWorkflowResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TriggerWorkflowResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TriggerWorkflowResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for TriggerWorkflowResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<TriggerWorkflowResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TriggerWorkflowResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into TriggerWorkflowResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Represents errors when the server understands the content type but cannot process the instructions
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UnprocessableEntityErrorMessageResponse {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<i32>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "errors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub errors: Option<Vec<models::FieldError>>,

    #[serde(rename = "errorResponse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_response: Option<models::ErrorResponse>,

}


impl UnprocessableEntityErrorMessageResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> UnprocessableEntityErrorMessageResponse {
        UnprocessableEntityErrorMessageResponse {
            code: None,
            message: None,
            errors: None,
            error_response: None,
        }
    }
}

/// Converts the UnprocessableEntityErrorMessageResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UnprocessableEntityErrorMessageResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.code.as_ref().map(|code| {
                [
                    "code".to_string(),
                    code.to_string(),
                ].join(",")
            }),


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),

            // Skipping errors in query parameter serialization

            // Skipping errorResponse in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UnprocessableEntityErrorMessageResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UnprocessableEntityErrorMessageResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<i32>,
            pub message: Vec<String>,
            pub errors: Vec<Vec<models::FieldError>>,
            pub error_response: Vec<models::ErrorResponse>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UnprocessableEntityErrorMessageResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "errors" => return std::result::Result::Err("Parsing a container in this style is not supported in UnprocessableEntityErrorMessageResponse".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "errorResponse" => intermediate_rep.error_response.push(<models::ErrorResponse as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UnprocessableEntityErrorMessageResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UnprocessableEntityErrorMessageResponse {
            code: intermediate_rep.code.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
            errors: intermediate_rep.errors.into_iter().next(),
            error_response: intermediate_rep.error_response.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UnprocessableEntityErrorMessageResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UnprocessableEntityErrorMessageResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UnprocessableEntityErrorMessageResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UnprocessableEntityErrorMessageResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UnprocessableEntityErrorMessageResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UnprocessableEntityErrorMessageResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UnprocessableEntityErrorMessageResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateAccountRequest {
    #[serde(rename = "account")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub account: Option<models::Account>,

}


impl UpdateAccountRequest {
    #[allow(clippy::new_without_default)]
    pub fn new() -> UpdateAccountRequest {
        UpdateAccountRequest {
            account: None,
        }
    }
}

/// Converts the UpdateAccountRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UpdateAccountRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping account in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UpdateAccountRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UpdateAccountRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub account: Vec<models::Account>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UpdateAccountRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "account" => intermediate_rep.account.push(<models::Account as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UpdateAccountRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UpdateAccountRequest {
            account: intermediate_rep.account.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UpdateAccountRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UpdateAccountRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UpdateAccountRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UpdateAccountRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UpdateAccountRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UpdateAccountRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UpdateAccountRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateAccountResponse {
    #[serde(rename = "account")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub account: Option<models::Account>,

}


impl UpdateAccountResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> UpdateAccountResponse {
        UpdateAccountResponse {
            account: None,
        }
    }
}

/// Converts the UpdateAccountResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UpdateAccountResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping account in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UpdateAccountResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UpdateAccountResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub account: Vec<models::Account>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UpdateAccountResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "account" => intermediate_rep.account.push(<models::Account as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UpdateAccountResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UpdateAccountResponse {
            account: intermediate_rep.account.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UpdateAccountResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UpdateAccountResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UpdateAccountResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UpdateAccountResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UpdateAccountResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UpdateAccountResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UpdateAccountResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateAccountSettingsRequest {
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub settings: Option<models::AccountSettings>,

}


impl UpdateAccountSettingsRequest {
    #[allow(clippy::new_without_default)]
    pub fn new() -> UpdateAccountSettingsRequest {
        UpdateAccountSettingsRequest {
            settings: None,
        }
    }
}

/// Converts the UpdateAccountSettingsRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UpdateAccountSettingsRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping settings in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UpdateAccountSettingsRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UpdateAccountSettingsRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub settings: Vec<models::AccountSettings>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UpdateAccountSettingsRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "settings" => intermediate_rep.settings.push(<models::AccountSettings as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UpdateAccountSettingsRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UpdateAccountSettingsRequest {
            settings: intermediate_rep.settings.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UpdateAccountSettingsRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UpdateAccountSettingsRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UpdateAccountSettingsRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UpdateAccountSettingsRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UpdateAccountSettingsRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UpdateAccountSettingsRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UpdateAccountSettingsRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateAccountSettingsResponse {
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub settings: Option<models::AccountSettings>,

}


impl UpdateAccountSettingsResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> UpdateAccountSettingsResponse {
        UpdateAccountSettingsResponse {
            settings: None,
        }
    }
}

/// Converts the UpdateAccountSettingsResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UpdateAccountSettingsResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping settings in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UpdateAccountSettingsResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UpdateAccountSettingsResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub settings: Vec<models::AccountSettings>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UpdateAccountSettingsResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "settings" => intermediate_rep.settings.push(<models::AccountSettings as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UpdateAccountSettingsResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UpdateAccountSettingsResponse {
            settings: intermediate_rep.settings.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UpdateAccountSettingsResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UpdateAccountSettingsResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UpdateAccountSettingsResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UpdateAccountSettingsResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UpdateAccountSettingsResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UpdateAccountSettingsResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UpdateAccountSettingsResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateWorkflowRequest {
    #[serde(rename = "workflow")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workflow: Option<models::ScrapingWorkflow>,

}


impl UpdateWorkflowRequest {
    #[allow(clippy::new_without_default)]
    pub fn new() -> UpdateWorkflowRequest {
        UpdateWorkflowRequest {
            workflow: None,
        }
    }
}

/// Converts the UpdateWorkflowRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UpdateWorkflowRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping workflow in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UpdateWorkflowRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UpdateWorkflowRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub workflow: Vec<models::ScrapingWorkflow>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UpdateWorkflowRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "workflow" => intermediate_rep.workflow.push(<models::ScrapingWorkflow as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UpdateWorkflowRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UpdateWorkflowRequest {
            workflow: intermediate_rep.workflow.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UpdateWorkflowRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UpdateWorkflowRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UpdateWorkflowRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UpdateWorkflowRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UpdateWorkflowRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UpdateWorkflowRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UpdateWorkflowRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateWorkflowResponse {
    #[serde(rename = "workflow")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workflow: Option<models::ScrapingWorkflow>,

}


impl UpdateWorkflowResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> UpdateWorkflowResponse {
        UpdateWorkflowResponse {
            workflow: None,
        }
    }
}

/// Converts the UpdateWorkflowResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UpdateWorkflowResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping workflow in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UpdateWorkflowResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UpdateWorkflowResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub workflow: Vec<models::ScrapingWorkflow>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UpdateWorkflowResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "workflow" => intermediate_rep.workflow.push(<models::ScrapingWorkflow as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UpdateWorkflowResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UpdateWorkflowResponse {
            workflow: intermediate_rep.workflow.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UpdateWorkflowResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UpdateWorkflowResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UpdateWorkflowResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UpdateWorkflowResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UpdateWorkflowResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UpdateWorkflowResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UpdateWorkflowResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateWorkspaceRequest {
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workspace: Option<models::Workspace>,

}


impl UpdateWorkspaceRequest {
    #[allow(clippy::new_without_default)]
    pub fn new() -> UpdateWorkspaceRequest {
        UpdateWorkspaceRequest {
            workspace: None,
        }
    }
}

/// Converts the UpdateWorkspaceRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UpdateWorkspaceRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping workspace in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UpdateWorkspaceRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UpdateWorkspaceRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub workspace: Vec<models::Workspace>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UpdateWorkspaceRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "workspace" => intermediate_rep.workspace.push(<models::Workspace as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UpdateWorkspaceRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UpdateWorkspaceRequest {
            workspace: intermediate_rep.workspace.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UpdateWorkspaceRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UpdateWorkspaceRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UpdateWorkspaceRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UpdateWorkspaceRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UpdateWorkspaceRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UpdateWorkspaceRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UpdateWorkspaceRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateWorkspaceResponse {
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workspace: Option<models::Workspace>,

}


impl UpdateWorkspaceResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> UpdateWorkspaceResponse {
        UpdateWorkspaceResponse {
            workspace: None,
        }
    }
}

/// Converts the UpdateWorkspaceResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UpdateWorkspaceResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping workspace in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UpdateWorkspaceResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UpdateWorkspaceResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub workspace: Vec<models::Workspace>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UpdateWorkspaceResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "workspace" => intermediate_rep.workspace.push(<models::Workspace as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UpdateWorkspaceResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UpdateWorkspaceResponse {
            workspace: intermediate_rep.workspace.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UpdateWorkspaceResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UpdateWorkspaceResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UpdateWorkspaceResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UpdateWorkspaceResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UpdateWorkspaceResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UpdateWorkspaceResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UpdateWorkspaceResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// - VALIDATION_ERROR: General validation errors  - AUTHORIZATION_MODEL_NOT_FOUND: Resource not found errors  - INVALID_CONTINUATION_TOKEN: Input errors  - DUPLICATE_ENTRY: Conflict errors  - INVALID_SCHEMA_VERSION: Schema errors  - INVALID_FORMAT: New validation error codes  General invalid format  - INVALID_EMAIL_FORMAT: Email format is invalid  - INVALID_PASSWORD_STRENGTH: Password does not meet strength requirements  - FIELD_TOO_SHORT: Field value is too short  - FIELD_TOO_LONG: Field value is too long  - UNSUPPORTED_MEDIA_TYPE: Media type is not supported
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum ValidationErrorCode {
    #[serde(rename = "NO_ERROR")]
    NoError,
    #[serde(rename = "VALIDATION_ERROR")]
    ValidationError,
    #[serde(rename = "INVALID_WRITE_INPUT")]
    InvalidWriteInput,
    #[serde(rename = "INVALID_CHECK_INPUT")]
    InvalidCheckInput,
    #[serde(rename = "INVALID_EXPAND_INPUT")]
    InvalidExpandInput,
    #[serde(rename = "INVALID_OBJECT_FORMAT")]
    InvalidObjectFormat,
    #[serde(rename = "INVALID_USER")]
    InvalidUser,
    #[serde(rename = "INVALID_TUPLE")]
    InvalidTuple,
    #[serde(rename = "AUTHORIZATION_MODEL_NOT_FOUND")]
    AuthorizationModelNotFound,
    #[serde(rename = "TYPE_NOT_FOUND")]
    TypeNotFound,
    #[serde(rename = "RELATION_NOT_FOUND")]
    RelationNotFound,
    #[serde(rename = "STORE_ID_INVALID_LENGTH")]
    StoreIdInvalidLength,
    #[serde(rename = "ID_TOO_LONG")]
    IdTooLong,
    #[serde(rename = "INVALID_CONTINUATION_TOKEN")]
    InvalidContinuationToken,
    #[serde(rename = "PAGE_SIZE_INVALID")]
    PageSizeInvalid,
    #[serde(rename = "PARAM_MISSING_VALUE")]
    ParamMissingValue,
    #[serde(rename = "EXCEEDED_ENTITY_LIMIT")]
    ExceededEntityLimit,
    #[serde(rename = "DUPLICATE_ENTRY")]
    DuplicateEntry,
    #[serde(rename = "CANNOT_ALLOW_DUPLICATE_TUPLES")]
    CannotAllowDuplicateTuples,
    #[serde(rename = "CANNOT_ALLOW_DUPLICATE_TYPES")]
    CannotAllowDuplicateTypes,
    #[serde(rename = "INVALID_SCHEMA_VERSION")]
    InvalidSchemaVersion,
    #[serde(rename = "INVALID_TYPE_PATTERN")]
    InvalidTypePattern,
    #[serde(rename = "INVALID_RELATION_PATTERN")]
    InvalidRelationPattern,
    #[serde(rename = "INVALID_OBJECT_PATTERN")]
    InvalidObjectPattern,
    #[serde(rename = "INVALID_FORMAT")]
    InvalidFormat,
    #[serde(rename = "INVALID_EMAIL_FORMAT")]
    InvalidEmailFormat,
    #[serde(rename = "INVALID_PASSWORD_STRENGTH")]
    InvalidPasswordStrength,
    #[serde(rename = "FIELD_TOO_SHORT")]
    FieldTooShort,
    #[serde(rename = "FIELD_TOO_LONG")]
    FieldTooLong,
    #[serde(rename = "UNSUPPORTED_MEDIA_TYPE")]
    UnsupportedMediaType,
}

impl std::fmt::Display for ValidationErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ValidationErrorCode::NoError => write!(f, "NO_ERROR"),
            ValidationErrorCode::ValidationError => write!(f, "VALIDATION_ERROR"),
            ValidationErrorCode::InvalidWriteInput => write!(f, "INVALID_WRITE_INPUT"),
            ValidationErrorCode::InvalidCheckInput => write!(f, "INVALID_CHECK_INPUT"),
            ValidationErrorCode::InvalidExpandInput => write!(f, "INVALID_EXPAND_INPUT"),
            ValidationErrorCode::InvalidObjectFormat => write!(f, "INVALID_OBJECT_FORMAT"),
            ValidationErrorCode::InvalidUser => write!(f, "INVALID_USER"),
            ValidationErrorCode::InvalidTuple => write!(f, "INVALID_TUPLE"),
            ValidationErrorCode::AuthorizationModelNotFound => write!(f, "AUTHORIZATION_MODEL_NOT_FOUND"),
            ValidationErrorCode::TypeNotFound => write!(f, "TYPE_NOT_FOUND"),
            ValidationErrorCode::RelationNotFound => write!(f, "RELATION_NOT_FOUND"),
            ValidationErrorCode::StoreIdInvalidLength => write!(f, "STORE_ID_INVALID_LENGTH"),
            ValidationErrorCode::IdTooLong => write!(f, "ID_TOO_LONG"),
            ValidationErrorCode::InvalidContinuationToken => write!(f, "INVALID_CONTINUATION_TOKEN"),
            ValidationErrorCode::PageSizeInvalid => write!(f, "PAGE_SIZE_INVALID"),
            ValidationErrorCode::ParamMissingValue => write!(f, "PARAM_MISSING_VALUE"),
            ValidationErrorCode::ExceededEntityLimit => write!(f, "EXCEEDED_ENTITY_LIMIT"),
            ValidationErrorCode::DuplicateEntry => write!(f, "DUPLICATE_ENTRY"),
            ValidationErrorCode::CannotAllowDuplicateTuples => write!(f, "CANNOT_ALLOW_DUPLICATE_TUPLES"),
            ValidationErrorCode::CannotAllowDuplicateTypes => write!(f, "CANNOT_ALLOW_DUPLICATE_TYPES"),
            ValidationErrorCode::InvalidSchemaVersion => write!(f, "INVALID_SCHEMA_VERSION"),
            ValidationErrorCode::InvalidTypePattern => write!(f, "INVALID_TYPE_PATTERN"),
            ValidationErrorCode::InvalidRelationPattern => write!(f, "INVALID_RELATION_PATTERN"),
            ValidationErrorCode::InvalidObjectPattern => write!(f, "INVALID_OBJECT_PATTERN"),
            ValidationErrorCode::InvalidFormat => write!(f, "INVALID_FORMAT"),
            ValidationErrorCode::InvalidEmailFormat => write!(f, "INVALID_EMAIL_FORMAT"),
            ValidationErrorCode::InvalidPasswordStrength => write!(f, "INVALID_PASSWORD_STRENGTH"),
            ValidationErrorCode::FieldTooShort => write!(f, "FIELD_TOO_SHORT"),
            ValidationErrorCode::FieldTooLong => write!(f, "FIELD_TOO_LONG"),
            ValidationErrorCode::UnsupportedMediaType => write!(f, "UNSUPPORTED_MEDIA_TYPE"),
        }
    }
}

impl std::str::FromStr for ValidationErrorCode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "NO_ERROR" => std::result::Result::Ok(ValidationErrorCode::NoError),
            "VALIDATION_ERROR" => std::result::Result::Ok(ValidationErrorCode::ValidationError),
            "INVALID_WRITE_INPUT" => std::result::Result::Ok(ValidationErrorCode::InvalidWriteInput),
            "INVALID_CHECK_INPUT" => std::result::Result::Ok(ValidationErrorCode::InvalidCheckInput),
            "INVALID_EXPAND_INPUT" => std::result::Result::Ok(ValidationErrorCode::InvalidExpandInput),
            "INVALID_OBJECT_FORMAT" => std::result::Result::Ok(ValidationErrorCode::InvalidObjectFormat),
            "INVALID_USER" => std::result::Result::Ok(ValidationErrorCode::InvalidUser),
            "INVALID_TUPLE" => std::result::Result::Ok(ValidationErrorCode::InvalidTuple),
            "AUTHORIZATION_MODEL_NOT_FOUND" => std::result::Result::Ok(ValidationErrorCode::AuthorizationModelNotFound),
            "TYPE_NOT_FOUND" => std::result::Result::Ok(ValidationErrorCode::TypeNotFound),
            "RELATION_NOT_FOUND" => std::result::Result::Ok(ValidationErrorCode::RelationNotFound),
            "STORE_ID_INVALID_LENGTH" => std::result::Result::Ok(ValidationErrorCode::StoreIdInvalidLength),
            "ID_TOO_LONG" => std::result::Result::Ok(ValidationErrorCode::IdTooLong),
            "INVALID_CONTINUATION_TOKEN" => std::result::Result::Ok(ValidationErrorCode::InvalidContinuationToken),
            "PAGE_SIZE_INVALID" => std::result::Result::Ok(ValidationErrorCode::PageSizeInvalid),
            "PARAM_MISSING_VALUE" => std::result::Result::Ok(ValidationErrorCode::ParamMissingValue),
            "EXCEEDED_ENTITY_LIMIT" => std::result::Result::Ok(ValidationErrorCode::ExceededEntityLimit),
            "DUPLICATE_ENTRY" => std::result::Result::Ok(ValidationErrorCode::DuplicateEntry),
            "CANNOT_ALLOW_DUPLICATE_TUPLES" => std::result::Result::Ok(ValidationErrorCode::CannotAllowDuplicateTuples),
            "CANNOT_ALLOW_DUPLICATE_TYPES" => std::result::Result::Ok(ValidationErrorCode::CannotAllowDuplicateTypes),
            "INVALID_SCHEMA_VERSION" => std::result::Result::Ok(ValidationErrorCode::InvalidSchemaVersion),
            "INVALID_TYPE_PATTERN" => std::result::Result::Ok(ValidationErrorCode::InvalidTypePattern),
            "INVALID_RELATION_PATTERN" => std::result::Result::Ok(ValidationErrorCode::InvalidRelationPattern),
            "INVALID_OBJECT_PATTERN" => std::result::Result::Ok(ValidationErrorCode::InvalidObjectPattern),
            "INVALID_FORMAT" => std::result::Result::Ok(ValidationErrorCode::InvalidFormat),
            "INVALID_EMAIL_FORMAT" => std::result::Result::Ok(ValidationErrorCode::InvalidEmailFormat),
            "INVALID_PASSWORD_STRENGTH" => std::result::Result::Ok(ValidationErrorCode::InvalidPasswordStrength),
            "FIELD_TOO_SHORT" => std::result::Result::Ok(ValidationErrorCode::FieldTooShort),
            "FIELD_TOO_LONG" => std::result::Result::Ok(ValidationErrorCode::FieldTooLong),
            "UNSUPPORTED_MEDIA_TYPE" => std::result::Result::Ok(ValidationErrorCode::UnsupportedMediaType),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// Represents data validation and business logic errors
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ValidationErrorMessageResponse {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<models::ValidationErrorCode>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "fieldViolations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub field_violations: Option<Vec<models::FieldViolation>>,

    #[serde(rename = "schemaValidation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_validation: Option<models::SchemaValidation>,

    #[serde(rename = "resourceValidation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_validation: Option<models::ResourceValidation>,

    #[serde(rename = "errorResponse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_response: Option<models::ErrorResponse>,

}


impl ValidationErrorMessageResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ValidationErrorMessageResponse {
        ValidationErrorMessageResponse {
            code: None,
            message: None,
            field_violations: None,
            schema_validation: None,
            resource_validation: None,
            error_response: None,
        }
    }
}

/// Converts the ValidationErrorMessageResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ValidationErrorMessageResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping code in query parameter serialization


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),

            // Skipping fieldViolations in query parameter serialization

            // Skipping schemaValidation in query parameter serialization

            // Skipping resourceValidation in query parameter serialization

            // Skipping errorResponse in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ValidationErrorMessageResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ValidationErrorMessageResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<models::ValidationErrorCode>,
            pub message: Vec<String>,
            pub field_violations: Vec<Vec<models::FieldViolation>>,
            pub schema_validation: Vec<models::SchemaValidation>,
            pub resource_validation: Vec<models::ResourceValidation>,
            pub error_response: Vec<models::ErrorResponse>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ValidationErrorMessageResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(<models::ValidationErrorCode as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "fieldViolations" => return std::result::Result::Err("Parsing a container in this style is not supported in ValidationErrorMessageResponse".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "schemaValidation" => intermediate_rep.schema_validation.push(<models::SchemaValidation as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "resourceValidation" => intermediate_rep.resource_validation.push(<models::ResourceValidation as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "errorResponse" => intermediate_rep.error_response.push(<models::ErrorResponse as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ValidationErrorMessageResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ValidationErrorMessageResponse {
            code: intermediate_rep.code.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
            field_violations: intermediate_rep.field_violations.into_iter().next(),
            schema_validation: intermediate_rep.schema_validation.into_iter().next(),
            resource_validation: intermediate_rep.resource_validation.into_iter().next(),
            error_response: intermediate_rep.error_response.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ValidationErrorMessageResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ValidationErrorMessageResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ValidationErrorMessageResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ValidationErrorMessageResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ValidationErrorMessageResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ValidationErrorMessageResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ValidationErrorMessageResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// WorkflowStatus represents the current state of a scraping workflow. This enum tracks the lifecycle of automated, recurring scraping configurations.  State transitions: 1. DRAFT -> ACTIVE/ARCHIVED 2. ACTIVE -> PAUSED/FAILED/COMPLETED/ARCHIVED 3. PAUSED -> ACTIVE/ARCHIVED 4. FAILED -> ACTIVE/ARCHIVED   - WORKFLOW_STATUS_UNSPECIFIED: Default state, should not be used explicitly  - WORKFLOW_STATUS_DRAFT: Initial draft state before activation  - WORKFLOW_STATUS_ACTIVE: Workflow is actively running on schedule  - WORKFLOW_STATUS_PAUSED: Workflow is temporarily suspended  - WORKFLOW_STATUS_FAILED: Workflow encountered an error  - WORKFLOW_STATUS_COMPLETED: Workflow completed all scheduled runs  - WORKFLOW_STATUS_ARCHIVED: Workflow is archived (soft-deleted)  - WORKFLOW_STATUS_PENDING_APPROVAL: Workflow is pending approval  - WORKFLOW_STATUS_VALIDATING: Workflow is being validated  - WORKFLOW_STATUS_QUOTA_EXCEEDED: Workflow exceeded resource quotas  - WORKFLOW_STATUS_WARNING: Workflow is running but with warnings
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum WorkflowStatus {
    #[serde(rename = "WORKFLOW_STATUS_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "WORKFLOW_STATUS_DRAFT")]
    Draft,
    #[serde(rename = "WORKFLOW_STATUS_ACTIVE")]
    Active,
    #[serde(rename = "WORKFLOW_STATUS_PAUSED")]
    Paused,
    #[serde(rename = "WORKFLOW_STATUS_FAILED")]
    Failed,
    #[serde(rename = "WORKFLOW_STATUS_COMPLETED")]
    Completed,
    #[serde(rename = "WORKFLOW_STATUS_ARCHIVED")]
    Archived,
    #[serde(rename = "WORKFLOW_STATUS_PENDING_APPROVAL")]
    PendingApproval,
    #[serde(rename = "WORKFLOW_STATUS_VALIDATING")]
    Validating,
    #[serde(rename = "WORKFLOW_STATUS_QUOTA_EXCEEDED")]
    QuotaExceeded,
    #[serde(rename = "WORKFLOW_STATUS_WARNING")]
    Warning,
}

impl std::fmt::Display for WorkflowStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            WorkflowStatus::Unspecified => write!(f, "WORKFLOW_STATUS_UNSPECIFIED"),
            WorkflowStatus::Draft => write!(f, "WORKFLOW_STATUS_DRAFT"),
            WorkflowStatus::Active => write!(f, "WORKFLOW_STATUS_ACTIVE"),
            WorkflowStatus::Paused => write!(f, "WORKFLOW_STATUS_PAUSED"),
            WorkflowStatus::Failed => write!(f, "WORKFLOW_STATUS_FAILED"),
            WorkflowStatus::Completed => write!(f, "WORKFLOW_STATUS_COMPLETED"),
            WorkflowStatus::Archived => write!(f, "WORKFLOW_STATUS_ARCHIVED"),
            WorkflowStatus::PendingApproval => write!(f, "WORKFLOW_STATUS_PENDING_APPROVAL"),
            WorkflowStatus::Validating => write!(f, "WORKFLOW_STATUS_VALIDATING"),
            WorkflowStatus::QuotaExceeded => write!(f, "WORKFLOW_STATUS_QUOTA_EXCEEDED"),
            WorkflowStatus::Warning => write!(f, "WORKFLOW_STATUS_WARNING"),
        }
    }
}

impl std::str::FromStr for WorkflowStatus {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "WORKFLOW_STATUS_UNSPECIFIED" => std::result::Result::Ok(WorkflowStatus::Unspecified),
            "WORKFLOW_STATUS_DRAFT" => std::result::Result::Ok(WorkflowStatus::Draft),
            "WORKFLOW_STATUS_ACTIVE" => std::result::Result::Ok(WorkflowStatus::Active),
            "WORKFLOW_STATUS_PAUSED" => std::result::Result::Ok(WorkflowStatus::Paused),
            "WORKFLOW_STATUS_FAILED" => std::result::Result::Ok(WorkflowStatus::Failed),
            "WORKFLOW_STATUS_COMPLETED" => std::result::Result::Ok(WorkflowStatus::Completed),
            "WORKFLOW_STATUS_ARCHIVED" => std::result::Result::Ok(WorkflowStatus::Archived),
            "WORKFLOW_STATUS_PENDING_APPROVAL" => std::result::Result::Ok(WorkflowStatus::PendingApproval),
            "WORKFLOW_STATUS_VALIDATING" => std::result::Result::Ok(WorkflowStatus::Validating),
            "WORKFLOW_STATUS_QUOTA_EXCEEDED" => std::result::Result::Ok(WorkflowStatus::QuotaExceeded),
            "WORKFLOW_STATUS_WARNING" => std::result::Result::Ok(WorkflowStatus::Warning),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Workspace {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "industry")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub industry: Option<String>,

    #[serde(rename = "domain")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub domain: Option<String>,

    #[serde(rename = "gdprCompliant")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gdpr_compliant: Option<bool>,

    #[serde(rename = "hipaaCompliant")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hipaa_compliant: Option<bool>,

    #[serde(rename = "soc2Compliant")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub soc2_compliant: Option<bool>,

    /// in bytes
    #[serde(rename = "storageQuota")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub storage_quota: Option<String>,

    #[serde(rename = "usedStorage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub used_storage: Option<String>,

    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "workflows")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workflows: Option<Vec<models::ScrapingWorkflow>>,

    #[serde(rename = "jobsRunThisMonth")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub jobs_run_this_month: Option<i32>,

    #[serde(rename = "workspaceJobLimit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workspace_job_limit: Option<i32>,

    #[serde(rename = "dailyJobQuota")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub daily_job_quota: Option<i32>,

    #[serde(rename = "activeScrapers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active_scrapers: Option<i32>,

    #[serde(rename = "totalLeadsCollected")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_leads_collected: Option<i32>,

    #[serde(rename = "lastJobRun")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_job_run: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "scrapingJobs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scraping_jobs: Option<Vec<models::ScrapingJob>>,

}


impl Workspace {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Workspace {
        Workspace {
            id: None,
            name: None,
            industry: None,
            domain: None,
            gdpr_compliant: None,
            hipaa_compliant: None,
            soc2_compliant: None,
            storage_quota: None,
            used_storage: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
            workflows: None,
            jobs_run_this_month: None,
            workspace_job_limit: None,
            daily_job_quota: None,
            active_scrapers: None,
            total_leads_collected: None,
            last_job_run: None,
            scraping_jobs: None,
        }
    }
}

/// Converts the Workspace value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Workspace {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.id.as_ref().map(|id| {
                [
                    "id".to_string(),
                    id.to_string(),
                ].join(",")
            }),


            self.name.as_ref().map(|name| {
                [
                    "name".to_string(),
                    name.to_string(),
                ].join(",")
            }),


            self.industry.as_ref().map(|industry| {
                [
                    "industry".to_string(),
                    industry.to_string(),
                ].join(",")
            }),


            self.domain.as_ref().map(|domain| {
                [
                    "domain".to_string(),
                    domain.to_string(),
                ].join(",")
            }),


            self.gdpr_compliant.as_ref().map(|gdpr_compliant| {
                [
                    "gdprCompliant".to_string(),
                    gdpr_compliant.to_string(),
                ].join(",")
            }),


            self.hipaa_compliant.as_ref().map(|hipaa_compliant| {
                [
                    "hipaaCompliant".to_string(),
                    hipaa_compliant.to_string(),
                ].join(",")
            }),


            self.soc2_compliant.as_ref().map(|soc2_compliant| {
                [
                    "soc2Compliant".to_string(),
                    soc2_compliant.to_string(),
                ].join(",")
            }),


            self.storage_quota.as_ref().map(|storage_quota| {
                [
                    "storageQuota".to_string(),
                    storage_quota.to_string(),
                ].join(",")
            }),


            self.used_storage.as_ref().map(|used_storage| {
                [
                    "usedStorage".to_string(),
                    used_storage.to_string(),
                ].join(",")
            }),

            // Skipping createdAt in query parameter serialization

            // Skipping updatedAt in query parameter serialization

            // Skipping deletedAt in query parameter serialization

            // Skipping workflows in query parameter serialization


            self.jobs_run_this_month.as_ref().map(|jobs_run_this_month| {
                [
                    "jobsRunThisMonth".to_string(),
                    jobs_run_this_month.to_string(),
                ].join(",")
            }),


            self.workspace_job_limit.as_ref().map(|workspace_job_limit| {
                [
                    "workspaceJobLimit".to_string(),
                    workspace_job_limit.to_string(),
                ].join(",")
            }),


            self.daily_job_quota.as_ref().map(|daily_job_quota| {
                [
                    "dailyJobQuota".to_string(),
                    daily_job_quota.to_string(),
                ].join(",")
            }),


            self.active_scrapers.as_ref().map(|active_scrapers| {
                [
                    "activeScrapers".to_string(),
                    active_scrapers.to_string(),
                ].join(",")
            }),


            self.total_leads_collected.as_ref().map(|total_leads_collected| {
                [
                    "totalLeadsCollected".to_string(),
                    total_leads_collected.to_string(),
                ].join(",")
            }),

            // Skipping lastJobRun in query parameter serialization

            // Skipping scrapingJobs in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Workspace value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Workspace {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub name: Vec<String>,
            pub industry: Vec<String>,
            pub domain: Vec<String>,
            pub gdpr_compliant: Vec<bool>,
            pub hipaa_compliant: Vec<bool>,
            pub soc2_compliant: Vec<bool>,
            pub storage_quota: Vec<String>,
            pub used_storage: Vec<String>,
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub updated_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub deleted_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub workflows: Vec<Vec<models::ScrapingWorkflow>>,
            pub jobs_run_this_month: Vec<i32>,
            pub workspace_job_limit: Vec<i32>,
            pub daily_job_quota: Vec<i32>,
            pub active_scrapers: Vec<i32>,
            pub total_leads_collected: Vec<i32>,
            pub last_job_run: Vec<chrono::DateTime::<chrono::Utc>>,
            pub scraping_jobs: Vec<Vec<models::ScrapingJob>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Workspace".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "industry" => intermediate_rep.industry.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "domain" => intermediate_rep.domain.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "gdprCompliant" => intermediate_rep.gdpr_compliant.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "hipaaCompliant" => intermediate_rep.hipaa_compliant.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "soc2Compliant" => intermediate_rep.soc2_compliant.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "storageQuota" => intermediate_rep.storage_quota.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "usedStorage" => intermediate_rep.used_storage.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "createdAt" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "updatedAt" => intermediate_rep.updated_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "deletedAt" => intermediate_rep.deleted_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "workflows" => return std::result::Result::Err("Parsing a container in this style is not supported in Workspace".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "jobsRunThisMonth" => intermediate_rep.jobs_run_this_month.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "workspaceJobLimit" => intermediate_rep.workspace_job_limit.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "dailyJobQuota" => intermediate_rep.daily_job_quota.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "activeScrapers" => intermediate_rep.active_scrapers.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "totalLeadsCollected" => intermediate_rep.total_leads_collected.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "lastJobRun" => intermediate_rep.last_job_run.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "scrapingJobs" => return std::result::Result::Err("Parsing a container in this style is not supported in Workspace".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing Workspace".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Workspace {
            id: intermediate_rep.id.into_iter().next(),
            name: intermediate_rep.name.into_iter().next(),
            industry: intermediate_rep.industry.into_iter().next(),
            domain: intermediate_rep.domain.into_iter().next(),
            gdpr_compliant: intermediate_rep.gdpr_compliant.into_iter().next(),
            hipaa_compliant: intermediate_rep.hipaa_compliant.into_iter().next(),
            soc2_compliant: intermediate_rep.soc2_compliant.into_iter().next(),
            storage_quota: intermediate_rep.storage_quota.into_iter().next(),
            used_storage: intermediate_rep.used_storage.into_iter().next(),
            created_at: intermediate_rep.created_at.into_iter().next(),
            updated_at: intermediate_rep.updated_at.into_iter().next(),
            deleted_at: intermediate_rep.deleted_at.into_iter().next(),
            workflows: intermediate_rep.workflows.into_iter().next(),
            jobs_run_this_month: intermediate_rep.jobs_run_this_month.into_iter().next(),
            workspace_job_limit: intermediate_rep.workspace_job_limit.into_iter().next(),
            daily_job_quota: intermediate_rep.daily_job_quota.into_iter().next(),
            active_scrapers: intermediate_rep.active_scrapers.into_iter().next(),
            total_leads_collected: intermediate_rep.total_leads_collected.into_iter().next(),
            last_job_run: intermediate_rep.last_job_run.into_iter().next(),
            scraping_jobs: intermediate_rep.scraping_jobs.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Workspace> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Workspace>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Workspace>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Workspace - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Workspace> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Workspace as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Workspace - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

