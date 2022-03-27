//
#[derive(Debug, Clone)]
pub enum Challenge {
    #[cfg(feature = "scheme-basic")]
    Basic(crate::schemes::basic::Challenge),
    #[cfg(feature = "scheme-bearer")]
    Bearer(crate::schemes::bearer::Challenge),
}

impl Challenge {
    //
    #[cfg(feature = "scheme-basic")]
    pub fn basic(challenge: crate::schemes::basic::Challenge) -> Self {
        Self::Basic(challenge)
    }

    #[cfg(feature = "scheme-basic")]
    pub fn as_basic(&self) -> Option<&crate::schemes::basic::Challenge> {
        match self {
            Self::Basic(c) => Some(c),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }

    //
    #[cfg(feature = "scheme-bearer")]
    pub fn bearer(challenge: crate::schemes::bearer::Challenge) -> Self {
        Self::Bearer(challenge)
    }

    #[cfg(feature = "scheme-bearer")]
    pub fn as_bearer(&self) -> Option<&crate::schemes::bearer::Challenge> {
        match self {
            Self::Bearer(c) => Some(c),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
}
