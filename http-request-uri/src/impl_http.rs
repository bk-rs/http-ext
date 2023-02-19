use http::Uri;

use crate::RequestUri;

impl<'a> TryFrom<&'a Uri> for RequestUri<'a> {
    type Error = &'static str;

    fn try_from(uri: &'a Uri) -> Result<Self, Self::Error> {
        if let Some(scheme) = uri.scheme() {
            if let Some(authority) = uri.authority() {
                let (mut username, mut password) = (None, None);

                let mut split = authority.as_str().split('@').rev();
                if split.next().is_none() {
                    debug_assert!(false, "unreachable");
                    return Err("authority invalid");
                }

                if let Some(username_password) = split.next() {
                    if split.next().is_some() {
                        debug_assert!(false, "unreachable");
                        return Err("authority invalid");
                    }

                    let mut split = username_password.split(':');
                    let username_tmp = split.next().ok_or("username missing")?;
                    if !username_tmp.is_empty() {
                        username = Some(username_tmp);
                    }
                    if let Some(password_tmp) = split.next() {
                        if split.next().is_some() {
                            debug_assert!(false, "unreachable");
                            return Err("authority invalid");
                        }

                        if !password_tmp.is_empty() {
                            password = Some(password_tmp);
                        }
                    }
                }

                Ok(Self::AbsoluteUri {
                    scheme: scheme.as_str(),
                    username,
                    password,
                    host: authority.host(),
                    port: authority.port_u16(),
                    path: uri.path(),
                    query: uri.query(),
                    fragment: None,
                })
            } else {
                Err("authority missing")
            }
        } else if let Some(authority) = uri.authority() {
            Ok(Self::Authority {
                host: authority.host(),
                port: authority.port_u16(),
            })
        } else if uri.path() == "*" {
            Ok(Self::Asterisk)
        } else {
            if !uri.path().starts_with('/') {
                debug_assert!(false, "unreachable");
                return Err("path invalid");
            }

            Ok(Self::Origin {
                path: uri.path(),
                query: uri.query(),
                fragment: None,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::TEST_DATA;

    #[test]
    fn test_from_uri() {
        for (uri_str, request_uri) in TEST_DATA {
            assert_eq!(
                &RequestUri::try_from(&uri_str.parse::<Uri>().unwrap()).unwrap(),
                request_uri
            );
        }
    }
}
