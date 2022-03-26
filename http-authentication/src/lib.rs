//! [Hypertext Transfer Protocol (HTTP/1.1): Authentication](https://www.rfc-editor.org/rfc/rfc7235)

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

//
pub mod credentials;

//
pub mod schemes;

//
#[cfg(all(feature = "std", feature = "http"))]
mod impl_http;

#[cfg(all(feature = "std", feature = "http"))]
pub use impl_http::{parse_header_authorization, parse_header_proxy_authorization};
