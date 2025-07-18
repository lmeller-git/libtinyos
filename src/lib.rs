#![no_std]
#![allow(unused_imports)]

extern crate alloc;

/// cbindgen:ignore
pub(crate) mod arch;
pub(crate) mod c_api;
/// cbindgen:ignore
pub(crate) mod internal;
/// cbindgen:ignore
pub(crate) mod syscalls;

pub use c_api::*;

pub use crate::internal::alloc as tiny_alloc;
pub use crate::internal::{collections, fs, io, path, process, sync, thread, time};
pub use syscalls::funcs::*;

#[panic_handler]
fn lib_panic(info: &core::panic::PanicInfo) -> ! {
    eprintln!("{}", info);
    exit(2)
}
