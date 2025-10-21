use crate::syscalls::{self, FileDescriptor, SysResult};

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
    ($($arg:tt)*) => {$crate::io::print_str($crate::syscalls::STDOUT_FILENO, &alloc::format!($($arg)*)).unwrap()};
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
    ($($arg:tt)*) => {$crate::io::print_str($crate::syscalls::STDERR_FILENO, &alloc::format!($($arg)*)).unwrap()};
}

pub fn print_str(fd: FileDescriptor, s: &str) -> SysResult<isize> {
    let ptr = s.as_ptr();
    let len = s.len();
    unsafe { syscalls::write(fd, ptr, len) }
}
