use alloc::{
    format,
    string::{String, ToString as _},
    vec,
    vec::Vec,
};
use core::{
    fmt,
    ops::Deref,
    str::{self, FromStr},
};

use http_auth::ChallengeParser;

use crate::{
    challenge::Challenge,
    schemes::{NAME_BASIC, NAME_BEARER, NAME_DIGEST},
    COMMA, SP,
};

//
#[derive(Debug, Clone)]
pub struct Challenges(pub Vec<Challenge>);

impl Deref for Challenges {
    type Target = Vec<Challenge>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Challenges {
    pub fn new(inner: Vec<Challenge>) -> Self {
        Self(inner)
    }

    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, ChallengesParseError> {
        let bytes = bytes.as_ref();
        let s = str::from_utf8(bytes).map_err(ChallengesParseError::ChallengesToStrFailed)?;
        Self::internal_from_str(s)
    }

    fn internal_from_str(s: impl AsRef<str>) -> Result<Self, ChallengesParseError> {
        let s = s.as_ref();

        if s.is_empty() {
            return Err(ChallengesParseError::Other("empty"));
        }

        let challenge_parser = ChallengeParser::new(s);

        #[allow(unused_mut)]
        let mut inner = vec![];
        for c in challenge_parser {
            let c = c.map_err(|err| ChallengesParseError::ChallengeParserError(err.to_string()))?;
            match c.scheme {
                x if x.eq_ignore_ascii_case(NAME_BASIC) => {
                    #[cfg(feature = "scheme-basic")]
                    {
                        let c = crate::schemes::basic::Challenge::try_from(&c)
                            .map(Challenge::Basic)
                            .map_err(ChallengesParseError::Basic)?;

                        inner.push(c)
                    }
                    #[cfg(not(feature = "scheme-basic"))]
                    {
                        return Err(ChallengesParseError::SchemeUnsupported(
                            "Require feature scheme-basic",
                        ));
                    }
                }
                x if x.eq_ignore_ascii_case(NAME_BEARER) => {
                    #[cfg(feature = "scheme-bearer")]
                    {
                        let c = crate::schemes::bearer::Challenge::try_from(&c)
                            .map(Challenge::Bearer)
                            .map_err(ChallengesParseError::Bearer)?;

                        inner.push(c)
                    }
                    #[cfg(not(feature = "scheme-bearer"))]
                    {
                        return Err(ChallengesParseError::SchemeUnsupported(
                            "Require feature scheme-bearer",
                        ));
                    }
                }
                x if x.eq_ignore_ascii_case(NAME_DIGEST) => {
                    return Err(ChallengesParseError::SchemeUnsupported("Unimplemented"))
                }
                _ => return Err(ChallengesParseError::SchemeUnknown),
            }
        }
        Ok(Self::new(inner))
    }
}

//
#[derive(Debug)]
pub enum ChallengesParseError {
    ChallengesToStrFailed(str::Utf8Error),
    ChallengeParserError(String),
    #[cfg(feature = "scheme-basic")]
    Basic(crate::schemes::basic::ChallengeParseError),
    #[cfg(feature = "scheme-bearer")]
    Bearer(crate::schemes::bearer::ChallengeParseError),
    SchemeUnknown,
    SchemeUnsupported(&'static str),
    Other(&'static str),
}

impl fmt::Display for ChallengesParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ChallengesParseError {}

impl fmt::Display for Challenges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", ChallengesWithSlice(self.0.as_ref()))
    }
}

//
impl FromStr for Challenges {
    type Err = ChallengesParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::internal_from_str(s)
    }
}

//
//
#[derive(Debug, Clone)]
pub(crate) struct ChallengesWithSlice<'a>(&'a [Challenge]);

impl<'a> ChallengesWithSlice<'a> {
    #[allow(dead_code)]
    pub(crate) fn new(inner: &'a [Challenge]) -> Self {
        Self(inner)
    }
}

//
impl fmt::Display for ChallengesWithSlice<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(format!("{COMMA}{SP}").as_str())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_render() {
        //
        #[cfg(feature = "scheme-basic")]
        {
            use crate::schemes::basic::{
                DEMO_CHALLENGE_CHARSET_STR, DEMO_CHALLENGE_REALM_STR, DEMO_CHALLENGE_STR,
                DEMO_CHALLENGE_STR_SIMPLE,
            };

            match DEMO_CHALLENGE_STR.parse::<Challenges>() {
                Ok(c) => {
                    assert_eq!(c.0.len(), 1);
                    let c = c.0.first().unwrap();
                    let c = c.as_basic().unwrap();
                    assert_eq!(c.realm, DEMO_CHALLENGE_REALM_STR.into());
                    assert_eq!(c.charset, Some(DEMO_CHALLENGE_CHARSET_STR.into()));
                    assert_eq!(c.to_string(), DEMO_CHALLENGE_STR);
                }
                x => panic!("{x:?}"),
            }

            match DEMO_CHALLENGE_STR_SIMPLE.parse::<Challenges>() {
                Ok(c) => {
                    assert_eq!(c.0.len(), 1);
                    let c = c.0.first().unwrap();
                    let c = c.as_basic().unwrap();
                    assert_eq!(c.realm, DEMO_CHALLENGE_REALM_STR.into());
                    assert_eq!(c.charset, None);
                    assert_eq!(c.to_string(), DEMO_CHALLENGE_STR_SIMPLE);
                }
                x => panic!("{x:?}"),
            }
        }
        #[cfg(not(feature = "scheme-basic"))]
        {
            match "Basic".parse::<Challenges>() {
                Err(ChallengesParseError::SchemeUnsupported(_)) => {}
                x => panic!("{x:?}"),
            }
        }

        //
        #[cfg(feature = "scheme-bearer")]
        {
            use crate::schemes::bearer::{
                DEMO_CHALLENGE_ERROR_DESCRIPTION_STR, DEMO_CHALLENGE_ERROR_STR,
                DEMO_CHALLENGE_REALM_STR, DEMO_CHALLENGE_STR, DEMO_CHALLENGE_STR_SIMPLE,
            };

            match DEMO_CHALLENGE_STR.parse::<Challenges>() {
                Ok(c) => {
                    assert_eq!(c.0.len(), 1);
                    let c = c.0.first().unwrap();
                    let c = c.as_bearer().unwrap();
                    assert_eq!(c.realm, DEMO_CHALLENGE_REALM_STR.into());
                    assert_eq!(c.scope, None);
                    assert_eq!(c.error, Some(DEMO_CHALLENGE_ERROR_STR.into()));
                    assert_eq!(
                        c.error_description,
                        Some(DEMO_CHALLENGE_ERROR_DESCRIPTION_STR.into())
                    );
                    assert_eq!(c.error_uri, None);
                    assert_eq!(c.to_string(), DEMO_CHALLENGE_STR);
                }
                x => panic!("{x:?}"),
            }

            match DEMO_CHALLENGE_STR_SIMPLE.parse::<Challenges>() {
                Ok(c) => {
                    assert_eq!(c.0.len(), 1);
                    let c = c.0.first().unwrap();
                    let c = c.as_bearer().unwrap();
                    assert_eq!(c.realm, DEMO_CHALLENGE_REALM_STR.into());
                    assert_eq!(c.scope, None);
                    assert_eq!(c.error, None);
                    assert_eq!(c.error_description, None);
                    assert_eq!(c.error_uri, None);
                    assert_eq!(c.to_string(), DEMO_CHALLENGE_STR_SIMPLE);
                }
                x => panic!("{x:?}"),
            }
        }
        #[cfg(not(feature = "scheme-bearer"))]
        {
            match "Bearer".parse::<Challenges>() {
                Err(ChallengesParseError::SchemeUnsupported(_)) => {}
                x => panic!("{x:?}"),
            }
        }

        #[cfg(all(feature = "scheme-basic", feature = "scheme-bearer"))]
        {
            use crate::schemes::{basic, bearer};

            let s = format!(
                "{}, {}",
                basic::DEMO_CHALLENGE_STR_SIMPLE,
                bearer::DEMO_CHALLENGE_STR_SIMPLE
            );

            match s.parse::<Challenges>() {
                Ok(c) => {
                    for (i, c) in c.iter().enumerate() {
                        match i {
                            0 => {
                                let c = c.as_basic().unwrap();
                                assert_eq!(c.realm, basic::DEMO_CHALLENGE_REALM_STR.into());
                            }
                            1 => {
                                let c = c.as_bearer().unwrap();
                                assert_eq!(c.realm, bearer::DEMO_CHALLENGE_REALM_STR.into());
                            }
                            i => panic!("{i} {c:?}"),
                        }
                    }

                    assert_eq!(c.to_string(), s);
                }
                x => panic!("{x:?}"),
            }
        }

        //
        match Challenges::from_str("") {
            Err(ChallengesParseError::Other(_)) => {}
            x => panic!("{x:?}"),
        }

        match Challenges::from_str("Foo") {
            Err(ChallengesParseError::SchemeUnknown) => {}
            x => panic!("{x:?}"),
        }
    }
}
