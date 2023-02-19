use core::{
    cmp::{Eq, PartialEq},
    fmt::{Debug, Display},
    marker::Copy,
    str::FromStr,
};

//
#[derive(Debug, Clone, Default)]
pub struct PathParam<T>(pub T)
where
    T: FromStr + Display + Clone + Default;

impl<T> Copy for PathParam<T> where T: FromStr + Display + Clone + Default + Copy {}

impl<T> PartialEq for PathParam<T>
where
    T: FromStr + Display + Clone + Default + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for PathParam<T> where T: FromStr + Display + Clone + Default + Eq {}
