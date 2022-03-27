//! [The OAuth 2.0 Authorization Framework: Bearer Token Usage](https://www.rfc-editor.org/rfc/rfc6750.html)

//
pub mod credentials;

pub use credentials::{Credentials, CredentialsParseError};
#[cfg(test)]
pub(crate) use credentials::{DEMO_CREDENTIALS_STR, DEMO_CREDENTIALS_TOKEN_STR};
