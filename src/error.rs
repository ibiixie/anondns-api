use reqwest;
use std::fmt;

// Shorthand
pub type Error = DnsApiError;

#[derive(Debug)]
pub enum DnsApiError {
    BadRequest((i32, String)),
    UnknownErrorCode((i32, String)), // On the off-chance that anondns actually does use error codes other than 0 and 1...
    Reqwest(reqwest::Error),
    AddressParse(std::net::AddrParseError)
}

impl fmt::Display for DnsApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DnsApiError::BadRequest((code, msg)) => write!(f, "anondns server received a bad request (code {}): {}", code, msg),
            DnsApiError::UnknownErrorCode((code, msg)) => write!(f, "anondns server responded with an unknown error (code {}): {}", code, msg),
            DnsApiError::Reqwest(err) => write!(f, "error in library 'reqwest': {}", err.to_string()),
            DnsApiError::AddressParse(err) => write!(f, "error when parsing address: {}", err.to_string())
        }
    }
}

impl From<reqwest::Error> for DnsApiError {
    fn from(err: reqwest::Error) -> DnsApiError {
        DnsApiError::Reqwest(err)
    }
}

impl From<std::net::AddrParseError> for DnsApiError {
    fn from(err: std::net::AddrParseError) -> DnsApiError {
        DnsApiError::AddressParse(err)
    }
}