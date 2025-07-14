#![no_std]
pub(crate) mod syscalls;
pub use syscalls::funcs::*;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[panic_handler]
fn lib_panic(info: &core::panic::PanicInfo) -> ! {
    exit(2)
}
