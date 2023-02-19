use alloc::string::{String, ToString as _};

use crate::PathParams;

use super::SetError;

//
impl PathParams for () {
    fn size(&self) -> usize {
        0
    }
    fn get(&self, _index: usize) -> (Option<&'static str>, String) {
        panic!()
    }
    fn set(&mut self, _index: usize, (_, _value): &(Option<&str>, &str)) -> Result<(), SetError> {
        panic!()
    }
    fn verify(&self, _index: usize, (_, _value): &(Option<&str>, &str)) -> Result<(), SetError> {
        panic!()
    }
}

//
impl PathParams for usize {
    fn size(&self) -> usize {
        1
    }
    fn get(&self, index: usize) -> (Option<&'static str>, String) {
        match index {
            0 => (None, self.to_string()),
            _ => panic!(),
        }
    }
    fn set(&mut self, index: usize, (_, value): &(Option<&str>, &str)) -> Result<(), SetError> {
        match index {
            0 => {
                *self = value
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

impl PathParams for String {
    fn size(&self) -> usize {
        1
    }
    fn get(&self, index: usize) -> (Option<&'static str>, String) {
        match index {
            0 => (None, self.to_string()),
            _ => panic!(),
        }
    }
    fn set(&mut self, index: usize, (_, value): &(Option<&str>, &str)) -> Result<(), SetError> {
        match index {
            0 => {
                *self = value.to_string();
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

//
impl PathParams for (usize, usize) {
    fn size(&self) -> usize {
        2
    }
    fn get(&self, index: usize) -> (Option<&'static str>, String) {
        match index {
            0 => (None, self.0.to_string()),
            1 => (None, self.1.to_string()),
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
            1 => {
                self.1 = value
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
            1 => {
                let _ = value
                    .parse::<usize>()
                    .map_err(|err| SetError::ValueParseFailed(err.to_string()))?;
                Ok(())
            }
            _ => panic!(),
        }
    }
}

impl PathParams for (String, String) {
    fn size(&self) -> usize {
        2
    }
    fn get(&self, index: usize) -> (Option<&'static str>, String) {
        match index {
            0 => (None, self.0.to_string()),
            1 => (None, self.1.to_string()),
            _ => panic!(),
        }
    }
    fn set(&mut self, index: usize, (_, value): &(Option<&str>, &str)) -> Result<(), SetError> {
        match index {
            0 => {
                self.0 = value.to_string();
                Ok(())
            }
            1 => {
                self.1 = value.to_string();
                Ok(())
            }
            _ => panic!(),
        }
    }
    fn verify(&self, index: usize, (_, _value): &(Option<&str>, &str)) -> Result<(), SetError> {
        match index {
            0 => Ok(()),
            1 => Ok(()),
            _ => panic!(),
        }
    }
}

impl PathParams for (usize, String) {
    fn size(&self) -> usize {
        2
    }
    fn get(&self, index: usize) -> (Option<&'static str>, String) {
        match index {
            0 => (None, self.0.to_string()),
            1 => (None, self.1.to_string()),
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
            1 => {
                self.1 = value.to_string();
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
            1 => Ok(()),
            _ => panic!(),
        }
    }
}

impl PathParams for (String, usize) {
    fn size(&self) -> usize {
        2
    }
    fn get(&self, index: usize) -> (Option<&'static str>, String) {
        match index {
            0 => (None, self.0.to_string()),
            1 => (None, self.1.to_string()),
            _ => panic!(),
        }
    }
    fn set(&mut self, index: usize, (_, value): &(Option<&str>, &str)) -> Result<(), SetError> {
        match index {
            0 => {
                self.0 = value.to_string();
                Ok(())
            }
            1 => {
                self.1 = value
                    .parse::<usize>()
                    .map_err(|err| SetError::ValueParseFailed(err.to_string()))?;
                Ok(())
            }
            _ => panic!(),
        }
    }
    fn verify(&self, index: usize, (_, value): &(Option<&str>, &str)) -> Result<(), SetError> {
        match index {
            0 => Ok(()),
            1 => {
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

    use crate::PathParamsInfo;

    #[test]
    fn test_get_and_set() {
        //
        let mut path_params: usize = Default::default();
        path_params
            .set_from_info(&PathParamsInfo(vec![(None, "1".to_string())]))
            .unwrap();
        assert_eq!(path_params, 1);
        assert_eq!(path_params.info(), vec![(None, "1".to_string())].into());
        assert!(path_params.verify(0, &(None, "1")).is_ok());

        let mut path_params: String = Default::default();
        path_params
            .set_from_info(&PathParamsInfo(vec![(None, "foo".to_string())]))
            .unwrap();
        assert_eq!(path_params, "foo");
        assert_eq!(path_params.info(), vec![(None, "foo".to_string())].into());
        assert!(path_params.verify(0, &(None, "foo")).is_ok());

        //
        let mut path_params: (usize, usize) = Default::default();
        path_params
            .set_from_info(&PathParamsInfo(vec![
                (None, "1".to_string()),
                (None, "2".to_string()),
            ]))
            .unwrap();
        assert_eq!(path_params, (1, 2));
        assert_eq!(
            path_params.info(),
            vec![(None, "1".to_string()), (None, "2".to_string())].into()
        );
        assert!(path_params.verify(0, &(None, "1")).is_ok());
        assert!(path_params.verify(1, &(None, "2")).is_ok());

        //
        let mut path_params: (String, String) = Default::default();
        path_params
            .set_from_info(&PathParamsInfo(vec![
                (None, "foo".to_string()),
                (None, "bar".to_string()),
            ]))
            .unwrap();
        assert_eq!(path_params, ("foo".to_string(), "bar".to_string()));
        assert_eq!(
            path_params.info(),
            vec![(None, "foo".to_string()), (None, "bar".to_string())].into()
        );
        assert!(path_params.verify(0, &(None, "foo")).is_ok());
        assert!(path_params.verify(1, &(None, "bar")).is_ok());

        //
        let mut path_params: (usize, String) = Default::default();
        path_params
            .set_from_info(&PathParamsInfo(vec![
                (None, "1".to_string()),
                (None, "foo".to_string()),
            ]))
            .unwrap();
        assert_eq!(path_params, (1, "foo".to_string()));
        assert_eq!(
            path_params.info(),
            vec![(None, "1".to_string()), (None, "foo".to_string())].into()
        );
        assert!(path_params.verify(0, &(None, "1")).is_ok());
        assert!(path_params.verify(1, &(None, "foo")).is_ok());

        //
        let mut path_params: (String, usize) = Default::default();
        path_params
            .set_from_info(&PathParamsInfo(vec![
                (None, "foo".to_string()),
                (None, "1".to_string()),
            ]))
            .unwrap();
        assert_eq!(path_params, ("foo".to_string(), 1));
        assert_eq!(
            path_params.info(),
            vec![(None, "foo".to_string()), (None, "1".to_string())].into()
        );
        assert!(path_params.verify(0, &(None, "foo")).is_ok());
        assert!(path_params.verify(1, &(None, "1")).is_ok());
    }
}
