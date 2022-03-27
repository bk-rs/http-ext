//! [Hypertext Transfer Protocol (HTTP/1.1): Authentication](https://www.rfc-editor.org/rfc/rfc7235)

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

//
pub(crate) const SP: char = ' ';
#[allow(dead_code)]
pub(crate) const CHALLENGE_PARAM_REALM: &str = "realm";
#[allow(dead_code)]
pub(crate) const COMMA: char = ',';
#[allow(dead_code)]
pub(crate) const EQ_S: char = '=';
#[allow(dead_code)]
pub(crate) const D_Q_M: char = '"';

//
pub mod challenge;
pub mod challenges;
pub mod credentials;

pub use challenge::Challenge;
pub use challenges::Challenges;
pub use credentials::Credentials;

//
pub mod schemes;

//
#[cfg(all(feature = "std", feature = "http"))]
pub mod header_utils;
