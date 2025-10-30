use crate::{
    syscall,
    syscalls::{FileDescriptor, OpenOptions, PageTableFlags, SysCallDispatch, SysResult},
};

pub unsafe fn exit(status: i64) -> ! {
    unsafe { syscall!(SysCallDispatch::Exit as u64, status) }.unwrap();
    unreachable!()
}

pub unsafe fn kill(id: u64, status: i64) -> SysResult<()> {
    unsafe { syscall!(SysCallDispatch::Kill as u64, id, status) }.map(|_| ())
}

pub unsafe fn open(path: *const u8, len: usize, flags: OpenOptions) -> SysResult<FileDescriptor> {
    unsafe { syscall!(SysCallDispatch::Open as u64, path, len, flags.bits()) }
        .map(|r| r as FileDescriptor)
}

pub unsafe fn seek(fd: FileDescriptor, offset: usize) -> SysResult<()> {
    unsafe { syscall!(SysCallDispatch::Seek as u64, fd, offset) }.map(|_| ())
}

pub unsafe fn dup(old: FileDescriptor, new: Option<FileDescriptor>) -> SysResult<FileDescriptor> {
    let new_ = new.map(|fd| fd as i32).unwrap_or(-1);
    unsafe { syscall!(SysCallDispatch::Dup as u64, old, new_) }.map(|r| r as FileDescriptor)
}

pub unsafe fn write(fd: FileDescriptor, buf: *const u8, len: usize) -> SysResult<isize> {
    unsafe { syscall!(SysCallDispatch::Write as u64, fd, buf, len) }.map(|r| r as isize)
}

pub unsafe fn read(
    fd: FileDescriptor,
    buf: *mut u8,
    len: usize,
    timeout: usize,
) -> SysResult<isize> {
    unsafe { syscall!(SysCallDispatch::Read as u64, fd, buf, len, timeout) }.map(|r| r as isize)
}

pub unsafe fn yield_now() {
    _ = unsafe { syscall!(SysCallDispatch::Yield as u64) };
}

pub unsafe fn mmap(
    len: usize,
    ptr: *mut u8,
    flags: PageTableFlags,
    fd: Option<FileDescriptor>,
) -> SysResult<*mut u8> {
    unsafe {
        syscall!(
            SysCallDispatch::Mmap as u64,
            len,
            ptr,
            flags.bits(),
            fd.map(|f| f as i32).unwrap_or(-1)
        )
    }
    .map(|r| r as *mut u8)
}

pub unsafe fn munmap(ptr: *mut u8, len: usize) {
    unsafe { syscall!(SysCallDispatch::Munmap as u64, ptr, len) };
}

pub unsafe fn clone() -> SysResult<bool> {
    unsafe { syscall!(SysCallDispatch::Clone as u64) }.map(|r| if r == 0 { false } else { true })
}

pub unsafe fn get_pid() -> SysResult<u64> {
    unsafe { syscall!(SysCallDispatch::GetPid as u64) }
}

pub unsafe fn chg_machine_state() {
    unsafe { syscall!(SysCallDispatch::Machine as u64) };
}

pub unsafe fn spawn(elf: *const u8, len: usize) -> SysResult<()> {
    unsafe { syscall!(SysCallDispatch::Spawn as u64, elf, len) }.map(|_| ())
}

pub unsafe fn dbg(buf: *const u8, len: usize) -> SysResult<()> {
    unsafe { syscall!(SysCallDispatch::Dbg as u64, buf, len) }.map(|_| ())
}
