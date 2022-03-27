use alloc::{boxed::Box, string::String, vec};
use core::fmt;

use http_auth::ChallengeRef;

use crate::{
    schemes::NAME_BEARER as NAME, CHALLENGE_PARAM_REALM as PARAM_REALM, COMMA, D_Q_M, EQ_S, SP,
};

//
const PARAM_SCOPE: &str = "scope";
const PARAM_ERROR: &str = "error";
const PARAM_ERROR_DESCRIPTION: &str = "error_description";
const PARAM_ERROR_URI: &str = "error_uri";

//
#[derive(Debug, Clone)]
pub struct Challenge {
    pub realm: Box<str>,
    pub scope: Option<Box<str>>,
    pub error: Option<Box<str>>,
    pub error_description: Option<Box<str>>,
    pub error_uri: Option<Box<str>>,
}

impl Challenge {
    pub fn new(realm: impl AsRef<str>) -> Self {
        Self {
            realm: realm.as_ref().into(),
            scope: None,
            error: None,
            error_description: None,
            error_uri: None,
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
        if let Some(scope) = &self.scope {
            params.push((PARAM_SCOPE, true, scope));
        }
        if let Some(error) = &self.error {
            params.push((PARAM_ERROR, true, error));
        }
        if let Some(error_description) = &self.error_description {
            params.push((PARAM_ERROR_DESCRIPTION, true, error_description));
        }
        if let Some(error_uri) = &self.error_uri {
            params.push((PARAM_ERROR_URI, true, error_uri));
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

        let scope = c
            .params
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(PARAM_SCOPE))
            .map(|(_, v)| v.as_escaped().into());
        let error = c
            .params
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(PARAM_ERROR))
            .map(|(_, v)| v.as_escaped().into());
        let error_description = c
            .params
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(PARAM_ERROR_DESCRIPTION))
            .map(|(_, v)| v.as_escaped().into());
        let error_uri = c
            .params
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(PARAM_ERROR_URI))
            .map(|(_, v)| v.as_escaped().into());

        Ok(Self {
            realm,
            scope,
            error,
            error_description,
            error_uri,
        })
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
        write!(f, "{:?}", self)
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
pub(crate) const DEMO_CHALLENGE_STR_SIMPLE: &str = r#"Bearer realm="example""#;
#[cfg(test)]
pub(crate) const DEMO_CHALLENGE_STR: &str = r#"Bearer realm="example", error="invalid_token", error_description="The access token expired""#;
#[cfg(test)]
pub(crate) const DEMO_CHALLENGE_REALM_STR: &str = "example";
#[cfg(test)]
pub(crate) const DEMO_CHALLENGE_ERROR_STR: &str = "invalid_token";
#[cfg(test)]
pub(crate) const DEMO_CHALLENGE_ERROR_DESCRIPTION_STR: &str = "The access token expired";

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
            PARAM_SCOPE,
            ParamValue::try_from_escaped("openid profile email").unwrap(),
        ));
        c.params.push((
            PARAM_ERROR,
            ParamValue::try_from_escaped("invalid_token").unwrap(),
        ));
        c.params.push((
            PARAM_ERROR_DESCRIPTION,
            ParamValue::try_from_escaped("The access token expired").unwrap(),
        ));
        c.params.push((
            PARAM_ERROR_URI,
            ParamValue::try_from_escaped("https://example.com").unwrap(),
        ));

        let c = Challenge::try_from(&c).unwrap();
        assert_eq!(c.realm, "foo".into());
        assert_eq!(c.scope, Some("openid profile email".into()));
        assert_eq!(c.error, Some("invalid_token".into()));
        assert_eq!(c.error_description, Some("The access token expired".into()));
        assert_eq!(c.error_uri, Some("https://example.com".into()));
    }

    #[test]
    fn test_render() {
        let mut c = Challenge::new(DEMO_CHALLENGE_REALM_STR);
        assert_eq!(c.to_string(), DEMO_CHALLENGE_STR_SIMPLE);

        c.error = Some(DEMO_CHALLENGE_ERROR_STR.into());
        c.error_description = Some(DEMO_CHALLENGE_ERROR_DESCRIPTION_STR.into());
        assert_eq!(c.to_string(), DEMO_CHALLENGE_STR);
    }
}
