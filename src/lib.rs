#![no_std]

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[panic_handler]
fn lib_panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}
