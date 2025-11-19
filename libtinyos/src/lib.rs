#![no_std]
#![allow(unused_imports)]
#![feature(unsafe_cell_access, ptr_metadata)]

extern crate alloc;

/// cbindgen:ignore
pub(crate) mod arch;
pub(crate) mod c_api;
/// cbindgen:ignore
pub(crate) mod internal;
/// cbindgen:ignore
pub mod syscalls;

pub use c_api::*;

pub use crate::internal::alloc as tiny_alloc;
pub use crate::internal::{collections, fs, io, path, process, sync, thread, time, utils};

#[panic_handler]
fn lib_panic(info: &core::panic::PanicInfo) -> ! {
    eprintln!("{}", info);
    eprintln!("exiting...");
    unsafe { syscalls::exit(2) }
}
