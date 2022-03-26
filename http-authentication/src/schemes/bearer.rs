//! [The OAuth 2.0 Authorization Framework: Bearer Token Usage](https://www.rfc-editor.org/rfc/rfc6750.html)

use alloc::boxed::Box;
use core::{fmt, str::FromStr};

use crate::schemes::{NAME_BEARER as NAME, SP_STR};

//
//
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
        todo!()
    }
}

impl fmt::Display for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", NAME, SP_STR, self.token)
    }
}

impl FromStr for Credentials {
    type Err = CredentialsParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < NAME.len() + 1 {
            return Err(Self::Err::Other("too short"));
        }

        if s[..NAME.len()] != *NAME {
            return Err(Self::Err::SchemeMismatch);
        }

        if s[NAME.len()..NAME.len() + 1] != *SP_STR {
            return Err(Self::Err::OneSPMismatch);
        }

        let token = &s[NAME.len() + 1..];

        Ok(Self::new(token))
    }
}

#[derive(Debug)]
pub enum CredentialsParseError {
    SchemeMismatch,
    OneSPMismatch,
    Other(&'static str),
}

impl fmt::Display for CredentialsParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CredentialsParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::string::ToString as _;

    #[test]
    fn test_credentials_parse_and_render() {
        let s = "Bearer mF_9.B5f-4.1JqM";
        let c = s.parse::<Credentials>().unwrap();
        assert_eq!(c.token, "mF_9.B5f-4.1JqM".into());
        assert_eq!(c.to_string(), s);

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
