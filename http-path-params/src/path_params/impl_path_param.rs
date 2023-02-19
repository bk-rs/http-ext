use alloc::string::{String, ToString as _};
use core::{fmt::Display, str::FromStr};

use crate::{PathParam, PathParams};

use super::SetError;

impl<T> PathParams for PathParam<T>
where
    T: FromStr + Display + Clone + Default + Send + Sync + 'static,
    <T as FromStr>::Err: Display,
{
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
                    .parse::<T>()
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
                    .parse::<T>()
                    .map_err(|err| SetError::ValueParseFailed(err.to_string()))?;
                Ok(())
            }
            _ => panic!(),
        }
    }
}

impl<T1, T2> PathParams for (PathParam<T1>, PathParam<T2>)
where
    T1: FromStr + Display + Clone + Default + Send + Sync + 'static,
    <T1 as FromStr>::Err: Display,
    T2: FromStr + Display + Clone + Default + Send + Sync + 'static,
    <T2 as FromStr>::Err: Display,
{
    fn size(&self) -> usize {
        2
    }
    fn get(&self, index: usize) -> (Option<&'static str>, String) {
        match index {
            0 => (None, self.0 .0.to_string()),
            1 => (None, self.1 .0.to_string()),
            _ => panic!(),
        }
    }
    fn set(&mut self, index: usize, (_, value): &(Option<&str>, &str)) -> Result<(), SetError> {
        match index {
            0 => {
                self.0 .0 = value
                    .parse::<T1>()
                    .map_err(|err| SetError::ValueParseFailed(err.to_string()))?;
                Ok(())
            }
            1 => {
                self.1 .0 = value
                    .parse::<T2>()
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
                    .parse::<T1>()
                    .map_err(|err| SetError::ValueParseFailed(err.to_string()))?;
                Ok(())
            }
            1 => {
                let _ = value
                    .parse::<T2>()
                    .map_err(|err| SetError::ValueParseFailed(err.to_string()))?;
                Ok(())
            }
            _ => panic!(),
        }
    }
}

impl<T1, T2, T3> PathParams for (PathParam<T1>, PathParam<T2>, PathParam<T3>)
where
    T1: FromStr + Display + Clone + Default + Send + Sync + 'static,
    <T1 as FromStr>::Err: Display,
    T2: FromStr + Display + Clone + Default + Send + Sync + 'static,
    <T2 as FromStr>::Err: Display,
    T3: FromStr + Display + Clone + Default + Send + Sync + 'static,
    <T3 as FromStr>::Err: Display,
{
    fn size(&self) -> usize {
        3
    }
    fn get(&self, index: usize) -> (Option<&'static str>, String) {
        match index {
            0 => (None, self.0 .0.to_string()),
            1 => (None, self.1 .0.to_string()),
            2 => (None, self.2 .0.to_string()),
            _ => panic!(),
        }
    }
    fn set(&mut self, index: usize, (_, value): &(Option<&str>, &str)) -> Result<(), SetError> {
        match index {
            0 => {
                self.0 .0 = value
                    .parse::<T1>()
                    .map_err(|err| SetError::ValueParseFailed(err.to_string()))?;
                Ok(())
            }
            1 => {
                self.1 .0 = value
                    .parse::<T2>()
                    .map_err(|err| SetError::ValueParseFailed(err.to_string()))?;
                Ok(())
            }
            2 => {
                self.2 .0 = value
                    .parse::<T3>()
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
                    .parse::<T1>()
                    .map_err(|err| SetError::ValueParseFailed(err.to_string()))?;
                Ok(())
            }
            1 => {
                let _ = value
                    .parse::<T2>()
                    .map_err(|err| SetError::ValueParseFailed(err.to_string()))?;
                Ok(())
            }
            2 => {
                let _ = value
                    .parse::<T3>()
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
        let mut path_params: PathParam<usize> = Default::default();
        path_params
            .set_from_info(&PathParamsInfo(vec![(None, "1".to_string())]))
            .unwrap();
        assert_eq!(path_params, PathParam(1));
        assert_eq!(path_params.info(), vec![(None, "1".to_string())].into());
        assert!(path_params.verify(0, &(None, "1")).is_ok());

        //
        let mut path_params: (PathParam<usize>, PathParam<String>) = Default::default();
        path_params
            .set_from_info(&PathParamsInfo(vec![
                (None, "1".to_string()),
                (None, "foo".to_string()),
            ]))
            .unwrap();
        assert_eq!(path_params, (PathParam(1), PathParam("foo".to_string())));
        assert_eq!(
            path_params.info(),
            vec![(None, "1".to_string()), (None, "foo".to_string())].into()
        );
        assert!(path_params.verify(0, &(None, "1")).is_ok());
        assert!(path_params.verify(1, &(None, "foo")).is_ok());

        //
        let mut path_params: (PathParam<usize>, PathParam<String>, PathParam<bool>) =
            Default::default();
        path_params
            .set_from_info(&PathParamsInfo(vec![
                (None, "1".to_string()),
                (None, "foo".to_string()),
                (None, "true".to_string()),
            ]))
            .unwrap();
        assert_eq!(
            path_params,
            (PathParam(1), PathParam("foo".to_string()), PathParam(true))
        );
        assert_eq!(
            path_params.info(),
            vec![
                (None, "1".to_string()),
                (None, "foo".to_string()),
                (None, "true".to_string())
            ]
            .into()
        );
        assert!(path_params.verify(0, &(None, "1")).is_ok());
        assert!(path_params.verify(1, &(None, "foo")).is_ok());
        assert!(path_params.verify(2, &(None, "true")).is_ok());
    }
}
