use core::{array, ptr};

pub(crate) unsafe fn memset<T: Copy>(dest: *mut T, len: usize, val: T) {
    let arr = ptr::slice_from_raw_parts_mut(dest, len);
    unsafe {
        (*arr).fill(val);
    }
}
