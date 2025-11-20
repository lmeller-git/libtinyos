#![no_std]
#![allow(unused_imports)]
#![feature(unsafe_cell_access, ptr_metadata)]

#[cfg(feature = "alloc")]
extern crate alloc;

/// cbindgen:ignore
pub(crate) mod arch;
pub(crate) mod c_api;
/// cbindgen:ignore
pub(crate) mod internal;
/// cbindgen:ignore
pub mod syscalls;

#[cfg(feature = "alloc")]
pub use crate::internal::alloc as tiny_alloc;
pub use crate::internal::{collections, fs, io, path, process, sync, thread, time, utils};
pub use c_api::*;

#[panic_handler]
fn lib_panic(info: &core::panic::PanicInfo) -> ! {
    #[cfg(feature = "alloc")]
    {
        eprintln!("{}", info);
        eprintln!("exiting...");
    }
    unsafe { syscalls::exit(2) }
}
