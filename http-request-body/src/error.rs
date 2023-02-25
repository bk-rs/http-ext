//
#[derive(Debug)]
pub enum Error {
    Other(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}
