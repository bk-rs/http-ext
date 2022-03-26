pub const NAME_BASIC: &str = "Basic";
pub const NAME_BEARER: &str = "Bearer";
pub const NAME_DIGEST: &str = "Digest";

//
pub(crate) const SP: &str = " ";

//
#[cfg(feature = "scheme-basic")]
pub mod basic;

#[cfg(feature = "scheme-bearer")]
pub mod bearer;
