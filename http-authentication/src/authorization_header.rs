use core::{fmt, str::FromStr};

use crate::schemes::{NAME_BASIC, NAME_BEARER, SP};

//
#[derive(Debug, Clone)]
pub enum AuthorizationHeaderValue {
    #[cfg(feature = "scheme-basic")]
    Basic(crate::schemes::basic::Credentials),
    #[cfg(feature = "scheme-bearer")]
    Bearer(crate::schemes::bearer::Credentials),
}

impl fmt::Display for AuthorizationHeaderValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "scheme-basic")]
            Self::Basic(c) => c.fmt(f),
            #[cfg(feature = "scheme-bearer")]
            Self::Bearer(c) => c.fmt(f),
        }
    }
}

impl FromStr for AuthorizationHeaderValue {
    type Err = AuthorizationHeaderValueParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(SP);
        let scheme = split.next().ok_or(Self::Err::SchemeMismatch)?;
        match scheme {
            NAME_BASIC => {
                #[cfg(feature = "scheme-basic")]
                {
                    crate::schemes::basic::Credentials::from_str(s)
                        .map(Self::Basic)
                        .map_err(Self::Err::Basic)
                }
                #[cfg(not(feature = "scheme-basic"))]
                {
                    Err(Self::Err::SchemeUnsupported)
                }
            }
            NAME_BEARER => {
                #[cfg(feature = "scheme-bearer")]
                {
                    crate::schemes::bearer::Credentials::from_str(s)
                        .map(Self::Bearer)
                        .map_err(Self::Err::Bearer)
                }
                #[cfg(not(feature = "scheme-bearer"))]
                {
                    Err(Self::Err::SchemeUnsupported)
                }
            }
            _ => Err(Self::Err::SchemeUnknown),
        }
    }
}

#[derive(Debug)]
pub enum AuthorizationHeaderValueParseError {
    #[cfg(feature = "scheme-basic")]
    Basic(crate::schemes::basic::CredentialsParseError),
    #[cfg(feature = "scheme-bearer")]
    Bearer(crate::schemes::bearer::CredentialsParseError),
    SchemeMismatch,
    SchemeUnknown,
    SchemeUnsupported,
}
