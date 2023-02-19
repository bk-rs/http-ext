//! [Ref](https://datatracker.ietf.org/doc/html/rfc2616#section-5.1.2)

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RequestUri<'a> {
    /// The alias is AbsPath
    Origin {
        path: &'a str,
        query: Option<&'a str>,
        fragment: Option<&'a str>,
    },
    AbsoluteUri {
        scheme: &'a str,
        username: Option<&'a str>,
        password: Option<&'a str>,
        host: &'a str,
        port: Option<u16>,
        path: &'a str,
        query: Option<&'a str>,
        fragment: Option<&'a str>,
    },
    Authority {
        host: &'a str,
        port: Option<u16>,
    },
    Asterisk,
}

#[cfg(feature = "http")]
mod impl_http;

#[cfg(test)]
#[allow(dead_code)]
const TEST_DATA: &[(&str, RequestUri<'_>)] = &[
    //
    ("*", RequestUri::Asterisk),
    (
        "http://www.w3.org/pub/WWW/TheProject.html",
        RequestUri::AbsoluteUri {
            scheme: "http",
            username: None,
            password: None,
            host: "www.w3.org",
            port: None,
            path: "/pub/WWW/TheProject.html",
            query: None,
            fragment: None,
        },
    ),
    (
        "http://www.w3.org",
        RequestUri::AbsoluteUri {
            scheme: "http",
            username: None,
            password: None,
            host: "www.w3.org",
            port: None,
            path: "/",
            query: None,
            fragment: None,
        },
    ),
    (
        "http://username:password@www.w3.org",
        RequestUri::AbsoluteUri {
            scheme: "http",
            username: Some("username"),
            password: Some("password"),
            host: "www.w3.org",
            port: None,
            path: "/",
            query: None,
            fragment: None,
        },
    ),
    (
        "http://username@www.w3.org",
        RequestUri::AbsoluteUri {
            scheme: "http",
            username: Some("username"),
            password: None,
            host: "www.w3.org",
            port: None,
            path: "/",
            query: None,
            fragment: None,
        },
    ),
    (
        "http://:password@www.w3.org",
        RequestUri::AbsoluteUri {
            scheme: "http",
            username: None,
            password: Some("password"),
            host: "www.w3.org",
            port: None,
            path: "/",
            query: None,
            fragment: None,
        },
    ),
    (
        "/pub/WWW/TheProject.html",
        RequestUri::Origin {
            path: "/pub/WWW/TheProject.html",
            query: None,
            fragment: None,
        },
    ),
    (
        "/",
        RequestUri::Origin {
            path: "/",
            query: None,
            fragment: None,
        },
    ),
    (
        "/foo?bar=",
        RequestUri::Origin {
            path: "/foo",
            query: Some("bar="),
            fragment: None,
        },
    ),
    //
    (
        "proxy.com",
        RequestUri::Authority {
            host: "proxy.com",
            port: None,
        },
    ),
    (
        "proxy.com:443",
        RequestUri::Authority {
            host: "proxy.com",
            port: Some(443),
        },
    ),
];
