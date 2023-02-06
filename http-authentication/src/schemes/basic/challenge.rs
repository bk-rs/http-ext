use alloc::{boxed::Box, string::String, vec};
use core::fmt;

use http_auth::ChallengeRef;

use crate::{
    schemes::NAME_BASIC as NAME, CHALLENGE_PARAM_REALM as PARAM_REALM, COMMA, D_Q_M, EQ_S, SP,
};

//
const PARAM_CHARSET: &str = "charset";

//
#[derive(Debug, Clone)]
pub struct Challenge {
    pub realm: Box<str>,
    pub charset: Option<Box<str>>,
}

impl Challenge {
    pub fn new(realm: impl AsRef<str>) -> Self {
        Self {
            realm: realm.as_ref().into(),
            charset: None,
        }
    }

    fn internal_to_string(&self) -> String {
        let mut s = String::with_capacity(30);
        s.push_str(NAME);
        s.push(SP);

        s.push_str(PARAM_REALM);
        s.push(EQ_S);
        s.push(D_Q_M);
        s.push_str(self.realm.as_ref());
        s.push(D_Q_M);

        let mut params = vec![];
        if let Some(charset) = &self.charset {
            params.push((PARAM_CHARSET, true, charset));
        }

        for (k, is_quoted, v) in params {
            s.push(COMMA);
            s.push(SP);
            s.push_str(k);
            s.push(EQ_S);
            if is_quoted {
                s.push(D_Q_M);
            }
            s.push_str(v.as_ref());
            if is_quoted {
                s.push(D_Q_M);
            }
        }

        s
    }
}

//
// Ref https://github.com/scottlamb/http-auth/blob/v0.1.5/src/basic.rs#L69-L90
//
impl TryFrom<&ChallengeRef<'_>> for Challenge {
    type Error = ChallengeParseError;

    fn try_from(c: &ChallengeRef<'_>) -> Result<Self, Self::Error> {
        if !c.scheme.eq_ignore_ascii_case(NAME) {
            return Err(ChallengeParseError::SchemeMismatch);
        }

        let realm = c
            .params
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(PARAM_REALM))
            .map(|(_, v)| v.as_escaped().into());
        //
        // TODO, Optional
        // Ref https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/WWW-Authenticate#basic
        //
        let realm = realm.unwrap_or_default();

        let charset = c
            .params
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(PARAM_CHARSET))
            .map(|(_, v)| v.as_escaped().into());

        Ok(Self { realm, charset })
    }
}

//
#[derive(Debug)]
pub enum ChallengeParseError {
    SchemeMismatch,
    Other(&'static str),
}

impl fmt::Display for ChallengeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ChallengeParseError {}

//
impl fmt::Display for Challenge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.internal_to_string())
    }
}

//
//
//
#[cfg(test)]
pub(crate) const DEMO_CHALLENGE_STR_SIMPLE: &str = r#"Basic realm="foo""#;
#[cfg(test)]
pub(crate) const DEMO_CHALLENGE_STR: &str = r#"Basic realm="foo", charset="UTF-8""#;
#[cfg(test)]
pub(crate) const DEMO_CHALLENGE_REALM_STR: &str = "foo";
#[cfg(test)]
pub(crate) const DEMO_CHALLENGE_CHARSET_STR: &str = "UTF-8";

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::string::ToString as _;

    use http_auth::ParamValue;

    #[test]
    fn test_try_from_challenge_ref() {
        let mut c = ChallengeRef::new(NAME);
        c.params
            .push((PARAM_REALM, ParamValue::try_from_escaped("foo").unwrap()));
        c.params.push((
            PARAM_CHARSET,
            ParamValue::try_from_escaped("UTF-8").unwrap(),
        ));

        let c = Challenge::try_from(&c).unwrap();
        assert_eq!(c.realm, "foo".into());
        assert_eq!(c.charset, Some("UTF-8".into()));
    }

    #[test]
    fn test_render() {
        let mut c = Challenge::new(DEMO_CHALLENGE_REALM_STR);
        assert_eq!(c.to_string(), DEMO_CHALLENGE_STR_SIMPLE);

        c.charset = Some(DEMO_CHALLENGE_CHARSET_STR.into());
        assert_eq!(c.to_string(), DEMO_CHALLENGE_STR);
    }
}
