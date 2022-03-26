//! [The 'Basic' HTTP Authentication Scheme](https://www.rfc-editor.org/rfc/rfc7617.html)

use alloc::{
    boxed::Box,
    format,
    string::{self, String},
};
use core::{fmt, str::FromStr};

use crate::schemes::{NAME_BASIC as NAME, SP};

//
const COLON: &str = ":";

//
//
//
#[derive(Debug, Clone)]
pub struct Credentials {
    pub user_id: Box<str>,
    pub password: Box<str>,
}

impl Credentials {
    pub fn new(user_id: impl AsRef<str>, password: impl AsRef<str>) -> Self {
        Self {
            user_id: user_id.as_ref().into(),
            password: password.as_ref().into(),
        }
    }
}

impl fmt::Display for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            NAME,
            SP,
            base64::encode(format!("{}{}{}", self.user_id, COLON, self.password))
        )
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

        if s[NAME.len()..NAME.len() + 1] != *SP {
            return Err(Self::Err::OneSPMismatch);
        }

        let param_bytes =
            base64::decode(&s[NAME.len() + 1..]).map_err(Self::Err::ParamBase64DecodeFailed)?;

        let param_str =
            String::from_utf8(param_bytes).map_err(Self::Err::ParamFromBase64DecodeBytesFailed)?;

        let mut split = param_str.split(COLON);
        let user_id = split.next().ok_or(Self::Err::ParamUserIdMissing)?;
        let password = split.next().ok_or(Self::Err::ParamPasswordMissing)?;
        if split.next().is_some() {
            return Err(Self::Err::ParamPairsMismatch);
        }

        Ok(Self::new(user_id, password))
    }
}

#[derive(Debug)]
pub enum CredentialsParseError {
    SchemeMismatch,
    OneSPMismatch,
    ParamBase64DecodeFailed(base64::DecodeError),
    ParamFromBase64DecodeBytesFailed(string::FromUtf8Error),
    ParamUserIdMissing,
    ParamPasswordMissing,
    ParamPairsMismatch,
    Other(&'static str),
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::string::ToString as _;

    #[test]
    fn test_credentials_parse_and_render() {
        let s = "Basic YWxhZGRpbjpvcGVuc2VzYW1l";
        let c = s.parse::<Credentials>().unwrap();
        assert_eq!(c.user_id, "aladdin".into());
        assert_eq!(c.password, "opensesame".into());
        assert_eq!(c.to_string(), s);

        //
        match Credentials::from_str("Basic") {
            Err(CredentialsParseError::Other(err)) => {
                assert_eq!(err, "too short")
            }
            x => panic!("{:?}", x),
        }

        match Credentials::from_str("MyScheme ") {
            Err(CredentialsParseError::SchemeMismatch) => {}
            x => panic!("{:?}", x),
        }

        match Credentials::from_str("Basic-") {
            Err(CredentialsParseError::OneSPMismatch) => {}
            x => panic!("{:?}", x),
        }

        match Credentials::from_str("Basic dGVzdDoxMjM6Zm9v") {
            Err(CredentialsParseError::ParamPairsMismatch) => {}
            x => panic!("{:?}", x),
        }
    }
}
