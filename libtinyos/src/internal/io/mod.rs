use crate::syscalls::{self, FileDescriptor, SysResult};

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}

#[cfg(feature = "alloc")]
#[macro_export]
macro_rules! print {
    () => {};
    ($($arg:tt)*) => {
        $crate::io::print_str($crate::syscalls::STDOUT_FILENO, &alloc::format!($($arg)*)).unwrap()
    };
}

#[cfg(not(feature = "alloc"))]
#[macro_export]
macro_rules! print {
    () => {};
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! eprintln {
    () => {
        $crate::eprint!("\n")
    };
    ($($arg:tt)*) => {
        $crate::eprint!("{}\n", format_args!($($arg)*))
    };
}

#[cfg(feature = "alloc")]
#[macro_export]
macro_rules! eprint {
    () => {};
    ($($arg:tt)*) => {
        $crate::io::print_str($crate::syscalls::STDERR_FILENO, &alloc::format!($($arg)*)).unwrap()
    };
}

#[cfg(not(feature = "alloc"))]
#[macro_export]
macro_rules! eprint {
    () => {};
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! serial_println {
    () => {
        $crate::serial_print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::serial_print!("{}\n", format_args!($($arg)*))
    };
}

#[cfg(feature = "alloc")]
#[macro_export]
macro_rules! serial_print {
    () => {};
    ($($arg:tt)*) => {
        $crate::io::dbg_print_str(&alloc::format!("\x1b[96m[USRINFO]\x1b[0m {}", format_args!($($arg)*))).unwrap()
    };
}

#[cfg(not(feature = "alloc"))]
#[macro_export]
macro_rules! serial_print {
    () => {};
    ($($arg:tt)*) => {};
}

pub fn dbg_print_str(s: &str) -> SysResult<()> {
    let ptr = s.as_ptr();
    let len = s.len();
    unsafe { syscalls::dbg(ptr, len) }
}

pub fn print_str(fd: FileDescriptor, s: &str) -> SysResult<isize> {
    let ptr = s.as_ptr();
    let len = s.len();
    unsafe { syscalls::write(fd, ptr, len) }
}
