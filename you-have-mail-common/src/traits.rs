//! Collection of Traits expected to be implemented by the respective application targets.

use crate::backend::EmailInfo;
use crate::{AccountError, Proxy};
#[cfg(test)]
use mockall::automock;

/// Notification issued to a [`Notifier`].
#[derive(Debug)]
pub enum Notification<'a> {
    /// New email has arrived for an account.
    NewEmail {
        account: &'a str,
        backend: &'a str,
        emails: &'a [EmailInfo],
    },
    /// A new account was recently added
    AccountAdded(&'a str),
    /// An account got logged out
    AccountLoggedOut(&'a str),
    /// An Account got removed
    AccountRemoved(&'a str),
    /// An Account went offline
    AccountOffline(&'a str),
    /// An Account went online
    AccountOnline(&'a str),
    /// An error occurred with an account
    AccountError(&'a str, AccountError),
    /// A proxy configuration was applied
    ProxyApplied(&'a str, Option<&'a Proxy>),
    /// New config file for refreshed accounts
    AccountsRefreshed(Vec<String>, String),
    /// Generic Error Message
    Error(String),
}

/// When an email has been received the notifier will be called.
#[cfg_attr(test, automock)]
pub trait Notifier: Send + Sync {
    /// The given account has received `email_count` new emails since the last check.
    #[allow(clippy::needless_lifetimes)] // Lifetime annotations required for automock.
    fn notify<'a>(&self, notification: Notification<'a>);
}

/// All reports as consumed and ignored.
#[derive(Copy, Clone)]
pub struct NullNotifier {}

impl Notifier for NullNotifier {
    fn notify(&self, _: Notification) {}
}
