pub const USER_PRINCIPAL_ID_STORE: &str = "user-principal-id";
pub const REFERRER_STORE: &str = "referrer";
pub const ACCOUNT_CONNECTED_STORE: &str = "account-connected-1";
#[cfg(any(feature = "local-bin", feature = "local-lib"))]
pub mod local;
#[cfg(any(feature = "local-bin", feature = "local-lib"))]
pub use local::*;

#[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
mod remote;
#[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
pub use remote::*;

pub mod auth {
    use web_time::Duration;

    /// Delegation Expiry, 7 days
    pub const DELEGATION_MAX_AGE: Duration = Duration::from_secs(60 * 60 * 24 * 7);
    /// Refresh expiry, 30 days
    pub const REFRESH_MAX_AGE: Duration = Duration::from_secs(60 * 60 * 24 * 30);
    pub const REFRESH_TOKEN_COOKIE: &str = "user-identity";
}
