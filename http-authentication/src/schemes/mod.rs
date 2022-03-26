pub const NAME_BASIC: &str = "Basic";
pub const NAME_BEARER: &str = "Bearer";
pub const NAME_DIGEST: &str = "Digest";

//
#[allow(dead_code)]
pub(crate) const SP_STR: &str = " ";
pub(crate) const SP: char = ' ';

//
#[cfg(feature = "scheme-basic")]
pub mod basic;

#[cfg(feature = "scheme-bearer")]
pub mod bearer;
