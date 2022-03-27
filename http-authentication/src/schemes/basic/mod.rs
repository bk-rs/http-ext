//! [The 'Basic' HTTP Authentication Scheme](https://www.rfc-editor.org/rfc/rfc7617.html)

//
pub mod credentials;

pub use credentials::{Credentials, CredentialsParseError};
#[cfg(test)]
pub(crate) use credentials::{
    DEMO_CREDENTIALS_PASSWORD_STR, DEMO_CREDENTIALS_STR, DEMO_CREDENTIALS_USER_ID_STR,
};
