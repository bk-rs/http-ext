use alloc::string::{String, ToString as _};

use crate::{Id, PathParams};

use super::SetError;

impl PathParams for Id {
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
                self.0 = value
                    .parse::<usize>()
                    .map_err(|err| SetError::ValueParseFailed(err.to_string()))?;
                Ok(())
            }
            _ => panic!(),
        }
    }
    fn verify(&self, index: usize, (_, value): &(Option<&str>, &str)) -> Result<(), SetError> {
        match index {
            0 => {
                let _ = value
                    .parse::<usize>()
                    .map_err(|err| SetError::ValueParseFailed(err.to_string()))?;
                Ok(())
            }
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
        let mut path_params: Id = Default::default();
        path_params
            .set_from_info(&PathParamsInfo(vec![(None, "1".to_string())]))
            .unwrap();
        assert_eq!(path_params, Id(1));
        assert_eq!(path_params.info(), vec![(None, "1".to_string())].into());
        assert!(path_params.verify(0, &(None, "1")).is_ok());

        //
        let mut path_params: PathParam<Id> = Default::default();
        path_params
            .set_from_info(&PathParamsInfo(vec![(None, "1".to_string())]))
            .unwrap();
        assert_eq!(path_params, PathParam(Id(1)));
        assert_eq!(path_params.info(), vec![(None, "1".to_string())].into());
    }
}
