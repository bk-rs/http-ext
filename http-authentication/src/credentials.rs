use alloc::vec::Vec;
use core::{fmt, str::FromStr};

use crate::schemes::{NAME_BASIC, NAME_BEARER, NAME_DIGEST, SP};

//
#[derive(Debug, Clone)]
pub enum Credentials {
    #[cfg(feature = "scheme-basic")]
    Basic(crate::schemes::basic::Credentials),
    #[cfg(feature = "scheme-bearer")]
    Bearer(crate::schemes::bearer::Credentials),
}

impl Credentials {
    #[cfg(feature = "scheme-basic")]
    pub fn basic(user_id: impl AsRef<str>, password: impl AsRef<str>) -> Self {
        Self::Basic(crate::schemes::basic::Credentials::new(user_id, password))
    }

    #[cfg(feature = "scheme-bearer")]
    pub fn bearer(token: impl AsRef<str>) -> Self {
        Self::Bearer(crate::schemes::bearer::Credentials::new(token))
    }

    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, CredentialsParseError> {
        let bytes = bytes.as_ref();

        let scheme = bytes
            .iter()
            .take_while(|x| **x != SP as u8)
            .cloned()
            .collect::<Vec<_>>();
        match scheme {
            x if x == NAME_BASIC.as_bytes() => {
                #[cfg(feature = "scheme-basic")]
                {
                    crate::schemes::basic::Credentials::from_bytes(bytes)
                        .map(Self::Basic)
                        .map_err(CredentialsParseError::Basic)
                }
                #[cfg(not(feature = "scheme-basic"))]
                {
                    Err(CredentialsParseError::SchemeUnsupported(
                        "Require feature scheme-basic",
                    ))
                }
            }
            x if x == NAME_BEARER.as_bytes() => {
                #[cfg(feature = "scheme-bearer")]
                {
                    crate::schemes::bearer::Credentials::from_bytes(bytes)
                        .map(Self::Bearer)
                        .map_err(CredentialsParseError::Bearer)
                }
                #[cfg(not(feature = "scheme-bearer"))]
                {
                    Err(CredentialsParseError::SchemeUnsupported(
                        "Require feature scheme-bearer",
                    ))
                }
            }
            x if x == NAME_DIGEST.as_bytes() => {
                Err(CredentialsParseError::SchemeUnsupported("Unimplemented"))
            }
            _ => Err(CredentialsParseError::SchemeUnknown),
        }
    }
}

//
#[derive(Debug)]
pub enum CredentialsParseError {
    #[cfg(feature = "scheme-basic")]
    Basic(crate::schemes::basic::CredentialsParseError),
    #[cfg(feature = "scheme-bearer")]
    Bearer(crate::schemes::bearer::CredentialsParseError),
    SchemeUnknown,
    SchemeUnsupported(&'static str),
}

impl fmt::Display for CredentialsParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CredentialsParseError {}

//
#[allow(unused_variables)]
impl fmt::Display for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "scheme-basic")]
            Self::Basic(c) => c.fmt(f),
            #[cfg(feature = "scheme-bearer")]
            Self::Bearer(c) => c.fmt(f),
            #[allow(unreachable_patterns)]
            _ => unimplemented!(),
        }
    }
}

//
impl FromStr for Credentials {
    type Err = CredentialsParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use alloc::string::ToString as _;

    #[test]
    fn test_credentials_parse_and_render() {
        //
        #[cfg(feature = "scheme-basic")]
        {
            use crate::schemes::basic::{
                DEMO_CREDENTIALS_PASSWORD_STR, DEMO_CREDENTIALS_STR, DEMO_CREDENTIALS_USER_ID_STR,
            };

            match DEMO_CREDENTIALS_STR.parse::<Credentials>() {
                Ok(Credentials::Basic(c)) => {
                    assert_eq!(c.user_id, DEMO_CREDENTIALS_USER_ID_STR.into());
                    assert_eq!(c.password, DEMO_CREDENTIALS_PASSWORD_STR.into());
                    assert_eq!(c.to_string(), DEMO_CREDENTIALS_STR);
                }
                x => panic!("{:?}", x),
            }
        }
        #[cfg(not(feature = "scheme-basic"))]
        {
            match "Basic bar".parse::<Credentials>() {
                Err(CredentialsParseError::SchemeUnsupported(_)) => {}
                x => panic!("{:?}", x),
            }
        }

        //
        #[cfg(feature = "scheme-bearer")]
        {
            use crate::schemes::bearer::{DEMO_CREDENTIALS_STR, DEMO_CREDENTIALS_TOKEN_STR};

            match DEMO_CREDENTIALS_STR.parse::<Credentials>() {
                Ok(Credentials::Bearer(c)) => {
                    assert_eq!(c.token, DEMO_CREDENTIALS_TOKEN_STR.into());
                    assert_eq!(c.to_string(), DEMO_CREDENTIALS_STR);
                }
                x => panic!("{:?}", x),
            }
        }
        #[cfg(not(feature = "scheme-bearer"))]
        {
            match "Bearer bar".parse::<Credentials>() {
                Err(CredentialsParseError::SchemeUnsupported(_)) => {}
                x => panic!("{:?}", x),
            }
        }

        //
        match Credentials::from_str("") {
            Err(CredentialsParseError::SchemeUnknown) => {}
            x => panic!("{:?}", x),
        }

        match Credentials::from_str("Foo bar") {
            Err(CredentialsParseError::SchemeUnknown) => {}
            x => panic!("{:?}", x),
        }
    }
}
