//! [The 'Basic' HTTP Authentication Scheme](https://www.rfc-editor.org/rfc/rfc7617.html)

//
pub mod credentials;

pub use credentials::{Credentials, CredentialsParseError};
#[cfg(test)]
pub(crate) use credentials::{
    DEMO_CREDENTIALS_PASSWORD_STR, DEMO_CREDENTIALS_STR, DEMO_CREDENTIALS_USER_ID_STR,
};

//
pub mod challenge;

pub use challenge::{Challenge, ChallengeParseError};
#[cfg(test)]
pub(crate) use challenge::{
    DEMO_CHALLENGE_CHARSET_STR, DEMO_CHALLENGE_REALM_STR, DEMO_CHALLENGE_STR,
    DEMO_CHALLENGE_STR_SIMPLE,
};
