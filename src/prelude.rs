//! Crate prelude

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;
pub type ResultS<T> = core::result::Result<T, &'static str>;


// Generic Wrapper tuple struct
pub struct W<T>(pub T);

// Personal preference
pub use std::format as f;