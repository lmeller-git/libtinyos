use bitflags::bitflags;

mod funcs;
pub use funcs::*;
pub use x86_64::structures::paging::PageTableFlags;

#[repr(u64)]
pub enum SysCallDispatch {
    Open = 0,
    Close = 1,
    Read = 2,
    Write = 3,
    Yield = 4,
    Exit = 5,
    Kill = 6,
    Mmap = 7,
    Munmap = 8,
    Clone = 9,
    Wait = 10,
    Machine = 11,
    GetPid = 12,
    Seek = 13,
    Dup = 14,
    Spawn = 15,
    Dbg = 16,
    Execve = 17,
    PThreadCreate = 18,
    PThreadExit = 19,
    PThreadCancel = 20,
    PThreadJoin = 21,
}

pub type FileDescriptor = u32;

pub const STDIN_FILENO: FileDescriptor = 0;
pub const STDOUT_FILENO: FileDescriptor = 1;
pub const STDERR_FILENO: FileDescriptor = 2;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct OpenOptions: u32 {
        const READ = 1 << 0;
        const WRITE = 1 << 1;
        const APPEND = 1 << 2;
        const TRUNCATE = 1 << 3;
        const CREATE = 1 << 4;
        const CREATE_DIR = 1 << 5;
        const CREATE_ALL = 1 << 6;
        const CREATE_LINK = 1 << 7;
        const NO_FOLLOW_LINK = 1 << 8;
    }
}

impl OpenOptions {
    pub fn with_read(self) -> Self {
        self | Self::READ
    }

    pub fn with_write(self) -> Self {
        self | Self::WRITE
    }

    pub fn with_no_follow_symlink(self) -> Self {
        self | Self::NO_FOLLOW_LINK
    }

    pub fn with_truncate(self) -> Self {
        self | Self::TRUNCATE
    }

    pub fn with_append(self) -> Self {
        self | Self::APPEND
    }
}

impl Default for OpenOptions {
    fn default() -> Self {
        Self::READ
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct UnlinkOptions: u32 {
        const FORCE = 1 << 0;
        const RECURSIVE = 1 << 1;
        const NO_PRESERVE_ROOT = 1 << 2;
    }
}

impl UnlinkOptions {
    fn with_force(self) -> Self {
        self | Self::FORCE
    }

    fn with_rmdir(self) -> Self {
        self | Self::RECURSIVE
    }
}

impl Default for UnlinkOptions {
    fn default() -> Self {
        Self::empty()
    }
}

pub type SysResult<T> = Result<T, SysResultCode>;

#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysResultCode {
    Fail = -1,
    Success = 0,
}

impl TryFrom<i64> for SysResultCode {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Success),
            -1 => Ok(Self::Fail),
            _ => Err(value),
        }
    }
}

pub fn as_res(r1: u64, r2: i64) -> SysResult<u64> {
    // in case of success r2 is guaranteed to be == SysResultCode::Success
    if r2 != SysResultCode::Success as i64 {
        return Err(r2.try_into().unwrap_or(SysResultCode::Fail));
    }
    Ok(r1)
}

#[macro_export]
macro_rules! syscall {
    ($rax:expr) => {{
        let ret: u64;
        let ret2: u64;

            core::arch::asm!(
                "int 0x80",
                in("rax") $rax,
                lateout("rax") ret,
                lateout("rdx") ret2
            );

        $crate::syscalls::as_res(ret, ret2 as i64)
    }};

    ($rax:expr, $rdi:expr) => {{
        let ret: u64;
        let ret2: u64;

            core::arch::asm!(
                "int 0x80",
                in("rax") $rax,
                in("rdi") $rdi,
                lateout("rax") ret,
                lateout("rdx") ret2
            );

        $crate::syscalls::as_res(ret, ret2 as i64)
    }};

    ($rax:expr, $rdi:expr, $rsi:expr) => {{
        let ret: u64;
        let ret2: u64;

            core::arch::asm!(
                "int 0x80",
                in("rax") $rax,
                in("rdi") $rdi,
                in("rsi") $rsi,
                lateout("rax") ret,
                lateout("rdx") ret2
            );

        $crate::syscalls::as_res(ret, ret2 as i64)
    }};

    ($rax:expr, $rdi:expr, $rsi:expr, $rdx:expr) => {{
        let ret: u64;
        let ret2: u64;

            core::arch::asm!(
                "int 0x80",
                in("rax") $rax,
                in("rdi") $rdi,
                in("rsi") $rsi,
                in("rdx") $rdx,
                lateout("rax") ret,
                lateout("rdx") ret2
            );

        $crate::syscalls::as_res(ret, ret2 as i64)
    }};

    ($rax:expr, $rdi:expr, $rsi:expr, $rdx:expr, $r10:expr) => {{
        let ret: u64;
        let ret2: u64;

            core::arch::asm!(
                "int 0x80",
                in("rax") $rax,
                in("rdi") $rdi,
                in("rsi") $rsi,
                in("rdx") $rdx,
                in("r10") $r10,
                lateout("rax") ret,
                lateout("rdx") ret2
            );

        $crate::syscalls::as_res(ret, ret2 as i64)
    }};

    ($rax:expr, $rdi:expr, $rsi:expr, $rdx:expr, $r10:expr, $r9:expr) => {{
        let ret: u64;
        let ret2: u64;

            core::arch::asm!(
                "int 0x80",
                in("rax") $rax,
                in("rdi") $rdi,
                in("rsi") $rsi,
                in("rdx") $rdx,
                in("r10") $r10,
                in("r9") $r9,
                lateout("rax") ret,
                lateout("rdx") ret2
            );

        $crate::syscalls::as_res(ret, ret2 as i64)
    }};

    ($rax:expr, $rdi:expr, $rsi:expr, $rdx:expr, $r10:expr, $r9:expr, $r8:expr) => {{
        let ret: u64;
        let ret2: u64;

            core::arch::asm!(
                "int 0x80",
                in("rax") $rax,
                in("rdi") $rdi,
                in("rsi") $rsi,
                in("rdx") $rdx,
                in("r10") $r10,
                in("r9") $r9,
                in("r8") $r8,
                lateout("rax") ret,
                lateout("rdx") ret2
            );

        $crate::syscalls::as_res(ret, ret2 as i64)
    }};
}
