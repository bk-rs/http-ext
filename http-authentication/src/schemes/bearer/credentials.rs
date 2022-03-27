use alloc::boxed::Box;
use core::{
    fmt,
    str::{self, FromStr},
};

use crate::{schemes::NAME_BEARER as NAME, SP_STR};

//
#[derive(Debug, Clone)]
pub struct Credentials {
    pub token: Box<str>,
}

impl Credentials {
    pub fn new(token: impl AsRef<str>) -> Self {
        Self {
            token: token.as_ref().into(),
        }
    }

    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, CredentialsParseError> {
        let bytes = bytes.as_ref();

        if bytes.len() < NAME.len() + 1 {
            return Err(CredentialsParseError::Other("too short"));
        }

        if !&bytes[..NAME.len()].eq_ignore_ascii_case(NAME.as_bytes()) {
            return Err(CredentialsParseError::SchemeMismatch);
        }

        if &bytes[NAME.len()..NAME.len() + 1] != SP_STR.as_bytes() {
            return Err(CredentialsParseError::OneSPMismatch);
        }

        let token68_bytes = &bytes[NAME.len() + 1..];

        let token = token68_bytes;
        let token = str::from_utf8(token).map_err(CredentialsParseError::TokenToStrFailed)?;

        Ok(Self::new(token))
    }
}

//
#[derive(Debug)]
pub enum CredentialsParseError {
    SchemeMismatch,
    OneSPMismatch,
    TokenToStrFailed(str::Utf8Error),
    Other(&'static str),
}

impl fmt::Display for CredentialsParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CredentialsParseError {}

//
impl fmt::Display for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", NAME, SP_STR, self.token)
    }
}

//
impl FromStr for Credentials {
    type Err = CredentialsParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes())
    }
}

//
//
//
#[cfg(test)]
pub(crate) const DEMO_CREDENTIALS_STR: &str = "Bearer mF_9.B5f-4.1JqM";
#[cfg(test)]
pub(crate) const DEMO_CREDENTIALS_TOKEN_STR: &str = "mF_9.B5f-4.1JqM";

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::string::ToString as _;

    #[test]
    fn test_credentials_parse_and_render() {
        let c = DEMO_CREDENTIALS_STR.parse::<Credentials>().unwrap();
        assert_eq!(c.token, DEMO_CREDENTIALS_TOKEN_STR.into());
        assert_eq!(c.to_string(), DEMO_CREDENTIALS_STR);

        //
        match Credentials::from_str("Bearer") {
            Err(CredentialsParseError::Other(err)) => {
                assert_eq!(err, "too short")
            }
            x => panic!("{:?}", x),
        }

        match Credentials::from_str("MyScheme ") {
            Err(CredentialsParseError::SchemeMismatch) => {}
            x => panic!("{:?}", x),
        }

        match Credentials::from_str("Bearer-") {
            Err(CredentialsParseError::OneSPMismatch) => {}
            x => panic!("{:?}", x),
        }
    }
}
