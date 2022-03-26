use alloc::string::String;
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
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, CredentialsParseError> {
        todo!()
    }
}

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

impl FromStr for Credentials {
    type Err = CredentialsParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scheme = s.chars().take_while(|x| x != &SP).collect::<String>();
        match scheme.as_str() {
            NAME_BASIC => {
                #[cfg(feature = "scheme-basic")]
                {
                    crate::schemes::basic::Credentials::from_str(s)
                        .map(Self::Basic)
                        .map_err(Self::Err::Basic)
                }
                #[cfg(not(feature = "scheme-basic"))]
                {
                    Err(Self::Err::SchemeUnsupported("Require feature scheme-basic"))
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
                    Err(Self::Err::SchemeUnsupported(
                        "Require feature scheme-bearer",
                    ))
                }
            }
            NAME_DIGEST => Err(Self::Err::SchemeUnsupported("Unimplemented")),
            _ => Err(Self::Err::SchemeUnknown),
        }
    }
}

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
            let s = "Basic YWxhZGRpbjpvcGVuc2VzYW1l";
            match s.parse::<Credentials>() {
                Ok(Credentials::Basic(c)) => {
                    assert_eq!(c.user_id, "aladdin".into());
                    assert_eq!(c.password, "opensesame".into());
                    assert_eq!(c.to_string(), s);
                }
                x => panic!("{:?}", x),
            }
        }
        #[cfg(not(feature = "scheme-basic"))]
        {
            let s = "Basic YWxhZGRpbjpvcGVuc2VzYW1l";
            match s.parse::<Credentials>() {
                Err(CredentialsParseError::SchemeUnsupported(_)) => {}
                x => panic!("{:?}", x),
            }
        }

        //
        #[cfg(feature = "scheme-bearer")]
        {
            let s = "Bearer mF_9.B5f-4.1JqM";
            match s.parse::<Credentials>() {
                Ok(Credentials::Bearer(c)) => {
                    assert_eq!(c.token, "mF_9.B5f-4.1JqM".into());
                    assert_eq!(c.to_string(), s);
                }
                x => panic!("{:?}", x),
            }
        }
        #[cfg(not(feature = "scheme-bearer"))]
        {
            let s = "Bearer mF_9.B5f-4.1JqM";
            match s.parse::<Credentials>() {
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
