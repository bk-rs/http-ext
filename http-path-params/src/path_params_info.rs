use alloc::{boxed::Box, string::String, vec::Vec};
use core::ops::{Deref, DerefMut};

//
pub type PathParamsInfoInner = Vec<(Option<Box<str>>, String)>;

//
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PathParamsInfo(pub PathParamsInfoInner);

impl PathParamsInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

//
impl Deref for PathParamsInfo {
    type Target = PathParamsInfoInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PathParamsInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

//
impl From<PathParamsInfoInner> for PathParamsInfo {
    fn from(v: PathParamsInfoInner) -> Self {
        Self(v)
    }
}
