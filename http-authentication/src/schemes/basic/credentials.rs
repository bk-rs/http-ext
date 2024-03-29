use alloc::{boxed::Box, format, string::String};
use core::str::{self, FromStr};

use base64::{engine::general_purpose, Engine as _};

use crate::{schemes::NAME_BASIC as NAME, SP};

//
const COLON: char = ':';

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

    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, CredentialsParseError> {
        let bytes = bytes.as_ref();

        if bytes.len() < NAME.len() + 1 {
            return Err(CredentialsParseError::Other("too short"));
        }

        if !&bytes[..NAME.len()].eq_ignore_ascii_case(NAME.as_bytes()) {
            return Err(CredentialsParseError::SchemeMismatch);
        }

        if bytes[NAME.len()..NAME.len() + 1] != [SP as u8] {
            return Err(CredentialsParseError::OneSPMismatch);
        }

        let token68_bytes = &bytes[NAME.len() + 1..];

        let token68_b64_decoded_bytes = general_purpose::STANDARD
            .decode(token68_bytes)
            .map_err(CredentialsParseError::Token68DecodeFailed)?;

        let mut token68_split = token68_b64_decoded_bytes.split(|x| *x == COLON as u8);
        let user_id = token68_split
            .next()
            .ok_or(CredentialsParseError::UserIdMissing)?;
        let user_id = str::from_utf8(user_id).map_err(CredentialsParseError::UserIdToStrFailed)?;
        let password = token68_split
            .next()
            .ok_or(CredentialsParseError::PasswordMissing)?;
        let password =
            str::from_utf8(password).map_err(CredentialsParseError::PasswordToStrFailed)?;
        if token68_split.next().is_some() {
            return Err(CredentialsParseError::Token68PairsMismatch);
        }

        Ok(Self::new(user_id, password))
    }

    fn internal_to_string(&self) -> String {
        format!(
            "{NAME}{SP}{}",
            general_purpose::STANDARD.encode(format!("{}{COLON}{}", self.user_id, self.password))
        )
    }
}

//
#[derive(Debug)]
pub enum CredentialsParseError {
    SchemeMismatch,
    OneSPMismatch,
    Token68DecodeFailed(base64::DecodeError),
    UserIdMissing,
    UserIdToStrFailed(str::Utf8Error),
    PasswordMissing,
    PasswordToStrFailed(str::Utf8Error),
    Token68PairsMismatch,
    Other(&'static str),
}

impl core::fmt::Display for CredentialsParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CredentialsParseError {}

//
impl core::fmt::Display for Credentials {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.internal_to_string())
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
pub(crate) const DEMO_CREDENTIALS_STR: &str = "Basic YWxhZGRpbjpvcGVuc2VzYW1l";
#[cfg(test)]
pub(crate) const DEMO_CREDENTIALS_USER_ID_STR: &str = "aladdin";
#[cfg(test)]
pub(crate) const DEMO_CREDENTIALS_PASSWORD_STR: &str = "opensesame";

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::string::ToString as _;

    #[test]
    fn test_parse_and_render() {
        let c = DEMO_CREDENTIALS_STR.parse::<Credentials>().unwrap();
        assert_eq!(c.user_id, DEMO_CREDENTIALS_USER_ID_STR.into());
        assert_eq!(c.password, DEMO_CREDENTIALS_PASSWORD_STR.into());
        assert_eq!(c.to_string(), DEMO_CREDENTIALS_STR);

        //
        match Credentials::from_str("Basic") {
            Err(CredentialsParseError::Other(err)) => {
                assert_eq!(err, "too short")
            }
            x => panic!("{x:?}"),
        }

        match Credentials::from_str("MyScheme ") {
            Err(CredentialsParseError::SchemeMismatch) => {}
            x => panic!("{x:?}"),
        }

        match Credentials::from_str("Basic-") {
            Err(CredentialsParseError::OneSPMismatch) => {}
            x => panic!("{x:?}"),
        }

        match Credentials::from_str("Basic dGVzdDoxMjM6Zm9v") {
            Err(CredentialsParseError::Token68PairsMismatch) => {}
            x => panic!("{x:?}"),
        }
    }
}
