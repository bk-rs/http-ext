use alloc::string::String;

use downcast_rs::{impl_downcast, DowncastSync};
use dyn_clone::{clone_trait_object, DynClone};

use crate::PathParamsInfo;

mod impl_id;
mod impl_internal;
mod impl_path_param;
mod impl_username;

//
pub trait PathParams: DowncastSync + DynClone {
    fn size(&self) -> usize;

    fn get(&self, _index: usize) -> (Option<&str>, String);
    fn set(
        &mut self,
        _index: usize,
        _name_and_value: &(Option<&str>, &str),
    ) -> Result<(), SetError>;
    fn verify(&self, _index: usize, _name_and_value: &(Option<&str>, &str))
        -> Result<(), SetError>;

    fn info(&self) -> PathParamsInfo {
        PathParamsInfo(
            (0..self.size())
                .map(|i| {
                    let (name, value) = self.get(i);
                    (name.map(Into::into), value)
                })
                .collect(),
        )
    }
    fn set_from_info(&mut self, info: &PathParamsInfo) -> Result<(), SetError> {
        if info.0.len() != self.size() {
            return Err(SetError::InfoLengthMismatch);
        }
        (0..self.size()).try_for_each(|i| {
            let (name, value) = info.get(i).expect("unreachable");
            self.set(i, &(name.as_deref(), value.as_ref()))
        })
    }
}

impl_downcast!(PathParams);
clone_trait_object!(PathParams);

impl core::fmt::Debug for dyn PathParams {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PathParams").field(&self.info()).finish()
    }
}
impl core::fmt::Debug for dyn PathParams + Send {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PathParams").field(&self.info()).finish()
    }
}
impl core::fmt::Debug for dyn PathParams + Send + Sync {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PathParams").field(&self.info()).finish()
    }
}

//
#[derive(Debug, Clone)]
pub enum SetError {
    InfoLengthMismatch,
    ValueParseFailed(String),
}
impl core::fmt::Display for SetError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
#[cfg(feature = "std")]
impl std::error::Error for SetError {}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::{boxed::Box, string::ToString as _, vec, vec::Vec};

    #[derive(Debug, Clone)]
    struct PathParamsStorage(Vec<Box<dyn PathParams>>);

    #[test]
    fn test_path_params_dyn_clone_and_debug() {
        let path_params_list = PathParamsStorage(vec![Box::new(1), Box::new("foo".to_string())]);
        #[allow(clippy::redundant_clone)]
        let path_params_list = path_params_list.clone();
        #[cfg(feature = "std")]
        println!("path_params_list: {path_params_list:?}");
        assert_eq!(path_params_list.0.len(), 2);
    }

    #[test]
    fn test_path_params_downcast() {
        let path_params: Box<dyn PathParams> = Box::new("foo".to_string());
        assert_eq!(path_params.downcast_ref::<String>().unwrap(), "foo");
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_set_error_impl_std_error() {
        use crate::PathParam;

        fn doing() -> Result<(), Box<dyn std::error::Error>> {
            let mut path_params = (PathParam(1_usize), PathParam("foo".to_string()));
            path_params.set(0, &(None, "bar"))?;
            Ok(())
        }
        assert!(doing().is_err());
    }
}
