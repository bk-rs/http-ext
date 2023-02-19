use core::cmp::PartialEq;

//
#[derive(Clone)]
pub struct HandlerData<T>(pub T);

impl<T> Default for HandlerData<T>
where
    T: Default,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> core::fmt::Debug for HandlerData<T>
where
    T: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HandlerData").field(&self.0).finish()
    }
}

impl<T> PartialEq for HandlerData<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
