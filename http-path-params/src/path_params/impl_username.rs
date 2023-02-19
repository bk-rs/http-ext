use alloc::string::{String, ToString as _};

use crate::{PathParams, Username};

use super::SetError;

impl PathParams for Username {
    fn size(&self) -> usize {
        1
    }
    fn get(&self, index: usize) -> (Option<&'static str>, String) {
        match index {
            0 => (None, self.0.to_string()),
            _ => panic!(),
        }
    }
    fn set(&mut self, index: usize, (_, value): &(Option<&str>, &str)) -> Result<(), SetError> {
        match index {
            0 => {
                self.0 = value.to_string();
                Ok(())
            }
            _ => panic!(),
        }
    }
    fn verify(&self, index: usize, (_, _value): &(Option<&str>, &str)) -> Result<(), SetError> {
        match index {
            0 => Ok(()),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::vec;

    use crate::{PathParam, PathParamsInfo};

    #[test]
    fn test_get_and_set() {
        //
        let mut path_params: Username = Default::default();
        path_params
            .set_from_info(&PathParamsInfo(vec![(None, "foo".to_string())]))
            .unwrap();
        assert_eq!(path_params, Username("foo".to_string()));
        assert_eq!(path_params.info(), vec![(None, "foo".to_string())].into());
        assert!(path_params.verify(0, &(None, "foo")).is_ok());

        //
        let mut path_params: PathParam<Username> = Default::default();
        path_params
            .set_from_info(&PathParamsInfo(vec![(None, "foo".to_string())]))
            .unwrap();
        assert_eq!(path_params, PathParam(Username("foo".to_string())));
        assert_eq!(path_params.info(), vec![(None, "foo".to_string())].into());
    }
}
