use core::ffi::{CStr, c_char};

use crate::{stdout, syscalls};

pub mod abi;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __print(buf: *const c_char) {
    let buf = unsafe { CStr::from_ptr(buf) };
    let buf = buf.to_string_lossy();
    let len = buf.len();
    let buf = buf.as_ptr();
    __c_write(stdout(), buf, len);
}

#[unsafe(no_mangle)]
pub extern "C" fn __c_write(handle: usize, buf: *const u8, len: usize) -> isize {
    syscalls::funcs::write(handle, buf, len)
}

#[unsafe(no_mangle)]
pub extern "C" fn __c_exit(status: u64) -> ! {
    syscalls::funcs::exit(status)
}

#[unsafe(no_mangle)]
pub extern "C" fn __c_read(handle: usize, buf: *mut u8, len: usize) -> isize {
    todo!()
}

#[unsafe(no_mangle)]
pub extern "C" fn __c_yield() {
    todo!()
}

#[unsafe(no_mangle)]
pub extern "C" fn __c_heap(size: usize) -> *mut u8 {
    todo!()
}
