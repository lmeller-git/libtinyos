use core::{
    alloc::{GlobalAlloc, Layout},
    ffi::{CStr, c_char},
    mem,
};

use crate::{
    syscalls::{self, STDOUT_FILENO},
    tiny_alloc,
};

pub mod abi;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __print(buf: *const c_char) {
    let buf = unsafe { CStr::from_ptr(buf) };
    let buf = buf.to_string_lossy();
    let len = buf.len();
    let buf = buf.as_ptr();
    unsafe { __c_write(STDOUT_FILENO, buf, len) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __c_write(handle: u32, buf: *const u8, len: usize) -> isize {
    let r = unsafe { syscalls::write(handle, buf, len) };
    if let Ok(res) = r { res } else { -1 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __c_exit(status: i64) -> ! {
    unsafe { syscalls::exit(status) }
}

#[unsafe(no_mangle)]
pub extern "C" fn __c_read(handle: u32, buf: *mut u8, len: usize, timeout: usize) -> isize {
    let r = unsafe { syscalls::read(handle, buf, len, timeout) };
    if let Ok(res) = r { res } else { -1 }
}

#[unsafe(no_mangle)]
pub extern "C" fn __c_yield() {
    unsafe { syscalls::yield_now() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
    let layout = Layout::from_size_align(size, mem::align_of::<usize>()).unwrap();
    unsafe { tiny_alloc::GLOBAL_ALLOC.alloc(layout) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free(ptr: *mut u8) {
    // TODO need to keep a map of ptr -> layout to deallocate the correct chunk
    todo!()
}
