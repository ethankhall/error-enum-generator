//! # error-enum-generator
//!
//! This tool is used to automatically generate error codes, and messages
//! for an enum. The major intent of this is to make error in the CLI
//! easier to generate.

#[allow(unused_imports)]
#[macro_use]
extern crate error_enum_macros;

#[doc(hidden)]
pub use error_enum_macros::{ErrorContainer, ErrorEnum};

pub trait PrettyError: std::fmt::Display {
    fn get_error_code(&self) -> &str;

    fn description(&self) -> &str;
}
