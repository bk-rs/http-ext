//! [Hypertext Transfer Protocol (HTTP/1.1): Authentication](https://www.rfc-editor.org/rfc/rfc7235)

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

//
#[allow(dead_code)]
pub(crate) const SP_STR: &str = " ";
pub(crate) const SP: char = ' ';

//
pub mod credentials;

//
pub mod schemes;

//
#[cfg(all(feature = "std", feature = "http"))]
pub mod header_utils;
