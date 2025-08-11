#![no_std]
#![allow(unused_imports, dead_code)]

extern crate alloc;

pub(crate) mod c_api;
/// cbindgen:ignore
pub(crate) mod internal;

use core::fmt::Display;

pub use c_api::*;
pub use internal::{backend, repr::*, utils};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error, Default)]
pub enum GraphicsError {
    #[default]
    Unknown,
}

impl Display for GraphicsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Unknown graphics error")
    }
}
