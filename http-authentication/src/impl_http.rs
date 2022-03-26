use http::{
    header::{AUTHORIZATION, PROXY_AUTHORIZATION},
    HeaderMap,
};

use crate::credentials::{Credentials, CredentialsParseError};

//
//
//
pub fn parse_header_authorization(
    header_map: &HeaderMap,
) -> Option<Result<Credentials, CredentialsParseError>> {
    header_map
        .get(AUTHORIZATION)
        .map(|x| Credentials::from_bytes(x.as_bytes()))
}

//
//
//
pub fn parse_header_proxy_authorization(
    header_map: &HeaderMap,
) -> Option<Result<Credentials, CredentialsParseError>> {
    header_map
        .get(PROXY_AUTHORIZATION)
        .map(|x| Credentials::from_bytes(x.as_bytes()))
}
