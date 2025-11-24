use tinyos_abi::{
    flags::{TaskStateChange, TaskWaitOptions, WaitOptions},
    types::{FromSyscall, SysErrCode, SysResult},
};

use crate::{
    syscall,
    syscalls::{FileDescriptor, OpenOptions, PageTableFlags, SysCallDispatch},
};

pub unsafe fn exit(status: i64) -> ! {
    unsafe { syscall!(SysCallDispatch::Exit as u64, status) };
    unreachable!()
}

pub unsafe fn kill(pid: u64, status: i64) -> SysResult<()> {
    let (rax, rdx) = unsafe { syscall!(SysCallDispatch::Kill as u64, pid, status) };
    SysResult::<u64>::parse_from(rax, rdx).map(|_| ())
}

pub unsafe fn open(path: *const u8, len: usize, flags: OpenOptions) -> SysResult<FileDescriptor> {
    let (rax, rdx) = unsafe { syscall!(SysCallDispatch::Open as u64, path, len, flags.bits()) };
    SysResult::parse_from(rax, rdx)
}

pub unsafe fn close(fd: FileDescriptor) -> SysResult<()> {
    let (rax, rdx) = unsafe { syscall!(SysCallDispatch::Close as u64, fd) };
    SysResult::<u64>::parse_from(rax, rdx).map(|_| ())
}

pub unsafe fn seek(fd: FileDescriptor, offset: usize) -> SysResult<()> {
    let (rax, rdx) = unsafe { syscall!(SysCallDispatch::Seek as u64, fd, offset) };
    SysResult::<u64>::parse_from(rax, rdx).map(|_| ())
}

pub unsafe fn dup(old: FileDescriptor, new: Option<FileDescriptor>) -> SysResult<FileDescriptor> {
    let new_ = new.map(|fd| fd as i32).unwrap_or(-1);
    let (rax, rdx) = unsafe { syscall!(SysCallDispatch::Dup as u64, old, new_) };
    SysResult::parse_from(rax, rdx)
}

pub unsafe fn write(fd: FileDescriptor, buf: *const u8, len: usize) -> SysResult<isize> {
    let (rax, rdx) = unsafe { syscall!(SysCallDispatch::Write as u64, fd, buf, len) };
    SysResult::parse_from(rax, rdx)
}

pub unsafe fn read(
    fd: FileDescriptor,
    buf: *mut u8,
    len: usize,
    timeout: usize,
) -> SysResult<isize> {
    let (rax, rdx) = unsafe { syscall!(SysCallDispatch::Read as u64, fd, buf, len, timeout) };
    SysResult::parse_from(rax, rdx)
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
    let (rax, rdx) = unsafe {
        syscall!(
            SysCallDispatch::Mmap as u64,
            len,
            ptr,
            flags.bits(),
            fd.map(|f| f as i32).unwrap_or(-1)
        )
    };
    SysResult::<u64>::parse_from(rax, rdx).map(|r| r as *mut u8)
}

pub unsafe fn munmap(ptr: *mut u8, len: usize) {
    let (_rax, _rdx) = unsafe { syscall!(SysCallDispatch::Munmap as u64, ptr, len) };
}

pub unsafe fn fork() -> SysResult<bool> {
    let (rax, rdx) = unsafe { syscall!(SysCallDispatch::Fork as u64) };
    SysResult::<u64>::parse_from(rax, rdx).map(|r| if r > 0 { true } else { false })
}

pub unsafe fn get_pid() -> SysResult<u64> {
    let (rax, rdx) = unsafe { syscall!(SysCallDispatch::GetPID as u64) };
    SysResult::parse_from(rax, rdx)
}

pub unsafe fn spawn(elf: *const u8, len: usize) -> SysResult<()> {
    let (rax, rdx) = unsafe { syscall!(SysCallDispatch::Spawn as u64, elf, len) };
    SysResult::<u64>::parse_from(rax, rdx).map(|_| ())
}

pub unsafe fn dbg(buf: *const u8, len: usize) -> SysResult<()> {
    let (rax, rdx) = unsafe { syscall!(SysCallDispatch::Dbg as u64, buf, len) };
    SysResult::<u64>::parse_from(rax, rdx).map(|_| ())
}

pub unsafe fn execve(
    path: *const u8,
    len: usize,
    argc: *const u8,
    argc_size: usize,
    argv: *const u8,
    argv_size: usize,
) -> SysResult<u64> {
    let (rax, rdx) = unsafe {
        syscall!(
            SysCallDispatch::Execve as u64,
            path,
            len,
            argc,
            argc_size,
            argv,
            argv_size
        )
    };
    SysResult::parse_from(rax, rdx)
}

pub unsafe fn thread_create(start_routine: *const (), args: *const ()) -> SysResult<u64> {
    let (rax, rdx) = unsafe { syscall!(SysCallDispatch::ThreadCreate as u64, start_routine, args) };
    SysResult::parse_from(rax, rdx)
}

pub unsafe fn thread_exit() -> ! {
    _ = unsafe { syscall!(SysCallDispatch::ThreadExit as u64) };
    unreachable!()
}

pub unsafe fn thread_cancel(tid: u64) -> SysResult<i64> {
    let (rax, rdx) = unsafe { syscall!(SysCallDispatch::ThreadCancel as u64, tid) };
    SysResult::parse_from(rax, rdx)
}

pub unsafe fn thread_join(
    tid: u64,
    timeout: i64,
    w_flags: WaitOptions,
    tw_flags: TaskWaitOptions,
) -> SysResult<TaskStateChange> {
    let (rax, rdx) = unsafe {
        syscall!(
            SysCallDispatch::ThreadJoin as u64,
            tid,
            timeout,
            w_flags.bits(),
            tw_flags.bits()
        )
    };
    SysResult::<u64>::parse_from(rax, rdx).map(|r| TaskStateChange::from_bits_truncate(r as u16))
}

pub unsafe fn wait_pid(
    pid: u64,
    timeout: i64,
    w_flags: WaitOptions,
    tw_flags: TaskWaitOptions,
) -> SysResult<TaskStateChange> {
    let (rax, rdx) = unsafe {
        syscall!(
            SysCallDispatch::WaitPID as u64,
            pid,
            timeout,
            w_flags.bits(),
            tw_flags.bits()
        )
    };
    SysResult::<u64>::parse_from(rax, rdx).map(|r| TaskStateChange::from_bits_truncate(r as u16))
}

pub unsafe fn eventfd() -> SysResult<u64> {
    let (rax, rdx) = unsafe { syscall!(SysCallDispatch::EventFD as u64) };
    SysResult::parse_from(rax, rdx)
}

pub unsafe fn waittime(time: u64) -> SysResult<()> {
    let (rax, rdx) = unsafe { syscall!(SysCallDispatch::WaitTime as u64, time) };
    SysResult::<u64>::parse_from(rax, rdx).map(|_| ())
}

pub unsafe fn time() -> SysResult<u64> {
    let (rax, rdx) = unsafe { syscall!(SysCallDispatch::Time as u64) };
    SysResult::parse_from(rax, rdx)
}

pub unsafe fn get_tid() -> u64 {
    let (rax, rdx) = unsafe { syscall!(SysCallDispatch::GetTID as u64) };
    SysResult::parse_from(rax, rdx).unwrap()
}

pub unsafe fn get_pgrid() -> u64 {
    let (rax, rdx) = unsafe { syscall!(SysCallDispatch::GetPgrID as u64) };
    SysResult::parse_from(rax, rdx).unwrap()
}

pub unsafe fn pipe(fds: *mut [u32; 2]) -> SysResult<()> {
    let (rax, rdx) = unsafe { syscall!(SysCallDispatch::Pipe as u64, fds as u64) };
    SysResult::<u64>::parse_from(rax, rdx).map(|_| ())
}
