#![cfg_attr(not(feature = "std"), no_std)]

pub use cloneable_any::CloneableAnySync as HandlerDataTrait;

//
pub mod handler_data;

pub use handler_data::HandlerData;
