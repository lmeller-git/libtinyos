use crate::syscall;

pub fn exit(status: u64) -> ! {
    unsafe { syscall!(1, status) };
    unreachable!()
}

pub fn kill() {
    todo!()
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {$crate::print!("{}\n", format_args!($($arg)*))};
}

#[macro_export]
macro_rules! print {
    () => {};
    ($($arg:tt)*) => {$crate::print_str($crate::stdout(), &alloc::format!($($arg)*))};
}

#[macro_export]
macro_rules! eprintln {
    () => {
        $crate::eprint!("\n")
    };
    ($($arg:tt)*) => {$crate::eprint!("{}\n", format_args!($($arg)*))};
}

#[macro_export]
macro_rules! eprint {
    () => {};
    ($($arg:tt)*) => {$crate::print_str($crate::stderr(), &alloc::format!($($arg)*))};
}

pub fn print_str(handle: usize, s: &str) -> isize {
    let ptr = s.as_ptr();
    let len = s.len();
    write(handle, ptr, len)
}

pub fn write(handle: usize, buf: *const u8, len: usize) -> isize {
    let r = unsafe { syscall!(4, handle, buf, len) };
    r.1 as isize
}

pub fn read(handle: usize, buf: *mut u8, len: usize) -> isize {
    let r = unsafe { syscall!(6, handle, buf, len) };
    r.1 as isize
}

pub fn stdout() -> usize {
    1
}

pub fn stderr() -> usize {
    2
}

pub fn stdin() -> usize {
    0
}

pub fn graphics() -> usize {
    4
}

pub fn request_heap(size: usize) -> *mut u8 {
    let r = unsafe { syscall!(7, size as u64) };
    r.1 as *mut u8
}

pub fn yield_now() -> i64 {
    let r = unsafe { syscall!(3) };
    r.0 as i64
}

pub fn map_device(device_type: usize, addr: &mut *mut ()) {
    let (r, r2) = unsafe { syscall!(8, device_type, *addr) };
    if (r as i64) < 0 {
        panic!("mmap failed");
    }
    *addr = r2 as usize as *mut ();
}

pub enum SysRes {
    Fail,
}
