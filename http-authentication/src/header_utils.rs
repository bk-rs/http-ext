use http::{
    header::{AUTHORIZATION, PROXY_AUTHORIZATION},
    HeaderMap,
};

use crate::credentials::{Credentials, CredentialsParseError};

//
//
//
pub fn get_authorization(
    header_map: &HeaderMap,
) -> Option<Result<Credentials, CredentialsParseError>> {
    header_map
        .get(AUTHORIZATION)
        .map(|x| Credentials::from_bytes(x.as_bytes()))
}

#[cfg(feature = "scheme-basic")]
pub fn set_authorization_with_basic(
    header_map: &mut HeaderMap,
    user_id: impl AsRef<str>,
    password: impl AsRef<str>,
) -> Result<(), http::header::InvalidHeaderValue> {
    use alloc::string::ToString as _;

    header_map.remove(AUTHORIZATION);
    header_map.append(
        AUTHORIZATION,
        http::HeaderValue::from_str(Credentials::basic(user_id, password).to_string().as_str())?,
    );
    Ok(())
}

#[cfg(feature = "scheme-bearer")]
pub fn set_authorization_with_bearer(
    header_map: &mut HeaderMap,
    token: impl AsRef<str>,
) -> Result<(), http::header::InvalidHeaderValue> {
    use alloc::string::ToString as _;

    header_map.remove(AUTHORIZATION);
    header_map.append(
        AUTHORIZATION,
        http::HeaderValue::from_str(Credentials::bearer(token).to_string().as_str())?,
    );
    Ok(())
}

//
//
//
pub fn get_proxy_authorization(
    header_map: &HeaderMap,
) -> Option<Result<Credentials, CredentialsParseError>> {
    header_map
        .get(PROXY_AUTHORIZATION)
        .map(|x| Credentials::from_bytes(x.as_bytes()))
}

#[cfg(feature = "scheme-basic")]
pub fn set_proxy_authorization_with_basic(
    header_map: &mut HeaderMap,
    user_id: impl AsRef<str>,
    password: impl AsRef<str>,
) -> Result<(), http::header::InvalidHeaderValue> {
    use alloc::string::ToString as _;

    header_map.remove(PROXY_AUTHORIZATION);
    header_map.append(
        PROXY_AUTHORIZATION,
        http::HeaderValue::from_str(Credentials::basic(user_id, password).to_string().as_str())?,
    );
    Ok(())
}

#[cfg(feature = "scheme-bearer")]
pub fn set_proxy_authorization_with_bearer(
    header_map: &mut HeaderMap,
    token: impl AsRef<str>,
) -> Result<(), http::header::InvalidHeaderValue> {
    use alloc::string::ToString as _;

    header_map.remove(PROXY_AUTHORIZATION);
    header_map.append(
        PROXY_AUTHORIZATION,
        http::HeaderValue::from_str(Credentials::bearer(token).to_string().as_str())?,
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "scheme-basic")]
    #[test]
    fn test_get_set_authorization() {
        use crate::schemes::basic::{
            DEMO_CREDENTIALS_PASSWORD_STR, DEMO_CREDENTIALS_STR, DEMO_CREDENTIALS_USER_ID_STR,
        };

        //
        let mut map = HeaderMap::new();
        assert!(get_authorization(&map).is_none());
        map.append(AUTHORIZATION, DEMO_CREDENTIALS_STR.parse().unwrap());
        let c = get_authorization(&map).map(|x| x.unwrap()).unwrap();
        match c {
            Credentials::Basic(c) => {
                assert_eq!(c.user_id, DEMO_CREDENTIALS_USER_ID_STR.into());
                assert_eq!(c.password, DEMO_CREDENTIALS_PASSWORD_STR.into());
            }
            x => panic!("{:?}", x),
        }

        //
        let mut map = HeaderMap::new();
        set_authorization_with_basic(
            &mut map,
            DEMO_CREDENTIALS_USER_ID_STR,
            DEMO_CREDENTIALS_PASSWORD_STR,
        )
        .unwrap();
        assert_eq!(map.get(AUTHORIZATION).unwrap(), DEMO_CREDENTIALS_STR);
    }

    #[cfg(feature = "scheme-basic")]
    #[test]
    fn test_get_set_proxy_authorization() {
        use crate::schemes::basic::{
            DEMO_CREDENTIALS_PASSWORD_STR, DEMO_CREDENTIALS_STR, DEMO_CREDENTIALS_USER_ID_STR,
        };

        //
        let mut map = HeaderMap::new();
        assert!(get_proxy_authorization(&map).is_none());
        map.append(PROXY_AUTHORIZATION, DEMO_CREDENTIALS_STR.parse().unwrap());
        let c = get_proxy_authorization(&map).map(|x| x.unwrap()).unwrap();
        match c {
            Credentials::Basic(c) => {
                assert_eq!(c.user_id, DEMO_CREDENTIALS_USER_ID_STR.into());
                assert_eq!(c.password, DEMO_CREDENTIALS_PASSWORD_STR.into());
            }
            x => panic!("{:?}", x),
        }

        //
        let mut map = HeaderMap::new();
        set_proxy_authorization_with_basic(
            &mut map,
            DEMO_CREDENTIALS_USER_ID_STR,
            DEMO_CREDENTIALS_PASSWORD_STR,
        )
        .unwrap();
        assert_eq!(map.get(PROXY_AUTHORIZATION).unwrap(), DEMO_CREDENTIALS_STR);
    }
}
