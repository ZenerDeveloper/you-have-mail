use crate::{Account, ConfigGenError, ObserverAccount, ObserverError, Proxy};
use crossbeam_channel::Sender;
use std::time::Duration;

/// RPC Requests for the `Observer`.
pub enum ObserverRequest {
    Exit,
    AddAccount(Account, Sender<Result<(), ObserverError>>),
    LogoutAccount(String, Sender<Result<(), ObserverError>>),
    RemoveAccount(String, Sender<Result<(), ObserverError>>),
    GetAccounts(Sender<Result<Vec<ObserverAccount>, ObserverError>>),
    Pause,
    Resume,
    GenConfig(Sender<Result<String, ConfigGenError>>),
    SetPollInterval(Duration),
    GetPollInterval(Sender<Result<Duration, ()>>),
    ApplyProxy(String, Option<Proxy>, Sender<Result<(), ObserverError>>),
    GetProxy(String, Sender<Result<Option<Proxy>, ObserverError>>),
}

#[doc(hidden)]
pub trait ObserverPRC {
    type Output;
    type Error;
    type SendFailedValue;
    fn into_request(self, reply: Sender<Result<Self::Output, Self::Error>>) -> ObserverRequest;
    fn recover_send_value(request: ObserverRequest) -> Option<Self::SendFailedValue>;
}

#[doc(hidden)]
pub struct RemoveAccountRequest {
    pub email: String,
}

#[doc(hidden)]
impl ObserverPRC for RemoveAccountRequest {
    type Output = ();
    type Error = ObserverError;
    type SendFailedValue = String;

    fn into_request(self, reply: Sender<Result<Self::Output, Self::Error>>) -> ObserverRequest {
        ObserverRequest::RemoveAccount(self.email, reply)
    }

    fn recover_send_value(r: ObserverRequest) -> Option<Self::SendFailedValue> {
        match r {
            ObserverRequest::RemoveAccount(s, _) => Some(s),
            _ => None,
        }
    }
}

#[doc(hidden)]
pub struct LogoutAccountRequest {
    pub email: String,
}

#[doc(hidden)]
impl ObserverPRC for LogoutAccountRequest {
    type Output = ();
    type Error = ObserverError;
    type SendFailedValue = String;

    fn into_request(self, reply: Sender<Result<Self::Output, Self::Error>>) -> ObserverRequest {
        ObserverRequest::LogoutAccount(self.email, reply)
    }

    fn recover_send_value(r: ObserverRequest) -> Option<Self::SendFailedValue> {
        match r {
            ObserverRequest::LogoutAccount(s, _) => Some(s),
            _ => None,
        }
    }
}

#[doc(hidden)]
pub struct AddAccountRequest {
    pub account: Account,
}

impl ObserverPRC for AddAccountRequest {
    type Output = ();
    type Error = ObserverError;
    type SendFailedValue = Account;

    fn into_request(self, sender: Sender<Result<Self::Output, Self::Error>>) -> ObserverRequest {
        ObserverRequest::AddAccount(self.account, sender)
    }

    fn recover_send_value(r: ObserverRequest) -> Option<Self::SendFailedValue> {
        match r {
            ObserverRequest::AddAccount(a, _) => Some(a),
            _ => None,
        }
    }
}

#[doc(hidden)]
pub struct GetAccountListRequest {}

impl ObserverPRC for GetAccountListRequest {
    type Output = Vec<ObserverAccount>;
    type Error = ObserverError;
    type SendFailedValue = ();

    fn into_request(self, reply: Sender<Result<Self::Output, Self::Error>>) -> ObserverRequest {
        ObserverRequest::GetAccounts(reply)
    }

    fn recover_send_value(_: ObserverRequest) -> Option<Self::SendFailedValue> {
        Some(())
    }
}

#[doc(hidden)]
pub struct GenConfigRequest {}

impl ObserverPRC for GenConfigRequest {
    type Output = String;
    type Error = ConfigGenError;
    type SendFailedValue = ();

    fn into_request(self, reply: Sender<Result<Self::Output, Self::Error>>) -> ObserverRequest {
        ObserverRequest::GenConfig(reply)
    }

    fn recover_send_value(_: ObserverRequest) -> Option<Self::SendFailedValue> {
        Some(())
    }
}

#[doc(hidden)]
pub struct GetPollIntervalRequest {}

#[doc(hidden)]
impl ObserverPRC for GetPollIntervalRequest {
    type Output = Duration;
    type Error = ();
    type SendFailedValue = ();

    fn into_request(self, reply: Sender<Result<Self::Output, Self::Error>>) -> ObserverRequest {
        ObserverRequest::GetPollInterval(reply)
    }

    fn recover_send_value(_: ObserverRequest) -> Option<Self::SendFailedValue> {
        None
    }
}

#[doc(hidden)]
pub struct ApplyProxyRequest {
    pub email: String,
    pub proxy: Option<Proxy>,
}

impl ObserverPRC for ApplyProxyRequest {
    type Output = ();
    type Error = ObserverError;
    type SendFailedValue = ();

    fn into_request(self, reply: Sender<Result<Self::Output, Self::Error>>) -> ObserverRequest {
        ObserverRequest::ApplyProxy(self.email, self.proxy, reply)
    }

    fn recover_send_value(_: ObserverRequest) -> Option<Self::SendFailedValue> {
        None
    }
}

#[doc(hidden)]
pub struct GetProxyRequest {
    pub email: String,
}

#[doc(hidden)]
impl ObserverPRC for GetProxyRequest {
    type Output = Option<Proxy>;
    type Error = ObserverError;
    type SendFailedValue = ();

    fn into_request(self, reply: Sender<Result<Self::Output, Self::Error>>) -> ObserverRequest {
        ObserverRequest::GetProxy(self.email, reply)
    }

    fn recover_send_value(_: ObserverRequest) -> Option<Self::SendFailedValue> {
        None
    }
}
