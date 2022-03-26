//! [The 'Basic' HTTP Authentication Scheme](https://www.rfc-editor.org/rfc/rfc7617.html)

use alloc::boxed::Box;

//
pub const NAME: &str = "Basic";

//
#[derive(Debug, Clone)]
pub struct Credentials {
    pub user_id: Box<str>,
    pub password: Box<str>,
}
