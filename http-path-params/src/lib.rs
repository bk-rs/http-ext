#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

//
pub mod path_param;
pub mod path_params;
pub mod path_params_info;

pub use path_param::PathParam;
pub use path_params::PathParams;
pub use path_params_info::PathParamsInfo;

//
pub mod id;
pub mod username;

pub use id::Id;
pub use username::Username;
